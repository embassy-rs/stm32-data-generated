include!("../metadata_1039.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32WBA54CG",
    family: "STM32WBA",
    line: "STM32WBAx4",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 1048576,
            settings: Some(FlashSettings {
                erase_size: 8192,
                write_size: 16,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "OTP",
            kind: MemoryRegionKind::Flash,
            address: 0xbf90000,
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
            size: 131072,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
    power: Some(Power {
        dialect: PowerDialect::LpmsSrWuscr,
        stopwuck: false,
        modes: &[
            PowerMode {
                kind: PowerModeKind::Stop0,
                selector: 0,
                min_svos: None,
                surviving_clocks: &["HSI", "LSE", "LSI"],
                min_residency_us: 100,
                exit_latency_us: 20,
            },
            PowerMode {
                kind: PowerModeKind::Stop1,
                selector: 1,
                min_svos: None,
                surviving_clocks: &["HSI", "LSE", "LSI"],
                min_residency_us: 500,
                exit_latency_us: 40,
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
