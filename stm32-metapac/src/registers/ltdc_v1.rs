
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Ltdc",
            extends: None,
            description: Some(
                "LCD-TFT Controller",
            ),
            items: &[
                BlockItem {
                    name: "sscr",
                    description: Some(
                        "Synchronization Size Configuration Register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Sscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bpcr",
                    description: Some(
                        "Back Porch Configuration Register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "awcr",
                    description: Some(
                        "Active Width Configuration Register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Awcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "twcr",
                    description: Some(
                        "Total Width Configuration Register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Twcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gcr",
                    description: Some(
                        "Global Control Register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "srcr",
                    description: Some(
                        "Shadow Reload Configuration Register",
                    ),
                    array: None,
                    byte_offset: 36,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Srcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bccr",
                    description: Some(
                        "Background Color Configuration Register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier",
                    description: Some(
                        "Interrupt Enable Register",
                    ),
                    array: None,
                    byte_offset: 52,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr",
                    description: Some(
                        "Interrupt Status Register",
                    ),
                    array: None,
                    byte_offset: 56,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "icr",
                    description: Some(
                        "Interrupt Clear Register",
                    ),
                    array: None,
                    byte_offset: 60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Icr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lipcr",
                    description: Some(
                        "Line Interrupt Position Configuration Register",
                    ),
                    array: None,
                    byte_offset: 64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lipcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cpsr",
                    description: Some(
                        "Current Position Status Register",
                    ),
                    array: None,
                    byte_offset: 68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cpsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cdsr",
                    description: Some(
                        "Current Display Status Register",
                    ),
                    array: None,
                    byte_offset: 72,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Cdsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "layer",
                    description: Some(
                        "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR",
                    ),
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 128,
                            },
                        ),
                    ),
                    byte_offset: 132,
                    inner: BlockItemInner::Block(
                        BlockItemBlock {
                            block: "Layer",
                        },
                    ),
                },
            ],
        },
        Block {
            name: "Layer",
            extends: None,
            description: Some(
                "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR",
            ),
            items: &[
                BlockItem {
                    name: "cr",
                    description: Some(
                        "Layerx Control Register",
                    ),
                    array: None,
                    byte_offset: 0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "whpcr",
                    description: Some(
                        "Layerx Window Horizontal Position Configuration Register",
                    ),
                    array: None,
                    byte_offset: 4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Whpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wvpcr",
                    description: Some(
                        "Layerx Window Vertical Position Configuration Register",
                    ),
                    array: None,
                    byte_offset: 8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wvpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ckcr",
                    description: Some(
                        "Layerx Color Keying Configuration Register",
                    ),
                    array: None,
                    byte_offset: 12,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ckcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pfcr",
                    description: Some(
                        "Layerx Pixel Format Configuration Register",
                    ),
                    array: None,
                    byte_offset: 16,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cacr",
                    description: Some(
                        "Layerx Constant Alpha Configuration Register",
                    ),
                    array: None,
                    byte_offset: 20,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dccr",
                    description: Some(
                        "Layerx Default Color Configuration Register",
                    ),
                    array: None,
                    byte_offset: 24,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "bfcr",
                    description: Some(
                        "Layerx Blending Factors Configuration Register",
                    ),
                    array: None,
                    byte_offset: 28,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Bfcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfbar",
                    description: Some(
                        "Layerx Color Frame Buffer Address Register",
                    ),
                    array: None,
                    byte_offset: 40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfbar",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfblr",
                    description: Some(
                        "Layerx Color Frame Buffer Length Register",
                    ),
                    array: None,
                    byte_offset: 44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfblr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cfblnr",
                    description: Some(
                        "Layerx ColorFrame Buffer Line Number Register",
                    ),
                    array: None,
                    byte_offset: 48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cfblnr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clutwr",
                    description: Some(
                        "Layerx CLUT Write Register",
                    ),
                    array: None,
                    byte_offset: 64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Clutwr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Sscr",
            extends: None,
            description: Some(
                "Synchronization Size Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsh",
                    description: Some(
                        "Vertical Synchronization Height (in units of horizontal scan line)",
                    ),
                    bit_offset: 0,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsw",
                    description: Some(
                        "Horizontal Synchronization Width (in units of pixel clock period)",
                    ),
                    bit_offset: 16,
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cpsr",
            extends: None,
            description: Some(
                "Current Position Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cypos",
                    description: Some(
                        "Current Y Position",
                    ),
                    bit_offset: 0,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cxpos",
                    description: Some(
                        "Current X Position",
                    ),
                    bit_offset: 16,
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pfcr",
            extends: None,
            description: Some(
                "Layerx Pixel Format Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pf",
                    description: Some(
                        "Pixel Format",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Pf",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Gcr",
            extends: None,
            description: Some(
                "Global Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ltdcen",
                    description: Some(
                        "LCD-TFT controller enable bit",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Ltdcen",
                    ),
                },
                Field {
                    name: "dbw",
                    description: Some(
                        "Dither Blue Width",
                    ),
                    bit_offset: 4,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dgw",
                    description: Some(
                        "Dither Green Width",
                    ),
                    bit_offset: 8,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "drw",
                    description: Some(
                        "Dither Red Width",
                    ),
                    bit_offset: 12,
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "den",
                    description: Some(
                        "Dither Enable",
                    ),
                    bit_offset: 16,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Den",
                    ),
                },
                Field {
                    name: "pcpol",
                    description: Some(
                        "Pixel Clock Polarity",
                    ),
                    bit_offset: 28,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Pcpol",
                    ),
                },
                Field {
                    name: "depol",
                    description: Some(
                        "Data Enable Polarity",
                    ),
                    bit_offset: 29,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Depol",
                    ),
                },
                Field {
                    name: "vspol",
                    description: Some(
                        "Vertical Synchronization Polarity",
                    ),
                    bit_offset: 30,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vspol",
                    ),
                },
                Field {
                    name: "hspol",
                    description: Some(
                        "Horizontal Synchronization Polarity",
                    ),
                    bit_offset: 31,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hspol",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Lipcr",
            extends: None,
            description: Some(
                "Line Interrupt Position Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lipos",
                    description: Some(
                        "Line Interrupt Position",
                    ),
                    bit_offset: 0,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Clutwr",
            extends: None,
            description: Some(
                "Layerx CLUT Write Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "blue",
                    description: Some(
                        "Blue value",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "green",
                    description: Some(
                        "Green value",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "red",
                    description: Some(
                        "Red value",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "clutadd",
                    description: Some(
                        "CLUT Address",
                    ),
                    bit_offset: 24,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr",
            extends: None,
            description: Some(
                "Interrupt Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lif",
                    description: Some(
                        "Line Interrupt flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lif",
                    ),
                },
                Field {
                    name: "fuif",
                    description: Some(
                        "FIFO Underrun Interrupt flag",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fuif",
                    ),
                },
                Field {
                    name: "terrif",
                    description: Some(
                        "Transfer Error interrupt flag",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Terrif",
                    ),
                },
                Field {
                    name: "rrif",
                    description: Some(
                        "Register Reload Interrupt Flag",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rrif",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Ckcr",
            extends: None,
            description: Some(
                "Layerx Color Keying Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ckblue",
                    description: Some(
                        "Color Key Blue value",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckgreen",
                    description: Some(
                        "Color Key Green value",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ckred",
                    description: Some(
                        "Color Key Red value",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Whpcr",
            extends: None,
            description: Some(
                "Layerx Window Horizontal Position Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "whstpos",
                    description: Some(
                        "Window Horizontal Start Position",
                    ),
                    bit_offset: 0,
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "whsppos",
                    description: Some(
                        "Window Horizontal Stop Position",
                    ),
                    bit_offset: 16,
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bfcr",
            extends: None,
            description: Some(
                "Layerx Blending Factors Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bf",
                    description: Some(
                        "Blending Factor 2",
                    ),
                    bit_offset: 0,
                    bit_size: 3,
                    array: Some(
                        Array::Regular(
                            RegularArray {
                                len: 2,
                                stride: 8,
                            },
                        ),
                    ),
                    enumm: Some(
                        "Bf2",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Srcr",
            extends: None,
            description: Some(
                "Shadow Reload Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "imr",
                    description: Some(
                        "Immediate Reload",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Imr",
                    ),
                },
                Field {
                    name: "vbr",
                    description: Some(
                        "Vertical Blanking Reload",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vbr",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Bccr",
            extends: None,
            description: Some(
                "Background Color Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bcblue",
                    description: Some(
                        "Background color blue value",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bcgreen",
                    description: Some(
                        "Background color green value",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "bcred",
                    description: Some(
                        "Background color red value",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Icr",
            extends: None,
            description: Some(
                "Interrupt Clear Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "clif",
                    description: Some(
                        "Clears the Line Interrupt Flag",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Clif",
                    ),
                },
                Field {
                    name: "cfuif",
                    description: Some(
                        "Clears the FIFO Underrun Interrupt flag",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cfuif",
                    ),
                },
                Field {
                    name: "cterrif",
                    description: Some(
                        "Clears the Transfer Error Interrupt Flag",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cterrif",
                    ),
                },
                Field {
                    name: "crrif",
                    description: Some(
                        "Clears Register Reload Interrupt Flag",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Crrif",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cacr",
            extends: None,
            description: Some(
                "Layerx Constant Alpha Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "consta",
                    description: Some(
                        "Constant Alpha",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ier",
            extends: None,
            description: Some(
                "Interrupt Enable Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lie",
                    description: Some(
                        "Line Interrupt Enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Lie",
                    ),
                },
                Field {
                    name: "fuie",
                    description: Some(
                        "FIFO Underrun Interrupt Enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Fuie",
                    ),
                },
                Field {
                    name: "terrie",
                    description: Some(
                        "Transfer Error Interrupt Enable",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Terrie",
                    ),
                },
                Field {
                    name: "rrie",
                    description: Some(
                        "Register Reload interrupt enable",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Rrie",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Twcr",
            extends: None,
            description: Some(
                "Total Width Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "totalh",
                    description: Some(
                        "Total Height (in units of horizontal scan line)",
                    ),
                    bit_offset: 0,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "totalw",
                    description: Some(
                        "Total Width (in units of pixel clock period)",
                    ),
                    bit_offset: 16,
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wvpcr",
            extends: None,
            description: Some(
                "Layerx Window Vertical Position Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "wvstpos",
                    description: Some(
                        "Window Vertical Start Position",
                    ),
                    bit_offset: 0,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wvsppos",
                    description: Some(
                        "Window Vertical Stop Position",
                    ),
                    bit_offset: 16,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfblr",
            extends: None,
            description: Some(
                "Layerx Color Frame Buffer Length Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfbll",
                    description: Some(
                        "Color Frame Buffer Line Length",
                    ),
                    bit_offset: 0,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cfbp",
                    description: Some(
                        "Color Frame Buffer Pitch in bytes",
                    ),
                    bit_offset: 16,
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfbar",
            extends: None,
            description: Some(
                "Layerx Color Frame Buffer Address Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfbadd",
                    description: Some(
                        "Color Frame Buffer Start Address",
                    ),
                    bit_offset: 0,
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Bpcr",
            extends: None,
            description: Some(
                "Back Porch Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "avbp",
                    description: Some(
                        "Accumulated Vertical back porch (in units of horizontal scan line)",
                    ),
                    bit_offset: 0,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ahbp",
                    description: Some(
                        "Accumulated Horizontal back porch (in units of pixel clock period)",
                    ),
                    bit_offset: 16,
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cfblnr",
            extends: None,
            description: Some(
                "Layerx ColorFrame Buffer Line Number Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cfblnbr",
                    description: Some(
                        "Frame Buffer Line Number",
                    ),
                    bit_offset: 0,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "Layerx Control Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "len",
                    description: Some(
                        "Layer Enable",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Len",
                    ),
                },
                Field {
                    name: "colken",
                    description: Some(
                        "Color Keying Enable",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Colken",
                    ),
                },
                Field {
                    name: "cluten",
                    description: Some(
                        "Color Look-Up Table Enable",
                    ),
                    bit_offset: 4,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Cluten",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Cdsr",
            extends: None,
            description: Some(
                "Current Display Status Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vdes",
                    description: Some(
                        "Vertical Data Enable display Status",
                    ),
                    bit_offset: 0,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vdes",
                    ),
                },
                Field {
                    name: "hdes",
                    description: Some(
                        "Horizontal Data Enable display Status",
                    ),
                    bit_offset: 1,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hdes",
                    ),
                },
                Field {
                    name: "vsyncs",
                    description: Some(
                        "Vertical Synchronization display Status",
                    ),
                    bit_offset: 2,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Vsyncs",
                    ),
                },
                Field {
                    name: "hsyncs",
                    description: Some(
                        "Horizontal Synchronization display Status",
                    ),
                    bit_offset: 3,
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Hsyncs",
                    ),
                },
            ],
        },
        FieldSet {
            name: "Dccr",
            extends: None,
            description: Some(
                "Layerx Default Color Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dcblue",
                    description: Some(
                        "Default Color Blue",
                    ),
                    bit_offset: 0,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcgreen",
                    description: Some(
                        "Default Color Green",
                    ),
                    bit_offset: 8,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcred",
                    description: Some(
                        "Default Color Red",
                    ),
                    bit_offset: 16,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dcalpha",
                    description: Some(
                        "Default Color Alpha",
                    ),
                    bit_offset: 24,
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Awcr",
            extends: None,
            description: Some(
                "Active Width Configuration Register",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "aah",
                    description: Some(
                        "Accumulated Active Height (in units of horizontal scan line)",
                    ),
                    bit_offset: 0,
                    bit_size: 11,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "aaw",
                    description: Some(
                        "Accumulated Active Width (in units of pixel clock period)",
                    ),
                    bit_offset: 16,
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Terrie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Transfer error interrupt disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Transfer error interrupt enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cluten",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Color look-up table disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Color look-up table enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Bf2",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "CONSTANT",
                    description: Some(
                        "BF2 = 1 - constant alpha",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "PIXEL",
                    description: Some(
                        "BF2 = 1 - pixel alpha * constant alpha",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Colken",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Color keying disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Color keying enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Ltdcen",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "LCD-TFT controller disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "LCD-TFT controller enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hdes",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTACTIVE",
                    description: Some(
                        "Currently not in horizontal Data Enable phase",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE",
                    description: Some(
                        "Currently in horizontal Data Enable phase",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hsyncs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTACTIVE",
                    description: Some(
                        "Currently not in HSYNC phase",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE",
                    description: Some(
                        "Currently in HSYNC phase",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Hspol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVELOW",
                    description: Some(
                        "Horizontal synchronization polarity is active low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVEHIGH",
                    description: Some(
                        "Horizontal synchronization polarity is active high",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Line interrupt disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Line interrupt enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pf",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "ARGB8888",
                    description: Some(
                        "ARGB8888",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RGB888",
                    description: Some(
                        "RGB888",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "RGB565",
                    description: Some(
                        "RGB565",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "ARGB1555",
                    description: Some(
                        "ARGB1555",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "ARGB4444",
                    description: Some(
                        "ARGB4444",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "L8",
                    description: Some(
                        "L8 (8-bit luminance)",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "AL44",
                    description: Some(
                        "AL44 (4-bit alpha, 4-bit luminance)",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "AL88",
                    description: Some(
                        "AL88 (8-bit alpha, 8-bit luminance)",
                    ),
                    value: 7,
                },
            ],
        },
        Enum {
            name: "Vspol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVELOW",
                    description: Some(
                        "Vertical synchronization polarity is active low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVEHIGH",
                    description: Some(
                        "Vertical synchronization polarity is active high",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vdes",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTACTIVE",
                    description: Some(
                        "Currently not in vertical Data Enable phase",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE",
                    description: Some(
                        "Currently in vertical Data Enable phase",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fuif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOUNDERRUN",
                    description: Some(
                        "No FIFO underrun",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "UNDERRUN",
                    description: Some(
                        "FIFO underrun interrupt generated, if one of the layer FIFOs is empty and pixel data is read from the FIFO",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cfuif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears the FUIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Imr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOEFFECT",
                    description: Some(
                        "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RELOAD",
                    description: Some(
                        "The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Den",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Dither disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Dither enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vbr",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOEFFECT",
                    description: Some(
                        "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RELOAD",
                    description: Some(
                        "The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area).",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rrie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Register reload interrupt disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Register reload interrupt enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Lif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTREACHED",
                    description: Some(
                        "Programmed line not reached",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "REACHED",
                    description: Some(
                        "Line interrupt generated when a programmed line is reached",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Vsyncs",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOTACTIVE",
                    description: Some(
                        "Currently not in VSYNC phase",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVE",
                    description: Some(
                        "Currently in VSYNC phase",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Cterrif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears the TERRIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Crrif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears the RRIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Pcpol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "RISINGEDGE",
                    description: Some(
                        "Pixel clock on rising edge",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "FALLINGEDGE",
                    description: Some(
                        "Pixel clock on falling edge",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Clif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "CLEAR",
                    description: Some(
                        "Clears the LIF flag in the ISR register",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Terrif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NOERROR",
                    description: Some(
                        "No transfer error",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ERROR",
                    description: Some(
                        "Transfer error interrupt generated when a bus error occurs",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Len",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "Layer disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "Layer enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Fuie",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "DISABLED",
                    description: Some(
                        "FIFO underrun interrupt disabled",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ENABLED",
                    description: Some(
                        "FIFO underrun interrupt enabled",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Rrif",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "NORELOAD",
                    description: Some(
                        "No register reload",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "RELOAD",
                    description: Some(
                        "Register reload interrupt generated when a vertical blanking reload occurs (and the first line after the active area is reached)",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Depol",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "ACTIVELOW",
                    description: Some(
                        "Data enable polarity is active low",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "ACTIVEHIGH",
                    description: Some(
                        "Data enable polarity is active high",
                    ),
                    value: 1,
                },
            ],
        },
    ],
};
