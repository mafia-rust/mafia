export NVM_DIR="$([ -z "${XDG_CONFIG_HOME-}" ] && printf %s "${HOME}/.nvm" || printf %s "${XDG_CONFIG_HOME}/nvm")"
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"

cd /mafia

echo Pulling Updates
git stash
git pull --rebase
git stash pop

echo Building Server
cd /mafia/server
cargo build --release

echo Building Client
cd /mafia/client
npm install
npm run build
