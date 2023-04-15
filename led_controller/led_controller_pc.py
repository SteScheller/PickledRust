#!/usr/bin/env python3

import argparse
import serial


class LedController:
    COMMANDS = {
        ("led", "on"): lambda: b"ON\r\n",
        ("led", "off"): lambda: b"OFF\r\n",
    }

    def __init__(self, port: str, baudrate: int = 9600):
        s = serial.Serial()
        s.port = port
        s.baudrate = baudrate

        self.serial = s

    def parse_line(self, line: str) -> None:
        key = tuple(item.lower().strip() for item in line.split(" "))
        command = LedController.COMMANDS.get(key, lambda: b"invalid command\r\n")
        self.serial.write(command())

    def run(self) -> None:
        try:
            self.serial.open()
            while self.serial.is_open:
                line = self.serial.read_until(b"\r", 80)
                self.parse_line(line.decode())
        except serial.SerialException as e:
            print(f"Failed to open port: {e}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="LED controller terminal interface")
    parser.add_argument(
        "port", metavar="PORT", type=str, help="tty port to be used by the controller"
    )

    args = parser.parse_args()
    LedController(args.port).run()
