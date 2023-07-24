#!/bin/bash
saveFile="/etc/acpi/actions/tablet-mode.txt"
if [ "$4" = "00000001" ]; then
  # Save the current keyboard brightness
  currentBrightness=$(cat /sys/class/leds/*::kbd_backlight/brightness | head -n 1)
  echo "$currentBrightness" >$saveFile
  echo "0" | tee /sys/class/leds/*::kbd_backlight/brightness | head -n 1
else
  # Restore the saved keyboard brightness
  savedBrightness=$(<"$saveFile")
  echo "$savedBrightness" | tee /sys/class/leds/*::kbd_backlight/brightness | head -n 1
  rm $saveFile
fi
