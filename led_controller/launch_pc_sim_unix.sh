#!/bin/sh

# setup virtual serial and create local named symlinks
socat pty,link=tty_minicom pty,link=tty_led &
sleep 1

# launch led controller PC implementation
/usr/bin/env python3 led_controller_pc.py tty_led &

# launch minicom serial communication program
minicom -b 9600 -D $(readlink -f tty_minicom) 