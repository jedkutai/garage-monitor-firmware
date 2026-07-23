# ST-Link Wiring

## Wiring

```text
ST-Link 3.3V  → STM32 3V3
ST-Link GND   → STM32 GND
ST-Link SWDIO → STM32 DIO
ST-Link SWCLK → STM32 SCK
```

## Connection Details

### 3.3V
Powers the STM32 during programming and provides the voltage reference for the Serial Wire Debug (SWD) interface.

### GND
Provides a common electrical reference between the ST-Link and the STM32. Both devices must share the same ground for debugging and programming to work correctly.

### SWDIO
The Serial Wire Debug (SWD) data line. Used by the ST-Link to transfer firmware, communicate with the STM32, and control the debugger.

### SWCLK
The Serial Wire Debug (SWD) clock line. Synchronizes communication between the ST-Link and the STM32 during programming and debugging.