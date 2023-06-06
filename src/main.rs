use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde_derive::Deserialize;
use ssh2::Session;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::error::Error;
use std::sync::{Arc, Mutex};

#[derive(Debug, Deserialize)]
struct Config {
    hosts: Vec<Host>,
    packages: Packages,
    users: Users,
    services: Services,
    repositories: Repositories,
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

#[derive(Debug, Deserialize, Clone)] // Added Clone trait
struct Packages {
    global: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)] // Added Clone trait
struct Users {
    global: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)] // Added Clone trait
struct Services {
    enable: Vec<String>,
    restart: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct Repositories {
    global: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config("config.toml");

    let m = MultiProgress::new();
    let progress_bars: Vec<Arc<Mutex<ProgressBar>>> = config.hosts.iter().map(|host| {
        let pb = m.add(ProgressBar::new(1));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );
        pb.set_message(format!("Pushing to {}", host.name).to_owned());
        Arc::new(Mutex::new(pb))
    }).collect();

    let message = "Linux Fleet Manager - Working...".to_string();
    // Set the message on the first progress bar only
    progress_bars[0].lock().unwrap().set_message(message);

    let packages_ref = Arc::new(config.packages);
    let users_ref = Arc::new(config.users);
    let services_ref = Arc::new(config.services);
    let repos_ref = Arc::new(config.repositories);   

    let handles: Vec<_> = config
        .hosts
        .into_iter()
        .enumerate()
        .map(|(i, host)| {
            let host_name = host.name.clone();
            let host_config = host.clone();
            let pb = Arc::clone(&progress_bars[i]);
        
            let packages_ref = Arc::clone(&packages_ref);
            let users_ref = Arc::clone(&users_ref);
            let services_ref = Arc::clone(&services_ref);
            let repos_ref = Arc::clone(&repos_ref);
        
            tokio::spawn(async move {
                match connect_to_host(&host_config).await {
                    Ok(session) => {
                        manage_packages(&session, &packages_ref.global);
                        manage_users(&session, &users_ref.global);
                        manage_services(&session, &services_ref.enable, &services_ref.restart);
                        manage_repositories(&session, &repos_ref.global);
                    }
                    Err(e) => {
                        eprintln!("Error connecting to host {}: {}", host_name, e);
                    }
                }
                pb.lock().unwrap().finish_with_message(format!("Done with {}", host_name).to_owned());
                drop(pb);
            })
        })        
        .collect();

    for handle in handles {
        handle.await.unwrap();
    }

    println!("Done with all hosts!");
    Ok(())
}

async fn connect_to_host(host: &Host) -> Result<Session, Box<dyn Error>> {
    let tcp = tokio::net::TcpStream::connect((host.address.as_str(), host.port)).await?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    if let Some(key_path) = &host.key_path {
        if !key_path.is_empty() {
            let key_path = Path::new(key_path);
            session
                .userauth_pubkey_file(&host.user, None, key_path, None)?;
        } else {
            session.userauth_password(&host.user, &host.password)?;
        }
    } else {
        session.userauth_password(&host.user, &host.password)?;
    }

    assert!(session.authenticated());
    Ok(session)
}

fn manage_packages(session: &Session, packages: &[String]) {
    let package_list = packages.join(" ");
    let command = format!(
        "sudo apt-get update && sudo apt-get install -y {}",
        package_list
    );
    if let Err(e) = execute_command(session, &command) {
        eprintln!("Error executing command '{}': {}", command, e);
    }
}

fn manage_users(session: &Session, users: &[String]) {
    for user in users {
        let command = format!("sudo adduser --disabled-password --gecos '' {}", user);
        if let Err(e) = execute_command(session, &command) {
            eprintln!("Error executing command '{}': {}", command, e);
        }
    }
}

fn manage_services(session: &Session, enable: &[String], restart: &[String]) {
    for service in enable {
        let command = format!("sudo systemctl enable {}", service);
        if let Err(e) = execute_command(session, &command) {
            eprintln!("Error executing command '{}': {}", command, e);
        }
    }

    for service in restart {
        let command = format!("sudo systemctl restart {}", service);
        if let Err(e) = execute_command(session, &command) {
            eprintln!("Error executing command '{}': {}", command, e);
        }
    }
}

fn manage_repositories(session: &Session, repos: &[String]) {
  for repo in repos {
      let command = format!("git clone {}", repo);
      if let Err(e) = execute_command(session, &command) {
          eprintln!("Error executing command '{}': {}", command, e);
      }
  }
}


fn execute_command(session: &Session, command: &str) -> Result<(), Box<dyn Error>> {
    let mut channel = session.channel_session()?;
    channel.exec(command)?;
    let mut response = String::new();
    channel.read_to_string(&mut response)?;
    // println!("Output: {}", response);
    channel.wait_close()?;
    // println!("Status: {}", channel.exit_status()?);
    Ok(())
}

fn read_config<P: AsRef<Path>>(path: P) -> Config {
    let mut file = File::open(path).expect("Config file not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Error reading config file");

    toml::from_str(&contents).expect("Error parsing config file")
}
