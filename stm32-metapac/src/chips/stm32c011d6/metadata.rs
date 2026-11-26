include!("../metadata_0000.rs");
use crate::metadata::PeripheralRccKernelClock::{Clock, Mux};
pub static METADATA: Metadata = Metadata {
    name: "STM32C011D6",
    family: "STM32C0",
    line: "STM32C0x1",
    memory: &[&[
        MemoryRegion {
            name: "BANK_1",
            kind: MemoryRegionKind::Flash,
            address: 0x8000000,
            size: 32768,
            settings: Some(FlashSettings {
                erase_size: 2048,
                write_size: 8,
                erase_value: 255,
            }),
        },
        MemoryRegion {
            name: "SRAM",
            kind: MemoryRegionKind::Ram,
            address: 0x20000000,
            size: 6144,
            settings: None,
        },
    ]],
    peripherals: PERIPHERALS,
    nvic_priority_bits: Some(2),
    interrupts: INTERRUPTS,
    dma_channels: DMA_CHANNELS,
    pins: PINS,
    power: Some(Power {
        dialect: PowerDialect::LpmsScr,
        stopwuck: false,
        modes: &[PowerMode {
            kind: PowerModeKind::Stop0,
            selector: 0,
            min_svos: None,
            surviving_clocks: &["LSE", "LSI"],
            min_residency_us: 400,
            exit_latency_us: 300,
        }],
    }),
    adc_calibration: Some(AdcCalibration {
        vrefint: Some(VrefCalibration {
            vref_mv: 3000,
            cal_bits: 12,
        }),
        temp: Some(TempCalibration::OnePoint(OnePointTempCal {
            vref_mv: 3000,
            cal_bits: 12,
            cal1_temp_c: 30,
            avg_slope_uv_per_c: 2530,
        })),
    }),
};
