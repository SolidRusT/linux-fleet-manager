# Linux Fleet Manager
Linux Fleet Manager is a simple and efficient tool for managing multiple remote Linux hosts concurrently. It helps you automate the process of installing packages, creating users, and managing services across a fleet of Linux servers.

## Features
* Parallel execution on multiple hosts
* Automated package installation
* User management
* Service management

## Requirements
* Rust 1.53 or higher
* Cargo
* Access to target Linux hosts via SSH with appropriate credentials

## Configuration
To configure the Linux Fleet Manager, create a config.toml file in the root directory of the project. This file should contain the following sections:

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

* `name`: A human-readable name for the host
* `address`: The IP address or hostname of the remote host
* `port`: The SSH port of the remote host (default: 22)
* `user`: The username to use when connecting to the remote host
* `password`: The password to use when connecting to the remote host
* `key_path`: The path to the private key for the remote host (optional, if not provided, password authentication will be used)

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

## Building
To build the Linux Fleet Manager, navigate to the project directory and run:

```bash
cargo build
```

## Running
To run the Linux Fleet Manager, make sure you have a properly configured config.toml file, then execute the following command:

```bash
cargo run
```

The tool will display progress bars for each host as it executes the tasks. When finished, it will display a summary of the actions taken.

## Contributing
Contributions are welcome! If you find a bug or have a feature request, please open an issue on the project's GitHub repository. If you want to contribute code, feel free to fork the repository and submit a pull request.

Thank you for using Linux Fleet Manager! If you find this tool helpful, please consider starring the repository on GitHub and sharing it with your friends and colleagues. Your support is greatly appreciated.
