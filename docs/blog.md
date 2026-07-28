# Garage Sensor Progress

## Phase 1: Initial Setup and Blink Test

### 7/21 — 15:00
It took a while but I set up the enviroment to run with no_std and no_main.  
Found a great [repo](https://github.com/stm32-rs/stm32f4xx-hal) to help with that.

Connected [ST-LINK V2](Stage_1/vt_linker_v2.JPG)  
[ST-LINK V2 Wiring](wiring/st_link.md)

Blink test was succesful.  
Light [on](Stage_1_/light_on.JPG)  
Light [off](Stage_1_/light_off.JPG)  

---

## Phase 2: UART Logging

### 7/22 — 16:00
I didn't have a USB to TTL addapter so I had to wait for one to be delivered.  
I got it [connected](Stage_2/uart_logging.JPG) and tested some slop code on it and it worked.

---

## Phase 3: BME280 Setup

### 7/22 — 23:00
Connected the [BME280](Stage_3/bme280_wired.jpg)  
Testing the setup

Also updated the wiring docs.

I am having an issue where I keep getting this message:  
ERROR nusb::platform::macos_iokit::device: Failed to submit Out transfer 0x8885e3480 of len 16 on endpoint 02: e000404f
Error: Failed to open probe: Failed to open the debug probe.

### 7/23 — 12:30
The bug may be hardware related. 
I am using these awful adapters because I can't find a USB-C hub.  
One should be arriving in the morning/afternoon.

### 7/23 — 15:00
The usb hub arrived.  
Cargo run worked once then went back to giving me the same error.  
I am losing my mind. Been here for an hour replugging everything.


Well good news and bad news.
I got the st link to work by unplugging the usb hub everytime before use.
The bad news is the BME280 doesn't work. I get this error.

Garage monitor starting...
Failed to communicate with BME280

I don't have the pins soldered (because I can't solder).
Looking for one that comes pre soldered I guess.

Hardware hates me.

### 7/23 — 16:30
Going to try to solder tonight (have no faith that it will work)  
Ordered a sensor that is pre soldered that will arrive tomorrow.

### 7/24 — 20:00
Sensor is here. Have a bit of time before I have to go to work.

### 7/24 — 20:20
It doesn't work either. So the sensor isn't the problem.  
It is most likely my code.

### 7/25 — 17:00
Can't really explain how but everything works now.  
Unplugged and replugged for the 800th time and it decided to work.

### 7/27 — 18:00
I have made zero progress. Just getting error messages.
Don't know what to do, I have tried reddit chatgpt stackoverflow etc.
Wish I knew someone that was semi familiar with this stuff.

### 7/28 — 01:30
I am switching to flashing via [usbc](Stage_3/usb_c_flash.jpg), wish me luck.
It worked. The steps are [here](Stage_3/usb_c_flash_steps.md).
---