echo Stopping Services
systemctl stop mafia-game-server
systemctl stop mafia-game-client

runuser -l mafia -c './update-rootless.sh'

echo Starting Services
systemctl start mafia-game-server
systemctl start mafia-game-client
