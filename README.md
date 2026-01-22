# stm32-data generated output

This repo contains generated output for [`stm32-data`](https://github.com/embassy-rs/stm32-data). It is updated for every push to the `main` branch. See the `stm32-data` README for more details.

## STM32 Peripheral Support Matrix

The following table shows which STM32 peripheral versions are supported across different families:

## Peripheral support by family

| Peripheral | C0 | F0 | F1 | F2 | F3 | F4 | F7 | G0 | G4 | H5 | H7 | L0 | L1 | L4 | L4+ | L5 | N6 | U0 | U3 | U5 | WB | WB0 | WBA | WL |
|------------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|--------|
| [ADC](#adc) | c0 | v1, ❌ | f1, ❌ | v2 | f3v1, f3v3 | v2 | v2 | g0 | g4 | h5 | h7rs, v4 | l0, ❌ | f3v2, ❌ | v3 | v3 | v3 | ❌ | u0 | u3 | u5 | v3, ❌ | ❌ | wba | g0 |
| [ADCCOMMON](#adccommon) | c0 | | | v2 | f3 | v2 | v2 | v3 | g4 | h5, h50 | h5, v4 | | | v3 | v3 | v3 | | v3 | u3 | u5 | v3 | | v3 | v3 |
| [ADF](#adf) | | | | | | | | | | | ❌ | | | | | | ❌ | | ❌ | v1 | | | | |
| [AES](#aes) | | | | | | v1 | f7 | v2 | v2 | v3b | | v1 | v1 | v1 | v1 | v2 | | v2 | ❌ | v3a | ❌ | | v3b | v2 |
| [AFIO](#afio) | | | f1 | | | | | | | | | | | | | | | | | | | | | |
| [BDMA](#bdma) | v1 | v1, v2 | v1 | | v1 | | | v1 | v1 | | v1 | v2 | v1 | v2 | v1 | v1 | | v1 | | | v1 | v1 | | v1 |
| [BKP](#bkp) | | | v1 | | | | | | | | | | | | | | | | | | | | | |
| [BSEC](#bsec) | | | | | | | | | | | | | | | | | v2 | | | | | | | |
| [CACHEAXI](#cacheaxi) | | | | | | | | | | | | | | | | | ❌ | | | | | | | |
| [CAN](#can) | fdcan_v1 | bxcan | bxcan | bxcan | bxcan | bxcan | bxcan | fdcan_v1 | fdcan_v1 | fdcan_v1 | fdcan_h7, fdcan_v1 | | | bxcan | bxcan | fdcan_v1 | | | fdcan_v1 | fdcan_v1 | | | | |
| [CCB](#ccb) | | | | | | | | | | | | | | | | | | | ❌ | | | | | |
| [CEC](#cec) | | v2 | v1 | | v2 | v2 | v2 | v2 | | v2 | v2 | | | | | | | | | | | | | |
| [COMP](#comp) | | ❌ | | | f3_v1, ❌ | | | v1 | v2 | h5 | h7_a, h7_b | ❌ | ❌ | v3 | v3 | v3 | | u0 | u3 | u5 | ❌ | | u5 | v3 |
| [CORDIC](#cordic) | | | | | | | | | v1 | v1 | v1 | | | | | | | | | v1 | | | | |
| [CRC](#crc) | v3 | v2, v3 | v1 | v1 | v3 | v1 | v3 | v3 | v3 | v3 | v3 | v3 | v1 | v3 | v3 | v3 | v3 | v3 | v3 | v3 | v3 | v2 | v3 | v3 |
| [CRS](#crs) | v1 | v1 | | | | | | v1 | v1 | v1 | v1 | v1 | | v1 | v1 | v1 | | v1 | v1 | v1 | v1 | | | |
| [CRYP](#cryp) | | | | v1 | | v1, v2 | v2 | | | | v3, v4 | | | | | | v2 | | | | | | | |
| [CSI](#csi) | | | | | | | | | | | | | | | | | ❌ | | | | | | | |
| [DAC](#dac) | | v2 | v1, v2 | v2 | v2 | v2 | v2 | v4 | v7 | v6 | v4 | v2 | v2 | v3 | v5 | v5 | | v4 | v6 | v6 | | | | v4 |
| [DBGMCU](#dbgmcu) | c0 | f0 | f1 | f2 | f3 | f4 | f7 | g0 | g4 | h5 | h7 | l0 | l1 | l4 | l4 | l5 | n6 | u0 | u3 | u5 | wb | | wba | wl |
| [DCACHE](#dcache) | | | | | | | | | | v1 | | | | | | | | | | v1 | | | | |
| [DCMI](#dcmi) | | | | v1 | | v1 | v1 | | | v1 | v1 | | | v1 | v1 | | v1 | | | v1 | | | | |
| [DCMIPP](#dcmipp) | | | | | | | | | | | ❌ | | | | | | ❌ | | | | | | | |
| [DESIG](#desig) | | | | | | | | | | | | | | | | | | | | | | | wba | |
| [DFSDM](#dfsdm) | | | | | | ❌ | ❌ | | | | ❌ | | | ❌ | ❌ | | | | | | | | | |
| [DMA](#dma) | | | | v2 | | v2 | v2 | | | | v1 | | | | | | | | | | | | | |
| [DMA2D](#dma2d) | | | | | | v1 | v1 | | | | v2 | | | v1 | v1 | | v1 | | | v2 | | | | |
| [DMAMUX](#dmamux) | v1 | | | | | | | v1 | v1 | | v1 | | | | v1 | v1 | | v1 | | | v1 | v1 | | v1 |
| [DSIHOST](#dsihost) | | | | | | v1 | v1 | | | | v1 | | | | v1 | | | | | u5 | | | | |
| [DTS](#dts) | | | | | | | | | | v1 | v1 | | | | | | v1 | | | | | | | |
| [ETH](#eth) | | | v1a | v1b | | v1b | v1c | | | v2 | v2 | | | | | | ❌ | | | | | | | |
| [EXTI](#exti) | c0 | v1 | v1 | v1 | v1 | v1 | v1 | g0 | v1 | h5, h50 | h7 | v1 | v1 | v1 | v1 | l5 | n6 | u0 | u3 | u5 | w | | l5 | w, wle |
| [FDCAN](#fdcan) | | | | | | | | | | | | | | | | | ❌ | | | | | | | |
| [FDCANRAM](#fdcanram) | v1 | | | | | | | v1 | v1 | v1 | h7, v1 | | | | | v1 | v1 | | v1 | v1 | | | | |
| [FLASH](#flash) | c0 | f0 | f1 | f2 | f3 | f4 | f7 | g0x0, g0x1 | g4c2, g4c3, g4c4 | h5, h50 | h7, h7ab, h7rs | l0 | l1 | l4 | l4 | l5 | | u0 | u3 | u5 | wb | | wba | wl |
| [FMAC](#fmac) | | | | | | | | | v1 | v1 | v1 | | | | | | | | | v1 | | | | |
| [FMC](#fmc) | | | | | ❌ | v1x3, v2x1, ❌ | v2x1 | | ❌ | v4 | v3x1 | | | ❌ | ❌ | ❌ | n6 | | | ❌ | | | | |
| [FMPI2C](#fmpi2c) | | | | | | v2 | | | | | | | | | | | | | | | | | | |
| [FSMC](#fsmc) | | | v1x0, v1x3 | v1x3 | | v1x0, v1x3 | | | | | | | v1x0 | | | | | | | | | | | |
| [GFXMMU](#gfxmmu) | | | | | | | | | | | ❌ | | | | v1 | | ❌ | | | v2 | | | | |
| [GFXTIM](#gfxtim) | | | | | | | | | | | ❌ | | | | | | ❌ | | | ❌ | | | | |
| [GPDMA](#gpdma) | | | | | | | | | | v1 | v1 | | | | | | v1 | | v1 | v1 | | | v1 | |
| [GPIO](#gpio) | v2 | v2 | v1 | v2 | v2 | v2 | v2 | v2 | v2 | v2 | v2 | v2 | v2 | v2, v2_l478 | v2 | v2 | v2 | v2 | v2 | v2 | v2 | v1 | v2 | v2 |
| [GPU](#gpu) | | | | | | | | | | | ❌ | | | | | | ❌ | | | ❌ | | | | |
| [GTZC](#gtzc) | | | | | | | | | | h503, v1 | | | | | | ❌ | | | ❌ | v1 | | | wba | |
| [HASH](#hash) | | | | v1 | | v1, v2 | v2 | | | v3 | v2, v3 | | | v2 | v2 | v2 | v3 | | v3 | v4 | | | v4 | |
| [HRTIM](#hrtim) | | | | | v1 | | | | v2 | | v1 | | | | | | | | | | | | | |
| [HSEM](#hsem) | | | | | | | | | | | v1, v2, ❌ | | | | | | | | | | v1, ❌ | | ❌ | v3, v4 |
| [HSPI](#hspi) | | | | | | | | | | | | | | | | | | | | v1 | | | | |
| [I2C](#i2c) | v2 | v2 | v1 | v1 | v2 | v1 | v2 | v2 | v2 | v2 | v2, v3 | v2 | v1 | v2 | v2 | v2 | v3 | v2 | v2 | v2 | v2 | v1 | v2 | v2 |
| [I3C](#i3c) | | | | | | | | | | v1 | ❌ | | | | | | ❌ | | v1 | | | | | |
| [ICACHE](#icache) | | | | | | | | | | v1_0crr, v1_4crr, ❌ | | | | | | v1_4crr | v1_0crr | | v1_3crr | v1_3crr | | | v1_4crr | |
| [IPCC](#ipcc) | | | | | | | | | | | | | | | | | | | | | v1 | | | v1 |
| [IWDG](#iwdg) | v2 | v2 | v1 | v1 | v2 | v1 | v2 | v2 | v2 | v3 | v2, v3 | v2 | v1 | v2 | v2 | v2 | v3 | v3 | v3 | v3 | v2 | v2 | v3 | v2 |
| [JPEG](#jpeg) | | | | | | | v1 | | | | v1 | | | | | | v1 | | | v1 | | | | |
| [LCD](#lcd) | | | | | | | | | | | | v2 | v1 | v2 | | | | v2 | | | v2 | | | |
| [LPDMA](#lpdma) | | | | | | | | | | | | | | | | | | | | v1 | | | | |
| [LPGPIO](#lpgpio) | | | | | | | | | | | | | | | | | | | | ❌ | | | | |
| [LPTIM](#lptim) | | | | | | v1a | v1a | v1b | v1b_g4 | v2a | v1b_h7 | v1 | | v1a | v1b | v1c | ❌ | v2b | v2a | v2a | v1b | | v2a | v1c |
| [LPUART](#lpuart) | | | | | | | | | | | | | | | | | ❌ | | | | | | | |
| [LTDC](#ltdc) | | | | | | v1 | v1 | | | | v1 | | | | v1, ❌ | | ❌ | | | v1 | | | | |
| [MCE](#mce) | | | | | | | | | | | ❌ | | | | | | ❌ | | | | | | | |
| [MDF](#mdf) | | | | | | | | | | | | | | | | | ❌ | | | ❌ | | | | |
| [MDIOS](#mdios) | | | | | | | v1 | | | | v1 | | | | | | v1 | | | | | | | |
| [MDMA](#mdma) | | | | | | | | | | | v1 | | | | | | | | | | | | | |
| [OCTOSPI](#octospi) | | | | | | | | | | v2 | v1 | | | | v1 | v2 | | | ❌ | v1 | | | | |
| [OCTOSPIM](#octospim) | | | | | | | | | | | v1 | | | | v1 | | | | | v1 | | | | |
| [OPAMP](#opamp) | | | | | v2 | | | | v5 | v4 | v4 | | ❌ | v3 | v3 | v3 | | v3 | v3 | v3 | | | | |
| [OTFDEC](#otfdec) | | | | | | | | | | v1 | v1 | | | | | v1 | | | | v1 | | | | |
| [OTG](#otg) | | | v1 | v1 | | v1 | v1 | | | | v1 | | | v1 | v1 | | v1 | | | v1 | | | v1 | |
| [PKA](#pka) | | | | | | | | | | v1a | ❌ | | | | v1c | v1c | ❌ | | ❌ | v1b | v1c | v1c | v1a | v1c |
| [PSSI](#pssi) | | | | | | | | | | v1 | v1 | | | | v1 | | v1 | | | v1 | | | | |
| [PWR](#pwr) | c0 | f0, f0x0 | f1 | f2 | f3 | f4 | f7 | g0 | g4 | h5, h50 | h7rm0399, h7rm0433, h7rm0455, h7rm0468, h7rs | l0 | l1 | l4 | l4 | l5 | n6 | u0 | u3 | u5 | wb, wb55 | wb | wba | wl5 |
| [QUADSPI](#quadspi) | | | | | | v1 | v1 | | v1 | | v1 | | | v1 | | | | | | | v1 | | | |
| [RADIO](#radio) | | | | | | | | | | | | | | | | | | | | | | ❌ | | |
| [RAMCFG](#ramcfg) | | | | | | | | | | h5 | | | | | | | h5 | | u5 | u5 | | | wba | |
| [RAMECC](#ramecc) | | | | | | | | | | | ❌ | | | | | | | | | | | | | |
| [RCC](#rcc) | c0, c0v2 | f0v1, f0v2, f0v3, f0v4 | f1, f100, f1cl | f2 | f37, f3v1, f3v2, f3v3 | f4, f410 | f7 | g0x0, g0x1 | g4 | h5, h50 | h7, h7ab, h7rm0433, h7rs | l0, l0_v2 | l1 | l4 | l4plus | l5 | n6 | u0 | u3 | u5 | wb | wb0 | wba | wl5, wle |
| [RIFSC](#rifsc) | | | | | | | | | | | | | | | | | n6 | | | | | | | |
| [RISAF](#risaf) | | | | | | | | | | | | | | | | | n6 | | | | | | | |
| [RNG](#rng) | | | | v1 | | v1 | v1 | v1 | v1 | v3 | v1 | v1 | | v1 | v1, v2 | v2 | ❌ | v3 | ❌ | v3 | v1 | v1 | v3, wba6 | v2 |
| [RTC](#rtc) | v3_c0 | v2_f0 | v1 | v2_f2 | v2_f3 | v2_f4 | v2_f7 | v3_base | v3_base | v3_u5 | v2_h7, v3_h7rs | v2_l0 | v2_l1 | v2_l4, v3_l4 | v2_l4, v3_base | v3_l5 | ❌ | v3_base | v3_u3 | v3_u5 | v2_wb | v3_base | v3_u5 | v3_base |
| [SAES](#saes) | | | | | | | | | | v1a | ❌ | | | | | | ❌ | | ❌ | v1b | | | v1a | |
| [SAI](#sai) | | | | | | v1, v2 | v2 | | v4_4pdm | v4_2pdm | v3_4pdm, v4_4pdm | | | v2 | v1 | v3_2pdm | ❌ | | ❌ | v4_2pdm | v1_4pdm | | v4_2pdm | |
| [SDADC](#sdadc) | | | | | v1, ❌ | | | | | | | | | | | | | | | | | | | |
| [SDMMC](#sdmmc) | | | v1 | v1 | | v1 | v1 | | | v2 | v2 | | v1 | v1 | ❌ | v2 | ❌ | | v2 | v2 | | | | |
| [SPDIFRX](#spdifrx) | | | | | | v1 | v1 | | | | h7 | | | | | | ❌ | | | | | | | |
| [SPI](#spi) | v3_i2s | v3, v3_i2s | v1, v1_i2s | v2_i2s | v3, v3_i2s | v2_i2s | v3_i2s | v3_i2s | v3_i2s | v5_i2s | v4_i2s, v5_i2s | v2, v2_i2s | v2, v2_i2s | v3 | v3 | v3 | v5 | v3 | v6 | v6 | v3 | v3 | v6 | v3_i2s |
| [SWPMI](#swpmi) | | | | | | | | | | | ❌ | | | ❌ | | | | | | | | | | |
| [SYSCFG](#syscfg) | c0 | f0 | | f2 | f3 | f4 | f7 | g0 | g4 | h5, h50 | h7, h7od, h7rs | l0 | l1 | l4 | l4 | l5 | n6 | u0 | u3 | u5 | wb | wb | wba | wl5, wle |
| [TAMP](#tamp) | | | | | | | | g0 | g4 | h5 | ❌ | | | | | l5 | ❌ | ❌ | ❌ | u5 | | | wba | wl |
| [TIM](#tim) | | | | | | | | | | | | | | | | | ❌ | | | | | | | |
| [TIMER](#timer) | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v2 | v2 | v1 | l0 | v1 | v1 | v1 | v1 | v1 | v2 | v2 | v2 | v1 | v1 | v2 | v1 |
| [TSC](#tsc) | | v1 | | | v1, ❌ | | | | | | | v3 | | v3 | v3 | v3 | | v2 | ❌ | v3 | v2, ❌ | | v1 | |
| [UCPD](#ucpd) | | | | | | | | v1 | v1 | h5 | v1 | | | | | v1 | v1 | | | v1 | | | | |
| [UID](#uid) | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | v1 | | v1 | v1 |
| [USART](#usart) | v4 | v3 | v1, ❌ | v2, ❌ | v3 | v2 | v3 | v4 | v4 | v4 | v4 | v3 | v2, ❌ | v3 | v4 | v4 | ❌ | v4 | v4 | v4 | v4 | v4 | v4 | v4 |
| [USB](#usb) | v4 | v3 | v1 | | v1, v2 | | | v4 | v3 | v4 | | v3 | v1 | v3 | | v3 | | v4 | v4 | v4 | v3 | | | |
| [USBRAM](#usbram) | 32_2048 | 16x2_1024 | 16x1_512 | | 16x1_512, 16x2_1024, 16x2_512 | | | 32_2048 | 16x2_1024 | 32_2048 | | 16x2_1024 | 16x1_512 | 16x2_1024 | | 16x2_1024 | | 32_1024 | 32_2048 | 32_2048 | 16x2_1024 | | | |
| [VENC](#venc) | | | | | | | | | | | | | | | | | ❌ | | | | | | | |
| [VREFBUF](#vrefbuf) | | | | | | | | v1 | v2b | v2a2 | v2a1 | | | v1 | v1 | v1 | ❌ | ❌ | v2a1 | v2a1 | v1 | | v2a1 | v1 |
| [VREFINTCAL](#vrefintcal) | | | | | v1 | | v1 | | | | | | v1 | v1 | v1 | | | | | | v1 | | v2 | |
| [WWDG](#wwdg) | v2 | v1 | v1 | v1 | v1 | v1 | v1 | v2 | v2 | v2 | v2 | v1 | v1 | v1 | v1 | v2 | v2 | v2 | v2 | v2 | v2 | | v2 | v2 |
| [XSPI](#xspi) | | | | | | | | | | | v1 | | | | | | v1 | | | | | | | |
| [XSPIM](#xspim) | | | | | | | | | | | v1 | | | | | | v1 | | | | | | | |

## Detailed peripheral information

### ADC

**Versions by family:**

- **c0**: STM32C0
- **f1**: STM32F1
- **f3v1**: STM32F3
- **f3v2**: STM32L1
- **f3v3**: STM32F3
- **g0**: STM32G0, STM32WL
- **g4**: STM32G4
- **h5**: STM32H5
- **h7rs**: STM32H7
- **l0**: STM32L0
- **u0**: STM32U0
- **u3**: STM32U3
- **u5**: STM32U5
- **v1**: STM32F0
- **v2**: STM32F2, STM32F4, STM32F7
- **v3**: STM32L4, STM32L4+, STM32L5, STM32WB
- **v4**: STM32H7
- **wba**: STM32WBA
- **❌ Unsupported**: STM32F0, STM32F1, STM32L0, STM32L1, STM32N6, STM32WB, STM32WB0

### ADCCOMMON

**Versions by family:**

- **c0**: STM32C0
- **f3**: STM32F3
- **g4**: STM32G4
- **h5**: STM32H5, STM32H7
- **h50**: STM32H5
- **u3**: STM32U3
- **u5**: STM32U5
- **v2**: STM32F2, STM32F4, STM32F7
- **v3**: STM32G0, STM32L4, STM32L4+, STM32L5, STM32U0, STM32WB, STM32WBA, STM32WL
- **v4**: STM32H7

### ADF

**Versions by family:**

- **v1**: STM32U5
- **❌ Unsupported**: STM32H7, STM32N6, STM32U3

### AES

**Versions by family:**

- **f7**: STM32F7
- **v1**: STM32F4, STM32L0, STM32L1, STM32L4, STM32L4+
- **v2**: STM32G0, STM32G4, STM32L5, STM32U0, STM32WL
- **v3a**: STM32U5
- **v3b**: STM32H5, STM32WBA
- **❌ Unsupported**: STM32U3, STM32WB

### AFIO

**Versions by family:**

- **f1**: STM32F1

### BDMA

**Versions by family:**

- **v1**: STM32C0, STM32F0, STM32F1, STM32F3, STM32G0, STM32G4, STM32H7, STM32L1, STM32L4+, STM32L5, STM32U0, STM32WB, STM32WB0, STM32WL
- **v2**: STM32F0, STM32L0, STM32L4

### BKP

**Versions by family:**

- **v1**: STM32F1

### BSEC

**Versions by family:**

- **v2**: STM32N6

### CACHEAXI

**Versions by family:**

- **❌ Unsupported**: STM32N6

### CAN

**Versions by family:**

- **bxcan**: STM32F0, STM32F1, STM32F2, STM32F3, STM32F4, STM32F7, STM32L4, STM32L4+
- **fdcan_h7**: STM32H7
- **fdcan_v1**: STM32C0, STM32G0, STM32G4, STM32H5, STM32H7, STM32L5, STM32U3, STM32U5

### CCB

**Versions by family:**

- **❌ Unsupported**: STM32U3

### CEC

**Versions by family:**

- **v1**: STM32F1
- **v2**: STM32F0, STM32F3, STM32F4, STM32F7, STM32G0, STM32H5, STM32H7

### COMP

**Versions by family:**

- **f3_v1**: STM32F3
- **h5**: STM32H5
- **h7_a**: STM32H7
- **h7_b**: STM32H7
- **u0**: STM32U0
- **u3**: STM32U3
- **u5**: STM32U5, STM32WBA
- **v1**: STM32G0
- **v2**: STM32G4
- **v3**: STM32L4, STM32L4+, STM32L5, STM32WL
- **❌ Unsupported**: STM32F0, STM32F3, STM32L0, STM32L1, STM32WB

### CORDIC

**Versions by family:**

- **v1**: STM32G4, STM32H5, STM32H7, STM32U5

### CRC

**Versions by family:**

- **v1**: STM32F1, STM32F2, STM32F4, STM32L1
- **v2**: STM32F0, STM32WB0
- **v3**: STM32C0, STM32F0, STM32F3, STM32F7, STM32G0, STM32G4, STM32H5, STM32H7, STM32L0, STM32L4, STM32L4+, STM32L5, STM32N6, STM32U0, STM32U3, STM32U5, STM32WB, STM32WBA, STM32WL

### CRS

**Versions by family:**

- **v1**: STM32C0, STM32F0, STM32G0, STM32G4, STM32H5, STM32H7, STM32L0, STM32L4, STM32L4+, STM32L5, STM32U0, STM32U3, STM32U5, STM32WB

### CRYP

**Versions by family:**

- **v1**: STM32F2, STM32F4
- **v2**: STM32F4, STM32F7, STM32N6
- **v3**: STM32H7
- **v4**: STM32H7

### CSI

**Versions by family:**

- **❌ Unsupported**: STM32N6

### DAC

**Versions by family:**

- **v1**: STM32F1
- **v2**: STM32F0, STM32F1, STM32F2, STM32F3, STM32F4, STM32F7, STM32L0, STM32L1
- **v3**: STM32L4
- **v4**: STM32G0, STM32H7, STM32U0, STM32WL
- **v5**: STM32L4+, STM32L5
- **v6**: STM32H5, STM32U3, STM32U5
- **v7**: STM32G4

### DBGMCU

**Versions by family:**

- **c0**: STM32C0
- **f0**: STM32F0
- **f1**: STM32F1
- **f2**: STM32F2
- **f3**: STM32F3
- **f4**: STM32F4
- **f7**: STM32F7
- **g0**: STM32G0
- **g4**: STM32G4
- **h5**: STM32H5
- **h7**: STM32H7
- **l0**: STM32L0
- **l1**: STM32L1
- **l4**: STM32L4, STM32L4+
- **l5**: STM32L5
- **n6**: STM32N6
- **u0**: STM32U0
- **u3**: STM32U3
- **u5**: STM32U5
- **wb**: STM32WB
- **wba**: STM32WBA
- **wl**: STM32WL

### DCACHE

**Versions by family:**

- **v1**: STM32H5, STM32U5

### DCMI

**Versions by family:**

- **v1**: STM32F2, STM32F4, STM32F7, STM32H5, STM32H7, STM32L4, STM32L4+, STM32N6, STM32U5

### DCMIPP

**Versions by family:**

- **❌ Unsupported**: STM32H7, STM32N6

### DESIG

**Versions by family:**

- **wba**: STM32WBA

### DFSDM

**Versions by family:**

- **❌ Unsupported**: STM32F4, STM32F7, STM32H7, STM32L4, STM32L4+

### DMA

**Versions by family:**

- **v1**: STM32H7
- **v2**: STM32F2, STM32F4, STM32F7

### DMA2D

**Versions by family:**

- **v1**: STM32F4, STM32F7, STM32L4, STM32L4+, STM32N6
- **v2**: STM32H7, STM32U5

### DMAMUX

**Versions by family:**

- **v1**: STM32C0, STM32G0, STM32G4, STM32H7, STM32L4+, STM32L5, STM32U0, STM32WB, STM32WB0, STM32WL

### DSIHOST

**Versions by family:**

- **u5**: STM32U5
- **v1**: STM32F4, STM32F7, STM32H7, STM32L4+

### DTS

**Versions by family:**

- **v1**: STM32H5, STM32H7, STM32N6

### ETH

**Versions by family:**

- **v1a**: STM32F1
- **v1b**: STM32F2, STM32F4
- **v1c**: STM32F7
- **v2**: STM32H5, STM32H7
- **❌ Unsupported**: STM32N6

### EXTI

**Versions by family:**

- **c0**: STM32C0
- **g0**: STM32G0
- **h5**: STM32H5
- **h50**: STM32H5
- **h7**: STM32H7
- **l5**: STM32L5, STM32WBA
- **n6**: STM32N6
- **u0**: STM32U0
- **u3**: STM32U3
- **u5**: STM32U5
- **v1**: STM32F0, STM32F1, STM32F2, STM32F3, STM32F4, STM32F7, STM32G4, STM32L0, STM32L1, STM32L4, STM32L4+
- **w**: STM32WB, STM32WL
- **wle**: STM32WL

### FDCAN

**Versions by family:**

- **❌ Unsupported**: STM32N6

### FDCANRAM

**Versions by family:**

- **h7**: STM32H7
- **v1**: STM32C0, STM32G0, STM32G4, STM32H5, STM32H7, STM32L5, STM32N6, STM32U3, STM32U5

### FLASH

**Versions by family:**

- **c0**: STM32C0
- **f0**: STM32F0
- **f1**: STM32F1
- **f2**: STM32F2
- **f3**: STM32F3
- **f4**: STM32F4
- **f7**: STM32F7
- **g0x0**: STM32G0
- **g0x1**: STM32G0
- **g4c2**: STM32G4
- **g4c3**: STM32G4
- **g4c4**: STM32G4
- **h5**: STM32H5
- **h50**: STM32H5
- **h7**: STM32H7
- **h7ab**: STM32H7
- **h7rs**: STM32H7
- **l0**: STM32L0
- **l1**: STM32L1
- **l4**: STM32L4, STM32L4+
- **l5**: STM32L5
- **u0**: STM32U0
- **u3**: STM32U3
- **u5**: STM32U5
- **wb**: STM32WB
- **wba**: STM32WBA
- **wl**: STM32WL

### FMAC

**Versions by family:**

- **v1**: STM32G4, STM32H5, STM32H7, STM32U5

### FMC

**Versions by family:**

- **n6**: STM32N6
- **v1x3**: STM32F4
- **v2x1**: STM32F4, STM32F7
- **v3x1**: STM32H7
- **v4**: STM32H5
- **❌ Unsupported**: STM32F3, STM32F4, STM32G4, STM32L4, STM32L4+, STM32L5, STM32U5

### FMPI2C

**Versions by family:**

- **v2**: STM32F4

### FSMC

**Versions by family:**

- **v1x0**: STM32F1, STM32F4, STM32L1
- **v1x3**: STM32F1, STM32F2, STM32F4

### GFXMMU

**Versions by family:**

- **v1**: STM32L4+
- **v2**: STM32U5
- **❌ Unsupported**: STM32H7, STM32N6

### GFXTIM

**Versions by family:**

- **❌ Unsupported**: STM32H7, STM32N6, STM32U5

### GPDMA

**Versions by family:**

- **v1**: STM32H5, STM32H7, STM32N6, STM32U3, STM32U5, STM32WBA

### GPIO

**Versions by family:**

- **v1**: STM32F1, STM32WB0
- **v2**: STM32C0, STM32F0, STM32F2, STM32F3, STM32F4, STM32F7, STM32G0, STM32G4, STM32H5, STM32H7, STM32L0, STM32L1, STM32L4, STM32L4+, STM32L5, STM32N6, STM32U0, STM32U3, STM32U5, STM32WB, STM32WBA, STM32WL
- **v2_l478**: STM32L4

### GPU

**Versions by family:**

- **❌ Unsupported**: STM32H7, STM32N6, STM32U5

### GTZC

**Versions by family:**

- **h503**: STM32H5
- **v1**: STM32H5, STM32U5
- **wba**: STM32WBA
- **❌ Unsupported**: STM32L5, STM32U3

### HASH

**Versions by family:**

- **v1**: STM32F2, STM32F4
- **v2**: STM32F4, STM32F7, STM32H7, STM32L4, STM32L4+, STM32L5
- **v3**: STM32H5, STM32H7, STM32N6, STM32U3
- **v4**: STM32U5, STM32WBA

### HRTIM

**Versions by family:**

- **v1**: STM32F3, STM32H7
- **v2**: STM32G4

### HSEM

**Versions by family:**

- **v1**: STM32H7, STM32WB
- **v2**: STM32H7
- **v3**: STM32WL
- **v4**: STM32WL
- **❌ Unsupported**: STM32H7, STM32WB, STM32WBA

### HSPI

**Versions by family:**

- **v1**: STM32U5

### I2C

**Versions by family:**

- **v1**: STM32F1, STM32F2, STM32F4, STM32L1, STM32WB0
- **v2**: STM32C0, STM32F0, STM32F3, STM32F7, STM32G0, STM32G4, STM32H5, STM32H7, STM32L0, STM32L4, STM32L4+, STM32L5, STM32U0, STM32U3, STM32U5, STM32WB, STM32WBA, STM32WL
- **v3**: STM32H7, STM32N6

### I3C

**Versions by family:**

- **v1**: STM32H5, STM32U3
- **❌ Unsupported**: STM32H7, STM32N6

### ICACHE

**Versions by family:**

- **v1_0crr**: STM32H5, STM32N6
- **v1_3crr**: STM32U3, STM32U5
- **v1_4crr**: STM32H5, STM32L5, STM32WBA
- **❌ Unsupported**: STM32H5

### IPCC

**Versions by family:**

- **v1**: STM32WB, STM32WL

### IWDG

**Versions by family:**

- **v1**: STM32F1, STM32F2, STM32F4, STM32L1
- **v2**: STM32C0, STM32F0, STM32F3, STM32F7, STM32G0, STM32G4, STM32H7, STM32L0, STM32L4, STM32L4+, STM32L5, STM32WB, STM32WB0, STM32WL
- **v3**: STM32H5, STM32H7, STM32N6, STM32U0, STM32U3, STM32U5, STM32WBA

### JPEG

**Versions by family:**

- **v1**: STM32F7, STM32H7, STM32N6, STM32U5

### LCD

**Versions by family:**

- **v1**: STM32L1
- **v2**: STM32L0, STM32L4, STM32U0, STM32WB

### LPDMA

**Versions by family:**

- **v1**: STM32U5

### LPGPIO

**Versions by family:**

- **❌ Unsupported**: STM32U5

### LPTIM

**Versions by family:**

- **v1**: STM32L0
- **v1a**: STM32F4, STM32F7, STM32L4
- **v1b**: STM32G0, STM32L4+, STM32WB
- **v1b_g4**: STM32G4
- **v1b_h7**: STM32H7
- **v1c**: STM32L5, STM32WL
- **v2a**: STM32H5, STM32U3, STM32U5, STM32WBA
- **v2b**: STM32U0
- **❌ Unsupported**: STM32N6

### LPUART

**Versions by family:**

- **❌ Unsupported**: STM32N6

### LTDC

**Versions by family:**

- **v1**: STM32F4, STM32F7, STM32H7, STM32L4+, STM32U5
- **❌ Unsupported**: STM32L4+, STM32N6

### MCE

**Versions by family:**

- **❌ Unsupported**: STM32H7, STM32N6

### MDF

**Versions by family:**

- **❌ Unsupported**: STM32N6, STM32U5

### MDIOS

**Versions by family:**

- **v1**: STM32F7, STM32H7, STM32N6

### MDMA

**Versions by family:**

- **v1**: STM32H7

### OCTOSPI

**Versions by family:**

- **v1**: STM32H7, STM32L4+, STM32U5
- **v2**: STM32H5, STM32L5
- **❌ Unsupported**: STM32U3

### OCTOSPIM

**Versions by family:**

- **v1**: STM32H7, STM32L4+, STM32U5

### OPAMP

**Versions by family:**

- **v2**: STM32F3
- **v3**: STM32L4, STM32L4+, STM32L5, STM32U0, STM32U3, STM32U5
- **v4**: STM32H5, STM32H7
- **v5**: STM32G4
- **❌ Unsupported**: STM32L1

### OTFDEC

**Versions by family:**

- **v1**: STM32H5, STM32H7, STM32L5, STM32U5

### OTG

**Versions by family:**

- **v1**: STM32F1, STM32F2, STM32F4, STM32F7, STM32H7, STM32L4, STM32L4+, STM32N6, STM32U5, STM32WBA

### PKA

**Versions by family:**

- **v1a**: STM32H5, STM32WBA
- **v1b**: STM32U5
- **v1c**: STM32L4+, STM32L5, STM32WB, STM32WB0, STM32WL
- **❌ Unsupported**: STM32H7, STM32N6, STM32U3

### PSSI

**Versions by family:**

- **v1**: STM32H5, STM32H7, STM32L4+, STM32N6, STM32U5

### PWR

**Versions by family:**

- **c0**: STM32C0
- **f0**: STM32F0
- **f0x0**: STM32F0
- **f1**: STM32F1
- **f2**: STM32F2
- **f3**: STM32F3
- **f4**: STM32F4
- **f7**: STM32F7
- **g0**: STM32G0
- **g4**: STM32G4
- **h5**: STM32H5
- **h50**: STM32H5
- **h7rm0399**: STM32H7
- **h7rm0433**: STM32H7
- **h7rm0455**: STM32H7
- **h7rm0468**: STM32H7
- **h7rs**: STM32H7
- **l0**: STM32L0
- **l1**: STM32L1
- **l4**: STM32L4, STM32L4+
- **l5**: STM32L5
- **n6**: STM32N6
- **u0**: STM32U0
- **u3**: STM32U3
- **u5**: STM32U5
- **wb**: STM32WB, STM32WB0
- **wb55**: STM32WB
- **wba**: STM32WBA
- **wl5**: STM32WL

### QUADSPI

**Versions by family:**

- **v1**: STM32F4, STM32F7, STM32G4, STM32H7, STM32L4, STM32WB

### RADIO

**Versions by family:**

- **❌ Unsupported**: STM32WB0

### RAMCFG

**Versions by family:**

- **h5**: STM32H5, STM32N6
- **u5**: STM32U3, STM32U5
- **wba**: STM32WBA

### RAMECC

**Versions by family:**

- **❌ Unsupported**: STM32H7

### RCC

**Versions by family:**

- **c0**: STM32C0
- **c0v2**: STM32C0
- **f0v1**: STM32F0
- **f0v2**: STM32F0
- **f0v3**: STM32F0
- **f0v4**: STM32F0
- **f1**: STM32F1
- **f100**: STM32F1
- **f1cl**: STM32F1
- **f2**: STM32F2
- **f37**: STM32F3
- **f3v1**: STM32F3
- **f3v2**: STM32F3
- **f3v3**: STM32F3
- **f4**: STM32F4
- **f410**: STM32F4
- **f7**: STM32F7
- **g0x0**: STM32G0
- **g0x1**: STM32G0
- **g4**: STM32G4
- **h5**: STM32H5
- **h50**: STM32H5
- **h7**: STM32H7
- **h7ab**: STM32H7
- **h7rm0433**: STM32H7
- **h7rs**: STM32H7
- **l0**: STM32L0
- **l0_v2**: STM32L0
- **l1**: STM32L1
- **l4**: STM32L4
- **l4plus**: STM32L4+
- **l5**: STM32L5
- **n6**: STM32N6
- **u0**: STM32U0
- **u3**: STM32U3
- **u5**: STM32U5
- **wb**: STM32WB
- **wb0**: STM32WB0
- **wba**: STM32WBA
- **wl5**: STM32WL
- **wle**: STM32WL

### RIFSC

**Versions by family:**

- **n6**: STM32N6

### RISAF

**Versions by family:**

- **n6**: STM32N6

### RNG

**Versions by family:**

- **v1**: STM32F2, STM32F4, STM32F7, STM32G0, STM32G4, STM32H7, STM32L0, STM32L4, STM32L4+, STM32WB, STM32WB0
- **v2**: STM32L4+, STM32L5, STM32WL
- **v3**: STM32H5, STM32U0, STM32U5, STM32WBA
- **wba6**: STM32WBA
- **❌ Unsupported**: STM32N6, STM32U3

### RTC

**Versions by family:**

- **v1**: STM32F1
- **v2_f0**: STM32F0
- **v2_f2**: STM32F2
- **v2_f3**: STM32F3
- **v2_f4**: STM32F4
- **v2_f7**: STM32F7
- **v2_h7**: STM32H7
- **v2_l0**: STM32L0
- **v2_l1**: STM32L1
- **v2_l4**: STM32L4, STM32L4+
- **v2_wb**: STM32WB
- **v3_base**: STM32G0, STM32G4, STM32L4+, STM32U0, STM32WB0, STM32WL
- **v3_c0**: STM32C0
- **v3_h7rs**: STM32H7
- **v3_l4**: STM32L4
- **v3_l5**: STM32L5
- **v3_u3**: STM32U3
- **v3_u5**: STM32H5, STM32U5, STM32WBA
- **❌ Unsupported**: STM32N6

### SAES

**Versions by family:**

- **v1a**: STM32H5, STM32WBA
- **v1b**: STM32U5
- **❌ Unsupported**: STM32H7, STM32N6, STM32U3

### SAI

**Versions by family:**

- **v1**: STM32F4, STM32L4+
- **v1_4pdm**: STM32WB
- **v2**: STM32F4, STM32F7, STM32L4
- **v3_2pdm**: STM32L5
- **v3_4pdm**: STM32H7
- **v4_2pdm**: STM32H5, STM32U5, STM32WBA
- **v4_4pdm**: STM32G4, STM32H7
- **❌ Unsupported**: STM32N6, STM32U3

### SDADC

**Versions by family:**

- **v1**: STM32F3
- **❌ Unsupported**: STM32F3

### SDMMC

**Versions by family:**

- **v1**: STM32F1, STM32F2, STM32F4, STM32F7, STM32L1, STM32L4
- **v2**: STM32H5, STM32H7, STM32L5, STM32U3, STM32U5
- **❌ Unsupported**: STM32L4+, STM32N6

### SPDIFRX

**Versions by family:**

- **h7**: STM32H7
- **v1**: STM32F4, STM32F7
- **❌ Unsupported**: STM32N6

### SPI

**Versions by family:**

- **v1**: STM32F1
- **v1_i2s**: STM32F1
- **v2**: STM32L0, STM32L1
- **v2_i2s**: STM32F2, STM32F4, STM32L0, STM32L1
- **v3**: STM32F0, STM32F3, STM32L4, STM32L4+, STM32L5, STM32U0, STM32WB, STM32WB0
- **v3_i2s**: STM32C0, STM32F0, STM32F3, STM32F7, STM32G0, STM32G4, STM32WL
- **v4_i2s**: STM32H7
- **v5**: STM32N6
- **v5_i2s**: STM32H5, STM32H7
- **v6**: STM32U3, STM32U5, STM32WBA

### SWPMI

**Versions by family:**

- **❌ Unsupported**: STM32H7, STM32L4

### SYSCFG

**Versions by family:**

- **c0**: STM32C0
- **f0**: STM32F0
- **f2**: STM32F2
- **f3**: STM32F3
- **f4**: STM32F4
- **f7**: STM32F7
- **g0**: STM32G0
- **g4**: STM32G4
- **h5**: STM32H5
- **h50**: STM32H5
- **h7**: STM32H7
- **h7od**: STM32H7
- **h7rs**: STM32H7
- **l0**: STM32L0
- **l1**: STM32L1
- **l4**: STM32L4, STM32L4+
- **l5**: STM32L5
- **n6**: STM32N6
- **u0**: STM32U0
- **u3**: STM32U3
- **u5**: STM32U5
- **wb**: STM32WB, STM32WB0
- **wba**: STM32WBA
- **wl5**: STM32WL
- **wle**: STM32WL

### TAMP

**Versions by family:**

- **g0**: STM32G0
- **g4**: STM32G4
- **h5**: STM32H5
- **l5**: STM32L5
- **u5**: STM32U5
- **wba**: STM32WBA
- **wl**: STM32WL
- **❌ Unsupported**: STM32H7, STM32N6, STM32U0, STM32U3

### TIM

**Versions by family:**

- **❌ Unsupported**: STM32N6

### TIMER

**Versions by family:**

- **l0**: STM32L0
- **v1**: STM32C0, STM32F0, STM32F1, STM32F2, STM32F3, STM32F4, STM32F7, STM32G0, STM32H7, STM32L1, STM32L4, STM32L4+, STM32L5, STM32N6, STM32WB, STM32WB0, STM32WL
- **v2**: STM32G4, STM32H5, STM32U0, STM32U3, STM32U5, STM32WBA

### TSC

**Versions by family:**

- **v1**: STM32F0, STM32F3, STM32WBA
- **v2**: STM32U0, STM32WB
- **v3**: STM32L0, STM32L4, STM32L4+, STM32L5, STM32U5
- **❌ Unsupported**: STM32F3, STM32U3, STM32WB

### UCPD

**Versions by family:**

- **h5**: STM32H5
- **v1**: STM32G0, STM32G4, STM32H7, STM32L5, STM32N6, STM32U5

### UID

**Versions by family:**

- **v1**: STM32C0, STM32F0, STM32F1, STM32F2, STM32F3, STM32F4, STM32F7, STM32G0, STM32G4, STM32H5, STM32H7, STM32L0, STM32L1, STM32L4, STM32L4+, STM32L5, STM32N6, STM32U0, STM32U3, STM32U5, STM32WB, STM32WBA, STM32WL

### USART

**Versions by family:**

- **v1**: STM32F1
- **v2**: STM32F2, STM32F4, STM32L1
- **v3**: STM32F0, STM32F3, STM32F7, STM32L0, STM32L4
- **v4**: STM32C0, STM32G0, STM32G4, STM32H5, STM32H7, STM32L4+, STM32L5, STM32U0, STM32U3, STM32U5, STM32WB, STM32WB0, STM32WBA, STM32WL
- **❌ Unsupported**: STM32F1, STM32F2, STM32L1, STM32N6

### USB

**Versions by family:**

- **v1**: STM32F1, STM32F3, STM32L1
- **v2**: STM32F3
- **v3**: STM32F0, STM32G4, STM32L0, STM32L4, STM32L5, STM32WB
- **v4**: STM32C0, STM32G0, STM32H5, STM32U0, STM32U3, STM32U5

### USBRAM

**Versions by family:**

- **16x1_512**: STM32F1, STM32F3, STM32L1
- **16x2_1024**: STM32F0, STM32F3, STM32G4, STM32L0, STM32L4, STM32L5, STM32WB
- **16x2_512**: STM32F3
- **32_1024**: STM32U0
- **32_2048**: STM32C0, STM32G0, STM32H5, STM32U3, STM32U5

### VENC

**Versions by family:**

- **❌ Unsupported**: STM32N6

### VREFBUF

**Versions by family:**

- **v1**: STM32G0, STM32L4, STM32L4+, STM32L5, STM32WB, STM32WL
- **v2a1**: STM32H7, STM32U3, STM32U5, STM32WBA
- **v2a2**: STM32H5
- **v2b**: STM32G4
- **❌ Unsupported**: STM32N6, STM32U0

### VREFINTCAL

**Versions by family:**

- **v1**: STM32F3, STM32F7, STM32L1, STM32L4, STM32L4+, STM32WB
- **v2**: STM32WBA

### WWDG

**Versions by family:**

- **v1**: STM32F0, STM32F1, STM32F2, STM32F3, STM32F4, STM32F7, STM32L0, STM32L1, STM32L4, STM32L4+
- **v2**: STM32C0, STM32G0, STM32G4, STM32H5, STM32H7, STM32L5, STM32N6, STM32U0, STM32U3, STM32U5, STM32WB, STM32WBA, STM32WL

### XSPI

**Versions by family:**

- **v1**: STM32H7, STM32N6

### XSPIM

**Versions by family:**

- **v1**: STM32H7, STM32N6

