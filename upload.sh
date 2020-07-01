#!/usr/bin/env bash
set -e

SYSTEMD_UNIT=$1

echo "---------------------------------" >> /home/augustus/code/fit-upload/systemd.log
date >> /home/augustus/code/fit-upload/systemd.log
echo >> /home/augustus/code/fit-upload/systemd.log
echo >> /home/augustus/code/fit-upload/systemd.log
echo "systemd unit is $SYSTEMD_UNIT" >> /home/augustus/code/fit-upload/systemd.log
echo >> /home/augustus/code/fit-upload/systemd.log

GARMIN_DIRECTORY=$( systemctl show $SYSTEMD_UNIT --no-page | grep Where | cut --characters=7- )

echo "mount point is: $GARMIN_DIRECTORY" >> /home/augustus/code/fit-upload/systemd.log

echo "Uploading..."
/home/augustus/code/fit-upload/target/debug/fit-upload upload $GARMIN_DIRECTORY

echo "Uploading to Strava..."
#/media/augustus/GARMIN/GARMIN/ACTIVITY/*.FIT

#xdg-open https://www.strava.com/athlete/training
