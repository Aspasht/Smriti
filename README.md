# Smriti

## Keep track of the commands your memory missed.

Smriti is a command-line interface (CLI) tool that supercharges your terminal by helping you save, organize, and execute commands like a pro. Tired of typing out long commands or remembering complex bash aliases? This tool makes managing your go-to commands easy, memorable, and portable!

## Features

- **Save and Execute Commands**: Like bash aliases but better! Save any command with a custom name and execute it directly by that name.
- **Organize with Aliases and Groups**: Organize commands by aliases and groups so you can keep similar commands together.
- **Search by Alias**: Can’t remember that command you saved last month? Quickly search by alias to find it in seconds.
- **Update and Delete Commands Easily**: Modify or remove commands as your workflow evolves.
- **Portable Command Database**: Your commands are saved in an SQLite database, making it easy to transfer your whole command setup to another device. No need to reconfigure all your aliases each time!

## Installation

[Download binary release](https://github.com/Aspasht/Smriti/releases/tag/v0.1.0)

or

Clone the repository and set it up yourself:

```bash
git clone https://github.com/Aspasht/Smriti.git
cd Smriti
# Compile or install as necessary (instructions may vary)
cargo build --release
sudo mv target/release/smriti /usr/local/bin/smriti
```

## Usage
```
> smriti --help
Keep track of the commands your memory missed.

Usage: smriti <COMMAND>

Commands:
  run     Execute a saved command
  add     Add a new command to the saved list
  delete  Remove an existing command, alias, or group
  update  Modify an existing command, alias, or group
  view    Display a list of all saved commands, aliases, or groups
  show    Find and display the command associated with a specific alias or service
  rename  Update an existing alias
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

## Example Workflow
1. **Save a command for quickly checking system updates**:
```
  smriti add  -a check-updates -c "sudo apt update && sudo apt upgrade -y" -i "Update and upgrade system" -s "linux"
```

2. **No need to type it out every time!**
Execute it anytime by alias:
```
  smriti run check-updates
```

3. **Take your commands with you**: This tool creates .smriti.db file in your home directory. Copy the SQLite database file to another device and keep all your saved commands at your fingertips.
```
  cp ~/.smriti.db your_desired_location
```

4. **supports the use of placeholders**: Create using the {placeholder} syntax and dynamically by substituting them with specific values when the command is executed.
```
 > smriti add -a "nmapv" -c "nmap -sC -sV {ip}" -s "Network" -i "performs a version detection scan"
> smriti run 192.168.0.0 //replace with the actual ip address
```

## Why Use This Over Bash Aliases?

If you've tried to use bash aliases to remember commands like `git pull origin main && npm install && npm run build` but found yourself asking "Wait, what did I name that alias again?"—you’re not alone. With this tool, commands are saved with all their details and can be searched, updated, or deleted without getting buried in `.bashrc`.


## Why SQLite?

With SQLite, you can effortlessly back up and transport your entire command collection, unlike bash aliases that require a file-by-file transfer and potentially clunky reconfiguration.

## Contributing

Contributions are welcome! Fork the repo, make a feature branch, and submit a pull request.

---

Say goodbye to bash alias nightmares and hello to organized, easy-to-find commands! 😄
