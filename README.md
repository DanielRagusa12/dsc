# dsc

## Overview
dsc is a rust CLI program that can quickly search and copy files from any directory it is called from. The program can also copy and reconstruct the directory that it searches, placing the found files appropriately. The multithreaded file searching is possible due to [Rust_Search](https://github.com/ParthJadhav/Rust_Search). The motivation behind this project was to quickly copy a bunch of poorly organized photos and videos on some of my old machines. 


## Compilation
To compile the program, you need to have Rust installed on your system. If not, you can install it by following the instructions at [Rust Installation](https://www.rust-lang.org/learn/get-started).

Once Rust is installed, navigate to the directory containing the `dsc.rs` file in your terminal and run the following command:

```bash
cargo build --release
```

This will create an executable named `dsc` in the `target/release/` directory.

## Environment Variables
To make the program accessible from anywhere in the terminal, you can add the directory containing the `dsc` executable to your system's `PATH` environment variable. The steps to do this depend on your operating system:

### On Unix/Linux/MacOS
Add the following line to your shell profile file (e.g., `~/.bashrc`, `~/.zshrc`):

```bash
export PATH="$PATH:/path/to/dsc/target/release"
```

Replace `/path/to/dsc` with the actual path to the directory containing the executable.

### On Windows
Add the following to your system's `PATH` environment variable:
- Open the Start menu and search for "Environment Variables."
- Click on "Edit the system environment variables."
- In the System Properties window, click the "Environment Variables" button.
- Under "System variables," find the "Path" variable and click "Edit."
- Click "New" and add the path to the directory containing the `dsc.exe` executable.

## Usage
After adding the executable directory to your `PATH`, you can run the program from any location in the terminal using the following command:

```bash
dsc <subcommand>
```

Replace `<subcommand>` with one of the following:

### List Command
```bash
dsc list --extension <file_extension> [--limit <file_limit>]
```
- `--extension`: Specify the file extension to search for.
- `--limit`: (Optional) Limit the number of files displayed.

### Copy Command
```bash
dsc copy --reconstruct --extension <file_extension>
```
- `--reconstruct`: Reconstruct directory structure during copying.
- `--extension`: Specify the file extension to search for.

### Clear Command
```bash
dsc clear
```
Clears the 'searches' and 'reconstructions' directories.

## Examples
### List files with a limit
```bash
dsc list --extension txt --limit 10
```

### Copy PDF files with reconstruction
```bash
dsc copy --reconstruct --extension pdf
```

### Clear searches
```bash
dsc clear
```

Feel free to explore and modify the code according to your requirements. Happy coding!
