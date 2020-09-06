<p align="center">
  <img src="./assets/logo.png" width="40%" />
  <br />
  <i>An extensible task execution and generation tool</i>
</p>

<p align="center">
  <img alt="Version" src="https://img.shields.io/badge/Version-1.0-red.svg" />
  <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-orange.svg" />
  <img alt="Made with Rust" src="https://img.shields.io/badge/Made%20with-Rust-yellow.svg" />
  <img alt="gh-actions" src="https://github.com/safinsingh/rune/workflows/CI/badge.svg" />
  <img alt="PRs Welcome" src="https://img.shields.io/badge/PRs-Welcome-blue.svg">
  <img alt="awesome" src="https://img.shields.io/badge/Awesome-Yes-purple">
  <br />
</p>

<hr>

## ✨ Installation

From AUR:

> Not complete yet

From `cargo`:

```sh
cargo install rune-rs
```

From source:

```sh
git clone https://github.com/safinsingh/rune.git
cargo build --release

# for subsequent builds
rune release
```

## 🔮 In action

<p align="center">
  <img src="./assets/demo.gif" width="80%" />
</p>

## 📖 Usage

All Rune targets are defined in `Rune.example` like so:

```yaml
# Global information, name and version
# Are required
name: Rune
version: 1.0.0
author: Safin Singh

# Array of all goals
goals:
  # Default goal runs when none are specified
  default: echo hi

  # An array of commands
  two:
    - echo hi
    - echo hi2

  # An array of commands with messages
  # that are displayed in verbose mode
  three:
    - message: execute command
      cmd:
        - echo hello
        - echo hello2
    - message: execute command
      cmd:
        - echo hello
        - echo hello2
```

## 👨‍💻 Author

Linkedin: [Safin Singh](https://www.linkedin.com/in/safin-singh-b2630918a/) <br>
GitHub: [safinsingh](https://github.com/safinsingh) <br>
Dribbble: [Safin Singh](https://dribbble.com/safinsingh/) <br>
YouTube: [Safin Singh](https://www.youtube.com/channel/UCvb01sUdAgcPAG1j0SLxAtA)

## 🤝 Contributing

Contributions, PRs, issues and feature requests are welcome! Feel free to check out my [issues page](https://github.com/safinsingh/rune/issues).

## ❤️ Show your support

Give a ⭐️ if this project helped you!
Hope you enjoy it!
