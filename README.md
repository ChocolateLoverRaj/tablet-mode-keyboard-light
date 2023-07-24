# tablet-mode-keyboard-light
Automatically turn off the keyboard backlight when a spin laptop is in tablet mode (for Linux). It turns on the light back to how it was before once you rotate back into laptop mode.

## Tested Devices
- jinlon (HP Elite c1030 Chromebook)

## Install
### Install `acpid`
```bash
sudo apt install acpid
```
U probably don't need `apt` for this to work, so if u use a different package manager just figure out how to install `acpid`.

### Clone this repo
```bash
git clone https://github.com/ChocolateLoverRaj/tablet-mode-keyboard-light.git
cd tablet-mode-keyboard-light
```

### Install the script
```bash
. ./install.sh
```

### (Optional) Delete the cloned repo
The folder u cloned isn't needed anymore. U can delete it if u want.

## Uninstall
### Uninstall the script
```bash
. ./uninstall.sh
```
It's ok if u get 
> rm: cannot remove '/etc/actions/tablet-mode.txt': No such file or directory

### (Optional) Delete the cloned repo
Delete the folder that has the cloned repo

### (Optional) Remove `acpid`
If ur not using `acpid` for anything else, u can uninstall it
```bash
sudo apt autoremove acpid
```
