# Smriti

Smriti is a command-line interface (CLI) tool that supercharges your terminal by helping you save, organize, and execute commands like a pro. Tired of typing out long commands or remembering complex bash aliases? This tool makes managing your go-to commands easy, memorable, and portable!

## Features

- **Save and Execute Commands**: Like bash aliases but better! Save any command with a custom name and execute it directly by that name.
- **Organize with Aliases and Groups**: Organize commands by aliases and groups so you can keep similar commands together.
- **Search by Alias**: Can’t remember that command you saved last month? Quickly search by alias to find it in seconds.
- **Update and Delete Commands Easily**: Modify or remove commands as your workflow evolves.
- **Portable Command Database**: Your commands are saved in an SQLite database, making it easy to transfer your whole command setup to another device. No need to reconfigure all your aliases each time!

## Why Use This Over Bash Aliases?

If you've tried to use bash aliases to remember commands like `git pull origin main && npm install && npm run build` but found yourself asking "Wait, what did I name that alias again?"—you’re not alone. With this tool, commands are saved with all their details and can be searched, updated, or deleted without getting buried in `.bashrc`.

## Installation

Clone the repository and set it up:

```bash
git clone https://github.com/Aspasht/Smriti.git
cd Smriti
# Compile or install as necessary (instructions may vary)
cargo build --release
    sudo mv target/release/smriti /usr/local/bin/smriti
```

## Usage
```
  smriti --help
  smriti <COMMAND> [OPTIONS]
```

## Example Workflow
1. Save a command for quickly checking system updates:
```
  smriti add  -a check-updates -c "sudo apt update && sudo apt upgrade -y" -i "Update and upgrade system" -s "linux"
```

2. No need to type it out every time!
Execute it anytime by alias:
```
  smriti run check-updates
```

3. **Take your commands with you**: Copy the SQLite database file to another device and keep all your saved commands at your fingertips.

## Why SQLite?

With SQLite, you can effortlessly back up and transport your entire command collection, unlike bash aliases that require a file-by-file transfer and potentially clunky reconfiguration.

## Contributing

Contributions are welcome! Fork the repo, make a feature branch, and submit a pull request.

---

Say goodbye to bash alias nightmares and hello to organized, easy-to-find commands! 😄
