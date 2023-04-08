use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::sync::Arc;
use ssh2::Session;
use serde_derive::Deserialize;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug, Deserialize)]
struct Config {
    hosts: Vec<Host>,
    packages: Packages,
    users: Users,
    services: Services,
}

#[derive(Debug, Deserialize, Clone)]
struct Host {
    name: String,
    address: String,
    port: u16,
    user: String,
    password: String,
    key_path: Option<String>,
}


#[derive(Debug, Deserialize)]
struct Packages {
    global: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Users {
    global: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Services {
    enable: Vec<String>,
    restart: Vec<String>,
}

#[tokio::main]
async fn main() {
    let config = read_config("config.toml");

    let handles: Vec<_> = config.hosts.iter().map(|host| {
        let host_name = host.name.clone();
        let host_config = host.clone();
        let packages = config.packages.global.clone();
        let users = config.users.global.clone();
        let enable = config.services.enable.clone();
        let restart = config.services.restart.clone();

        tokio::spawn(async move {
            println!("Connecting to {}", host_name);
            let session = connect_to_host(&host_config).await;
            manage_packages(&session, &packages);
            manage_users(&session, &users);
            manage_services(&session, &enable, &restart);
        })
    }).collect();

    futures::future::join_all(handles).await;

    println!("\nDone with all hosts!");
}

async fn connect_to_host(host: &Host) -> Session {
    let tcp = tokio::net::TcpStream::connect(format!("{}:{}", host.address, host.port)).await.unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();

    if let Some(key_path) = &host.key_path {
        if !key_path.is_empty() {
            let key_path = Path::new(key_path);
            session
                .userauth_pubkey_file(&host.user, None, key_path, None)
                .unwrap();
        } else {
            session.userauth_password(&host.user, &host.password).unwrap();
        }
    } else {
        session.userauth_password(&host.user, &host.password).unwrap();
    }

    assert!(session.authenticated());
    session
}

fn manage_packages(session: &Session, packages: &[String]) {
    let package_list = packages.join(" ");
    let command = format!("sudo apt-get update && sudo apt-get install -y {}", package_list);
    execute_command(session, &command);
}

fn manage_users(session: &Session, users: &[String]) {
    for user in users {
        let command = format!("sudo adduser --disabled-password --gecos '' {}", user);
        execute_command(session, &command);
    }
}

fn manage_services(session: &Session, enable: &[String], restart: &[String]) {
  for service in enable {
      let command = format!("sudo systemctl enable {}", service);
      execute_command(session, &command);
  }

  for service in restart {
      let command = format!("sudo systemctl restart {}", service);
      execute_command(session, &command);
  }
}

fn execute_command(session: &Session, command: &str) {
  let mut channel = session.channel_session().unwrap();
  channel.exec(command).unwrap();
  let mut response = String::new();
  channel.read_to_string(&mut response).unwrap();
  println!("Output: {}", response);
  channel.wait_close().unwrap();
  println!("Status: {}", channel.exit_status().unwrap());
}

fn read_config<P: AsRef<Path>>(path: P) -> Config {
  let mut file = File::open(path).expect("Config file not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Error reading config file");

  toml::from_str(&contents).expect("Error parsing config file")
}


