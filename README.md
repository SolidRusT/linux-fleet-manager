# 🐧 Linux Fleet Manager

Linux Fleet Manager is a robust and efficient tool for managing multiple remote Linux hosts concurrently. It allows you to automate the process of installing packages, creating users, managing repositories, and controlling services across a collection of Linux servers. Built with Rust, this tool leverages the concurrency capabilities of Tokio and SSH2 libraries to enable parallel execution on multiple hosts.

## 🌟 Features

- **Parallel Execution**: Manage multiple remote hosts simultaneously, speeding up the configuration time drastically when dealing with a fleet of servers.
- **Automated Package Installation**: Define global packages that need to be installed on all hosts.
- **User Management**: Specify users that need to be created across your servers.
- **Service Management**: Control services that need to be enabled or restarted across your fleet.
- **Repository Management**: Automate the cloning of git repositories across your servers.

## 🔧 Requirements

- Rust 1.53 or higher
- Cargo
- Access to target Linux hosts via SSH with appropriate credentials

## 📝 Configuration

Configure the Linux Fleet Manager by creating a `config.toml` file in the root directory of the project. This file should contain the following sections:

### Hosts

Define the remote hosts you want to manage:

```toml
[[hosts]]
name = "example-host"
address = "192.168.1.100"
port = 22
user = "user"
password = "password"
key_path = "/path/to/your/private/key"
```

### Packages

Define the global packages you want to install on all hosts:

```toml
[packages]
global = ["package1", "package2", "package3"]
```

### Users

Define the global users you want to create on all hosts:

```toml
[users]
global = ["user1", "user2", "user3"]
```

### Services

Define the services you want to enable and restart on all hosts:

```toml
[services]
enable = ["service1", "service2"]
restart = ["service3", "service4"]
```

### Repositories

Define the git repositories you want to clone on all hosts:

```toml
[repositories]
global = ["https://github.com/user/repo1.git", "https://github.com/user/repo2.git"]
```

## 🛠 Building

To build the Linux Fleet Manager, navigate to the project directory and run:

```bash
cargo build
```

## 🚀 Running

To run the Linux Fleet Manager, ensure you have a properly configured `config.toml` file, then execute the following command:

```bash
cargo run
```

The tool will display progress bars for each host as it executes the tasks. When finished, it will display a summary of the actions taken.

## 🤝 Contributing

Contributions are welcome! If you find a bug or have a feature request, please open an issue on the project's GitHub repository. If you want to contribute code, feel free to fork the repository and submit a pull request.

---

🙏 Thank you for using Linux Fleet Manager! If you find this tool helpful, please consider starring the repository on GitHub and sharing it with your friends and colleagues. Your support is greatly appreciated.
