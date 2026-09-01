#!/usr/bin/sh


CUSER=$(whoami)

echo $CUSER
exit(1)

echo $(which /usr/bin/sh)

cargo build --release > /dev/null

if [$? != 0]; then
  exit 1
done

cp ./target/release/hotkeys2bash /usr/local/bin/hotkeys2bashd

mkdir -p /home/$(whoami)/.config/systemd/user
# TODO

mkdir -p /home/$(whoami)/.config/hotkeys2bashd
cp ./config.ini.example /home/$(whoami)/.config/hotkeys2bashd/config.ini

