#!/usr/bin/env bash
set -e

# TODO: prompt for `systemctl list-units -t mount`, requires device is currently mounted
DEVICE_LABEL=media-augustus-GARMIN.mount
SYSTEMD_SERVICE=fit-upload.service
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )"
SYSTEMD_FILE=$SCRIPT_DIR/$SYSTEMD_SERVICE

cat <<- EOF > $SYSTEMD_FILE
	[Unit]
	Description=Upload Garmin/fit activity files to GDrive and Strava
	Requires=$DEVICE_LABEL
	After=$DEVICE_LABEL

	[Service]
	ExecStart=$SCRIPT_DIR/upload.sh $DEVICE_LABEL

	[Install]
	WantedBy=$DEVICE_LABEL
EOF

echo "Enabling systemd service (requires sudo)..."
sudo systemctl enable $SYSTEMD_FILE

echo "Starting systemd service (requires sudo)..."
sudo systemctl start $SYSTEMD_SERVICE

exit 0

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
