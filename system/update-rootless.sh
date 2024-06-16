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
