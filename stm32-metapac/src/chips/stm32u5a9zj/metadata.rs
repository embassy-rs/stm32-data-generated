include!("../metadata_1006.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32U5A9ZJ",
    family: "STM32U5",
    line: "STM32U5x9",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 2097152,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "BANK_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8200000,
            size: 2097152,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 0xbfa0000,
            size: 512,
            settings: Some(FlashSettings {
                erase_size: 0,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 786432,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM2",
            kind: MemoryRegionKind::Ram,
            address: 0x200c0000,
            size: 65536,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM3",
            kind: MemoryRegionKind::Ram,
            address: 0x200d0000,
            size: 851968,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM5",
            kind: MemoryRegionKind::Ram,
            address: 0x201a0000,
            size: 851968,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM4",
            kind: MemoryRegionKind::Ram,
            address: 0x28000000,
            size: 16384,
            settings: None,
        },
        MemoryRegion {
            name: "BKPSRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x40036400,
            size: 2048,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
    adc_calibration: Some(AdcCalibration {
        vrefint: Some(VrefCalibration {
            vref_mv: 3000,
            cal_bits: 14,
        }),
        temp: Some(TempCalibration::TwoPoint(TwoPointTempCal {
            vref_mv: 3000,
            cal_bits: 14,
            cal1_temp_c: 30,
            cal2_temp_c: 130,
        })),
    }),
};
