
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Syscfg",
            extends: None,
            description: Some(
                "register block",
            ),
            items: &[
                BlockItem {
                    name: "cfgr1",
                    description: Some(
                        "configuration register 1",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr2",
                    description: Some(
                        "configuration register 2",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfgr3",
                    description: Some(
                        "configuration register 3",
                    ),
                    array: None,
                    byte_offset: 60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfgr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline0",
                    description: Some(
                        "interrupt line 0 status register",
                    ),
                    array: None,
                    byte_offset: 128,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline2",
                    description: Some(
                        "interrupt line 2 status register",
                    ),
                    array: None,
                    byte_offset: 136,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline3",
                    description: Some(
                        "interrupt line 3 status register",
                    ),
                    array: None,
                    byte_offset: 140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline4",
                    description: Some(
                        "interrupt line 4 status register",
                    ),
                    array: None,
                    byte_offset: 144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline5",
                    description: Some(
                        "interrupt line 5 status register",
                    ),
                    array: None,
                    byte_offset: 148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline6",
                    description: Some(
                        "interrupt line 6 status register",
                    ),
                    array: None,
                    byte_offset: 152,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline6",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline7",
                    description: Some(
                        "interrupt line 7 status register",
                    ),
                    array: None,
                    byte_offset: 156,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline7",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline9",
                    description: Some(
                        "interrupt line 9 status register",
                    ),
                    array: None,
                    byte_offset: 164,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline9",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline10",
                    description: Some(
                        "interrupt line 10 status register",
                    ),
                    array: None,
                    byte_offset: 168,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline10",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline11",
                    description: Some(
                        "interrupt line 11 status register",
                    ),
                    array: None,
                    byte_offset: 172,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline11",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline12",
                    description: Some(
                        "interrupt line 12 status register",
                    ),
                    array: None,
                    byte_offset: 176,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline12",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline13",
                    description: Some(
                        "interrupt line 13 status register",
                    ),
                    array: None,
                    byte_offset: 180,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline13",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline14",
                    description: Some(
                        "interrupt line 14 status register",
                    ),
                    array: None,
                    byte_offset: 184,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline14",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline16",
                    description: Some(
                        "interrupt line 16 status register",
                    ),
                    array: None,
                    byte_offset: 192,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline16",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline19",
                    description: Some(
                        "interrupt line 19 status register",
                    ),
                    array: None,
                    byte_offset: 204,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline19",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline21",
                    description: Some(
                        "interrupt line 21 status register",
                    ),
                    array: None,
                    byte_offset: 212,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline21",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline22",
                    description: Some(
                        "interrupt line 22 status register",
                    ),
                    array: None,
                    byte_offset: 216,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline22",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline23",
                    description: Some(
                        "interrupt line 23 status register",
                    ),
                    array: None,
                    byte_offset: 220,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline23",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline25",
                    description: Some(
                        "interrupt line 25 status register",
                    ),
                    array: None,
                    byte_offset: 228,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline25",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline27",
                    description: Some(
                        "interrupt line 27 status register",
                    ),
                    array: None,
                    byte_offset: 236,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline27",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "itline28",
                    description: Some(
                        "interrupt line 28 status register",
                    ),
                    array: None,
                    byte_offset: 240,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Itline28",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Cfgr1",
            extends: None,
            description: Some(
                "configuration register 1",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mem_mode",
                    description: Some(
                        "Memory mapping selection bits. This bitfield controlled by software selects the memory internally mapped at the address 0x0000_0000. Its reset value is determined by the boot mode configuration. Refer to Reference Manual section 2.5 for more details.",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "MemMode",
                    ),
                },
                Field {
                    name: "pa11_rmp",
                    description: Some(
                        "PA11 pin remapping\r This bit is set and cleared by software. When set, it remaps the PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port.",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pa12_rmp",
                    description: Some(
                        "PA12 pin remapping\r This bit is set and cleared by software. When set, it remaps the PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port.",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ir_pol",
                    description: Some(
                        "IR output polarity selection",
                    ),
                    bit_offset: 5,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ir_mod",
                    description: Some(
                        "IR Modulation Envelope signal selection\r This bitfield selects the signal for IR modulation envelope:",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "IrMod",
                    ),
                },
                Field {
                    name: "i2c_pb6_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PB6\r This bit is set and cleared by software. It enables I2C FM+ driving capability on PB6 I/O port.\r With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored.",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c_pb7_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PB7\r This bit is set and cleared by software. It enables I2C FM+ driving capability on PB7 I/O port.\r With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored.",
                    ),
                    bit_offset: 17,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c_pb8_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PB8\r This bit is set and cleared by software. It enables I2C FM+ driving capability on PB8 I/O port.\r With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored.",
                    ),
                    bit_offset: 18,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c_pb9_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PB9\r This bit is set and cleared by software. It enables I2C FM+ driving capability on PB9 I/O port.\r With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored.",
                    ),
                    bit_offset: 19,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c1_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for I2C1\r This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C1 through GPIOx_AFR registers.\r With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C1 can be enabled through their corresponding I2Cx_FMP bit. When I2C FM+ is enabled, the speed control is ignored.",
                    ),
                    bit_offset: 20,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c_pa9_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PA9\r This bit is set and cleared by software. It enables I2C FM+ driving capability on PA9 I/O port.\r With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored.",
                    ),
                    bit_offset: 22,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c_pa10_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PA10\r This bit is set and cleared by software. It enables I2C FM+ driving capability on PA10 I/O port.\r With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored.",
                    ),
                    bit_offset: 23,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "i2c_pc14_fmp",
                    description: Some(
                        "Fast Mode Plus (FM+) enable for PC14\r This bit is set and cleared by software. It enables I2C FM+ driving capability on PC14 I/O port.\r With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled through one of I2Cx_FMP bits. When I2C FM+ is enabled, the speed control is ignored.",
                    ),
                    bit_offset: 24,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr2",
            extends: None,
            description: Some(
                "configuration register 2",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lockup_lock",
                    description: Some(
                        "Cortex<Superscript>�<Default � Font>-M0+ LOCKUP enable\r This bit is set by software and cleared by system reset. When set, it enables the connection of Cortex<Superscript>�<Default � Font>-M0+ LOCKUP (HardFault) output to the TIM1/16/17 Break input.",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfgr3",
            extends: None,
            description: Some(
                "configuration register 3",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pinmux0",
                    description: Some(
                        "Pin GPIO multiplexer 0\r This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.\r 1x: Reserved\r Pin F2 of WLCSP14 package GPIO assignment\r 1x: Reserved",
                    ),
                    bit_offset: 0,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pinmux0",
                    ),
                },
                Field {
                    name: "pinmux1",
                    description: Some(
                        "Pin GPIO multiplexer 1\r This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.\r 1x: Reserved",
                    ),
                    bit_offset: 2,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pinmux1",
                    ),
                },
                Field {
                    name: "pinmux2",
                    description: Some(
                        "Pin GPIO multiplexer 2\r This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.\r 1x: Reserved\r 1x: Reserved",
                    ),
                    bit_offset: 4,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pinmux2",
                    ),
                },
                Field {
                    name: "pinmux3",
                    description: Some(
                        "Pin GPIO multiplexer 3\r This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.\r 1x: Reserved",
                    ),
                    bit_offset: 6,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pinmux3",
                    ),
                },
                Field {
                    name: "pinmux4",
                    description: Some(
                        "Pin GPIO multiplexer 4\r This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.\r 1x: Reserved\r 1x: Reserved",
                    ),
                    bit_offset: 8,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pinmux4",
                    ),
                },
                Field {
                    name: "pinmux5",
                    description: Some(
                        "Pin GPIO multiplexer 5\r This bit is set by software and cleared by system reset. It assigns a GPIO to a pin.\r 1x: Reserved",
                    ),
                    bit_offset: 10,
                    bit_size: 2,
                    array: None,
                    enumm: Some(
                        "Pinmux5",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Itline0",
            extends: None,
            description: Some(
                "interrupt line 0 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wwdg",
                    description: Some(
                        "Window watchdog interrupt pending flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline10",
            extends: None,
            description: Some(
                "interrupt line 10 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1_ch2",
                    description: Some(
                        "DMA1 channel 2 interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dma1_ch3",
                    description: Some(
                        "DMA1 channel 3 interrupt request pending",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline11",
            extends: None,
            description: Some(
                "interrupt line 11 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dmamux",
                    description: Some(
                        "DMAMUX interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline12",
            extends: None,
            description: Some(
                "interrupt line 12 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "adc",
                    description: Some(
                        "ADC interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline13",
            extends: None,
            description: Some(
                "interrupt line 13 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1_ccu",
                    description: Some(
                        "Timer 1 commutation interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1_trg",
                    description: Some(
                        "Timer 1 trigger interrupt request pending",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1_upd",
                    description: Some(
                        "Timer 1 update interrupt request pending",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tim1_brk",
                    description: Some(
                        "Timer 1 break interrupt request pending",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline14",
            extends: None,
            description: Some(
                "interrupt line 14 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim1_cc",
                    description: Some(
                        "Timer 1 capture compare interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline16",
            extends: None,
            description: Some(
                "interrupt line 16 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim3",
                    description: Some(
                        "Timer 3 interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline19",
            extends: None,
            description: Some(
                "interrupt line 19 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim14",
                    description: Some(
                        "Timer 14 interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline2",
            extends: None,
            description: Some(
                "interrupt line 2 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rtc",
                    description: Some(
                        "RTC interrupt request pending (EXTI line 19)",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline21",
            extends: None,
            description: Some(
                "interrupt line 21 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim16",
                    description: Some(
                        "Timer 16 interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline22",
            extends: None,
            description: Some(
                "interrupt line 22 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tim17",
                    description: Some(
                        "Timer 17 interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline23",
            extends: None,
            description: Some(
                "interrupt line 23 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "i2c1",
                    description: Some(
                        "I2C1 interrupt request pending, combined with EXTI line 23",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline25",
            extends: None,
            description: Some(
                "interrupt line 25 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "spi1",
                    description: Some(
                        "SPI1 interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline27",
            extends: None,
            description: Some(
                "interrupt line 27 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart1",
                    description: Some(
                        "USART1 interrupt request pending, combined with EXTI line 25",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline28",
            extends: None,
            description: Some(
                "interrupt line 28 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "usart2",
                    description: Some(
                        "USART2 interrupt request pending (EXTI line 26)",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline3",
            extends: None,
            description: Some(
                "interrupt line 3 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "flash_itf",
                    description: Some(
                        "Flash interface interrupt request pending",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline4",
            extends: None,
            description: Some(
                "interrupt line 4 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "rcc",
                    description: Some(
                        "Reset and clock control interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline5",
            extends: None,
            description: Some(
                "interrupt line 5 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "EXTI",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline6",
            extends: None,
            description: Some(
                "interrupt line 6 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "EXTI",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline7",
            extends: None,
            description: Some(
                "interrupt line 7 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "exti",
                    description: Some(
                        "EXTI",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 12,
                                stride: 1,
                            },
                        ),
                    ),
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Itline9",
            extends: None,
            description: Some(
                "interrupt line 9 status register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dma1_ch1",
                    description: Some(
                        "DMA1 channel 1interrupt request pending",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "IrMod",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "TIM16",
                    description: Some(
                        "TIM16",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "USART1",
                    description: Some(
                        "USART1",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "USART2",
                    description: Some(
                        "USART2",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "MemMode",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "MAIN_FLASH",
                    description: Some(
                        "Main Flash memory mapped at address 0",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "SYSTEM_FLASH",
                    description: Some(
                        "System Flash memory mapped at address 0",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "MAIN_FLASH_ALT",
                    description: Some(
                        "Main Flash memory mapped at address 0 (alternate encoding)",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "SRAM",
                    description: Some(
                        "Embedded SRAM mapped at address 0",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pinmux0",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_1",
                    description: Some(
                        "PB7",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X0_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_F2",
                    description: Some(
                        "PA1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_1",
                    description: Some(
                        "PC14",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X1_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_F2",
                    description: Some(
                        "PA2",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pinmux1",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_4",
                    description: Some(
                        "PF2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X0_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_G3",
                    description: Some(
                        "PF2",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_4",
                    description: Some(
                        "PA0",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X1_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_G3",
                    description: Some(
                        "PA0",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_4",
                    description: Some(
                        "PA1",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_4",
                    description: Some(
                        "PA2",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Pinmux2",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_5",
                    description: Some(
                        "PA8",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X0_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_J1",
                    description: Some(
                        "PA8",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_5",
                    description: Some(
                        "PA11",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X1_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_J1",
                    description: Some(
                        "PA11",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pinmux3",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_8",
                    description: Some(
                        "PA14",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X0_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_H2",
                    description: Some(
                        "PA5",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_8",
                    description: Some(
                        "PB6",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X1_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_H2",
                    description: Some(
                        "PA6",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2_STM32C011X___GPIO_ASSIGNED_TO_SO8_PIN_8",
                    description: Some(
                        "PC15",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Pinmux4",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0_STM32C011X___GPIO_ASSIGNED_TO_WLCSP12_PIN_E2",
                    description: Some(
                        "PA7",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X0_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_G1",
                    description: Some(
                        "PA7",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1_STM32C011X___GPIO_ASSIGNED_TO_WLCSP12_PIN_E2",
                    description: Some(
                        "PA12",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X1_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_G1",
                    description: Some(
                        "PA12",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pinmux5",
            description: None,
            bit_size: 2,
            variants: &[
                EnumVariant {
                    name: "B_0X0_STM32C011X___GPIO_ASSIGNED_TO_WLCSP12_PIN_F1",
                    description: Some(
                        "PA3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X0_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_J3",
                    description: Some(
                        "PA3",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "B_0X1_STM32C011X___GPIO_ASSIGNED_TO_WLCSP12_PIN_F1",
                    description: Some(
                        "PA4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X1_STM32C031X___GPIO_ASSIGNED_TO_WLCSP14_PIN_J3",
                    description: Some(
                        "PA4",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "B_0X2_STM32C011X___GPIO_ASSIGNED_TO_WLCSP12_PIN_F1",
                    description: Some(
                        "PA5",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "B_0X3_STM32C011X___GPIO_ASSIGNED_TO_WLCSP12_PIN_F1",
                    description: Some(
                        "PA6",
                    ),
                    value: 3,
                },
            ],
        },
    ],
};
