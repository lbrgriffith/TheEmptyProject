# Empty - A Secure Rust Touch Command

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/lbrgriffith/TheEmptyProject)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)

A modern, secure implementation of the Linux `touch` command written in Rust. Creates empty files and manages file timestamps with modern security protections.

## Features

✅ **Full Linux touch compatibility** - All major touch command flags supported  
🔒 **Security hardened** - Path traversal protection, symlink safety, input validation  
🌍 **Cross-platform** - Works on Windows, macOS, and Linux  
⚡ **Fast & reliable** - Built with modern Rust for performance and safety  
📖 **User-friendly** - Comprehensive help and examples built-in  

## Quick Start

```bash
# Create empty files
empty file1.txt file2.txt

# Set custom timestamp
empty -t 202501011200 document.txt

# Copy timestamp from reference file
empty -r template.txt new_file.txt

# Get help
empty --help
```

## Installation

### Prerequisites
- [Rust](https://rustup.rs/) 1.70 or later

### Option 1: Build from Source
```bash
# Clone the repository
git clone https://github.com/lbrgriffith/TheEmptyProject.git
cd TheEmptyProject

# Build and install
cargo build --release
cargo install --path .
```

### Option 2: Direct Build and Run
```bash
# Build the project
cargo build --release

# The binary will be available at:
# ./target/release/empty        (Linux/macOS)
# .\target\release\empty.exe    (Windows)
```

## Usage

The `empty` command supports all major Linux touch functionality:

### Basic File Creation
```bash
# Create single file
empty file.txt

# Create multiple files
empty file1.txt file2.txt file3.txt

# Create file in subdirectory (secure - stays within current directory)
empty subdir/file.txt
```

### Timestamp Management
```bash
# Set specific timestamp (YYMMDDhhmm format)
empty -t 202501011200 file.txt        # Jan 1, 2025 12:00

# Set timestamp with seconds
empty -t 20250101120030 file.txt       # Jan 1, 2025 12:00:30

# Copy timestamp from reference file
empty -r reference.txt target.txt

# Update only access time
empty -a file.txt

# Update only modification time  
empty -m file.txt
```

### Advanced Options
```bash
# Don't create file if it doesn't exist (only update timestamps)
empty -c existing_file.txt

# Combine flags - set modification time only to specific timestamp
empty -m -t 202501011200 file.txt

# Get detailed help with examples
empty --help
```

### Command Line Options

| Flag | Description |
|------|-------------|
| `-c, --no-create` | Don't create files if they don't exist |
| `-a, --access-time` | Change only the access time |
| `-m, --modification-time` | Change only the modification time |
| `-r, --reference FILE` | Use reference file's timestamps |
| `-t, --time STAMP` | Use specific timestamp (YYMMDDhhmm[ss]) |
| `-h, --help` | Show help information |
| `-V, --version` | Show version information |

## Platform-Specific Instructions

### 🐧 Linux
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Clone and build
git clone https://github.com/lbrgriffith/TheEmptyProject.git
cd TheEmptyProject
cargo build --release

# Run the application
./target/release/empty file.txt
```

### 🍎 macOS
```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# OR via Homebrew: brew install rust

# Clone and build
git clone https://github.com/lbrgriffith/TheEmptyProject.git
cd TheEmptyProject
cargo build --release

# Run the application
./target/release/empty file.txt
```

### 🪟 Windows

#### Option 1: Using Command Prompt/PowerShell
```cmd
# Install Rust from https://rustup.rs/ or via winget
winget install Rust.Rustup

# Clone and build
git clone https://github.com/lbrgriffith/TheEmptyProject.git
cd TheEmptyProject
cargo build --release

# Run the application
.\target\release\empty.exe file.txt
```

#### Option 2: Using WSL (Windows Subsystem for Linux)
```bash
# Follow the Linux instructions above within WSL
```

## Security Features

This implementation includes comprehensive security protections:

- 🛡️ **Path Traversal Protection** - Prevents `../../../etc/passwd` attacks
- 🔗 **Symlink Safety** - Refuses to operate on symbolic links
- 🚫 **Input Validation** - Sanitizes all user inputs and file paths
- 📏 **Resource Limits** - Prevents resource exhaustion attacks
- 🌐 **Cross-Platform** - Secure on Windows, macOS, and Linux

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/lbrgriffith/TheEmptyProject/blob/main/LICENSE) file for details.

## Project History

This project started as a learning exercise to build a simple file creation tool in Rust. It has evolved into a full-featured, security-hardened implementation of the Linux touch command, demonstrating modern Rust development practices and security-first design principles.
