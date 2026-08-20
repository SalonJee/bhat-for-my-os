# Connect to AirPods via Keybind (No Scanning)

End-to-end steps to bind a key that instantly connects to an already-paired Bluetooth device.

## 1. Check your BlueZ version

Different versions support different `bluetoothctl` commands.

```bash
bluetoothctl --version
```

## 2. Find your device's MAC address

Since your `bluetoothctl` doesn't support `paired-devices`, try these in order until one works:

```bash
bluetoothctl devices Paired
```

If that errors with "Invalid command":

```bash
bluetoothctl devices
```

Look for a line like:

```
Device AA:BB:CC:DD:EE:FF AirPods Pro
```

Copy that MAC address (`AA:BB:CC:DD:EE:FF`) — you'll hardcode it below.

## 3. Test the connect command manually

Run this in your terminal first to confirm it works before wiring it to a key:

```bash
bluetoothctl power on && bluetoothctl connect AA:BB:CC:DD:EE:FF
```

Replace `AA:BB:CC:DD:EE:FF` with your actual MAC. You should see `Connection successful` and your AirPods should connect.

> If your AirPods are currently connected to another device (phone, etc.), this will force-switch them over — that's normal.

# But did not work because my bluetooth radio was blocked 

You can check it and confirm it by : 

```bash
rfkill list bluetooth
```

if it says, " soft blocked : yes"

Below is the fix !

## The Fix

I first unblocked all the bluetooth radio 

```bash
rfkill unblock bluetooth
```

After that , i ran the command above then it worked !

## complete one-liner 

```bash
rfkill unblock bluetooth && bluetoothctl power on && bluetoothctl connect 3A:0D:7B:BE:B8:64
```

## or to be safe, unblock all radio interface

```bash
rfkill unblock all && bluetoothctl power on && bluetoothctl connect 3A:0D:7B:BE:B8:64
```


## my keybinding

Binding it with "ctrl + P" , to run that bash command auto.