#!/usr/bin/env bash
set -e

echo "Downloading Google Drive CLI..."
mkdir -p out
if [ ! -e out/gdrive ]
then
  wget https://github.com/gdrive-org/gdrive/releases/download/2.1.0/gdrive-linux-x64
  mv gdrive-linux-x64 out/gdrive
  chmod +x out/gdrive
fi

echo "Authenticating with Google Drive..."
./out/gdrive about

echo "Copying udev rule to $UDEV_RULE_PATH..."
UDEV_RULE_FILE=90-garmin.rules
UDEV_RULE_PATH=/etc/udev/rules.d/$UDEV_RULE_FILE
if [ ! -e $UDEV_RULE_PATH ]
then
  # TODO: lsusb and prompt for which device
  echo "Linking udev rule (requires sudo)..."
  sudo ln -s $UDEV_RULE_FILE $UDEV_RULE_PATH
else
  echo "udev rule exists!"
fi
