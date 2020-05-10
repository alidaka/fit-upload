#!/usr/bin/env bash
set -e

echo "Uploading to Google Drive..."
#./out/gdrive

echo "Uploading to Strava..."
for filename in /media/augustus/GARMIN/GARMIN/ACTIVITY/*.FIT; do
  echo "Uploading $filename..."
  curl -X POST https://www.strava.com/api/v3/uploads \
    -H "Authorization: Bearer abcd123" \
    -F data_type="fit" \
    -F file=@$filename
done

xdg-open https://www.strava.com/athlete/training
