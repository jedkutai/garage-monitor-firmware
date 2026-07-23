# UART Wiring

## Wiring

```text
UART RXD → STM32 pin A9
UART GND → STM32 GND
```

## Connection Details

### RXD
Receives serial data transmitted by the STM32 on pin A9. The UART adapter converts this serial data into USB so it can be displayed on the connected computer.

### GND
Provides a common electrical reference between the UART adapter and the STM32. Both devices must share the same ground for reliable serial communication.