from machine import Pin, UART
from collections import namedtuple
from time import sleep_ms
import gc


class LedController:
    Leds = namedtuple("Leds", ("r", "g", "b"))
    COMMAND_DELIMITER = b"\r"

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
            ("led", "off"): lambda: self.set_led_pins(False, False, False),
        }

    def set_led_pins(self, r: bool, g: bool, b: bool) -> bytes:
        self.leds.r.value(r)
        self.leds.g.value(g)
        self.leds.b.value(b)
        return b"done\r\n"

    def execute_command(self, command_string: str) -> None:
        key = tuple(item.lower().strip() for item in command_string.split(" "))
        command = self.commands.get(key, lambda: b"invalid command\r\n")
        self.uart.write(command())

    def receive_line_blocking(self) -> str:
        loop = True
        rxBytes = b""
        while loop:
            if self.uart.any():
                rxBytes += self.uart.read()
                loop = LedController.COMMAND_DELIMITER not in rxBytes
            else:
                sleep_ms(1)

        line = rxBytes.split(LedController.COMMAND_DELIMITER)[0].decode("utf-8")

        return line

    def run(self) -> None:
        while True:
            command = self.receive_line_blocking()
            self.execute_command(command)
            gc.collect()


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
