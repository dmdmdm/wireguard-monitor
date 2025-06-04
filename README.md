# wireguard-monitor
See what Wireguard is doing.

It runs 3 Wireguard-related commands in a TUI (terminal user interface).
- wg show: updated every second
- ifconfig wg0: updated every second
- tcpdump -i wg0: continuous scroll

Not very amazing but hopefully useful for somebody.

# Screenshot
![Screenshot](wireguard-monitor.png)

# To use
Linux only

    git clone https://github.com/dmdmdm/wireguard-monitor
    cd wireguard-monitor
    cargo run

# Option
By default it uses interface `wg0` but you can specify a different interface on the command line, eg:

    cargo run wg4

# Friendly peer names
If you have file /etc/wireguard/peers setup with

    [public-key1]:[friendly-name1]
    [public-key2]:[friendly-name2]
    ...

Our 'wg' window will display the friendly names like wgg - found here https://github.com/FlyveHest/wg-friendly-peer-names
