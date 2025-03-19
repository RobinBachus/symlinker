# SymLinker

A simple tool written in Rust to create symbolic links for files and directories.

It is written in Rust purely for learning purposes.

It keeps a list of all created symbolic links and has some configuration options. Both of these are stored in the `ProjectDirs::from("xyz", "server1rb", "symlinker")` directory. This is for example the %APPDATA% directory on Windows.

## Usage

It is a command line tool. You can run it with `symlinker.exe` or `cargo run -- <args>`.

### Commands

- `symlinker.exe add <source> <destination>`: Creates a symbolic link from the source to the destination.
- `symlinker.exe remove <source>`: [TODO] Removes the symbolic link at the source.
- `symlinker.exe list`: [TODO] Lists all created symbolic links.
- `symlinker.exe help`: [TODO] Shows the help message.
