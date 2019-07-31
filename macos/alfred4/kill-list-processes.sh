#!/bin/bash

echo '{"items": ['

while read -r line
do
  pid="${line%% *}"
  command="${line#* }"
  title="${command##*/}"
  if [[ $command == *'.app'* ]]
  then
    icontype='fileicon'
    iconpath="${command%%.app*}.app"
  else
    icontype=''
    iconpath='/System/Library/CoreServices/CoreTypes.bundle/Contents/Resources/ExecutableBinaryIcon.icns'
  fi
  echo "{\"uid\": \"$pid\", \"title\": \"$title\", \"subtitle\": \"$command\", \"arg\": \"$pid\", \"icon\": { \"type\": \"$icontype\", \"path\": \"$iconpath\" }},"
done < <( ps -ax -u $USER -o pid=,comm= )

echo ']}'
