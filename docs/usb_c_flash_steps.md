## Step 1

### Build and convert the firmware

Run this command from the project directory:

```bash
cargo objcopy --bin garage-monitor-firmware -- -O binary garage-monitor-firmware.bin
```

## Step 2

### Put the STM32 into DFU mode

Keep the ST-Link disconnected.

1. Connect the Black Pill to your Mac through its USB-C port.
2. Hold **BOOT0**.
3. Press and release **NRST**.
4. Release **BOOT0**.

Optionally, verify that the board is in DFU mode:

```bash
dfu-util -l
```

## Step 3

### Flash the firmware

```bash
dfu-util -d 0483:df11 -a 0 -s 0x08000000:leave -D garage-monitor-firmware.bin
```

The firmware should start automatically after flashing. If it does not, press and release **NRST** without holding **BOOT0**.

## TL;DR

```text
Edit code
   ↓
Run cargo objcopy
   ↓
Hold BOOT0
   ↓
Press and release NRST
   ↓
Release BOOT0
   ↓
Run the dfu-util flash command
   ↓
Firmware runs
```