
use crate::metadata::ir::*;
pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Cordic",
            extends: None,
            description: Some(
                "CORDIC co-processor.",
            ),
            items: &[
                BlockItem {
                    name: "csr",
                    description: Some(
                        "Control and status register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Csr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wdata",
                    description: Some(
                        "Argument register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
                BlockItem {
                    name: "rdata",
                    description: Some(
                        "Result register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: None,
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Csr",
            extends: None,
            description: Some(
                "Control and status register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "func",
                    description: Some(
                        "Function.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Func",
                    ),
                },
                Field {
                    name: "precision",
                    description: Some(
                        "Precision required (number of iterations/cycles), where PRECISION = (number of iterations/4).",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: Some(
                        "Precision",
                    ),
                },
                Field {
                    name: "scale",
                    description: Some(
                        "Scaling factor.\nInput value has been multiplied by 2^(-n) before for argument.\nOutput value will need to be multiplied by 2^n later for results.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: Some(
                        "Scale",
                    ),
                },
                Field {
                    name: "ien",
                    description: Some(
                        "Enable interrupt.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmaren",
                    description: Some(
                        "Enable DMA wread channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dmawen",
                    description: Some(
                        "Enable DMA write channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "nres",
                    description: Some(
                        "Number of results in the RDATA register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Num",
                    ),
                },
                Field {
                    name: "nargs",
                    description: Some(
                        "Number of arguments expected by the WDATA register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Num",
                    ),
                },
                Field {
                    name: "ressize",
                    description: Some(
                        "Width of output data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Size",
                    ),
                },
                Field {
                    name: "argsize",
                    description: Some(
                        "Width of input data.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: Some(
                        "Size",
                    ),
                },
                Field {
                    name: "rrdy",
                    description: Some(
                        "Result ready flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 31,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[
        Enum {
            name: "Func",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Arctangent",
                    description: Some(
                        "Arctangent function.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Arctanh",
                    description: Some(
                        "Arctanh function.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "Cosine",
                    description: Some(
                        "Cosine function.",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "HyperbolicCosine",
                    description: Some(
                        "Hyperbolic Cosine function.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "HyperbolicSine",
                    description: Some(
                        "Hyperbolic Sine function.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "Modulus",
                    description: Some(
                        "Modulus function.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "NaturalLogarithm",
                    description: Some(
                        "Natural Logarithm function.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "Phase",
                    description: Some(
                        "Phase function.",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "Sine",
                    description: Some(
                        "Sine function.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "SquareRoot",
                    description: Some(
                        "Square Root function.",
                    ),
                    value: 9,
                },
            ],
        },
        Enum {
            name: "Num",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Num1",
                    description: Some(
                        "1 input/output",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "Num2",
                    description: Some(
                        "2 input/output",
                    ),
                    value: 1,
                },
            ],
        },
        Enum {
            name: "Precision",
            description: None,
            bit_size: 4,
            variants: &[
                EnumVariant {
                    name: "Iters12",
                    description: Some(
                        "12 iterations.",
                    ),
                    value: 3,
                },
                EnumVariant {
                    name: "Iters16",
                    description: Some(
                        "16 iterations.",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "Iters20",
                    description: Some(
                        "20 iterations.",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "Iters24",
                    description: Some(
                        "24 iterations.",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "Iters28",
                    description: Some(
                        "28 iterations.",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "Iters32",
                    description: Some(
                        "32 iterations.",
                    ),
                    value: 8,
                },
                EnumVariant {
                    name: "Iters36",
                    description: Some(
                        "36 iterations.",
                    ),
                    value: 9,
                },
                EnumVariant {
                    name: "Iters4",
                    description: Some(
                        "4 iterations.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Iters40",
                    description: Some(
                        "40 iterations.",
                    ),
                    value: 10,
                },
                EnumVariant {
                    name: "Iters44",
                    description: Some(
                        "44 iterations.",
                    ),
                    value: 11,
                },
                EnumVariant {
                    name: "Iters48",
                    description: Some(
                        "48 iterations.",
                    ),
                    value: 12,
                },
                EnumVariant {
                    name: "Iters52",
                    description: Some(
                        "52 iterations.",
                    ),
                    value: 13,
                },
                EnumVariant {
                    name: "Iters56",
                    description: Some(
                        "56 iterations.",
                    ),
                    value: 14,
                },
                EnumVariant {
                    name: "Iters60",
                    description: Some(
                        "60 iterations.",
                    ),
                    value: 15,
                },
                EnumVariant {
                    name: "Iters8",
                    description: Some(
                        "8 iterations.",
                    ),
                    value: 2,
                },
            ],
        },
        Enum {
            name: "Scale",
            description: None,
            bit_size: 3,
            variants: &[
                EnumVariant {
                    name: "A1R1",
                    description: Some(
                        "Argument multiplied by 1, result multiplied by 1",
                    ),
                    value: 0,
                },
                EnumVariant {
                    name: "A1o128R128",
                    description: Some(
                        "Argument multiplied by 1/128, result multiplied by 128",
                    ),
                    value: 7,
                },
                EnumVariant {
                    name: "A1o16R16",
                    description: Some(
                        "Argument multiplied by 1/16, result multiplied by 16",
                    ),
                    value: 4,
                },
                EnumVariant {
                    name: "A1o2R2",
                    description: Some(
                        "Argument multiplied by 1/2, result multiplied by 2",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "A1o32R32",
                    description: Some(
                        "Argument multiplied by 1/32, result multiplied by 32",
                    ),
                    value: 5,
                },
                EnumVariant {
                    name: "A1o4R4",
                    description: Some(
                        "Argument multiplied by 1/4, result multiplied by 4",
                    ),
                    value: 2,
                },
                EnumVariant {
                    name: "A1o64R64",
                    description: Some(
                        "Argument multiplied by 1/64, result multiplied by 64",
                    ),
                    value: 6,
                },
                EnumVariant {
                    name: "A1o8R8",
                    description: Some(
                        "Argument multiplied by 1/8, result multiplied by 8",
                    ),
                    value: 3,
                },
            ],
        },
        Enum {
            name: "Size",
            description: None,
            bit_size: 1,
            variants: &[
                EnumVariant {
                    name: "Bits16",
                    description: Some(
                        "Use 16 bit input/output values.",
                    ),
                    value: 1,
                },
                EnumVariant {
                    name: "Bits32",
                    description: Some(
                        "Use 32 bit input/output values.",
                    ),
                    value: 0,
                },
            ],
        },
    ],
};
