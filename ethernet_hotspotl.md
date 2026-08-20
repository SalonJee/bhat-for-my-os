# useful if living in Hostel and want easier hotspot access ( in Ubuntu)

## First confirm the hotspot's Profile name : 

```bash
nmcli connection show
```
mine was "Hotspot"

## Enable autonnect
```bash 
sudo nmcli connection modify Hotspot connection.autoconnect yes
```

## OPTIONAL : set up priority 
```bash
sudo nmcli connection modify Hotspot connection.autoconnect-priority 100
```

## Give permission to not have to log in after booting up 

```bash
sudo nmcli connection modify Hotspot connection.permissions ""
```

# Suggestions 
 ## Key bind for manual hotspot on/off toggle 

 key bind manual hotspot off to something ( super + O)

```bash -c
 "nmcli connection down Hotspot"
 ```


 key bind manual hotspot on to something ( super + H)

```bash -c 
"nmcli radio wifi on && sleep 1 && nmcli connection up Hotspot"
 ```

(for when youre using Public wifi and not ethernet-hotspot)