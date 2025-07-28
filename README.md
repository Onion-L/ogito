# ogito🍸

![Crates.io Version](https://img.shields.io/crates/v/ogito)
![Crates.io License](https://img.shields.io/crates/l/ogito)
![Crates.io Total Downloads](https://img.shields.io/crates/d/ogito)
![NPM Version](https://img.shields.io/npm/v/ogito)
![NPM Downloads](https://img.shields.io/npm/d18m/ogito)


A simple and efficient Git repository cloning management tool. `ogito` allows you to quickly clone repositories to create a clean project.

## Features

- 🚀 Fast cloning of GitHub repositories
- 🧹 Create clean project copies
- 🔄 Force mode to override existing directories
- 📊 Beautiful progress indicators and status feedback
- 💻 User-friendly command line interface

## install

```bash
# cargo
cargo install ogito

# npm
npm install -g ogito
```

## Usage

```bash
# Basic usage
ogito <repository URL>

# Example
ogito https://github.com/user/repo

# Use specific direcotry name
ogito https://github.com/user/repo -d dirname

# Use force mode to override existing directory
ogito https://github.com/user/repo -d dirname -f

# Clone specific branch (you have to use --branch=branch_name or -b=branch_name for this option)
ogito https://github.com/user/repo --branch=branch_name
```

### Command Line Options

- `[url]` - Source file link
- `-d, --dir <DIRNAME>` - Destination directory name
- `-m, --mode <MODE>` - Operation mode (git or tar)
- `-f, --force` - Force operation, override existing directory
- `--keep-history` - Keep the history of the repository
- `-b --branch` - Select specific branch to clone (you have to use -b=<branch_name> or --branch=<branch_name> for this option)

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## License

This project is open source under the [MIT LICENSE](LICENSE).
