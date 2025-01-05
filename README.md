# 🐍 VenMan: Python Virtual Environment Manager
	
A fast, simple, and elegant Python virtual environment manager, built in Rust.

## 🌟 Features

- 🚀 Quick virtual environment creation
- 📦 Easy package management
- 🔍 List and manage multiple environments
- 💻 Cross-platform support (Linux, macOS)
- 🎨 Colorful and intuitive CLI

## 🛠 Prerequisites

- Rust
- Python 3.7+
- Cargo

## 🚀 Installation
### Linux
	
```bash
# Clone the repository
git clone https://github.com/ImLowFranki/venman.git
cd venman

# Build the project
cargo build --release

# Install the binary
sudo cp target/release/venman /usr/local/bin/
```
	
### Windows

```powershell
# Clone the repository
git clone https://github.com/ImLowFranki/venman.git
cd venman

# Build the project
cargo build --release

# Add to PATH manually or copy binary
```	
## 💡 Usage

### Create Virtual Environment
```bash
venman
# Choose option 1
# Enter:
# - Environment name
# - Description (optional)
# - Packages (optional)
	```
	
### Activate Virtual Environment
```bash
venman
# Choose option 2
# Select environment to activate
	```
	
### List Virtual Environments
```bash
venman
# Choose option 3
```

### Delete Virtual Environment
```bash
venman
# Choose option 4
```

## 🔧 Advanced Usage

### Creating Environment with Packages
```bash
# Example: Creating a Django project environment
venman
# Name: django_project
# Description: Web development environment
# Packages: django requests pillow
```
	
## 📦 Dependencies
	
- `colored`: For terminal coloring
- `serde`: JSON/TOML serialization
- `dirs`: Cross-platform directory paths
- `toml`: TOML parsing
	
## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request
	
## 🐛 Troubleshooting

- Ensure Python 3 is installed
- Check Rust and Cargo are in your PATH
- Verify permissions for binary installation
	
