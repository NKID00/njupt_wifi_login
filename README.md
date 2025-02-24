# NJUPT WiFi Login
A tool to help NJUPTer to connect the campus network conveniently.

## How it works?
It will listen for the network changed notifications and automatically do the authentication.

## How to use it?
### Use Configurator
*Configurator is available in v0.1.1 and afterwards.*
1. Download the binaries or build from the source on your own.
2. Open `njupt_wifi_login_configurator`, write down your account and click Save button.
3. Reboot your computer

### Manually configure
1. Download the binaries or build from the source on your own.
2. Write down your userid and password into the configuration file (eg. `njupt_wifi.yml`).
   ```yaml
   isp: CT # 移动用 CMCC，电信用 CT，南邮自身的用 EDU
   userid: "B22999999"
   password: "password123456"
   ```
3. Config to run `njupt_wifi_login` at startup and it will automatically do the rest.

## Requirements
Currently it's Windows-only since the author doesn't use Linux in desktop environments. 

For those who may be interested in Linux support, try to use `NETLINK_ROUTE` to implement the listener. The cargo package [`rtnetlink`](https://github.com/little-dude/netlink/tree/master/rtnetlink) may be helpful. 

PRs for narrowing the limitation is welcome.

## Remarks
It will use no proxy during the authentication for the proxy may be not available until the network is logged in.

It will use Google DNS Servers (in the white list of the firewall) internally to avoid dns not available during authentication.

We write it meticulously with Rust, thus you are mostly not needed to worry about the cost of performance.

## License
Licensed under [MIT license](LICENSE.txt).