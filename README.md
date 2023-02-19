# The Empty Project (Origins)
I've always had a special place in my heart for Linux!  I'm a Windows developer, who works on MacOS and Linux quite frequently and 
I start to miss all the tools and commands of the other operating systems.  One command in particular I find useful is the linux **touch** command.
The touch command does a few things, like changes a file's properties such as access time and modification time, but the most feature for me is creating empty files. 
I wouldn't mind building a simple CLI application to do much the same functions as Linux's tounch command, but on Windows.

Simple right?

Well, here is the challenge (for me, anyway) - I'm want to code it using [Rust langauge](https://www.rust-lang.org/). 
and btw, I don't know how to code in Rust...yet.  Sounds fun right?  You are welcome to join in the fun!

# About

This is a Rust command-line application that allows the user to create one or more empty files. The application takes in command line arguments and creates a file for each argument. If the `-b` flag is specified, the application will create "bare-bones" files that are empty except for a minimal header.

## Usage

To use the application, simply run it with one or more arguments:

```
./main.exe file1.txt file2.txt
```

This will create two empty files named `file1.txt` and `file2.txt` in the current directory.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/lbrgriffith/TheEmptyProject/blob/main/LICENSE) file for details.

## Compiling and Running
### Windows
1. Install Rust for Windows by downloading and running the installer from the [official Rust website]([url](https://www.rust-lang.org/tools/install)).
2. Open a command prompt and navigate to the directory where your `main.rs` file is located.
3. Run the command `cargo build --release` to compile the program.
4. Run the command `./target/release/main.exe` to execute the compiled program.

### Linux
Install Rust for Linux by running the following command in your terminal: 
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh.
```
1. Open a terminal and navigate to the directory where your `main.rs` file is located.
2. Run the command `cargo build --release` to compile the program.
3. Run the command `./target/release/main` to execute the compiled program.

### MacOS
1. Install Rust for macOS by downloading and running the installer from the official Rust website.
2. Open a terminal and navigate to the directory where your `main.rs` file is located.
3. Run the command `cargo build --release` to compile the program.
4. Run the command `./target/release/main` to execute the compiled program.

_Note that the compiled binary will be located in the target/release directory after you run `cargo build --release`._
