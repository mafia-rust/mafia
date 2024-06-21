#!/bin/bash
if [ $EUID != 0 ]; then
	echo "Please run this script as root!"
    exit
fi	

cd /
echo Updating Debian
apt-get update
apt-get upgrade -y

echo Installing Extras
apt-get install -y curl nano git build-essential pkg-config libssl-devel

echo Cloning game files
git clone https://github.com/mafia-rust/mafia.git
cd /mafia

echo Linking Services
ln -s /mafia/system/mafia-game-client.service /etc/systemd/system/mafia-game-client.service
ln -s /mafia/system/mafia-game-server.service /etc/systemd/system/mafia-game-server.service

echo Edit Client Configuration File
nano client/src/resources/config.json

echo Edit Server Configuration File
nano server/resources/config.json

echo Creating mafia user
adduser -disabled-password --gecos "" mafia

echo Setting Permissions
chmod +x update-rootless.sh
chmod +x install-deps.sh
chown -R mafia:mafia /mafia

echo Installing Build Dependencies
runuser -l mafia -c 'cd /mafia/system && ./install-deps.sh'

echo Bootstrapping Mafia
runuser -l mafia -c 'cd /mafia/system && ./update-rootless.sh'

echo Reloading Daemons
systemctl daemon-reload

echo Enabling Mafia Services
systemctl enable mafia-game-client
systemctl enable mafia-game-server

echo Starting Mafia Services
systemctl start mafia-game-client
systemctl start mafia-game-server

echo Done!
cat next-steps.txt
