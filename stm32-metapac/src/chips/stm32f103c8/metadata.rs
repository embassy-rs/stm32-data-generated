include!("../metadata_0155.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32F103C8",
    family: "STM32F1",
    line: "STM32F103",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 65536,
            settings: Some(FlashSettings {
                erase_size: 1024,
                write_size: 2,
                erase_value: 255,
            }),
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
        vrefint: None,
        temp: Some(TempCalibration::Nominal(NominalTempCal {
            ref_temp_c: 25,
            v_ref_mv: 1430,
            avg_slope_uv_per_c: -4300,
        })),
    }),
};
