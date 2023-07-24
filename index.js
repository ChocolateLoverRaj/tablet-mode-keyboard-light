#!/usr/bin/env node
// To get hinge angle: /sys/bus/iio/devices/iio:device3/in_angl_raw
// To set keyboard brightness: echo 100 > /sys/class/leds/chromeos::kbd_backlight/brightness
import { readFileSync, writeFileSync } from 'fs'
import { globSync } from 'glob'

const [lidAngleFile] = globSync('/sys/bus/iio/devices/iio:device*/in_angl_raw')
const [kbLightFile] = globSync('/sys/class/leds/*::kbd_backlight/brightness')

console.log('Watching file:', lidAngleFile)
console.log('Detected keyboard brightness file:', kbLightFile)

let kbLightToRemember
setInterval(() => {
  const lidAngle = parseInt(readFileSync(lidAngleFile, 'utf8'))
  if (lidAngle > 180) {
    if (kbLightToRemember === undefined) {
      kbLightToRemember = parseInt(readFileSync(kbLightFile, 'utf8'))
      console.log(`Turning off keyboard light. Remembering current brightness: ${kbLightToRemember}%`)
      writeFileSync(kbLightFile, (0).toString(), 'utf8')
    }
  } else {
    if (kbLightToRemember !== undefined) {
      console.log(`Turning on keyboard light. Setting brightness to ${kbLightToRemember}%`)
      writeFileSync(kbLightFile, kbLightToRemember.toString(), 'utf8')
      kbLightToRemember = undefined
    }
  }
}, 100)
