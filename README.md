# Mafia (Rust)
Local multiplayer TOS clone

## Getting Started
First, download and enter the [git](https://git-scm.com/) repository:
```bash
git clone https://www.github.com/ItsSammyM/mafia-rust
cd mafia-rust
```
From here it's recommended to split terminals (If you're using VSCode), or open up a second terminal - one for client and one for server.
## Client setup
Enter the client directory and install the required dependencies using [npm](https://www.npmjs.com/). Our dependencies are a little messed up at the moment, so you may need to use the `--force` flag.
```bash
cd client
npm install --force
```
You can now start the client frontend using npm. If you're using WSL, you need to execute with `sudo`
```bash
npm start
```
## Server setup
### VScode
If you're using VSCode, it's recommended to download the following extensions to make using rust easier:
 - [Better Toml](https://marketplace.visualstudio.com/items?itemName=bungcip.better-toml) - Language support for .TOML files
 - [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) - Helps manage crate versions
 - [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens) - Show error messages inline
 - [GitLens](https://marketplace.visualstudio.com/items?itemName=eamodio.gitlens) - View git blame inline

It's also a good idea to change your default linter to clippy using this setting:
```json
"rust-analyzer.check.command": "clippy",
```

### Starting the server
Enter the server directory and build the project using [cargo](https://www.rust-lang.org/).
```bash
cd server
cargo build
```
Note: If the above step fails, and you are using Linux or WSL, you may need to install OpenSSL first.

You can now start the server backend:
```bash
cargo run
```
It's recommended to use `cargo-watch` to automatically restart the server when a change is made on the server side. You can install and run it like this:
```bash
# install
cargo install cargo-watch
# run
cargo watch -x run
```