include!("../metadata_0709.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32L072RZ",
    family: "STM32L0",
    line: "STM32L0x2",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 98304,
            settings: Some(FlashSettings {
                erase_size: 128,
                write_size: 4,
                erase_value: 0,
            }),
        },
        MemoryRegion {
            name: "BANK_2",
            kind: MemoryRegionKind::Flash,
            address: 0x8018000,
            size: 98304,
            settings: Some(FlashSettings {
                erase_size: 128,
                write_size: 4,
                erase_value: 0,
            }),
        },
        MemoryRegion {
            name: "EEPROM_BANK_1",
            kind: MemoryRegionKind::Eeprom,
            address: 0x8080000,
            size: 3072,
            settings: None,
        },
        MemoryRegion {
            name: "EEPROM_BANK_2",
            kind: MemoryRegionKind::Eeprom,
            address: 0x8080c00,
            size: 3072,
            settings: None,
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 20480,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
    power: Some(Power {
        dialect: PowerDialect::Classic,
        stopwuck: true,
        modes: &[
            PowerMode {
                kind: PowerModeKind::Stop0,
                selector: 0,
                min_svos: None,
                surviving_clocks: &["HSI", "LSE", "LSI"],
                min_residency_us: 300,
                exit_latency_us: 100,
            },
            PowerMode {
                kind: PowerModeKind::Stop1,
                selector: 1,
                min_svos: None,
                surviving_clocks: &["HSI", "LSE", "LSI"],
                min_residency_us: 2000,
                exit_latency_us: 750,
            },
        ],
    }),
    adc_calibration: Some(AdcCalibration {
        vrefint: Some(VrefCalibration {
            vref_mv: 3000,
            cal_bits: 12,
        }),
        temp: Some(TempCalibration::TwoPoint(TwoPointTempCal {
            vref_mv: 3000,
            cal_bits: 12,
            cal1_temp_c: 30,
            cal2_temp_c: 130,
        })),
    }),
};
