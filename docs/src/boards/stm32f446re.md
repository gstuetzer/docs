# STM32F446RE

High-performance foundation line, `ARM Cortex-M4` core with DSP and FPU, 512 Kbytes Flash, 180 MHz CPU, ART Accelerator, Dual QSPI 

[ Hersteller Homepage](https://www.st.com/en/microcontrollers/stm32f446re.html)

## Datenblatt

- Core: ARM® 32-bit Cortex®-M4 CPU with FPU, Adaptive real-time accelerator (ART Accelerator™) allowing 0-wait state execution from Fl ash memory, frequency up to 180 MHz, MPU, 225 DMIPS/1.25 DMIPS/MHz (Dhrystone 2.1), and DSP instructions
- Memories
    - 512 kB of Flash memory
    - 128 KB of SRAM
    - Flexible external memory controller with up to 16-bit data bus: SRAM,PSRAM,SDRAM/LPSDR SDRAM, Flash NOR/NAND memories
    - Dual mode Quad SPI interface
- LCD parallel interface, 8080/6800 modes
- Clock, reset and supply management
    - 1.7 V to 3.6 V application supply and I/Os
    - POR, PDR, PVD and BOR
    - 4-to-26 MHz crystal oscillator
    - Internal 16 MHz factory-trimmed RC (1% accuracy)
    - 32 kHz oscillator for RTC with calibration
    - Internal 32 kHz RC with calibration
- Low power
    - Sleep, Stop and Standby modes
    - VBATsupply for RTC, 20×32 bit backup registers + optional 4 KB backup SRAM
- 3×12-bit, 2.4 MSPS ADC: up to 24 channels and 7.2 MSPS in triple interleaved mode
- 2×12-bit D/A converters
- General-purpose DMA: 16-stream DMA controller with FIFOs and burst support
- Up to 17 timers: 2x watchdog, 1x SysTick timer and up to twelve 16-bit and two 32-bit timers up to 180 MHz, each with up to 4 IC/OC/PWM or pulse counter
- Debug mode
    - SWD & JTAG interfaces
    - Cortex®-M4 Trace Macrocell™
- Up to 114 I/O ports with interrupt capability
    - Up to 111 fast I/Os up to 90 MHz
    - Up to 112 5 V-tolerant I/Os
- Up to 20 communication interfaces
    - SPDIF-Rx
    - Up to 4 × I2C interfaces (SMBus/PMBus)
    - Up to 4 USARTs/2 UARTs (11.25 Mbit/s, ISO7816 interface, LIN, IrDA, modem control)
    - Up to 4 SPIs (45 Mbits/s), 3 with muxed I2S for audio class accuracy via internal audio PLL or external clock
    - 2 x SAI (serial audio interface)
    - 2 × `CAN` (2.0B Active)
    - SDIO interface
    - Consumer electronics control (CEC) I/F
- Advanced connectivity
    - USB 2.0 full-speed device/host/OTG controller with on-chip PHY
    - USB 2.0 high-speed/full-speed device/host/OTG controller with dedicated DMA, on-chip full-speed PHY and ULPI
    - Dedicated USB power rail enabling on-chip PHYs operation throughout the entire MCU power supply range
- 8- to 14-bit parallel camera interface up to 54 Mbytes/s
- CRC calculation unit
- RTC: subsecond accuracy, hardware calendar
- 96-bit unique ID