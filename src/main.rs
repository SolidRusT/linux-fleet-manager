use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use ssh2::Session;
use toml::Value;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    hosts: Vec<Host>,
    packages: Packages,
    users: Users,
    services: Services,
}

#[derive(Debug, Deserialize)]
struct Host {
    name: String,
    address: String,
    port: u16,
    user: String,
    password: String,
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

    for host in &config.hosts {
        println!("Connecting to {}", host.name);
        let session = connect_to_host(host).await;
        manage_packages(&session, &config.packages.global);
        manage_users(&session, &config.users.global);
        manage_services(&session, &config.services.enable, &config.services.restart);
    }
}

async fn connect_to_host(host: &Host) -> Session {
    let tcp = tokio::net::TcpStream::connect(format!("{}:{}", host.address, host.port)).await.unwrap();
    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake().unwrap();
    session.userauth_password(&host.user, &host.password).unwrap();
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
  let mut channel = session.shell().unwrap();
  channel.write_all(command.as_bytes()).unwrap();
  channel.send_eof().unwrap();
  channel.wait_close().unwrap();
  println!("Status: {}", channel.exit_status().unwrap());
  channel.close().unwrap();
  channel.wait_close().unwrap();
}

fn read_config<P: AsRef<Path>>(path: P) -> Config {
  let mut file = File::open(path).expect("Config file not found");
  let mut contents = String::new();
  file.read_to_string(&mut contents).expect("Error reading config file");

  toml::from_str(&contents).expect("Error parsing config file")
}


