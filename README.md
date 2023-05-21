# Mafia (Rust)
Local multiplayer TOS clone

## Getting Started
First, download and enter the [git](https://git-scm.com/) repository:
```bash
git clone https://www.github.com/ItsSammyM/mafia-rust
cd mafia-rust
```
From here it's recommended to split terminals (If you're using VSCode), or open up a second terminal - one for client and one for server.
### Client setup
Enter the client directory and install the required dependencies using [npm](https://www.npmjs.com/). Our dependencies are a little messed up at the moment, so you may need to use the `--force` flag.
```bash
cd client
npm install --force
```
You can now start the client frontend using npm. If you're using WSL, you need to execute with `sudo`
```bash
npm start
```
### Server setup
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
## About
### Gamemodes
1. Mafia
2. Vampire
3. All any
4. Arsonist
5. CIA gamemomde (Regualar but replace 3 mafia with executioners)
