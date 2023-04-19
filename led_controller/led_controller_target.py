from machine import Pin, UART
from collections import namedtuple
from time import sleep


class LedController:
    Leds = namedtuple("Leds", "r, g, b")

    def __init__(
        self,
        r: int,
        g: int,
        b: int,
        uart_id: int,
        rx: int,
        tx: int,
        baudrate: int = 9600,
    ):
        self.uart = UART(uart_id, baudrate=baudrate, tx=Pin(tx), rx=Pin(rx))
        self.uart.init(timeout=0)

        self.leds = LedController.Leds(
            Pin(r, Pin.OUT),
            Pin(g, Pin.OUT),
            Pin(b, Pin.OUT),
        )

        self.commands = {
            ("led", "red"): lambda: self.set_led_pins(True, False, False),
            ("led", "green"): lambda: self.set_led_pins(False, True, False),
            ("led", "blue"): lambda: self.set_led_pins(False, False, True),
        }

    def set_led_pins(self, r: bool, g: bool, b: bool) -> bytes:
        self.leds.r.set_value(r)
        self.leds.g.set_value(g)
        self.leds.b.set_value(b)
        return b"done"

    def parse_line(self, line: str) -> None:
        key = tuple(item.lower().strip() for item in line.split(" "))
        print(key)
        command = self.commands.get(key, lambda: b"invalid command\r\n")
        self.uart.write(command())

    def run(self) -> None:
        self.uart.write(b"Hallo")
        while True:
            line = self.uart.readline()
            if line is not None:
                self.parse_line(line.decode("utf-8"))
            else:
                sleep(1)


"""
Pin Wiring

GP0: UART0 TX
GP1: UART0 RX

GP15: blue
GP16: green
GP17: red
"""
controller = LedController(17, 16, 15, 0, 1, 0)
controller.run()
