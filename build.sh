#!/usr/bin/sh

CUSER=$(whoami)
PNAME="hotkeys2bashd"
cargo build --release >/dev/null || exit 1
mkdir -p /home/$CUSER/.local/bin
cp -u ./target/release/hotkeys2bash /home/$CUSER/.local/bin/$PNAME || exit 1

mkdir -p /home/$CUSER/.config/$PNAME || exit 1
cp -n ./config.ini.example /home/$CUSER/.config/$PNAME/config.ini || exit 1

mkdir -p /home/$CUSER/.config/systemd/user || exit 1
touch $PNAME.service || exit
cat >/home/$CUSER/.config/systemd/user/$PNAME.service <<EOF
[Unit]
Description=$PNAME
After=network.target

[Service]
Type=simple
ExecStart=/home/$CUSER/.local/bin/$PNAME /home/$CUSER/.config/$PNAME/config.ini
User=$CUSER
Group=$CUSER
Restart=always 

[Install]
WantedBy=default.target

EOF

systemctl --user enable --now $PNAME.service || exit 1
