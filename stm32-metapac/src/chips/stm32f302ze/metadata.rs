include!("../metadata_0194.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32F302ZE",
    family: "STM32F3",
    line: "STM32F302",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 524288,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 2,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 65536,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(4),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
    power: Some(Power {
        dialect: PowerDialect::Classic,
        stopwuck: false,
        modes: &[
            PowerMode {
                kind: PowerModeKind::Stop0,
                selector: 0,
                min_svos: None,
                surviving_clocks: &["LSE", "LSI"],
                min_residency_us: 200,
                exit_latency_us: 50,
            },
            PowerMode {
                kind: PowerModeKind::Stop1,
                selector: 1,
                min_svos: None,
                surviving_clocks: &["LSE", "LSI"],
                min_residency_us: 600,
                exit_latency_us: 300,
            },
        ],
    }),
    adc_calibration: Some(AdcCalibration {
        vrefint: Some(VrefCalibration {
            vref_mv: 3300,
            cal_bits: 12,
        }),
        temp: Some(TempCalibration::TwoPoint(TwoPointTempCal {
            vref_mv: 3300,
            cal_bits: 12,
            cal1_temp_c: 30,
            cal2_temp_c: 110,
        })),
    }),
};
