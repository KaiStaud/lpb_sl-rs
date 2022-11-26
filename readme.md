export DATABASE_URL="sqlite:todos.db"
$ sqlx db create
$ sqlx migrate run
cargo run -- add "todo description"
## Start Roudi:
find target -type f -wholename "*/iceoryx-install/bin/iox-roudi" -exec {} \;
## Start SL Server in lpb_sl-rs/target/debug:
./lpb-sl
## Start CLI
./lpb-cli -h

# GPIO-Setup
| Button | GPIO  | GPIO    | #Header | gpio-cdev        |      |
|--------|-------|---------|---------|------------------|------|
| Up     | P8_15 | GPIO_47 | 3       | gpiochip2:line1  |  y   |
| Down   | P8_18 | GPIO_65 | 8       | gpiochip2:line1  |  y   |
| Right  | P8_16 | GPIO_46 | 7       | gpiochip1:line14 |  y   |
| Left   | P8_17 | GPIO_27 | 9       | gpiochip0:line27 |  y   |
| Map1   | P8_14 | GPIO_26 | 6       | gpiochip0:line26 |  y   |
| Map4   | P8_8  | GPIO_67 | 3       | gpiochip2:line3  |  y   |
| Map3   | P8_10 | GPIO_68 | 5       | gpiochip2:line4  |  y   |
| Map2   | P8_12 | GPIO_44 | 1       | gpiochip1:line12 |  y   |
| Map5   | P8_15 | GPIO_47 | 10      | gpiochip1:line15 | n.c. |

# CAN-Setup
| Signal | GPIO  | GPIO    | #Header |
|--------|-------|---------|---------|
| TXC1   | P9_20 | GPIO_67 | 1       |
| RXC1   | P9_19 | GPIO_68 | 2       |
| TXC2   | P9_26 | GPIO_44 | 3       |
| RXC2   | P8_24 | GPIO_26 | 4       |      |

config-pin p9.24 can
config-pin p9.26 can 
config-pin p9.19 can
config-pin p9.20 can
sudo /sbin/ip link set can1 up type can bitrate 1000000  
sudo /sbin/ip link set can0 up type can bitrate 1000000

# Display-Setup
| Signal      | GPIO  | GPIO    | #Header |
|-------------|-------|---------|---------|
| SCL-i2c1    | P9_17 | GPIO_xx | 1       |
| SDA-i2c1    | P9_18 | GPIO_xx | 2       |
| Reset       | P8_08 | GPIO_67 | 2       |
| SCL-i2c2    | P9_21 | GPIO_xx | 3       |
| SDA-i2c2    | P9_22 | GPIO_xx  | 4       |
due to pinoverlap w/ dcan1 the following bus is used: i2c-1 Mem-Address =0x4802_A000
i2c-2 is assigned to communicate with the external rtc.
## Checking GPIO
export Pins:
`echo 16 > /sys/class/gpio/export`
`echo "out" > /sys/class/gpio/gpio16/direction`
`echo 1 > /sys/class/gpio/gpio16/value`