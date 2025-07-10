# regit

A simple and efficient Git repository cloning management tool. `regit` allows you to quickly clone repositories to create a clean project starting point.

## Features

- 🚀 Fast cloning of GitHub repositories
- 🧹 Create clean project copies
- 🔄 Force mode to override existing directories
- 📊 Beautiful progress indicators and status feedback
- 💻 User-friendly command line interface

## install

```
cargo install regit
```

## Usage

```bash
# Basic usage
regit <repository URL> -d <destination directory>

# Example
regit https://github.com/user/repo -d my-project

# Use force mode to override existing directory
regit https://github.com/user/repo -d existing-dir -f

# Specify site (currently supports GitHub)
regit https://github.com/user/repo -d my-project -s github
```

### Command Line Options

- `[url]` - Source file link
- `-r, --repo <REPO>` - Repository name, e.g. 'user/repo'
- `-d, --dir <DIRNAME>` - Destination directory name (required)
- `-s, --site <SITE>` - Set site, default is GitHub
- `-m, --mode <MODE>` - Operation mode (currently supports git mode)
- `-f, --force` - Force operation, override existing directory

## TODO

- [ ] Implement TUI (Text User Interface) for post-clone management
- [x] Implement tar mode download
- [ ] Support more platforms (Gitlab, Bitbucket, Gitee, Gitcode, etc.)
- [ ] Publish npm package
- [ ] Implement caching mechanism
- [ ] Add more tests
- [ ] Optimize error handling and user feedback
- [ ] Add multilingual support
- [ ] Enhance project documentation

## Contributing

Contributions are welcome! Feel free to submit issues and pull requests.

## License

This project is open source under the [MIT LICENSE](LICENSE).
