# ogito🍸

A simple and efficient Git repository cloning management tool. `ogito` allows you to quickly clone repositories to create a clean project.

## Features

- 🚀 Fast cloning of GitHub repositories
- 🧹 Create clean project copies
- 🔄 Force mode to override existing directories
- 📊 Beautiful progress indicators and status feedback
- 💻 User-friendly command line interface

## install

```
cargo install ogito
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

# Specify site (currently supports GitHub and Gitlab)
ogito https://github.com/user/repo -d my-project -s github
```

### Command Line Options

- `[url]` - Source file link
- `-r, --repo <REPO>` - Repository name, e.g. 'user/repo'
- `-d, --dir <DIRNAME>` - Destination directory name
- `-m, --mode <MODE>` - Operation mode (git or tar)
- `-f, --force` - Force operation, override existing directory

## TODO

- [ ] Implement TUI (Text User Interface) for post-clone management
- [x] Implement tar mode download
- [ ] Support more platforms (Gitlab, Bitbucket, Gitee, Gitcode, etc.)
- [ ] Publish npm package
- [ ] Select from specific commit or branch
- [ ] Optimize error handling and user feedback
- [ ] Enhance project documentation

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## License

This project is open source under the [MIT LICENSE](LICENSE).
