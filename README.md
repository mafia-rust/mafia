# Mafia
Multiplayer Social Deduction game

## Getting Started
First, download and enter the [git](https://git-scm.com/) repository:
```bash
git clone https://www.github.com/mafia-rust/mafia
cd mafia
```
From here it's recommended to split terminals (If you're using VSCode), or open up a second terminal - one for client and one for server.
## Client setup
Enter the client directory and install the required dependencies using [npm](https://www.npmjs.com/).
```bash
cd client
npm install
```
You can now start the client frontend using npm. If you're using WSL, you need to execute with `sudo`
```bash
npm start
```
## Server setup
### Install Rust
Follow the [tutorial](https://www.rust-lang.org/learn/get-started) on the rust website.
### VScode
If you're using VSCode, it's recommended to download the following extensions to make working on the project easier:
 - [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) - You probably already have this. You definitely need it.
 - [Even Better Toml](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) - Language support for .TOML files
 - [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) - Helps manage crate versions
 - [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens) - Show error messages inline
 - [GitLens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens) - View git blame inline
 - [Spell checker](https://marketplace.visualstudio.com/items?itemName=streetsidesoftware.code-spell-checker) - Spelling corrections

It's also a good idea to install clippy (a linter):
```bash
rustup component add clippy
```
You can make it the default linter using this setting (but you don't need to):
```json
"rust-analyzer.check.command": "clippy",
```

### Starting the server
Enter the server directory and build the project using cargo.
```bash
cd server
cargo build
```
Note: If the above step fails, and you are using Linux or WSL, you may need to install OpenSSL first.

You can now start the server backend:
```bash
cargo run
```
