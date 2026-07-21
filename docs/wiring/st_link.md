# ST-Link Wiring

This document describes how to connect an ST-Link V2 programmer to the STM32F411 Black Pill for flashing and debugging firmware.

## Connections

| ST-Link | STM32 Black Pill |
|---------|------------------|
| 3.3V    | 3V3              |
| GND     | GND              |
| SWDIO   | DIO              |
| SWCLK   | SCK              |

> **Note:** On the Black Pill, the SWD header labels the clock pin as **SCK**. This is the same signal as **SWCLK**.

## Black Pill SWD Header

The four-pin programming header is located on the edge of the board.

```text
Top
● GND
● SCK   (SWCLK)
● DIO   (SWDIO)
● 3V3
Bottom
```

## ST-Link Notes

- Connect **3.3V**, **not 5V**.
- The ST-Link powers the board while flashing.
- An optional **NRST/RST** connection is not required for normal flashing.

## Verify Connection

Check that the ST-Link is detected:

```bash
probe-rs list
```

Expected output:

```text
The following debug probes were found:
[0]: STLink V2 ...
```

## Flash Firmware

```bash
cargo run
```

This will:

1. Build the firmware.
2. Flash it to the STM32.
3. Reset the microcontroller.
4. Start the program.

## After Flashing

The firmware is stored in the STM32's internal flash memory.

After disconnecting the ST-Link, the program will automatically start whenever the board is powered from:

- USB-C
- The ST-Link
- Any valid external power source

The ST-Link is only needed when flashing new firmware or debugging.