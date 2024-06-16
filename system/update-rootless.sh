cd /mafia

echo Pulling Updates
git stash
git pull --rebase

echo Refreshing Configs
cp system-config/client-config.json client/src/resources/config.json
cp system-config/server-config.json server/resources/config.json

echo Building Server
cd /mafia/server
cargo build --release

echo Building Client
cd /mafia/client
npm install
npm run build
