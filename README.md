# The Empty Project (Origins)
I've always had a special place in my heart for Linux!  I'm a Windows developer, who works on MacOS and Linux quite frequently and 
I start to miss all the tools and commands of the other operating systems.  One command in particular I find useful is the linux **touch** command.
The touch command does a few things, like changes a file's properties such as access time and modification time, but the most feature for me is creating empty files. 
I wouldn't mind building a simple CLI application to do much the same functions as Linux's tounch command, but on Windows.

Simple right?

Well, here is the challenge (for me, anyway) - I'm want to code it using [Rust langauge](https://www.rust-lang.org/). 
and btw, I don't know how to code in Rust...yet.  Sounds fun right?  You are welcome to join in the fun!

# My Rust File Creator

This is a Rust command-line application that allows the user to create one or more empty files. The application takes in command line arguments and creates a file for each argument. If the `-b` flag is specified, the application will create "bare-bones" files that are empty except for a minimal header.

## Usage

To use the application, simply run it with one or more arguments:

```
./main.exe file1.txt file2.txt

```

This will create two empty files named `file1.txt` and `file2.txt` in the current directory.

## License

This project is licensed under the MIT License - see the [LICENSE](https://github.com/lbrgriffith/TheEmptyProject/blob/main/LICENSE) file for details.

