#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "JPEG codec"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Jpeg {
    ptr: *mut u8,
}
unsafe impl Send for Jpeg {}
unsafe impl Sync for Jpeg {}
impl Jpeg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "JPEG codec configuration register 0"]
    #[inline(always)]
    pub const fn jpeg_confr0(self) -> crate::common::Reg<regs::JpegConfr0, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "JPEG codec configuration register 1"]
    #[inline(always)]
    pub const fn jpeg_confr1(self) -> crate::common::Reg<regs::JpegConfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "JPEG codec configuration register 2"]
    #[inline(always)]
    pub const fn jpeg_confr2(self) -> crate::common::Reg<regs::JpegConfr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "JPEG codec configuration register 3"]
    #[inline(always)]
    pub const fn jpeg_confr3(self) -> crate::common::Reg<regs::JpegConfr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "JPEG codec configuration register 4"]
    #[inline(always)]
    pub const fn jpeg_confr4(self) -> crate::common::Reg<regs::JpegConfr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "JPEG codec configuration register 5"]
    #[inline(always)]
    pub const fn jpeg_confr5(self) -> crate::common::Reg<regs::JpegConfr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "JPEG codec configuration register 6"]
    #[inline(always)]
    pub const fn jpeg_confr6(self) -> crate::common::Reg<regs::JpegConfr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
    #[doc = "JPEG codec configuration register 7"]
    #[inline(always)]
    pub const fn jpeg_confr7(self) -> crate::common::Reg<regs::JpegConfr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "JPEG control register"]
    #[inline(always)]
    pub const fn jpeg_cr(self) -> crate::common::Reg<regs::JpegCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x30usize) as _) }
    }
    #[doc = "JPEG status register"]
    #[inline(always)]
    pub const fn jpeg_sr(self) -> crate::common::Reg<regs::JpegSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x34usize) as _) }
    }
    #[doc = "JPEG clear flag register"]
    #[inline(always)]
    pub const fn jpeg_cfr(self) -> crate::common::Reg<regs::JpegCfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x38usize) as _) }
    }
    #[doc = "JPEG data input register"]
    #[inline(always)]
    pub const fn jpeg_dir(self) -> crate::common::Reg<regs::JpegDir, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x40usize) as _) }
    }
    #[doc = "JPEG data output register"]
    #[inline(always)]
    pub const fn jpeg_dor(self) -> crate::common::Reg<regs::JpegDor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x44usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_0(self) -> crate::common::Reg<regs::Qmem0_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x50usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_1(self) -> crate::common::Reg<regs::Qmem0_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x54usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_2(self) -> crate::common::Reg<regs::Qmem0_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x58usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_3(self) -> crate::common::Reg<regs::Qmem0_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x5cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_4(self) -> crate::common::Reg<regs::Qmem0_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x60usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_5(self) -> crate::common::Reg<regs::Qmem0_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x64usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_6(self) -> crate::common::Reg<regs::Qmem0_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x68usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_7(self) -> crate::common::Reg<regs::Qmem0_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x6cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_8(self) -> crate::common::Reg<regs::Qmem0_8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x70usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_9(self) -> crate::common::Reg<regs::Qmem0_9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x74usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_10(self) -> crate::common::Reg<regs::Qmem0_10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x78usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_11(self) -> crate::common::Reg<regs::Qmem0_11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x7cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_12(self) -> crate::common::Reg<regs::Qmem0_12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x80usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_13(self) -> crate::common::Reg<regs::Qmem0_13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x84usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_14(self) -> crate::common::Reg<regs::Qmem0_14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x88usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_15(self) -> crate::common::Reg<regs::Qmem0_15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x8cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_0(self) -> crate::common::Reg<regs::Qmem1_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x90usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_1(self) -> crate::common::Reg<regs::Qmem1_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x94usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_2(self) -> crate::common::Reg<regs::Qmem1_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x98usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_3(self) -> crate::common::Reg<regs::Qmem1_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x9cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_4(self) -> crate::common::Reg<regs::Qmem1_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_5(self) -> crate::common::Reg<regs::Qmem1_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_6(self) -> crate::common::Reg<regs::Qmem1_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xa8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_7(self) -> crate::common::Reg<regs::Qmem1_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xacusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_8(self) -> crate::common::Reg<regs::Qmem1_8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_9(self) -> crate::common::Reg<regs::Qmem1_9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_10(self) -> crate::common::Reg<regs::Qmem1_10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xb8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_11(self) -> crate::common::Reg<regs::Qmem1_11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xbcusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_12(self) -> crate::common::Reg<regs::Qmem1_12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_13(self) -> crate::common::Reg<regs::Qmem1_13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_14(self) -> crate::common::Reg<regs::Qmem1_14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xc8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_15(self) -> crate::common::Reg<regs::Qmem1_15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xccusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_0(self) -> crate::common::Reg<regs::Qmem2_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_1(self) -> crate::common::Reg<regs::Qmem2_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_2(self) -> crate::common::Reg<regs::Qmem2_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xd8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_3(self) -> crate::common::Reg<regs::Qmem2_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xdcusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_4(self) -> crate::common::Reg<regs::Qmem2_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_5(self) -> crate::common::Reg<regs::Qmem2_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_6(self) -> crate::common::Reg<regs::Qmem2_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xe8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_7(self) -> crate::common::Reg<regs::Qmem2_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xecusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_8(self) -> crate::common::Reg<regs::Qmem2_8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_9(self) -> crate::common::Reg<regs::Qmem2_9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_10(self) -> crate::common::Reg<regs::Qmem2_10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xf8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_11(self) -> crate::common::Reg<regs::Qmem2_11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0xfcusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_12(self) -> crate::common::Reg<regs::Qmem2_12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0100usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_13(self) -> crate::common::Reg<regs::Qmem2_13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0104usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_14(self) -> crate::common::Reg<regs::Qmem2_14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0108usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_15(self) -> crate::common::Reg<regs::Qmem2_15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x010cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_0(self) -> crate::common::Reg<regs::Qmem3_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0110usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_1(self) -> crate::common::Reg<regs::Qmem3_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0114usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_2(self) -> crate::common::Reg<regs::Qmem3_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0118usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_3(self) -> crate::common::Reg<regs::Qmem3_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x011cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_4(self) -> crate::common::Reg<regs::Qmem3_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0120usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_5(self) -> crate::common::Reg<regs::Qmem3_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0124usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_6(self) -> crate::common::Reg<regs::Qmem3_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0128usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_7(self) -> crate::common::Reg<regs::Qmem3_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x012cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_8(self) -> crate::common::Reg<regs::Qmem3_8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0130usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_9(self) -> crate::common::Reg<regs::Qmem3_9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0134usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_10(self) -> crate::common::Reg<regs::Qmem3_10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0138usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_11(self) -> crate::common::Reg<regs::Qmem3_11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x013cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_12(self) -> crate::common::Reg<regs::Qmem3_12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0140usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_13(self) -> crate::common::Reg<regs::Qmem3_13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0144usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_14(self) -> crate::common::Reg<regs::Qmem3_14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0148usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_15(self) -> crate::common::Reg<regs::Qmem3_15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x014cusize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_0(self) -> crate::common::Reg<regs::Huffmin0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0150usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_1(self) -> crate::common::Reg<regs::Huffmin1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0154usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_2(self) -> crate::common::Reg<regs::Huffmin2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0158usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_3(self) -> crate::common::Reg<regs::Huffmin3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x015cusize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_4(self) -> crate::common::Reg<regs::Huffmin4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0160usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_5(self) -> crate::common::Reg<regs::Huffmin5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0164usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_6(self) -> crate::common::Reg<regs::Huffmin6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0168usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_7(self) -> crate::common::Reg<regs::Huffmin7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x016cusize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_8(self) -> crate::common::Reg<regs::Huffmin8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0170usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_9(self) -> crate::common::Reg<regs::Huffmin9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0174usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_10(self) -> crate::common::Reg<regs::Huffmin10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0178usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_11(self) -> crate::common::Reg<regs::Huffmin11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x017cusize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_12(self) -> crate::common::Reg<regs::Huffmin12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0180usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_13(self) -> crate::common::Reg<regs::Huffmin13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0184usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_14(self) -> crate::common::Reg<regs::Huffmin14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0188usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_15(self) -> crate::common::Reg<regs::Huffmin15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x018cusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase0(self) -> crate::common::Reg<regs::Huffbase0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0190usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase1(self) -> crate::common::Reg<regs::Huffbase1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0194usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase2(self) -> crate::common::Reg<regs::Huffbase2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0198usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase3(self) -> crate::common::Reg<regs::Huffbase3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x019cusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase4(self) -> crate::common::Reg<regs::Huffbase4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase5(self) -> crate::common::Reg<regs::Huffbase5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase6(self) -> crate::common::Reg<regs::Huffbase6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01a8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase7(self) -> crate::common::Reg<regs::Huffbase7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01acusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase8(self) -> crate::common::Reg<regs::Huffbase8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase9(self) -> crate::common::Reg<regs::Huffbase9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase10(self) -> crate::common::Reg<regs::Huffbase10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01b8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase11(self) -> crate::common::Reg<regs::Huffbase11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01bcusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase12(self) -> crate::common::Reg<regs::Huffbase12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase13(self) -> crate::common::Reg<regs::Huffbase13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase14(self) -> crate::common::Reg<regs::Huffbase14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01c8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase15(self) -> crate::common::Reg<regs::Huffbase15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ccusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase16(self) -> crate::common::Reg<regs::Huffbase16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase17(self) -> crate::common::Reg<regs::Huffbase17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase18(self) -> crate::common::Reg<regs::Huffbase18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01d8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase19(self) -> crate::common::Reg<regs::Huffbase19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01dcusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase20(self) -> crate::common::Reg<regs::Huffbase20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase21(self) -> crate::common::Reg<regs::Huffbase21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase22(self) -> crate::common::Reg<regs::Huffbase22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01e8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase23(self) -> crate::common::Reg<regs::Huffbase23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01ecusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase24(self) -> crate::common::Reg<regs::Huffbase24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase25(self) -> crate::common::Reg<regs::Huffbase25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase26(self) -> crate::common::Reg<regs::Huffbase26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01f8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase27(self) -> crate::common::Reg<regs::Huffbase27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x01fcusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase28(self) -> crate::common::Reg<regs::Huffbase28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0200usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase29(self) -> crate::common::Reg<regs::Huffbase29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0204usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase30(self) -> crate::common::Reg<regs::Huffbase30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0208usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase31(self) -> crate::common::Reg<regs::Huffbase31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x020cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb0(self) -> crate::common::Reg<regs::Huffsymb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0210usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb1(self) -> crate::common::Reg<regs::Huffsymb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0214usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb2(self) -> crate::common::Reg<regs::Huffsymb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0218usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb3(self) -> crate::common::Reg<regs::Huffsymb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x021cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb4(self) -> crate::common::Reg<regs::Huffsymb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0220usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb5(self) -> crate::common::Reg<regs::Huffsymb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0224usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb6(self) -> crate::common::Reg<regs::Huffsymb6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0228usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb7(self) -> crate::common::Reg<regs::Huffsymb7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x022cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb8(self) -> crate::common::Reg<regs::Huffsymb8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0230usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb9(self) -> crate::common::Reg<regs::Huffsymb9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0234usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb10(self) -> crate::common::Reg<regs::Huffsymb10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0238usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb11(self) -> crate::common::Reg<regs::Huffsymb11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x023cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb12(self) -> crate::common::Reg<regs::Huffsymb12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0240usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb13(self) -> crate::common::Reg<regs::Huffsymb13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0244usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb14(self) -> crate::common::Reg<regs::Huffsymb14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0248usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb15(self) -> crate::common::Reg<regs::Huffsymb15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x024cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb16(self) -> crate::common::Reg<regs::Huffsymb16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0250usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb17(self) -> crate::common::Reg<regs::Huffsymb17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0254usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb18(self) -> crate::common::Reg<regs::Huffsymb18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0258usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb19(self) -> crate::common::Reg<regs::Huffsymb19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x025cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb20(self) -> crate::common::Reg<regs::Huffsymb20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0260usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb21(self) -> crate::common::Reg<regs::Huffsymb21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0264usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb22(self) -> crate::common::Reg<regs::Huffsymb22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0268usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb23(self) -> crate::common::Reg<regs::Huffsymb23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x026cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb24(self) -> crate::common::Reg<regs::Huffsymb24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0270usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb25(self) -> crate::common::Reg<regs::Huffsymb25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0274usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb26(self) -> crate::common::Reg<regs::Huffsymb26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0278usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb27(self) -> crate::common::Reg<regs::Huffsymb27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x027cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb28(self) -> crate::common::Reg<regs::Huffsymb28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0280usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb29(self) -> crate::common::Reg<regs::Huffsymb29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0284usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb30(self) -> crate::common::Reg<regs::Huffsymb30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0288usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb31(self) -> crate::common::Reg<regs::Huffsymb31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x028cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb32(self) -> crate::common::Reg<regs::Huffsymb32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0290usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb33(self) -> crate::common::Reg<regs::Huffsymb33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0294usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb34(self) -> crate::common::Reg<regs::Huffsymb34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0298usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb35(self) -> crate::common::Reg<regs::Huffsymb35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x029cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb36(self) -> crate::common::Reg<regs::Huffsymb36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb37(self) -> crate::common::Reg<regs::Huffsymb37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb38(self) -> crate::common::Reg<regs::Huffsymb38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02a8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb39(self) -> crate::common::Reg<regs::Huffsymb39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02acusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb40(self) -> crate::common::Reg<regs::Huffsymb40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb41(self) -> crate::common::Reg<regs::Huffsymb41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb42(self) -> crate::common::Reg<regs::Huffsymb42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02b8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb43(self) -> crate::common::Reg<regs::Huffsymb43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02bcusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb44(self) -> crate::common::Reg<regs::Huffsymb44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb45(self) -> crate::common::Reg<regs::Huffsymb45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb46(self) -> crate::common::Reg<regs::Huffsymb46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02c8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb47(self) -> crate::common::Reg<regs::Huffsymb47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ccusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb48(self) -> crate::common::Reg<regs::Huffsymb48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb49(self) -> crate::common::Reg<regs::Huffsymb49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb50(self) -> crate::common::Reg<regs::Huffsymb50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02d8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb51(self) -> crate::common::Reg<regs::Huffsymb51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02dcusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb52(self) -> crate::common::Reg<regs::Huffsymb52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb53(self) -> crate::common::Reg<regs::Huffsymb53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb54(self) -> crate::common::Reg<regs::Huffsymb54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02e8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb55(self) -> crate::common::Reg<regs::Huffsymb55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02ecusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb56(self) -> crate::common::Reg<regs::Huffsymb56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb57(self) -> crate::common::Reg<regs::Huffsymb57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb58(self) -> crate::common::Reg<regs::Huffsymb58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02f8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb59(self) -> crate::common::Reg<regs::Huffsymb59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x02fcusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb60(self) -> crate::common::Reg<regs::Huffsymb60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0300usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb61(self) -> crate::common::Reg<regs::Huffsymb61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0304usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb62(self) -> crate::common::Reg<regs::Huffsymb62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0308usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb63(self) -> crate::common::Reg<regs::Huffsymb63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x030cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb64(self) -> crate::common::Reg<regs::Huffsymb64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0310usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb65(self) -> crate::common::Reg<regs::Huffsymb65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0314usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb66(self) -> crate::common::Reg<regs::Huffsymb66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0318usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb67(self) -> crate::common::Reg<regs::Huffsymb67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x031cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb68(self) -> crate::common::Reg<regs::Huffsymb68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0320usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb69(self) -> crate::common::Reg<regs::Huffsymb69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0324usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb70(self) -> crate::common::Reg<regs::Huffsymb70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0328usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb71(self) -> crate::common::Reg<regs::Huffsymb71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x032cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb72(self) -> crate::common::Reg<regs::Huffsymb72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0330usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb73(self) -> crate::common::Reg<regs::Huffsymb73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0334usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb74(self) -> crate::common::Reg<regs::Huffsymb74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0338usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb75(self) -> crate::common::Reg<regs::Huffsymb75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x033cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb76(self) -> crate::common::Reg<regs::Huffsymb76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0340usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb77(self) -> crate::common::Reg<regs::Huffsymb77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0344usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb78(self) -> crate::common::Reg<regs::Huffsymb78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0348usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb79(self) -> crate::common::Reg<regs::Huffsymb79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x034cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb80(self) -> crate::common::Reg<regs::Huffsymb80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0350usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb81(self) -> crate::common::Reg<regs::Huffsymb81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0354usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb82(self) -> crate::common::Reg<regs::Huffsymb82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0358usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb83(self) -> crate::common::Reg<regs::Huffsymb83, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x035cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem0(self) -> crate::common::Reg<regs::Dhtmem0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0360usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem2(self) -> crate::common::Reg<regs::Dhtmem2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0364usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem3(self) -> crate::common::Reg<regs::Dhtmem3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0368usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem4(self) -> crate::common::Reg<regs::Dhtmem4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x036cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem5(self) -> crate::common::Reg<regs::Dhtmem5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0370usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem6(self) -> crate::common::Reg<regs::Dhtmem6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0374usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem7(self) -> crate::common::Reg<regs::Dhtmem7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0378usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem8(self) -> crate::common::Reg<regs::Dhtmem8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x037cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem9(self) -> crate::common::Reg<regs::Dhtmem9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0380usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem10(self) -> crate::common::Reg<regs::Dhtmem10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0384usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem11(self) -> crate::common::Reg<regs::Dhtmem11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0388usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem12(self) -> crate::common::Reg<regs::Dhtmem12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x038cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem13(self) -> crate::common::Reg<regs::Dhtmem13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0390usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem14(self) -> crate::common::Reg<regs::Dhtmem14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0394usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem15(self) -> crate::common::Reg<regs::Dhtmem15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0398usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem16(self) -> crate::common::Reg<regs::Dhtmem16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x039cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem17(self) -> crate::common::Reg<regs::Dhtmem17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem18(self) -> crate::common::Reg<regs::Dhtmem18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem19(self) -> crate::common::Reg<regs::Dhtmem19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03a8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem20(self) -> crate::common::Reg<regs::Dhtmem20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03acusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem21(self) -> crate::common::Reg<regs::Dhtmem21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem22(self) -> crate::common::Reg<regs::Dhtmem22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem23(self) -> crate::common::Reg<regs::Dhtmem23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03b8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem24(self) -> crate::common::Reg<regs::Dhtmem24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03bcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem25(self) -> crate::common::Reg<regs::Dhtmem25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem26(self) -> crate::common::Reg<regs::Dhtmem26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem27(self) -> crate::common::Reg<regs::Dhtmem27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03c8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem28(self) -> crate::common::Reg<regs::Dhtmem28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ccusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem29(self) -> crate::common::Reg<regs::Dhtmem29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem30(self) -> crate::common::Reg<regs::Dhtmem30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem31(self) -> crate::common::Reg<regs::Dhtmem31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03d8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem32(self) -> crate::common::Reg<regs::Dhtmem32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03dcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem33(self) -> crate::common::Reg<regs::Dhtmem33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem34(self) -> crate::common::Reg<regs::Dhtmem34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem35(self) -> crate::common::Reg<regs::Dhtmem35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03e8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem36(self) -> crate::common::Reg<regs::Dhtmem36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03ecusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem37(self) -> crate::common::Reg<regs::Dhtmem37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem38(self) -> crate::common::Reg<regs::Dhtmem38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem39(self) -> crate::common::Reg<regs::Dhtmem39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03f8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem40(self) -> crate::common::Reg<regs::Dhtmem40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x03fcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem41(self) -> crate::common::Reg<regs::Dhtmem41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0400usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem42(self) -> crate::common::Reg<regs::Dhtmem42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0404usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem43(self) -> crate::common::Reg<regs::Dhtmem43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0408usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem44(self) -> crate::common::Reg<regs::Dhtmem44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x040cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem45(self) -> crate::common::Reg<regs::Dhtmem45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0410usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem46(self) -> crate::common::Reg<regs::Dhtmem46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0414usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem47(self) -> crate::common::Reg<regs::Dhtmem47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0418usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem48(self) -> crate::common::Reg<regs::Dhtmem48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x041cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem49(self) -> crate::common::Reg<regs::Dhtmem49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0420usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem50(self) -> crate::common::Reg<regs::Dhtmem50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0424usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem51(self) -> crate::common::Reg<regs::Dhtmem51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0428usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem52(self) -> crate::common::Reg<regs::Dhtmem52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x042cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem53(self) -> crate::common::Reg<regs::Dhtmem53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0430usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem54(self) -> crate::common::Reg<regs::Dhtmem54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0434usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem55(self) -> crate::common::Reg<regs::Dhtmem55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0438usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem56(self) -> crate::common::Reg<regs::Dhtmem56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x043cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem57(self) -> crate::common::Reg<regs::Dhtmem57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0440usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem58(self) -> crate::common::Reg<regs::Dhtmem58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0444usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem59(self) -> crate::common::Reg<regs::Dhtmem59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0448usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem60(self) -> crate::common::Reg<regs::Dhtmem60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x044cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem61(self) -> crate::common::Reg<regs::Dhtmem61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0450usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem62(self) -> crate::common::Reg<regs::Dhtmem62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0454usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem63(self) -> crate::common::Reg<regs::Dhtmem63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0458usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem64(self) -> crate::common::Reg<regs::Dhtmem64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x045cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem65(self) -> crate::common::Reg<regs::Dhtmem65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0460usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem66(self) -> crate::common::Reg<regs::Dhtmem66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0464usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem67(self) -> crate::common::Reg<regs::Dhtmem67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0468usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem68(self) -> crate::common::Reg<regs::Dhtmem68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x046cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem69(self) -> crate::common::Reg<regs::Dhtmem69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0470usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem70(self) -> crate::common::Reg<regs::Dhtmem70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0474usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem71(self) -> crate::common::Reg<regs::Dhtmem71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0478usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem72(self) -> crate::common::Reg<regs::Dhtmem72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x047cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem73(self) -> crate::common::Reg<regs::Dhtmem73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0480usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem74(self) -> crate::common::Reg<regs::Dhtmem74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0484usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem75(self) -> crate::common::Reg<regs::Dhtmem75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0488usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem76(self) -> crate::common::Reg<regs::Dhtmem76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x048cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem77(self) -> crate::common::Reg<regs::Dhtmem77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0490usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem78(self) -> crate::common::Reg<regs::Dhtmem78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0494usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem79(self) -> crate::common::Reg<regs::Dhtmem79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0498usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem80(self) -> crate::common::Reg<regs::Dhtmem80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x049cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem81(self) -> crate::common::Reg<regs::Dhtmem81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem82(self) -> crate::common::Reg<regs::Dhtmem82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem83(self) -> crate::common::Reg<regs::Dhtmem83, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04a8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem84(self) -> crate::common::Reg<regs::Dhtmem84, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04acusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem85(self) -> crate::common::Reg<regs::Dhtmem85, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem86(self) -> crate::common::Reg<regs::Dhtmem86, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem87(self) -> crate::common::Reg<regs::Dhtmem87, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04b8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem88(self) -> crate::common::Reg<regs::Dhtmem88, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04bcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem89(self) -> crate::common::Reg<regs::Dhtmem89, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem90(self) -> crate::common::Reg<regs::Dhtmem90, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem91(self) -> crate::common::Reg<regs::Dhtmem91, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04c8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem92(self) -> crate::common::Reg<regs::Dhtmem92, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ccusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem93(self) -> crate::common::Reg<regs::Dhtmem93, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem94(self) -> crate::common::Reg<regs::Dhtmem94, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem95(self) -> crate::common::Reg<regs::Dhtmem95, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04d8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem96(self) -> crate::common::Reg<regs::Dhtmem96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04dcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem97(self) -> crate::common::Reg<regs::Dhtmem97, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem98(self) -> crate::common::Reg<regs::Dhtmem98, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem99(self) -> crate::common::Reg<regs::Dhtmem99, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04e8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem100(self) -> crate::common::Reg<regs::Dhtmem100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04ecusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem101(self) -> crate::common::Reg<regs::Dhtmem101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem102(self) -> crate::common::Reg<regs::Dhtmem102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem103(self) -> crate::common::Reg<regs::Dhtmem103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04f8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_0(self) -> crate::common::Reg<regs::HuffencAc0_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0500usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_1(self) -> crate::common::Reg<regs::HuffencAc0_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0504usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_2(self) -> crate::common::Reg<regs::HuffencAc0_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0508usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_3(self) -> crate::common::Reg<regs::HuffencAc0_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x050cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_4(self) -> crate::common::Reg<regs::HuffencAc0_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0510usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_5(self) -> crate::common::Reg<regs::HuffencAc0_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0514usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_6(self) -> crate::common::Reg<regs::HuffencAc0_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0518usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_7(self) -> crate::common::Reg<regs::HuffencAc0_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x051cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_8(self) -> crate::common::Reg<regs::HuffencAc0_8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0520usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_9(self) -> crate::common::Reg<regs::HuffencAc0_9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0524usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_10(self) -> crate::common::Reg<regs::HuffencAc0_10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0528usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_11(self) -> crate::common::Reg<regs::HuffencAc0_11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x052cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_12(self) -> crate::common::Reg<regs::HuffencAc0_12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0530usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_13(self) -> crate::common::Reg<regs::HuffencAc0_13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0534usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_14(self) -> crate::common::Reg<regs::HuffencAc0_14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0538usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_15(self) -> crate::common::Reg<regs::HuffencAc0_15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x053cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_16(self) -> crate::common::Reg<regs::HuffencAc0_16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0540usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_17(self) -> crate::common::Reg<regs::HuffencAc0_17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0544usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_18(self) -> crate::common::Reg<regs::HuffencAc0_18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0548usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_19(self) -> crate::common::Reg<regs::HuffencAc0_19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x054cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_20(self) -> crate::common::Reg<regs::HuffencAc0_20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0550usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_21(self) -> crate::common::Reg<regs::HuffencAc0_21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0554usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_22(self) -> crate::common::Reg<regs::HuffencAc0_22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0558usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_23(self) -> crate::common::Reg<regs::HuffencAc0_23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x055cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_24(self) -> crate::common::Reg<regs::HuffencAc0_24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0560usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_25(self) -> crate::common::Reg<regs::HuffencAc0_25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0564usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_26(self) -> crate::common::Reg<regs::HuffencAc0_26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0568usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_27(self) -> crate::common::Reg<regs::HuffencAc0_27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x056cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_28(self) -> crate::common::Reg<regs::HuffencAc0_28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0570usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_29(self) -> crate::common::Reg<regs::HuffencAc0_29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0574usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_30(self) -> crate::common::Reg<regs::HuffencAc0_30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0578usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_31(self) -> crate::common::Reg<regs::HuffencAc0_31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x057cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_32(self) -> crate::common::Reg<regs::HuffencAc0_32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0580usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_33(self) -> crate::common::Reg<regs::HuffencAc0_33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0584usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_34(self) -> crate::common::Reg<regs::HuffencAc0_34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0588usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_35(self) -> crate::common::Reg<regs::HuffencAc0_35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x058cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_36(self) -> crate::common::Reg<regs::HuffencAc0_36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0590usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_37(self) -> crate::common::Reg<regs::HuffencAc0_37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0594usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_38(self) -> crate::common::Reg<regs::HuffencAc0_38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0598usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_39(self) -> crate::common::Reg<regs::HuffencAc0_39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x059cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_40(self) -> crate::common::Reg<regs::HuffencAc0_40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_41(self) -> crate::common::Reg<regs::HuffencAc0_41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_42(self) -> crate::common::Reg<regs::HuffencAc0_42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05a8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_43(self) -> crate::common::Reg<regs::HuffencAc0_43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05acusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_44(self) -> crate::common::Reg<regs::HuffencAc0_44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_45(self) -> crate::common::Reg<regs::HuffencAc0_45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_46(self) -> crate::common::Reg<regs::HuffencAc0_46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05b8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_47(self) -> crate::common::Reg<regs::HuffencAc0_47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05bcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_48(self) -> crate::common::Reg<regs::HuffencAc0_48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_49(self) -> crate::common::Reg<regs::HuffencAc0_49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_50(self) -> crate::common::Reg<regs::HuffencAc0_50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05c8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_51(self) -> crate::common::Reg<regs::HuffencAc0_51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ccusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_52(self) -> crate::common::Reg<regs::HuffencAc0_52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_53(self) -> crate::common::Reg<regs::HuffencAc0_53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_54(self) -> crate::common::Reg<regs::HuffencAc0_54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05d8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_55(self) -> crate::common::Reg<regs::HuffencAc0_55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05dcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_56(self) -> crate::common::Reg<regs::HuffencAc0_56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_57(self) -> crate::common::Reg<regs::HuffencAc0_57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_58(self) -> crate::common::Reg<regs::HuffencAc0_58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05e8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_59(self) -> crate::common::Reg<regs::HuffencAc0_59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05ecusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_60(self) -> crate::common::Reg<regs::HuffencAc0_60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_61(self) -> crate::common::Reg<regs::HuffencAc0_61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_62(self) -> crate::common::Reg<regs::HuffencAc0_62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05f8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_63(self) -> crate::common::Reg<regs::HuffencAc0_63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x05fcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_64(self) -> crate::common::Reg<regs::HuffencAc0_64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0600usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_65(self) -> crate::common::Reg<regs::HuffencAc0_65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0604usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_66(self) -> crate::common::Reg<regs::HuffencAc0_66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0608usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_67(self) -> crate::common::Reg<regs::HuffencAc0_67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x060cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_68(self) -> crate::common::Reg<regs::HuffencAc0_68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0610usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_69(self) -> crate::common::Reg<regs::HuffencAc0_69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0614usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_70(self) -> crate::common::Reg<regs::HuffencAc0_70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0618usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_71(self) -> crate::common::Reg<regs::HuffencAc0_71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x061cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_72(self) -> crate::common::Reg<regs::HuffencAc0_72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0620usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_73(self) -> crate::common::Reg<regs::HuffencAc0_73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0624usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_74(self) -> crate::common::Reg<regs::HuffencAc0_74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0628usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_75(self) -> crate::common::Reg<regs::HuffencAc0_75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x062cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_76(self) -> crate::common::Reg<regs::HuffencAc0_76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0630usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_77(self) -> crate::common::Reg<regs::HuffencAc0_77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0634usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_78(self) -> crate::common::Reg<regs::HuffencAc0_78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0638usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_79(self) -> crate::common::Reg<regs::HuffencAc0_79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x063cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_80(self) -> crate::common::Reg<regs::HuffencAc0_80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0640usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_81(self) -> crate::common::Reg<regs::HuffencAc0_81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0644usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_82(self) -> crate::common::Reg<regs::HuffencAc0_82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0648usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_83(self) -> crate::common::Reg<regs::HuffencAc0_83, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x064cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_84(self) -> crate::common::Reg<regs::HuffencAc0_84, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0650usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_85(self) -> crate::common::Reg<regs::HuffencAc0_85, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0654usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_86(self) -> crate::common::Reg<regs::HuffencAc0_86, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0658usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_87(self) -> crate::common::Reg<regs::HuffencAc0_87, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x065cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_0(self) -> crate::common::Reg<regs::HuffencAc1_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0660usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_1(self) -> crate::common::Reg<regs::HuffencAc1_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0664usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_2(self) -> crate::common::Reg<regs::HuffencAc1_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0668usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_3(self) -> crate::common::Reg<regs::HuffencAc1_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x066cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_4(self) -> crate::common::Reg<regs::HuffencAc1_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0670usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_5(self) -> crate::common::Reg<regs::HuffencAc1_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0674usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_6(self) -> crate::common::Reg<regs::HuffencAc1_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0678usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_7(self) -> crate::common::Reg<regs::HuffencAc1_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x067cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_8(self) -> crate::common::Reg<regs::HuffencAc1_8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0680usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_9(self) -> crate::common::Reg<regs::HuffencAc1_9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0684usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_10(self) -> crate::common::Reg<regs::HuffencAc1_10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0688usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_11(self) -> crate::common::Reg<regs::HuffencAc1_11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x068cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_12(self) -> crate::common::Reg<regs::HuffencAc1_12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0690usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_13(self) -> crate::common::Reg<regs::HuffencAc1_13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0694usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_14(self) -> crate::common::Reg<regs::HuffencAc1_14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0698usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_15(self) -> crate::common::Reg<regs::HuffencAc1_15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x069cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_16(self) -> crate::common::Reg<regs::HuffencAc1_16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_17(self) -> crate::common::Reg<regs::HuffencAc1_17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_18(self) -> crate::common::Reg<regs::HuffencAc1_18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06a8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_19(self) -> crate::common::Reg<regs::HuffencAc1_19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06acusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_20(self) -> crate::common::Reg<regs::HuffencAc1_20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_21(self) -> crate::common::Reg<regs::HuffencAc1_21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_22(self) -> crate::common::Reg<regs::HuffencAc1_22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06b8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_23(self) -> crate::common::Reg<regs::HuffencAc1_23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06bcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_24(self) -> crate::common::Reg<regs::HuffencAc1_24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_25(self) -> crate::common::Reg<regs::HuffencAc1_25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_26(self) -> crate::common::Reg<regs::HuffencAc1_26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06c8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_27(self) -> crate::common::Reg<regs::HuffencAc1_27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06ccusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_28(self) -> crate::common::Reg<regs::HuffencAc1_28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_29(self) -> crate::common::Reg<regs::HuffencAc1_29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_30(self) -> crate::common::Reg<regs::HuffencAc1_30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06d8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_31(self) -> crate::common::Reg<regs::HuffencAc1_31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06dcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_32(self) -> crate::common::Reg<regs::HuffencAc1_32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_33(self) -> crate::common::Reg<regs::HuffencAc1_33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_34(self) -> crate::common::Reg<regs::HuffencAc1_34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06e8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_35(self) -> crate::common::Reg<regs::HuffencAc1_35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06ecusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_36(self) -> crate::common::Reg<regs::HuffencAc1_36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_37(self) -> crate::common::Reg<regs::HuffencAc1_37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_38(self) -> crate::common::Reg<regs::HuffencAc1_38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06f8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_39(self) -> crate::common::Reg<regs::HuffencAc1_39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x06fcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_40(self) -> crate::common::Reg<regs::HuffencAc1_40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0700usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_41(self) -> crate::common::Reg<regs::HuffencAc1_41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0704usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_42(self) -> crate::common::Reg<regs::HuffencAc1_42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0708usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_43(self) -> crate::common::Reg<regs::HuffencAc1_43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x070cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_44(self) -> crate::common::Reg<regs::HuffencAc1_44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0710usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_45(self) -> crate::common::Reg<regs::HuffencAc1_45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0714usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_46(self) -> crate::common::Reg<regs::HuffencAc1_46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0718usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_47(self) -> crate::common::Reg<regs::HuffencAc1_47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x071cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_48(self) -> crate::common::Reg<regs::HuffencAc1_48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0720usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_49(self) -> crate::common::Reg<regs::HuffencAc1_49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0724usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_50(self) -> crate::common::Reg<regs::HuffencAc1_50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0728usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_51(self) -> crate::common::Reg<regs::HuffencAc1_51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x072cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_52(self) -> crate::common::Reg<regs::HuffencAc1_52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0730usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_53(self) -> crate::common::Reg<regs::HuffencAc1_53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0734usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_54(self) -> crate::common::Reg<regs::HuffencAc1_54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0738usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_55(self) -> crate::common::Reg<regs::HuffencAc1_55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x073cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_56(self) -> crate::common::Reg<regs::HuffencAc1_56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0740usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_57(self) -> crate::common::Reg<regs::HuffencAc1_57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0744usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_58(self) -> crate::common::Reg<regs::HuffencAc1_58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0748usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_59(self) -> crate::common::Reg<regs::HuffencAc1_59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x074cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_60(self) -> crate::common::Reg<regs::HuffencAc1_60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0750usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_61(self) -> crate::common::Reg<regs::HuffencAc1_61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0754usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_62(self) -> crate::common::Reg<regs::HuffencAc1_62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0758usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_63(self) -> crate::common::Reg<regs::HuffencAc1_63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x075cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_64(self) -> crate::common::Reg<regs::HuffencAc1_64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0760usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_65(self) -> crate::common::Reg<regs::HuffencAc1_65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0764usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_66(self) -> crate::common::Reg<regs::HuffencAc1_66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0768usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_67(self) -> crate::common::Reg<regs::HuffencAc1_67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x076cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_68(self) -> crate::common::Reg<regs::HuffencAc1_68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0770usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_69(self) -> crate::common::Reg<regs::HuffencAc1_69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0774usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_70(self) -> crate::common::Reg<regs::HuffencAc1_70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0778usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_71(self) -> crate::common::Reg<regs::HuffencAc1_71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x077cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_72(self) -> crate::common::Reg<regs::HuffencAc1_72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0780usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_73(self) -> crate::common::Reg<regs::HuffencAc1_73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0784usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_74(self) -> crate::common::Reg<regs::HuffencAc1_74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0788usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_75(self) -> crate::common::Reg<regs::HuffencAc1_75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x078cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_76(self) -> crate::common::Reg<regs::HuffencAc1_76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0790usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_77(self) -> crate::common::Reg<regs::HuffencAc1_77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0794usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_78(self) -> crate::common::Reg<regs::HuffencAc1_78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0798usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_79(self) -> crate::common::Reg<regs::HuffencAc1_79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x079cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_80(self) -> crate::common::Reg<regs::HuffencAc1_80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_81(self) -> crate::common::Reg<regs::HuffencAc1_81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_82(self) -> crate::common::Reg<regs::HuffencAc1_82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07a8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_83(self) -> crate::common::Reg<regs::HuffencAc1_83, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07acusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_84(self) -> crate::common::Reg<regs::HuffencAc1_84, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_85(self) -> crate::common::Reg<regs::HuffencAc1_85, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_86(self) -> crate::common::Reg<regs::HuffencAc1_86, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07b8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_87(self) -> crate::common::Reg<regs::HuffencAc1_87, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07bcusize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_0(self) -> crate::common::Reg<regs::HuffencDc0_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c0usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_1(self) -> crate::common::Reg<regs::HuffencDc0_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c4usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_2(self) -> crate::common::Reg<regs::HuffencDc0_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07c8usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_3(self) -> crate::common::Reg<regs::HuffencDc0_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07ccusize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_4(self) -> crate::common::Reg<regs::HuffencDc0_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07d0usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_5(self) -> crate::common::Reg<regs::HuffencDc0_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07d4usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_6(self) -> crate::common::Reg<regs::HuffencDc0_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07d8usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_7(self) -> crate::common::Reg<regs::HuffencDc0_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07dcusize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_0(self) -> crate::common::Reg<regs::HuffencDc1_0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07e0usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_1(self) -> crate::common::Reg<regs::HuffencDc1_1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07e4usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_2(self) -> crate::common::Reg<regs::HuffencDc1_2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07e8usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_3(self) -> crate::common::Reg<regs::HuffencDc1_3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07ecusize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_4(self) -> crate::common::Reg<regs::HuffencDc1_4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f0usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_5(self) -> crate::common::Reg<regs::HuffencDc1_5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f4usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_6(self) -> crate::common::Reg<regs::HuffencDc1_6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07f8usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_7(self) -> crate::common::Reg<regs::HuffencDc1_7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x07fcusize) as _) }
    }
}
pub mod regs {
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem0(pub u32);
    impl Dhtmem0 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem0 {
        #[inline(always)]
        fn default() -> Dhtmem0 {
            Dhtmem0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem0")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem0 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem10(pub u32);
    impl Dhtmem10 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem10 {
        #[inline(always)]
        fn default() -> Dhtmem10 {
            Dhtmem10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem10")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem10 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem100(pub u32);
    impl Dhtmem100 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem100 {
        #[inline(always)]
        fn default() -> Dhtmem100 {
            Dhtmem100(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem100 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem100")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem100 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem100 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem101(pub u32);
    impl Dhtmem101 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem101 {
        #[inline(always)]
        fn default() -> Dhtmem101 {
            Dhtmem101(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem101 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem101")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem101 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem101 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem102(pub u32);
    impl Dhtmem102 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem102 {
        #[inline(always)]
        fn default() -> Dhtmem102 {
            Dhtmem102(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem102 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem102")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem102 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem102 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem103(pub u32);
    impl Dhtmem103 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem103 {
        #[inline(always)]
        fn default() -> Dhtmem103 {
            Dhtmem103(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem103 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem103")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem103 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem103 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem11(pub u32);
    impl Dhtmem11 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem11 {
        #[inline(always)]
        fn default() -> Dhtmem11 {
            Dhtmem11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem11")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem11 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem12(pub u32);
    impl Dhtmem12 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem12 {
        #[inline(always)]
        fn default() -> Dhtmem12 {
            Dhtmem12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem12")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem12 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem13(pub u32);
    impl Dhtmem13 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem13 {
        #[inline(always)]
        fn default() -> Dhtmem13 {
            Dhtmem13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem13")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem13 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem14(pub u32);
    impl Dhtmem14 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem14 {
        #[inline(always)]
        fn default() -> Dhtmem14 {
            Dhtmem14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem14")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem14 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem15(pub u32);
    impl Dhtmem15 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem15 {
        #[inline(always)]
        fn default() -> Dhtmem15 {
            Dhtmem15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem15")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem15 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem16(pub u32);
    impl Dhtmem16 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem16 {
        #[inline(always)]
        fn default() -> Dhtmem16 {
            Dhtmem16(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem16")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem16 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem17(pub u32);
    impl Dhtmem17 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem17 {
        #[inline(always)]
        fn default() -> Dhtmem17 {
            Dhtmem17(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem17")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem17 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem17 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem18(pub u32);
    impl Dhtmem18 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem18 {
        #[inline(always)]
        fn default() -> Dhtmem18 {
            Dhtmem18(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem18")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem18 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem18 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem19(pub u32);
    impl Dhtmem19 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem19 {
        #[inline(always)]
        fn default() -> Dhtmem19 {
            Dhtmem19(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem19")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem19 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem19 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem2(pub u32);
    impl Dhtmem2 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem2 {
        #[inline(always)]
        fn default() -> Dhtmem2 {
            Dhtmem2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem2")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem2 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem20(pub u32);
    impl Dhtmem20 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem20 {
        #[inline(always)]
        fn default() -> Dhtmem20 {
            Dhtmem20(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem20 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem20")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem20 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem20 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem21(pub u32);
    impl Dhtmem21 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem21 {
        #[inline(always)]
        fn default() -> Dhtmem21 {
            Dhtmem21(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem21 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem21")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem21 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem21 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem22(pub u32);
    impl Dhtmem22 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem22 {
        #[inline(always)]
        fn default() -> Dhtmem22 {
            Dhtmem22(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem22 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem22")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem22 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem22 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem23(pub u32);
    impl Dhtmem23 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem23 {
        #[inline(always)]
        fn default() -> Dhtmem23 {
            Dhtmem23(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem23")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem23 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem23 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem24(pub u32);
    impl Dhtmem24 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem24 {
        #[inline(always)]
        fn default() -> Dhtmem24 {
            Dhtmem24(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem24 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem24")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem24 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem24 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem25(pub u32);
    impl Dhtmem25 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem25 {
        #[inline(always)]
        fn default() -> Dhtmem25 {
            Dhtmem25(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem25 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem25")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem25 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem25 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem26(pub u32);
    impl Dhtmem26 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem26 {
        #[inline(always)]
        fn default() -> Dhtmem26 {
            Dhtmem26(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem26 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem26")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem26 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem26 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem27(pub u32);
    impl Dhtmem27 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem27 {
        #[inline(always)]
        fn default() -> Dhtmem27 {
            Dhtmem27(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem27")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem27 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem27 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem28(pub u32);
    impl Dhtmem28 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem28 {
        #[inline(always)]
        fn default() -> Dhtmem28 {
            Dhtmem28(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem28")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem28 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem28 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem29(pub u32);
    impl Dhtmem29 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem29 {
        #[inline(always)]
        fn default() -> Dhtmem29 {
            Dhtmem29(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem29 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem29")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem29 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem29 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem3(pub u32);
    impl Dhtmem3 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem3 {
        #[inline(always)]
        fn default() -> Dhtmem3 {
            Dhtmem3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem3")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem3 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem30(pub u32);
    impl Dhtmem30 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem30 {
        #[inline(always)]
        fn default() -> Dhtmem30 {
            Dhtmem30(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem30")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem30 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem30 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem31(pub u32);
    impl Dhtmem31 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem31 {
        #[inline(always)]
        fn default() -> Dhtmem31 {
            Dhtmem31(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem31 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem31")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem31 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem31 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem32(pub u32);
    impl Dhtmem32 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem32 {
        #[inline(always)]
        fn default() -> Dhtmem32 {
            Dhtmem32(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem32")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem32 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem33(pub u32);
    impl Dhtmem33 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem33 {
        #[inline(always)]
        fn default() -> Dhtmem33 {
            Dhtmem33(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem33 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem33")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem33 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem33 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem34(pub u32);
    impl Dhtmem34 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem34 {
        #[inline(always)]
        fn default() -> Dhtmem34 {
            Dhtmem34(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem34")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem34 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem34 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem35(pub u32);
    impl Dhtmem35 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem35 {
        #[inline(always)]
        fn default() -> Dhtmem35 {
            Dhtmem35(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem35 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem35")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem35 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem35 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem36(pub u32);
    impl Dhtmem36 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem36 {
        #[inline(always)]
        fn default() -> Dhtmem36 {
            Dhtmem36(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem36 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem36")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem36 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem36 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem37(pub u32);
    impl Dhtmem37 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem37 {
        #[inline(always)]
        fn default() -> Dhtmem37 {
            Dhtmem37(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem37 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem37")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem37 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem37 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem38(pub u32);
    impl Dhtmem38 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem38 {
        #[inline(always)]
        fn default() -> Dhtmem38 {
            Dhtmem38(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem38 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem38")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem38 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem38 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem39(pub u32);
    impl Dhtmem39 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem39 {
        #[inline(always)]
        fn default() -> Dhtmem39 {
            Dhtmem39(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem39 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem39")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem39 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem39 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem4(pub u32);
    impl Dhtmem4 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem4 {
        #[inline(always)]
        fn default() -> Dhtmem4 {
            Dhtmem4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem4")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem4 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem40(pub u32);
    impl Dhtmem40 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem40 {
        #[inline(always)]
        fn default() -> Dhtmem40 {
            Dhtmem40(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem40 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem40")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem40 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem40 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem41(pub u32);
    impl Dhtmem41 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem41 {
        #[inline(always)]
        fn default() -> Dhtmem41 {
            Dhtmem41(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem41 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem41")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem41 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem41 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem42(pub u32);
    impl Dhtmem42 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem42 {
        #[inline(always)]
        fn default() -> Dhtmem42 {
            Dhtmem42(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem42 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem42")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem42 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem42 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem43(pub u32);
    impl Dhtmem43 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem43 {
        #[inline(always)]
        fn default() -> Dhtmem43 {
            Dhtmem43(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem43 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem43")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem43 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem43 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem44(pub u32);
    impl Dhtmem44 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem44 {
        #[inline(always)]
        fn default() -> Dhtmem44 {
            Dhtmem44(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem44 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem44")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem44 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem44 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem45(pub u32);
    impl Dhtmem45 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem45 {
        #[inline(always)]
        fn default() -> Dhtmem45 {
            Dhtmem45(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem45 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem45")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem45 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem45 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem46(pub u32);
    impl Dhtmem46 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem46 {
        #[inline(always)]
        fn default() -> Dhtmem46 {
            Dhtmem46(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem46 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem46")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem46 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem46 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem47(pub u32);
    impl Dhtmem47 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem47 {
        #[inline(always)]
        fn default() -> Dhtmem47 {
            Dhtmem47(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem47 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem47")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem47 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem47 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem48(pub u32);
    impl Dhtmem48 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem48 {
        #[inline(always)]
        fn default() -> Dhtmem48 {
            Dhtmem48(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem48 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem48")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem48 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem48 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem49(pub u32);
    impl Dhtmem49 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem49 {
        #[inline(always)]
        fn default() -> Dhtmem49 {
            Dhtmem49(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem49 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem49")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem49 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem49 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem5(pub u32);
    impl Dhtmem5 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem5 {
        #[inline(always)]
        fn default() -> Dhtmem5 {
            Dhtmem5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem5")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem5 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem50(pub u32);
    impl Dhtmem50 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem50 {
        #[inline(always)]
        fn default() -> Dhtmem50 {
            Dhtmem50(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem50 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem50")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem50 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem50 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem51(pub u32);
    impl Dhtmem51 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem51 {
        #[inline(always)]
        fn default() -> Dhtmem51 {
            Dhtmem51(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem51 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem51")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem51 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem51 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem52(pub u32);
    impl Dhtmem52 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem52 {
        #[inline(always)]
        fn default() -> Dhtmem52 {
            Dhtmem52(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem52 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem52")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem52 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem52 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem53(pub u32);
    impl Dhtmem53 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem53 {
        #[inline(always)]
        fn default() -> Dhtmem53 {
            Dhtmem53(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem53 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem53")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem53 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem53 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem54(pub u32);
    impl Dhtmem54 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem54 {
        #[inline(always)]
        fn default() -> Dhtmem54 {
            Dhtmem54(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem54 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem54")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem54 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem54 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem55(pub u32);
    impl Dhtmem55 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem55 {
        #[inline(always)]
        fn default() -> Dhtmem55 {
            Dhtmem55(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem55 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem55")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem55 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem55 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem56(pub u32);
    impl Dhtmem56 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem56 {
        #[inline(always)]
        fn default() -> Dhtmem56 {
            Dhtmem56(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem56")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem56 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem56 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem57(pub u32);
    impl Dhtmem57 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem57 {
        #[inline(always)]
        fn default() -> Dhtmem57 {
            Dhtmem57(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem57 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem57")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem57 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem57 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem58(pub u32);
    impl Dhtmem58 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem58 {
        #[inline(always)]
        fn default() -> Dhtmem58 {
            Dhtmem58(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem58 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem58")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem58 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem58 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem59(pub u32);
    impl Dhtmem59 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem59 {
        #[inline(always)]
        fn default() -> Dhtmem59 {
            Dhtmem59(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem59 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem59")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem59 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem59 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem6(pub u32);
    impl Dhtmem6 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem6 {
        #[inline(always)]
        fn default() -> Dhtmem6 {
            Dhtmem6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem6")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem6 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem60(pub u32);
    impl Dhtmem60 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem60 {
        #[inline(always)]
        fn default() -> Dhtmem60 {
            Dhtmem60(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem60 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem60")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem60 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem60 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem61(pub u32);
    impl Dhtmem61 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem61 {
        #[inline(always)]
        fn default() -> Dhtmem61 {
            Dhtmem61(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem61 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem61")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem61 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem61 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem62(pub u32);
    impl Dhtmem62 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem62 {
        #[inline(always)]
        fn default() -> Dhtmem62 {
            Dhtmem62(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem62 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem62")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem62 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem62 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem63(pub u32);
    impl Dhtmem63 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem63 {
        #[inline(always)]
        fn default() -> Dhtmem63 {
            Dhtmem63(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem63 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem63")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem63 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem63 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem64(pub u32);
    impl Dhtmem64 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem64 {
        #[inline(always)]
        fn default() -> Dhtmem64 {
            Dhtmem64(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem64 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem64")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem64 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem64 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem65(pub u32);
    impl Dhtmem65 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem65 {
        #[inline(always)]
        fn default() -> Dhtmem65 {
            Dhtmem65(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem65 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem65")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem65 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem65 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem66(pub u32);
    impl Dhtmem66 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem66 {
        #[inline(always)]
        fn default() -> Dhtmem66 {
            Dhtmem66(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem66 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem66")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem66 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem66 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem67(pub u32);
    impl Dhtmem67 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem67 {
        #[inline(always)]
        fn default() -> Dhtmem67 {
            Dhtmem67(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem67 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem67")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem67 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem67 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem68(pub u32);
    impl Dhtmem68 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem68 {
        #[inline(always)]
        fn default() -> Dhtmem68 {
            Dhtmem68(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem68 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem68")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem68 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem68 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem69(pub u32);
    impl Dhtmem69 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem69 {
        #[inline(always)]
        fn default() -> Dhtmem69 {
            Dhtmem69(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem69 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem69")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem69 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem69 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem7(pub u32);
    impl Dhtmem7 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem7 {
        #[inline(always)]
        fn default() -> Dhtmem7 {
            Dhtmem7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem7")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem7 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem70(pub u32);
    impl Dhtmem70 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem70 {
        #[inline(always)]
        fn default() -> Dhtmem70 {
            Dhtmem70(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem70 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem70")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem70 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem70 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem71(pub u32);
    impl Dhtmem71 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem71 {
        #[inline(always)]
        fn default() -> Dhtmem71 {
            Dhtmem71(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem71 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem71")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem71 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem71 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem72(pub u32);
    impl Dhtmem72 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem72 {
        #[inline(always)]
        fn default() -> Dhtmem72 {
            Dhtmem72(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem72 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem72")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem72 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem72 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem73(pub u32);
    impl Dhtmem73 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem73 {
        #[inline(always)]
        fn default() -> Dhtmem73 {
            Dhtmem73(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem73 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem73")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem73 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem73 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem74(pub u32);
    impl Dhtmem74 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem74 {
        #[inline(always)]
        fn default() -> Dhtmem74 {
            Dhtmem74(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem74 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem74")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem74 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem74 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem75(pub u32);
    impl Dhtmem75 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem75 {
        #[inline(always)]
        fn default() -> Dhtmem75 {
            Dhtmem75(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem75 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem75")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem75 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem75 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem76(pub u32);
    impl Dhtmem76 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem76 {
        #[inline(always)]
        fn default() -> Dhtmem76 {
            Dhtmem76(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem76 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem76")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem76 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem76 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem77(pub u32);
    impl Dhtmem77 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem77 {
        #[inline(always)]
        fn default() -> Dhtmem77 {
            Dhtmem77(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem77 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem77")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem77 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem77 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem78(pub u32);
    impl Dhtmem78 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem78 {
        #[inline(always)]
        fn default() -> Dhtmem78 {
            Dhtmem78(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem78")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem78 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem78 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem79(pub u32);
    impl Dhtmem79 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem79 {
        #[inline(always)]
        fn default() -> Dhtmem79 {
            Dhtmem79(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem79 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem79")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem79 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem79 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem8(pub u32);
    impl Dhtmem8 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem8 {
        #[inline(always)]
        fn default() -> Dhtmem8 {
            Dhtmem8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem8")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem8 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem80(pub u32);
    impl Dhtmem80 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem80 {
        #[inline(always)]
        fn default() -> Dhtmem80 {
            Dhtmem80(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem80 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem80")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem80 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem80 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem81(pub u32);
    impl Dhtmem81 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem81 {
        #[inline(always)]
        fn default() -> Dhtmem81 {
            Dhtmem81(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem81 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem81")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem81 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem81 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem82(pub u32);
    impl Dhtmem82 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem82 {
        #[inline(always)]
        fn default() -> Dhtmem82 {
            Dhtmem82(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem82 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem82")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem82 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem82 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem83(pub u32);
    impl Dhtmem83 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem83 {
        #[inline(always)]
        fn default() -> Dhtmem83 {
            Dhtmem83(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem83 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem83")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem83 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem83 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem84(pub u32);
    impl Dhtmem84 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem84 {
        #[inline(always)]
        fn default() -> Dhtmem84 {
            Dhtmem84(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem84 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem84")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem84 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem84 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem85(pub u32);
    impl Dhtmem85 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem85 {
        #[inline(always)]
        fn default() -> Dhtmem85 {
            Dhtmem85(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem85 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem85")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem85 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem85 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem86(pub u32);
    impl Dhtmem86 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem86 {
        #[inline(always)]
        fn default() -> Dhtmem86 {
            Dhtmem86(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem86 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem86")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem86 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem86 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem87(pub u32);
    impl Dhtmem87 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem87 {
        #[inline(always)]
        fn default() -> Dhtmem87 {
            Dhtmem87(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem87 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem87")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem87 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem87 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem88(pub u32);
    impl Dhtmem88 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem88 {
        #[inline(always)]
        fn default() -> Dhtmem88 {
            Dhtmem88(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem88 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem88")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem88 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem88 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem89(pub u32);
    impl Dhtmem89 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem89 {
        #[inline(always)]
        fn default() -> Dhtmem89 {
            Dhtmem89(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem89 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem89")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem89 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem89 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem9(pub u32);
    impl Dhtmem9 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem9 {
        #[inline(always)]
        fn default() -> Dhtmem9 {
            Dhtmem9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem9")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem9 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem90(pub u32);
    impl Dhtmem90 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem90 {
        #[inline(always)]
        fn default() -> Dhtmem90 {
            Dhtmem90(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem90 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem90")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem90 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem90 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem91(pub u32);
    impl Dhtmem91 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem91 {
        #[inline(always)]
        fn default() -> Dhtmem91 {
            Dhtmem91(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem91 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem91")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem91 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem91 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem92(pub u32);
    impl Dhtmem92 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem92 {
        #[inline(always)]
        fn default() -> Dhtmem92 {
            Dhtmem92(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem92 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem92")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem92 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem92 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem93(pub u32);
    impl Dhtmem93 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem93 {
        #[inline(always)]
        fn default() -> Dhtmem93 {
            Dhtmem93(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem93 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem93")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem93 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem93 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem94(pub u32);
    impl Dhtmem94 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem94 {
        #[inline(always)]
        fn default() -> Dhtmem94 {
            Dhtmem94(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem94 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem94")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem94 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem94 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem95(pub u32);
    impl Dhtmem95 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem95 {
        #[inline(always)]
        fn default() -> Dhtmem95 {
            Dhtmem95(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem95 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem95")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem95 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem95 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem96(pub u32);
    impl Dhtmem96 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem96 {
        #[inline(always)]
        fn default() -> Dhtmem96 {
            Dhtmem96(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem96 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem96")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem96 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem96 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem97(pub u32);
    impl Dhtmem97 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem97 {
        #[inline(always)]
        fn default() -> Dhtmem97 {
            Dhtmem97(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem97 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem97")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem97 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem97 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem98(pub u32);
    impl Dhtmem98 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem98 {
        #[inline(always)]
        fn default() -> Dhtmem98 {
            Dhtmem98(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem98 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem98")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem98 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem98 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem99(pub u32);
    impl Dhtmem99 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem99 {
        #[inline(always)]
        fn default() -> Dhtmem99 {
            Dhtmem99(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Dhtmem99 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dhtmem99")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dhtmem99 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dhtmem99 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase0(pub u32);
    impl Huffbase0 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase0 {
        #[inline(always)]
        fn default() -> Huffbase0 {
            Huffbase0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase0")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase0 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase1(pub u32);
    impl Huffbase1 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase1 {
        #[inline(always)]
        fn default() -> Huffbase1 {
            Huffbase1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase1")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase1 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase10(pub u32);
    impl Huffbase10 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase10 {
        #[inline(always)]
        fn default() -> Huffbase10 {
            Huffbase10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase10")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase10 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase11(pub u32);
    impl Huffbase11 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase11 {
        #[inline(always)]
        fn default() -> Huffbase11 {
            Huffbase11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase11")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase11 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase12(pub u32);
    impl Huffbase12 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase12 {
        #[inline(always)]
        fn default() -> Huffbase12 {
            Huffbase12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase12")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase12 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase13(pub u32);
    impl Huffbase13 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase13 {
        #[inline(always)]
        fn default() -> Huffbase13 {
            Huffbase13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase13")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase13 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase14(pub u32);
    impl Huffbase14 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase14 {
        #[inline(always)]
        fn default() -> Huffbase14 {
            Huffbase14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase14")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase14 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase15(pub u32);
    impl Huffbase15 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase15 {
        #[inline(always)]
        fn default() -> Huffbase15 {
            Huffbase15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase15")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase15 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase16(pub u32);
    impl Huffbase16 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase16 {
        #[inline(always)]
        fn default() -> Huffbase16 {
            Huffbase16(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase16")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase16 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase17(pub u32);
    impl Huffbase17 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase17 {
        #[inline(always)]
        fn default() -> Huffbase17 {
            Huffbase17(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase17")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase17 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase17 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase18(pub u32);
    impl Huffbase18 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase18 {
        #[inline(always)]
        fn default() -> Huffbase18 {
            Huffbase18(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase18")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase18 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase18 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase19(pub u32);
    impl Huffbase19 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase19 {
        #[inline(always)]
        fn default() -> Huffbase19 {
            Huffbase19(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase19")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase19 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase19 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase2(pub u32);
    impl Huffbase2 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase2 {
        #[inline(always)]
        fn default() -> Huffbase2 {
            Huffbase2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase2")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase2 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase20(pub u32);
    impl Huffbase20 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase20 {
        #[inline(always)]
        fn default() -> Huffbase20 {
            Huffbase20(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase20 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase20")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase20 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase20 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase21(pub u32);
    impl Huffbase21 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase21 {
        #[inline(always)]
        fn default() -> Huffbase21 {
            Huffbase21(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase21 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase21")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase21 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase21 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase22(pub u32);
    impl Huffbase22 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase22 {
        #[inline(always)]
        fn default() -> Huffbase22 {
            Huffbase22(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase22 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase22")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase22 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase22 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase23(pub u32);
    impl Huffbase23 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase23 {
        #[inline(always)]
        fn default() -> Huffbase23 {
            Huffbase23(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase23")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase23 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase23 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase24(pub u32);
    impl Huffbase24 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase24 {
        #[inline(always)]
        fn default() -> Huffbase24 {
            Huffbase24(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase24 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase24")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase24 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase24 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase25(pub u32);
    impl Huffbase25 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase25 {
        #[inline(always)]
        fn default() -> Huffbase25 {
            Huffbase25(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase25 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase25")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase25 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase25 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase26(pub u32);
    impl Huffbase26 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase26 {
        #[inline(always)]
        fn default() -> Huffbase26 {
            Huffbase26(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase26 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase26")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase26 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase26 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase27(pub u32);
    impl Huffbase27 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase27 {
        #[inline(always)]
        fn default() -> Huffbase27 {
            Huffbase27(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase27")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase27 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase27 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase28(pub u32);
    impl Huffbase28 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase28 {
        #[inline(always)]
        fn default() -> Huffbase28 {
            Huffbase28(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase28")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase28 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase28 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase29(pub u32);
    impl Huffbase29 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase29 {
        #[inline(always)]
        fn default() -> Huffbase29 {
            Huffbase29(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase29 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase29")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase29 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase29 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase3(pub u32);
    impl Huffbase3 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase3 {
        #[inline(always)]
        fn default() -> Huffbase3 {
            Huffbase3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase3")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase3 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase30(pub u32);
    impl Huffbase30 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase30 {
        #[inline(always)]
        fn default() -> Huffbase30 {
            Huffbase30(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase30")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase30 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase30 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase31(pub u32);
    impl Huffbase31 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase31 {
        #[inline(always)]
        fn default() -> Huffbase31 {
            Huffbase31(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase31 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase31")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase31 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase31 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase4(pub u32);
    impl Huffbase4 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase4 {
        #[inline(always)]
        fn default() -> Huffbase4 {
            Huffbase4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase4")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase4 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase5(pub u32);
    impl Huffbase5 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase5 {
        #[inline(always)]
        fn default() -> Huffbase5 {
            Huffbase5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase5")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase5 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase6(pub u32);
    impl Huffbase6 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase6 {
        #[inline(always)]
        fn default() -> Huffbase6 {
            Huffbase6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase6")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase6 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase7(pub u32);
    impl Huffbase7 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase7 {
        #[inline(always)]
        fn default() -> Huffbase7 {
            Huffbase7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase7")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase7 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase8(pub u32);
    impl Huffbase8 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase8 {
        #[inline(always)]
        fn default() -> Huffbase8 {
            Huffbase8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase8")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase8 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase9(pub u32);
    impl Huffbase9 {
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase9 {
        #[inline(always)]
        fn default() -> Huffbase9 {
            Huffbase9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffbase9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffbase9")
                .field("huff_base_ram_0", &self.huff_base_ram_0())
                .field("huff_base_ram_1", &self.huff_base_ram_1())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffbase9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Huffbase9 {{ huff_base_ram_0: {=u16:?}, huff_base_ram_1: {=u16:?} }}",
                self.huff_base_ram_0(),
                self.huff_base_ram_1()
            )
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_0(pub u32);
    impl HuffencAc0_0 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_0 {
        #[inline(always)]
        fn default() -> HuffencAc0_0 {
            HuffencAc0_0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_0")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_0 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_1(pub u32);
    impl HuffencAc0_1 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_1 {
        #[inline(always)]
        fn default() -> HuffencAc0_1 {
            HuffencAc0_1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_1")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_1 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_10(pub u32);
    impl HuffencAc0_10 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_10 {
        #[inline(always)]
        fn default() -> HuffencAc0_10 {
            HuffencAc0_10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_10")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_10 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_11(pub u32);
    impl HuffencAc0_11 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_11 {
        #[inline(always)]
        fn default() -> HuffencAc0_11 {
            HuffencAc0_11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_11")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_11 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_12(pub u32);
    impl HuffencAc0_12 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_12 {
        #[inline(always)]
        fn default() -> HuffencAc0_12 {
            HuffencAc0_12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_12")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_12 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_13(pub u32);
    impl HuffencAc0_13 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_13 {
        #[inline(always)]
        fn default() -> HuffencAc0_13 {
            HuffencAc0_13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_13")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_13 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_14(pub u32);
    impl HuffencAc0_14 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_14 {
        #[inline(always)]
        fn default() -> HuffencAc0_14 {
            HuffencAc0_14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_14")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_14 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_15(pub u32);
    impl HuffencAc0_15 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_15 {
        #[inline(always)]
        fn default() -> HuffencAc0_15 {
            HuffencAc0_15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_15")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_15 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_16(pub u32);
    impl HuffencAc0_16 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_16 {
        #[inline(always)]
        fn default() -> HuffencAc0_16 {
            HuffencAc0_16(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_16")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_16 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_17(pub u32);
    impl HuffencAc0_17 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_17 {
        #[inline(always)]
        fn default() -> HuffencAc0_17 {
            HuffencAc0_17(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_17")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_17 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_17 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_18(pub u32);
    impl HuffencAc0_18 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_18 {
        #[inline(always)]
        fn default() -> HuffencAc0_18 {
            HuffencAc0_18(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_18")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_18 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_18 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_19(pub u32);
    impl HuffencAc0_19 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_19 {
        #[inline(always)]
        fn default() -> HuffencAc0_19 {
            HuffencAc0_19(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_19")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_19 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_19 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_2(pub u32);
    impl HuffencAc0_2 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_2 {
        #[inline(always)]
        fn default() -> HuffencAc0_2 {
            HuffencAc0_2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_2")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_2 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_20(pub u32);
    impl HuffencAc0_20 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_20 {
        #[inline(always)]
        fn default() -> HuffencAc0_20 {
            HuffencAc0_20(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_20 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_20")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_20 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_20 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_21(pub u32);
    impl HuffencAc0_21 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_21 {
        #[inline(always)]
        fn default() -> HuffencAc0_21 {
            HuffencAc0_21(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_21 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_21")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_21 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_21 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_22(pub u32);
    impl HuffencAc0_22 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_22 {
        #[inline(always)]
        fn default() -> HuffencAc0_22 {
            HuffencAc0_22(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_22 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_22")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_22 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_22 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_23(pub u32);
    impl HuffencAc0_23 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_23 {
        #[inline(always)]
        fn default() -> HuffencAc0_23 {
            HuffencAc0_23(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_23")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_23 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_23 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_24(pub u32);
    impl HuffencAc0_24 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_24 {
        #[inline(always)]
        fn default() -> HuffencAc0_24 {
            HuffencAc0_24(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_24 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_24")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_24 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_24 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_25(pub u32);
    impl HuffencAc0_25 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_25 {
        #[inline(always)]
        fn default() -> HuffencAc0_25 {
            HuffencAc0_25(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_25 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_25")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_25 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_25 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_26(pub u32);
    impl HuffencAc0_26 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_26 {
        #[inline(always)]
        fn default() -> HuffencAc0_26 {
            HuffencAc0_26(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_26 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_26")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_26 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_26 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_27(pub u32);
    impl HuffencAc0_27 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_27 {
        #[inline(always)]
        fn default() -> HuffencAc0_27 {
            HuffencAc0_27(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_27")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_27 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_27 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_28(pub u32);
    impl HuffencAc0_28 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_28 {
        #[inline(always)]
        fn default() -> HuffencAc0_28 {
            HuffencAc0_28(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_28")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_28 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_28 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_29(pub u32);
    impl HuffencAc0_29 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_29 {
        #[inline(always)]
        fn default() -> HuffencAc0_29 {
            HuffencAc0_29(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_29 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_29")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_29 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_29 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_3(pub u32);
    impl HuffencAc0_3 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_3 {
        #[inline(always)]
        fn default() -> HuffencAc0_3 {
            HuffencAc0_3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_3")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_3 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_30(pub u32);
    impl HuffencAc0_30 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_30 {
        #[inline(always)]
        fn default() -> HuffencAc0_30 {
            HuffencAc0_30(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_30")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_30 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_30 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_31(pub u32);
    impl HuffencAc0_31 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_31 {
        #[inline(always)]
        fn default() -> HuffencAc0_31 {
            HuffencAc0_31(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_31 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_31")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_31 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_31 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_32(pub u32);
    impl HuffencAc0_32 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_32 {
        #[inline(always)]
        fn default() -> HuffencAc0_32 {
            HuffencAc0_32(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_32")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_32 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_33(pub u32);
    impl HuffencAc0_33 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_33 {
        #[inline(always)]
        fn default() -> HuffencAc0_33 {
            HuffencAc0_33(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_33 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_33")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_33 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_33 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_34(pub u32);
    impl HuffencAc0_34 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_34 {
        #[inline(always)]
        fn default() -> HuffencAc0_34 {
            HuffencAc0_34(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_34")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_34 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_34 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_35(pub u32);
    impl HuffencAc0_35 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_35 {
        #[inline(always)]
        fn default() -> HuffencAc0_35 {
            HuffencAc0_35(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_35 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_35")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_35 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_35 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_36(pub u32);
    impl HuffencAc0_36 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_36 {
        #[inline(always)]
        fn default() -> HuffencAc0_36 {
            HuffencAc0_36(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_36 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_36")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_36 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_36 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_37(pub u32);
    impl HuffencAc0_37 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_37 {
        #[inline(always)]
        fn default() -> HuffencAc0_37 {
            HuffencAc0_37(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_37 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_37")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_37 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_37 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_38(pub u32);
    impl HuffencAc0_38 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_38 {
        #[inline(always)]
        fn default() -> HuffencAc0_38 {
            HuffencAc0_38(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_38 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_38")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_38 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_38 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_39(pub u32);
    impl HuffencAc0_39 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_39 {
        #[inline(always)]
        fn default() -> HuffencAc0_39 {
            HuffencAc0_39(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_39 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_39")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_39 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_39 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_4(pub u32);
    impl HuffencAc0_4 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_4 {
        #[inline(always)]
        fn default() -> HuffencAc0_4 {
            HuffencAc0_4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_4")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_4 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_40(pub u32);
    impl HuffencAc0_40 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_40 {
        #[inline(always)]
        fn default() -> HuffencAc0_40 {
            HuffencAc0_40(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_40 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_40")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_40 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_40 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_41(pub u32);
    impl HuffencAc0_41 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_41 {
        #[inline(always)]
        fn default() -> HuffencAc0_41 {
            HuffencAc0_41(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_41 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_41")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_41 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_41 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_42(pub u32);
    impl HuffencAc0_42 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_42 {
        #[inline(always)]
        fn default() -> HuffencAc0_42 {
            HuffencAc0_42(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_42 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_42")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_42 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_42 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_43(pub u32);
    impl HuffencAc0_43 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_43 {
        #[inline(always)]
        fn default() -> HuffencAc0_43 {
            HuffencAc0_43(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_43 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_43")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_43 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_43 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_44(pub u32);
    impl HuffencAc0_44 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_44 {
        #[inline(always)]
        fn default() -> HuffencAc0_44 {
            HuffencAc0_44(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_44 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_44")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_44 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_44 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_45(pub u32);
    impl HuffencAc0_45 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_45 {
        #[inline(always)]
        fn default() -> HuffencAc0_45 {
            HuffencAc0_45(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_45 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_45")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_45 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_45 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_46(pub u32);
    impl HuffencAc0_46 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_46 {
        #[inline(always)]
        fn default() -> HuffencAc0_46 {
            HuffencAc0_46(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_46 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_46")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_46 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_46 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_47(pub u32);
    impl HuffencAc0_47 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_47 {
        #[inline(always)]
        fn default() -> HuffencAc0_47 {
            HuffencAc0_47(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_47 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_47")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_47 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_47 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_48(pub u32);
    impl HuffencAc0_48 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_48 {
        #[inline(always)]
        fn default() -> HuffencAc0_48 {
            HuffencAc0_48(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_48 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_48")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_48 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_48 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_49(pub u32);
    impl HuffencAc0_49 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_49 {
        #[inline(always)]
        fn default() -> HuffencAc0_49 {
            HuffencAc0_49(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_49 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_49")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_49 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_49 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_5(pub u32);
    impl HuffencAc0_5 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_5 {
        #[inline(always)]
        fn default() -> HuffencAc0_5 {
            HuffencAc0_5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_5")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_5 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_50(pub u32);
    impl HuffencAc0_50 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_50 {
        #[inline(always)]
        fn default() -> HuffencAc0_50 {
            HuffencAc0_50(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_50 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_50")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_50 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_50 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_51(pub u32);
    impl HuffencAc0_51 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_51 {
        #[inline(always)]
        fn default() -> HuffencAc0_51 {
            HuffencAc0_51(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_51 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_51")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_51 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_51 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_52(pub u32);
    impl HuffencAc0_52 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_52 {
        #[inline(always)]
        fn default() -> HuffencAc0_52 {
            HuffencAc0_52(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_52 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_52")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_52 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_52 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_53(pub u32);
    impl HuffencAc0_53 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_53 {
        #[inline(always)]
        fn default() -> HuffencAc0_53 {
            HuffencAc0_53(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_53 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_53")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_53 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_53 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_54(pub u32);
    impl HuffencAc0_54 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_54 {
        #[inline(always)]
        fn default() -> HuffencAc0_54 {
            HuffencAc0_54(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_54 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_54")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_54 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_54 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_55(pub u32);
    impl HuffencAc0_55 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_55 {
        #[inline(always)]
        fn default() -> HuffencAc0_55 {
            HuffencAc0_55(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_55 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_55")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_55 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_55 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_56(pub u32);
    impl HuffencAc0_56 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_56 {
        #[inline(always)]
        fn default() -> HuffencAc0_56 {
            HuffencAc0_56(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_56")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_56 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_56 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_57(pub u32);
    impl HuffencAc0_57 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_57 {
        #[inline(always)]
        fn default() -> HuffencAc0_57 {
            HuffencAc0_57(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_57 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_57")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_57 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_57 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_58(pub u32);
    impl HuffencAc0_58 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_58 {
        #[inline(always)]
        fn default() -> HuffencAc0_58 {
            HuffencAc0_58(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_58 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_58")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_58 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_58 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_59(pub u32);
    impl HuffencAc0_59 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_59 {
        #[inline(always)]
        fn default() -> HuffencAc0_59 {
            HuffencAc0_59(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_59 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_59")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_59 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_59 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_6(pub u32);
    impl HuffencAc0_6 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_6 {
        #[inline(always)]
        fn default() -> HuffencAc0_6 {
            HuffencAc0_6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_6")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_6 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_60(pub u32);
    impl HuffencAc0_60 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_60 {
        #[inline(always)]
        fn default() -> HuffencAc0_60 {
            HuffencAc0_60(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_60 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_60")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_60 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_60 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_61(pub u32);
    impl HuffencAc0_61 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_61 {
        #[inline(always)]
        fn default() -> HuffencAc0_61 {
            HuffencAc0_61(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_61 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_61")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_61 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_61 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_62(pub u32);
    impl HuffencAc0_62 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_62 {
        #[inline(always)]
        fn default() -> HuffencAc0_62 {
            HuffencAc0_62(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_62 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_62")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_62 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_62 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_63(pub u32);
    impl HuffencAc0_63 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_63 {
        #[inline(always)]
        fn default() -> HuffencAc0_63 {
            HuffencAc0_63(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_63 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_63")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_63 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_63 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_64(pub u32);
    impl HuffencAc0_64 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_64 {
        #[inline(always)]
        fn default() -> HuffencAc0_64 {
            HuffencAc0_64(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_64 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_64")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_64 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_64 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_65(pub u32);
    impl HuffencAc0_65 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_65 {
        #[inline(always)]
        fn default() -> HuffencAc0_65 {
            HuffencAc0_65(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_65 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_65")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_65 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_65 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_66(pub u32);
    impl HuffencAc0_66 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_66 {
        #[inline(always)]
        fn default() -> HuffencAc0_66 {
            HuffencAc0_66(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_66 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_66")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_66 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_66 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_67(pub u32);
    impl HuffencAc0_67 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_67 {
        #[inline(always)]
        fn default() -> HuffencAc0_67 {
            HuffencAc0_67(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_67 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_67")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_67 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_67 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_68(pub u32);
    impl HuffencAc0_68 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_68 {
        #[inline(always)]
        fn default() -> HuffencAc0_68 {
            HuffencAc0_68(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_68 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_68")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_68 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_68 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_69(pub u32);
    impl HuffencAc0_69 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_69 {
        #[inline(always)]
        fn default() -> HuffencAc0_69 {
            HuffencAc0_69(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_69 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_69")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_69 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_69 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_7(pub u32);
    impl HuffencAc0_7 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_7 {
        #[inline(always)]
        fn default() -> HuffencAc0_7 {
            HuffencAc0_7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_7")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_7 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_70(pub u32);
    impl HuffencAc0_70 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_70 {
        #[inline(always)]
        fn default() -> HuffencAc0_70 {
            HuffencAc0_70(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_70 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_70")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_70 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_70 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_71(pub u32);
    impl HuffencAc0_71 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_71 {
        #[inline(always)]
        fn default() -> HuffencAc0_71 {
            HuffencAc0_71(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_71 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_71")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_71 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_71 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_72(pub u32);
    impl HuffencAc0_72 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_72 {
        #[inline(always)]
        fn default() -> HuffencAc0_72 {
            HuffencAc0_72(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_72 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_72")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_72 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_72 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_73(pub u32);
    impl HuffencAc0_73 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_73 {
        #[inline(always)]
        fn default() -> HuffencAc0_73 {
            HuffencAc0_73(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_73 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_73")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_73 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_73 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_74(pub u32);
    impl HuffencAc0_74 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_74 {
        #[inline(always)]
        fn default() -> HuffencAc0_74 {
            HuffencAc0_74(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_74 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_74")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_74 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_74 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_75(pub u32);
    impl HuffencAc0_75 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_75 {
        #[inline(always)]
        fn default() -> HuffencAc0_75 {
            HuffencAc0_75(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_75 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_75")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_75 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_75 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_76(pub u32);
    impl HuffencAc0_76 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_76 {
        #[inline(always)]
        fn default() -> HuffencAc0_76 {
            HuffencAc0_76(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_76 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_76")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_76 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_76 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_77(pub u32);
    impl HuffencAc0_77 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_77 {
        #[inline(always)]
        fn default() -> HuffencAc0_77 {
            HuffencAc0_77(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_77 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_77")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_77 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_77 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_78(pub u32);
    impl HuffencAc0_78 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_78 {
        #[inline(always)]
        fn default() -> HuffencAc0_78 {
            HuffencAc0_78(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_78")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_78 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_78 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_79(pub u32);
    impl HuffencAc0_79 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_79 {
        #[inline(always)]
        fn default() -> HuffencAc0_79 {
            HuffencAc0_79(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_79 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_79")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_79 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_79 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_8(pub u32);
    impl HuffencAc0_8 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_8 {
        #[inline(always)]
        fn default() -> HuffencAc0_8 {
            HuffencAc0_8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_8")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_8 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_80(pub u32);
    impl HuffencAc0_80 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_80 {
        #[inline(always)]
        fn default() -> HuffencAc0_80 {
            HuffencAc0_80(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_80 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_80")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_80 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_80 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_81(pub u32);
    impl HuffencAc0_81 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_81 {
        #[inline(always)]
        fn default() -> HuffencAc0_81 {
            HuffencAc0_81(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_81 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_81")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_81 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_81 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_82(pub u32);
    impl HuffencAc0_82 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_82 {
        #[inline(always)]
        fn default() -> HuffencAc0_82 {
            HuffencAc0_82(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_82 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_82")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_82 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_82 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_83(pub u32);
    impl HuffencAc0_83 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_83 {
        #[inline(always)]
        fn default() -> HuffencAc0_83 {
            HuffencAc0_83(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_83 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_83")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_83 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_83 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_84(pub u32);
    impl HuffencAc0_84 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_84 {
        #[inline(always)]
        fn default() -> HuffencAc0_84 {
            HuffencAc0_84(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_84 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_84")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_84 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_84 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_85(pub u32);
    impl HuffencAc0_85 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_85 {
        #[inline(always)]
        fn default() -> HuffencAc0_85 {
            HuffencAc0_85(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_85 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_85")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_85 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_85 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_86(pub u32);
    impl HuffencAc0_86 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_86 {
        #[inline(always)]
        fn default() -> HuffencAc0_86 {
            HuffencAc0_86(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_86 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_86")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_86 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_86 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_87(pub u32);
    impl HuffencAc0_87 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_87 {
        #[inline(always)]
        fn default() -> HuffencAc0_87 {
            HuffencAc0_87(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_87 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_87")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_87 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_87 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc0_9(pub u32);
    impl HuffencAc0_9 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc0_9 {
        #[inline(always)]
        fn default() -> HuffencAc0_9 {
            HuffencAc0_9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc0_9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc0_9")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc0_9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc0_9 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_0(pub u32);
    impl HuffencAc1_0 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_0 {
        #[inline(always)]
        fn default() -> HuffencAc1_0 {
            HuffencAc1_0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_0")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_0 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_1(pub u32);
    impl HuffencAc1_1 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_1 {
        #[inline(always)]
        fn default() -> HuffencAc1_1 {
            HuffencAc1_1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_1")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_1 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_10(pub u32);
    impl HuffencAc1_10 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_10 {
        #[inline(always)]
        fn default() -> HuffencAc1_10 {
            HuffencAc1_10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_10")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_10 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_11(pub u32);
    impl HuffencAc1_11 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_11 {
        #[inline(always)]
        fn default() -> HuffencAc1_11 {
            HuffencAc1_11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_11")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_11 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_12(pub u32);
    impl HuffencAc1_12 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_12 {
        #[inline(always)]
        fn default() -> HuffencAc1_12 {
            HuffencAc1_12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_12")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_12 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_13(pub u32);
    impl HuffencAc1_13 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_13 {
        #[inline(always)]
        fn default() -> HuffencAc1_13 {
            HuffencAc1_13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_13")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_13 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_14(pub u32);
    impl HuffencAc1_14 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_14 {
        #[inline(always)]
        fn default() -> HuffencAc1_14 {
            HuffencAc1_14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_14")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_14 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_15(pub u32);
    impl HuffencAc1_15 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_15 {
        #[inline(always)]
        fn default() -> HuffencAc1_15 {
            HuffencAc1_15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_15")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_15 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_16(pub u32);
    impl HuffencAc1_16 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_16 {
        #[inline(always)]
        fn default() -> HuffencAc1_16 {
            HuffencAc1_16(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_16")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_16 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_17(pub u32);
    impl HuffencAc1_17 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_17 {
        #[inline(always)]
        fn default() -> HuffencAc1_17 {
            HuffencAc1_17(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_17")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_17 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_17 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_18(pub u32);
    impl HuffencAc1_18 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_18 {
        #[inline(always)]
        fn default() -> HuffencAc1_18 {
            HuffencAc1_18(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_18")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_18 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_18 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_19(pub u32);
    impl HuffencAc1_19 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_19 {
        #[inline(always)]
        fn default() -> HuffencAc1_19 {
            HuffencAc1_19(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_19")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_19 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_19 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_2(pub u32);
    impl HuffencAc1_2 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_2 {
        #[inline(always)]
        fn default() -> HuffencAc1_2 {
            HuffencAc1_2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_2")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_2 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_20(pub u32);
    impl HuffencAc1_20 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_20 {
        #[inline(always)]
        fn default() -> HuffencAc1_20 {
            HuffencAc1_20(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_20 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_20")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_20 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_20 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_21(pub u32);
    impl HuffencAc1_21 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_21 {
        #[inline(always)]
        fn default() -> HuffencAc1_21 {
            HuffencAc1_21(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_21 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_21")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_21 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_21 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_22(pub u32);
    impl HuffencAc1_22 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_22 {
        #[inline(always)]
        fn default() -> HuffencAc1_22 {
            HuffencAc1_22(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_22 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_22")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_22 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_22 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_23(pub u32);
    impl HuffencAc1_23 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_23 {
        #[inline(always)]
        fn default() -> HuffencAc1_23 {
            HuffencAc1_23(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_23")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_23 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_23 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_24(pub u32);
    impl HuffencAc1_24 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_24 {
        #[inline(always)]
        fn default() -> HuffencAc1_24 {
            HuffencAc1_24(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_24 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_24")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_24 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_24 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_25(pub u32);
    impl HuffencAc1_25 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_25 {
        #[inline(always)]
        fn default() -> HuffencAc1_25 {
            HuffencAc1_25(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_25 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_25")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_25 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_25 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_26(pub u32);
    impl HuffencAc1_26 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_26 {
        #[inline(always)]
        fn default() -> HuffencAc1_26 {
            HuffencAc1_26(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_26 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_26")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_26 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_26 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_27(pub u32);
    impl HuffencAc1_27 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_27 {
        #[inline(always)]
        fn default() -> HuffencAc1_27 {
            HuffencAc1_27(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_27")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_27 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_27 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_28(pub u32);
    impl HuffencAc1_28 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_28 {
        #[inline(always)]
        fn default() -> HuffencAc1_28 {
            HuffencAc1_28(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_28")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_28 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_28 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_29(pub u32);
    impl HuffencAc1_29 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_29 {
        #[inline(always)]
        fn default() -> HuffencAc1_29 {
            HuffencAc1_29(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_29 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_29")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_29 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_29 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_3(pub u32);
    impl HuffencAc1_3 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_3 {
        #[inline(always)]
        fn default() -> HuffencAc1_3 {
            HuffencAc1_3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_3")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_3 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_30(pub u32);
    impl HuffencAc1_30 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_30 {
        #[inline(always)]
        fn default() -> HuffencAc1_30 {
            HuffencAc1_30(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_30")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_30 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_30 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_31(pub u32);
    impl HuffencAc1_31 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_31 {
        #[inline(always)]
        fn default() -> HuffencAc1_31 {
            HuffencAc1_31(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_31 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_31")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_31 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_31 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_32(pub u32);
    impl HuffencAc1_32 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_32 {
        #[inline(always)]
        fn default() -> HuffencAc1_32 {
            HuffencAc1_32(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_32")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_32 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_33(pub u32);
    impl HuffencAc1_33 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_33 {
        #[inline(always)]
        fn default() -> HuffencAc1_33 {
            HuffencAc1_33(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_33 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_33")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_33 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_33 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_34(pub u32);
    impl HuffencAc1_34 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_34 {
        #[inline(always)]
        fn default() -> HuffencAc1_34 {
            HuffencAc1_34(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_34")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_34 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_34 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_35(pub u32);
    impl HuffencAc1_35 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_35 {
        #[inline(always)]
        fn default() -> HuffencAc1_35 {
            HuffencAc1_35(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_35 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_35")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_35 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_35 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_36(pub u32);
    impl HuffencAc1_36 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_36 {
        #[inline(always)]
        fn default() -> HuffencAc1_36 {
            HuffencAc1_36(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_36 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_36")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_36 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_36 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_37(pub u32);
    impl HuffencAc1_37 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_37 {
        #[inline(always)]
        fn default() -> HuffencAc1_37 {
            HuffencAc1_37(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_37 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_37")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_37 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_37 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_38(pub u32);
    impl HuffencAc1_38 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_38 {
        #[inline(always)]
        fn default() -> HuffencAc1_38 {
            HuffencAc1_38(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_38 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_38")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_38 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_38 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_39(pub u32);
    impl HuffencAc1_39 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_39 {
        #[inline(always)]
        fn default() -> HuffencAc1_39 {
            HuffencAc1_39(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_39 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_39")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_39 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_39 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_4(pub u32);
    impl HuffencAc1_4 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_4 {
        #[inline(always)]
        fn default() -> HuffencAc1_4 {
            HuffencAc1_4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_4")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_4 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_40(pub u32);
    impl HuffencAc1_40 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_40 {
        #[inline(always)]
        fn default() -> HuffencAc1_40 {
            HuffencAc1_40(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_40 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_40")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_40 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_40 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_41(pub u32);
    impl HuffencAc1_41 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_41 {
        #[inline(always)]
        fn default() -> HuffencAc1_41 {
            HuffencAc1_41(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_41 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_41")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_41 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_41 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_42(pub u32);
    impl HuffencAc1_42 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_42 {
        #[inline(always)]
        fn default() -> HuffencAc1_42 {
            HuffencAc1_42(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_42 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_42")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_42 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_42 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_43(pub u32);
    impl HuffencAc1_43 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_43 {
        #[inline(always)]
        fn default() -> HuffencAc1_43 {
            HuffencAc1_43(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_43 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_43")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_43 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_43 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_44(pub u32);
    impl HuffencAc1_44 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_44 {
        #[inline(always)]
        fn default() -> HuffencAc1_44 {
            HuffencAc1_44(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_44 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_44")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_44 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_44 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_45(pub u32);
    impl HuffencAc1_45 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_45 {
        #[inline(always)]
        fn default() -> HuffencAc1_45 {
            HuffencAc1_45(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_45 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_45")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_45 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_45 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_46(pub u32);
    impl HuffencAc1_46 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_46 {
        #[inline(always)]
        fn default() -> HuffencAc1_46 {
            HuffencAc1_46(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_46 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_46")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_46 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_46 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_47(pub u32);
    impl HuffencAc1_47 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_47 {
        #[inline(always)]
        fn default() -> HuffencAc1_47 {
            HuffencAc1_47(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_47 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_47")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_47 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_47 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_48(pub u32);
    impl HuffencAc1_48 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_48 {
        #[inline(always)]
        fn default() -> HuffencAc1_48 {
            HuffencAc1_48(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_48 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_48")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_48 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_48 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_49(pub u32);
    impl HuffencAc1_49 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_49 {
        #[inline(always)]
        fn default() -> HuffencAc1_49 {
            HuffencAc1_49(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_49 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_49")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_49 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_49 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_5(pub u32);
    impl HuffencAc1_5 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_5 {
        #[inline(always)]
        fn default() -> HuffencAc1_5 {
            HuffencAc1_5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_5")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_5 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_50(pub u32);
    impl HuffencAc1_50 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_50 {
        #[inline(always)]
        fn default() -> HuffencAc1_50 {
            HuffencAc1_50(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_50 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_50")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_50 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_50 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_51(pub u32);
    impl HuffencAc1_51 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_51 {
        #[inline(always)]
        fn default() -> HuffencAc1_51 {
            HuffencAc1_51(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_51 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_51")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_51 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_51 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_52(pub u32);
    impl HuffencAc1_52 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_52 {
        #[inline(always)]
        fn default() -> HuffencAc1_52 {
            HuffencAc1_52(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_52 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_52")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_52 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_52 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_53(pub u32);
    impl HuffencAc1_53 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_53 {
        #[inline(always)]
        fn default() -> HuffencAc1_53 {
            HuffencAc1_53(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_53 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_53")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_53 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_53 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_54(pub u32);
    impl HuffencAc1_54 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_54 {
        #[inline(always)]
        fn default() -> HuffencAc1_54 {
            HuffencAc1_54(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_54 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_54")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_54 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_54 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_55(pub u32);
    impl HuffencAc1_55 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_55 {
        #[inline(always)]
        fn default() -> HuffencAc1_55 {
            HuffencAc1_55(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_55 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_55")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_55 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_55 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_56(pub u32);
    impl HuffencAc1_56 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_56 {
        #[inline(always)]
        fn default() -> HuffencAc1_56 {
            HuffencAc1_56(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_56")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_56 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_56 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_57(pub u32);
    impl HuffencAc1_57 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_57 {
        #[inline(always)]
        fn default() -> HuffencAc1_57 {
            HuffencAc1_57(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_57 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_57")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_57 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_57 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_58(pub u32);
    impl HuffencAc1_58 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_58 {
        #[inline(always)]
        fn default() -> HuffencAc1_58 {
            HuffencAc1_58(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_58 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_58")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_58 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_58 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_59(pub u32);
    impl HuffencAc1_59 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_59 {
        #[inline(always)]
        fn default() -> HuffencAc1_59 {
            HuffencAc1_59(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_59 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_59")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_59 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_59 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_6(pub u32);
    impl HuffencAc1_6 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_6 {
        #[inline(always)]
        fn default() -> HuffencAc1_6 {
            HuffencAc1_6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_6")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_6 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_60(pub u32);
    impl HuffencAc1_60 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_60 {
        #[inline(always)]
        fn default() -> HuffencAc1_60 {
            HuffencAc1_60(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_60 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_60")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_60 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_60 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_61(pub u32);
    impl HuffencAc1_61 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_61 {
        #[inline(always)]
        fn default() -> HuffencAc1_61 {
            HuffencAc1_61(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_61 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_61")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_61 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_61 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_62(pub u32);
    impl HuffencAc1_62 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_62 {
        #[inline(always)]
        fn default() -> HuffencAc1_62 {
            HuffencAc1_62(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_62 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_62")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_62 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_62 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_63(pub u32);
    impl HuffencAc1_63 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_63 {
        #[inline(always)]
        fn default() -> HuffencAc1_63 {
            HuffencAc1_63(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_63 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_63")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_63 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_63 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_64(pub u32);
    impl HuffencAc1_64 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_64 {
        #[inline(always)]
        fn default() -> HuffencAc1_64 {
            HuffencAc1_64(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_64 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_64")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_64 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_64 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_65(pub u32);
    impl HuffencAc1_65 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_65 {
        #[inline(always)]
        fn default() -> HuffencAc1_65 {
            HuffencAc1_65(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_65 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_65")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_65 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_65 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_66(pub u32);
    impl HuffencAc1_66 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_66 {
        #[inline(always)]
        fn default() -> HuffencAc1_66 {
            HuffencAc1_66(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_66 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_66")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_66 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_66 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_67(pub u32);
    impl HuffencAc1_67 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_67 {
        #[inline(always)]
        fn default() -> HuffencAc1_67 {
            HuffencAc1_67(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_67 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_67")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_67 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_67 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_68(pub u32);
    impl HuffencAc1_68 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_68 {
        #[inline(always)]
        fn default() -> HuffencAc1_68 {
            HuffencAc1_68(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_68 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_68")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_68 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_68 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_69(pub u32);
    impl HuffencAc1_69 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_69 {
        #[inline(always)]
        fn default() -> HuffencAc1_69 {
            HuffencAc1_69(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_69 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_69")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_69 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_69 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_7(pub u32);
    impl HuffencAc1_7 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_7 {
        #[inline(always)]
        fn default() -> HuffencAc1_7 {
            HuffencAc1_7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_7")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_7 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_70(pub u32);
    impl HuffencAc1_70 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_70 {
        #[inline(always)]
        fn default() -> HuffencAc1_70 {
            HuffencAc1_70(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_70 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_70")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_70 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_70 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_71(pub u32);
    impl HuffencAc1_71 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_71 {
        #[inline(always)]
        fn default() -> HuffencAc1_71 {
            HuffencAc1_71(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_71 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_71")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_71 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_71 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_72(pub u32);
    impl HuffencAc1_72 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_72 {
        #[inline(always)]
        fn default() -> HuffencAc1_72 {
            HuffencAc1_72(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_72 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_72")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_72 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_72 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_73(pub u32);
    impl HuffencAc1_73 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_73 {
        #[inline(always)]
        fn default() -> HuffencAc1_73 {
            HuffencAc1_73(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_73 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_73")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_73 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_73 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_74(pub u32);
    impl HuffencAc1_74 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_74 {
        #[inline(always)]
        fn default() -> HuffencAc1_74 {
            HuffencAc1_74(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_74 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_74")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_74 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_74 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_75(pub u32);
    impl HuffencAc1_75 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_75 {
        #[inline(always)]
        fn default() -> HuffencAc1_75 {
            HuffencAc1_75(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_75 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_75")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_75 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_75 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_76(pub u32);
    impl HuffencAc1_76 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_76 {
        #[inline(always)]
        fn default() -> HuffencAc1_76 {
            HuffencAc1_76(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_76 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_76")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_76 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_76 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_77(pub u32);
    impl HuffencAc1_77 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_77 {
        #[inline(always)]
        fn default() -> HuffencAc1_77 {
            HuffencAc1_77(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_77 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_77")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_77 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_77 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_78(pub u32);
    impl HuffencAc1_78 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_78 {
        #[inline(always)]
        fn default() -> HuffencAc1_78 {
            HuffencAc1_78(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_78")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_78 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_78 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_79(pub u32);
    impl HuffencAc1_79 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_79 {
        #[inline(always)]
        fn default() -> HuffencAc1_79 {
            HuffencAc1_79(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_79 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_79")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_79 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_79 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_8(pub u32);
    impl HuffencAc1_8 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_8 {
        #[inline(always)]
        fn default() -> HuffencAc1_8 {
            HuffencAc1_8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_8")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_8 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_80(pub u32);
    impl HuffencAc1_80 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_80 {
        #[inline(always)]
        fn default() -> HuffencAc1_80 {
            HuffencAc1_80(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_80 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_80")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_80 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_80 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_81(pub u32);
    impl HuffencAc1_81 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_81 {
        #[inline(always)]
        fn default() -> HuffencAc1_81 {
            HuffencAc1_81(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_81 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_81")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_81 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_81 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_82(pub u32);
    impl HuffencAc1_82 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_82 {
        #[inline(always)]
        fn default() -> HuffencAc1_82 {
            HuffencAc1_82(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_82 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_82")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_82 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_82 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_83(pub u32);
    impl HuffencAc1_83 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_83 {
        #[inline(always)]
        fn default() -> HuffencAc1_83 {
            HuffencAc1_83(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_83 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_83")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_83 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_83 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_84(pub u32);
    impl HuffencAc1_84 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_84 {
        #[inline(always)]
        fn default() -> HuffencAc1_84 {
            HuffencAc1_84(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_84 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_84")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_84 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_84 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_85(pub u32);
    impl HuffencAc1_85 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_85 {
        #[inline(always)]
        fn default() -> HuffencAc1_85 {
            HuffencAc1_85(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_85 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_85")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_85 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_85 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_86(pub u32);
    impl HuffencAc1_86 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_86 {
        #[inline(always)]
        fn default() -> HuffencAc1_86 {
            HuffencAc1_86(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_86 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_86")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_86 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_86 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_87(pub u32);
    impl HuffencAc1_87 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_87 {
        #[inline(always)]
        fn default() -> HuffencAc1_87 {
            HuffencAc1_87(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_87 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_87")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_87 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_87 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc1_9(pub u32);
    impl HuffencAc1_9 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc1_9 {
        #[inline(always)]
        fn default() -> HuffencAc1_9 {
            HuffencAc1_9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencAc1_9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencAc1_9")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencAc1_9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencAc1_9 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc0_0(pub u32);
    impl HuffencDc0_0 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc0_0 {
        #[inline(always)]
        fn default() -> HuffencDc0_0 {
            HuffencDc0_0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc0_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc0_0")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc0_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc0_0 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc0_1(pub u32);
    impl HuffencDc0_1 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc0_1 {
        #[inline(always)]
        fn default() -> HuffencDc0_1 {
            HuffencDc0_1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc0_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc0_1")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc0_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc0_1 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc0_2(pub u32);
    impl HuffencDc0_2 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc0_2 {
        #[inline(always)]
        fn default() -> HuffencDc0_2 {
            HuffencDc0_2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc0_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc0_2")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc0_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc0_2 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc0_3(pub u32);
    impl HuffencDc0_3 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc0_3 {
        #[inline(always)]
        fn default() -> HuffencDc0_3 {
            HuffencDc0_3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc0_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc0_3")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc0_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc0_3 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc0_4(pub u32);
    impl HuffencDc0_4 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc0_4 {
        #[inline(always)]
        fn default() -> HuffencDc0_4 {
            HuffencDc0_4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc0_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc0_4")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc0_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc0_4 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc0_5(pub u32);
    impl HuffencDc0_5 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc0_5 {
        #[inline(always)]
        fn default() -> HuffencDc0_5 {
            HuffencDc0_5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc0_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc0_5")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc0_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc0_5 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc0_6(pub u32);
    impl HuffencDc0_6 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc0_6 {
        #[inline(always)]
        fn default() -> HuffencDc0_6 {
            HuffencDc0_6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc0_6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc0_6")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc0_6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc0_6 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc0_7(pub u32);
    impl HuffencDc0_7 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc0_7 {
        #[inline(always)]
        fn default() -> HuffencDc0_7 {
            HuffencDc0_7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc0_7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc0_7")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc0_7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc0_7 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc1_0(pub u32);
    impl HuffencDc1_0 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc1_0 {
        #[inline(always)]
        fn default() -> HuffencDc1_0 {
            HuffencDc1_0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc1_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc1_0")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc1_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc1_0 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc1_1(pub u32);
    impl HuffencDc1_1 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc1_1 {
        #[inline(always)]
        fn default() -> HuffencDc1_1 {
            HuffencDc1_1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc1_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc1_1")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc1_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc1_1 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc1_2(pub u32);
    impl HuffencDc1_2 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc1_2 {
        #[inline(always)]
        fn default() -> HuffencDc1_2 {
            HuffencDc1_2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc1_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc1_2")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc1_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc1_2 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc1_3(pub u32);
    impl HuffencDc1_3 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc1_3 {
        #[inline(always)]
        fn default() -> HuffencDc1_3 {
            HuffencDc1_3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc1_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc1_3")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc1_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc1_3 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc1_4(pub u32);
    impl HuffencDc1_4 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc1_4 {
        #[inline(always)]
        fn default() -> HuffencDc1_4 {
            HuffencDc1_4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc1_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc1_4")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc1_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc1_4 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc1_5(pub u32);
    impl HuffencDc1_5 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc1_5 {
        #[inline(always)]
        fn default() -> HuffencDc1_5 {
            HuffencDc1_5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc1_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc1_5")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc1_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc1_5 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc1_6(pub u32);
    impl HuffencDc1_6 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc1_6 {
        #[inline(always)]
        fn default() -> HuffencDc1_6 {
            HuffencDc1_6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc1_6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc1_6")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc1_6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc1_6 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc1_7(pub u32);
    impl HuffencDc1_7 {
        #[doc = "DHTMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn dht_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn set_dht_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc1_7 {
        #[inline(always)]
        fn default() -> HuffencDc1_7 {
            HuffencDc1_7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for HuffencDc1_7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("HuffencDc1_7")
                .field("dht_mem_ram", &self.dht_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for HuffencDc1_7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "HuffencDc1_7 {{ dht_mem_ram: {=u32:?} }}", self.dht_mem_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin0(pub u32);
    impl Huffmin0 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin0 {
        #[inline(always)]
        fn default() -> Huffmin0 {
            Huffmin0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin0")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin0 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin1(pub u32);
    impl Huffmin1 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin1 {
        #[inline(always)]
        fn default() -> Huffmin1 {
            Huffmin1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin1")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin1 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin10(pub u32);
    impl Huffmin10 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin10 {
        #[inline(always)]
        fn default() -> Huffmin10 {
            Huffmin10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin10")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin10 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin11(pub u32);
    impl Huffmin11 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin11 {
        #[inline(always)]
        fn default() -> Huffmin11 {
            Huffmin11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin11")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin11 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin12(pub u32);
    impl Huffmin12 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin12 {
        #[inline(always)]
        fn default() -> Huffmin12 {
            Huffmin12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin12")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin12 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin13(pub u32);
    impl Huffmin13 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin13 {
        #[inline(always)]
        fn default() -> Huffmin13 {
            Huffmin13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin13")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin13 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin14(pub u32);
    impl Huffmin14 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin14 {
        #[inline(always)]
        fn default() -> Huffmin14 {
            Huffmin14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin14")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin14 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin15(pub u32);
    impl Huffmin15 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin15 {
        #[inline(always)]
        fn default() -> Huffmin15 {
            Huffmin15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin15")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin15 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin2(pub u32);
    impl Huffmin2 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin2 {
        #[inline(always)]
        fn default() -> Huffmin2 {
            Huffmin2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin2")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin2 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin3(pub u32);
    impl Huffmin3 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin3 {
        #[inline(always)]
        fn default() -> Huffmin3 {
            Huffmin3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin3")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin3 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin4(pub u32);
    impl Huffmin4 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin4 {
        #[inline(always)]
        fn default() -> Huffmin4 {
            Huffmin4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin4")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin4 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin5(pub u32);
    impl Huffmin5 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin5 {
        #[inline(always)]
        fn default() -> Huffmin5 {
            Huffmin5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin5")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin5 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin6(pub u32);
    impl Huffmin6 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin6 {
        #[inline(always)]
        fn default() -> Huffmin6 {
            Huffmin6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin6")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin6 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin7(pub u32);
    impl Huffmin7 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin7 {
        #[inline(always)]
        fn default() -> Huffmin7 {
            Huffmin7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin7")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin7 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin8(pub u32);
    impl Huffmin8 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin8 {
        #[inline(always)]
        fn default() -> Huffmin8 {
            Huffmin8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin8")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin8 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin9(pub u32);
    impl Huffmin9 {
        #[doc = "HuffMin RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin9 {
        #[inline(always)]
        fn default() -> Huffmin9 {
            Huffmin9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffmin9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffmin9")
                .field("huff_min_ram", &self.huff_min_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffmin9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffmin9 {{ huff_min_ram: {=u32:?} }}", self.huff_min_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb0(pub u32);
    impl Huffsymb0 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb0 {
        #[inline(always)]
        fn default() -> Huffsymb0 {
            Huffsymb0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb0")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb0 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb1(pub u32);
    impl Huffsymb1 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb1 {
        #[inline(always)]
        fn default() -> Huffsymb1 {
            Huffsymb1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb1")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb1 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb10(pub u32);
    impl Huffsymb10 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb10 {
        #[inline(always)]
        fn default() -> Huffsymb10 {
            Huffsymb10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb10")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb10 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb11(pub u32);
    impl Huffsymb11 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb11 {
        #[inline(always)]
        fn default() -> Huffsymb11 {
            Huffsymb11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb11")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb11 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb12(pub u32);
    impl Huffsymb12 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb12 {
        #[inline(always)]
        fn default() -> Huffsymb12 {
            Huffsymb12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb12")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb12 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb13(pub u32);
    impl Huffsymb13 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb13 {
        #[inline(always)]
        fn default() -> Huffsymb13 {
            Huffsymb13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb13")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb13 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb14(pub u32);
    impl Huffsymb14 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb14 {
        #[inline(always)]
        fn default() -> Huffsymb14 {
            Huffsymb14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb14")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb14 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb15(pub u32);
    impl Huffsymb15 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb15 {
        #[inline(always)]
        fn default() -> Huffsymb15 {
            Huffsymb15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb15")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb15 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb16(pub u32);
    impl Huffsymb16 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb16 {
        #[inline(always)]
        fn default() -> Huffsymb16 {
            Huffsymb16(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb16 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb16")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb16 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb16 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb17(pub u32);
    impl Huffsymb17 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb17 {
        #[inline(always)]
        fn default() -> Huffsymb17 {
            Huffsymb17(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb17 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb17")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb17 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb17 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb18(pub u32);
    impl Huffsymb18 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb18 {
        #[inline(always)]
        fn default() -> Huffsymb18 {
            Huffsymb18(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb18 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb18")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb18 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb18 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb19(pub u32);
    impl Huffsymb19 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb19 {
        #[inline(always)]
        fn default() -> Huffsymb19 {
            Huffsymb19(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb19 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb19")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb19 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb19 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb2(pub u32);
    impl Huffsymb2 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb2 {
        #[inline(always)]
        fn default() -> Huffsymb2 {
            Huffsymb2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb2")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb2 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb20(pub u32);
    impl Huffsymb20 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb20 {
        #[inline(always)]
        fn default() -> Huffsymb20 {
            Huffsymb20(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb20 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb20")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb20 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb20 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb21(pub u32);
    impl Huffsymb21 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb21 {
        #[inline(always)]
        fn default() -> Huffsymb21 {
            Huffsymb21(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb21 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb21")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb21 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb21 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb22(pub u32);
    impl Huffsymb22 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb22 {
        #[inline(always)]
        fn default() -> Huffsymb22 {
            Huffsymb22(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb22 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb22")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb22 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb22 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb23(pub u32);
    impl Huffsymb23 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb23 {
        #[inline(always)]
        fn default() -> Huffsymb23 {
            Huffsymb23(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb23 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb23")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb23 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb23 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb24(pub u32);
    impl Huffsymb24 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb24 {
        #[inline(always)]
        fn default() -> Huffsymb24 {
            Huffsymb24(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb24 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb24")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb24 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb24 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb25(pub u32);
    impl Huffsymb25 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb25 {
        #[inline(always)]
        fn default() -> Huffsymb25 {
            Huffsymb25(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb25 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb25")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb25 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb25 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb26(pub u32);
    impl Huffsymb26 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb26 {
        #[inline(always)]
        fn default() -> Huffsymb26 {
            Huffsymb26(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb26 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb26")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb26 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb26 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb27(pub u32);
    impl Huffsymb27 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb27 {
        #[inline(always)]
        fn default() -> Huffsymb27 {
            Huffsymb27(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb27 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb27")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb27 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb27 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb28(pub u32);
    impl Huffsymb28 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb28 {
        #[inline(always)]
        fn default() -> Huffsymb28 {
            Huffsymb28(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb28 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb28")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb28 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb28 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb29(pub u32);
    impl Huffsymb29 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb29 {
        #[inline(always)]
        fn default() -> Huffsymb29 {
            Huffsymb29(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb29 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb29")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb29 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb29 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb3(pub u32);
    impl Huffsymb3 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb3 {
        #[inline(always)]
        fn default() -> Huffsymb3 {
            Huffsymb3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb3")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb3 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb30(pub u32);
    impl Huffsymb30 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb30 {
        #[inline(always)]
        fn default() -> Huffsymb30 {
            Huffsymb30(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb30 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb30")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb30 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb30 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb31(pub u32);
    impl Huffsymb31 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb31 {
        #[inline(always)]
        fn default() -> Huffsymb31 {
            Huffsymb31(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb31 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb31")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb31 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb31 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb32(pub u32);
    impl Huffsymb32 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb32 {
        #[inline(always)]
        fn default() -> Huffsymb32 {
            Huffsymb32(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb32 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb32")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb32 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb32 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb33(pub u32);
    impl Huffsymb33 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb33 {
        #[inline(always)]
        fn default() -> Huffsymb33 {
            Huffsymb33(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb33 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb33")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb33 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb33 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb34(pub u32);
    impl Huffsymb34 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb34 {
        #[inline(always)]
        fn default() -> Huffsymb34 {
            Huffsymb34(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb34 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb34")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb34 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb34 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb35(pub u32);
    impl Huffsymb35 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb35 {
        #[inline(always)]
        fn default() -> Huffsymb35 {
            Huffsymb35(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb35 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb35")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb35 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb35 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb36(pub u32);
    impl Huffsymb36 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb36 {
        #[inline(always)]
        fn default() -> Huffsymb36 {
            Huffsymb36(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb36 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb36")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb36 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb36 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb37(pub u32);
    impl Huffsymb37 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb37 {
        #[inline(always)]
        fn default() -> Huffsymb37 {
            Huffsymb37(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb37 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb37")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb37 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb37 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb38(pub u32);
    impl Huffsymb38 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb38 {
        #[inline(always)]
        fn default() -> Huffsymb38 {
            Huffsymb38(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb38 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb38")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb38 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb38 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb39(pub u32);
    impl Huffsymb39 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb39 {
        #[inline(always)]
        fn default() -> Huffsymb39 {
            Huffsymb39(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb39 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb39")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb39 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb39 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb4(pub u32);
    impl Huffsymb4 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb4 {
        #[inline(always)]
        fn default() -> Huffsymb4 {
            Huffsymb4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb4")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb4 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb40(pub u32);
    impl Huffsymb40 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb40 {
        #[inline(always)]
        fn default() -> Huffsymb40 {
            Huffsymb40(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb40 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb40")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb40 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb40 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb41(pub u32);
    impl Huffsymb41 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb41 {
        #[inline(always)]
        fn default() -> Huffsymb41 {
            Huffsymb41(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb41 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb41")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb41 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb41 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb42(pub u32);
    impl Huffsymb42 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb42 {
        #[inline(always)]
        fn default() -> Huffsymb42 {
            Huffsymb42(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb42 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb42")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb42 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb42 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb43(pub u32);
    impl Huffsymb43 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb43 {
        #[inline(always)]
        fn default() -> Huffsymb43 {
            Huffsymb43(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb43 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb43")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb43 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb43 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb44(pub u32);
    impl Huffsymb44 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb44 {
        #[inline(always)]
        fn default() -> Huffsymb44 {
            Huffsymb44(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb44 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb44")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb44 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb44 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb45(pub u32);
    impl Huffsymb45 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb45 {
        #[inline(always)]
        fn default() -> Huffsymb45 {
            Huffsymb45(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb45 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb45")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb45 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb45 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb46(pub u32);
    impl Huffsymb46 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb46 {
        #[inline(always)]
        fn default() -> Huffsymb46 {
            Huffsymb46(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb46 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb46")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb46 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb46 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb47(pub u32);
    impl Huffsymb47 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb47 {
        #[inline(always)]
        fn default() -> Huffsymb47 {
            Huffsymb47(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb47 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb47")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb47 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb47 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb48(pub u32);
    impl Huffsymb48 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb48 {
        #[inline(always)]
        fn default() -> Huffsymb48 {
            Huffsymb48(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb48 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb48")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb48 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb48 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb49(pub u32);
    impl Huffsymb49 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb49 {
        #[inline(always)]
        fn default() -> Huffsymb49 {
            Huffsymb49(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb49 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb49")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb49 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb49 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb5(pub u32);
    impl Huffsymb5 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb5 {
        #[inline(always)]
        fn default() -> Huffsymb5 {
            Huffsymb5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb5")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb5 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb50(pub u32);
    impl Huffsymb50 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb50 {
        #[inline(always)]
        fn default() -> Huffsymb50 {
            Huffsymb50(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb50 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb50")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb50 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb50 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb51(pub u32);
    impl Huffsymb51 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb51 {
        #[inline(always)]
        fn default() -> Huffsymb51 {
            Huffsymb51(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb51 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb51")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb51 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb51 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb52(pub u32);
    impl Huffsymb52 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb52 {
        #[inline(always)]
        fn default() -> Huffsymb52 {
            Huffsymb52(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb52 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb52")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb52 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb52 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb53(pub u32);
    impl Huffsymb53 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb53 {
        #[inline(always)]
        fn default() -> Huffsymb53 {
            Huffsymb53(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb53 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb53")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb53 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb53 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb54(pub u32);
    impl Huffsymb54 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb54 {
        #[inline(always)]
        fn default() -> Huffsymb54 {
            Huffsymb54(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb54 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb54")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb54 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb54 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb55(pub u32);
    impl Huffsymb55 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb55 {
        #[inline(always)]
        fn default() -> Huffsymb55 {
            Huffsymb55(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb55 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb55")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb55 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb55 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb56(pub u32);
    impl Huffsymb56 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb56 {
        #[inline(always)]
        fn default() -> Huffsymb56 {
            Huffsymb56(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb56 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb56")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb56 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb56 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb57(pub u32);
    impl Huffsymb57 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb57 {
        #[inline(always)]
        fn default() -> Huffsymb57 {
            Huffsymb57(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb57 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb57")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb57 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb57 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb58(pub u32);
    impl Huffsymb58 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb58 {
        #[inline(always)]
        fn default() -> Huffsymb58 {
            Huffsymb58(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb58 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb58")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb58 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb58 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb59(pub u32);
    impl Huffsymb59 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb59 {
        #[inline(always)]
        fn default() -> Huffsymb59 {
            Huffsymb59(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb59 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb59")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb59 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb59 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb6(pub u32);
    impl Huffsymb6 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb6 {
        #[inline(always)]
        fn default() -> Huffsymb6 {
            Huffsymb6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb6")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb6 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb60(pub u32);
    impl Huffsymb60 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb60 {
        #[inline(always)]
        fn default() -> Huffsymb60 {
            Huffsymb60(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb60 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb60")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb60 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb60 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb61(pub u32);
    impl Huffsymb61 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb61 {
        #[inline(always)]
        fn default() -> Huffsymb61 {
            Huffsymb61(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb61 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb61")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb61 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb61 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb62(pub u32);
    impl Huffsymb62 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb62 {
        #[inline(always)]
        fn default() -> Huffsymb62 {
            Huffsymb62(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb62 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb62")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb62 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb62 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb63(pub u32);
    impl Huffsymb63 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb63 {
        #[inline(always)]
        fn default() -> Huffsymb63 {
            Huffsymb63(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb63 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb63")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb63 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb63 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb64(pub u32);
    impl Huffsymb64 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb64 {
        #[inline(always)]
        fn default() -> Huffsymb64 {
            Huffsymb64(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb64 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb64")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb64 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb64 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb65(pub u32);
    impl Huffsymb65 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb65 {
        #[inline(always)]
        fn default() -> Huffsymb65 {
            Huffsymb65(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb65 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb65")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb65 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb65 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb66(pub u32);
    impl Huffsymb66 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb66 {
        #[inline(always)]
        fn default() -> Huffsymb66 {
            Huffsymb66(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb66 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb66")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb66 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb66 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb67(pub u32);
    impl Huffsymb67 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb67 {
        #[inline(always)]
        fn default() -> Huffsymb67 {
            Huffsymb67(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb67 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb67")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb67 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb67 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb68(pub u32);
    impl Huffsymb68 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb68 {
        #[inline(always)]
        fn default() -> Huffsymb68 {
            Huffsymb68(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb68 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb68")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb68 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb68 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb69(pub u32);
    impl Huffsymb69 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb69 {
        #[inline(always)]
        fn default() -> Huffsymb69 {
            Huffsymb69(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb69 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb69")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb69 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb69 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb7(pub u32);
    impl Huffsymb7 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb7 {
        #[inline(always)]
        fn default() -> Huffsymb7 {
            Huffsymb7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb7")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb7 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb70(pub u32);
    impl Huffsymb70 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb70 {
        #[inline(always)]
        fn default() -> Huffsymb70 {
            Huffsymb70(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb70 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb70")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb70 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb70 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb71(pub u32);
    impl Huffsymb71 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb71 {
        #[inline(always)]
        fn default() -> Huffsymb71 {
            Huffsymb71(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb71 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb71")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb71 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb71 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb72(pub u32);
    impl Huffsymb72 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb72 {
        #[inline(always)]
        fn default() -> Huffsymb72 {
            Huffsymb72(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb72 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb72")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb72 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb72 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb73(pub u32);
    impl Huffsymb73 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb73 {
        #[inline(always)]
        fn default() -> Huffsymb73 {
            Huffsymb73(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb73 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb73")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb73 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb73 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb74(pub u32);
    impl Huffsymb74 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb74 {
        #[inline(always)]
        fn default() -> Huffsymb74 {
            Huffsymb74(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb74 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb74")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb74 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb74 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb75(pub u32);
    impl Huffsymb75 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb75 {
        #[inline(always)]
        fn default() -> Huffsymb75 {
            Huffsymb75(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb75 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb75")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb75 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb75 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb76(pub u32);
    impl Huffsymb76 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb76 {
        #[inline(always)]
        fn default() -> Huffsymb76 {
            Huffsymb76(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb76 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb76")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb76 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb76 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb77(pub u32);
    impl Huffsymb77 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb77 {
        #[inline(always)]
        fn default() -> Huffsymb77 {
            Huffsymb77(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb77 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb77")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb77 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb77 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb78(pub u32);
    impl Huffsymb78 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb78 {
        #[inline(always)]
        fn default() -> Huffsymb78 {
            Huffsymb78(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb78 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb78")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb78 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb78 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb79(pub u32);
    impl Huffsymb79 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb79 {
        #[inline(always)]
        fn default() -> Huffsymb79 {
            Huffsymb79(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb79 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb79")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb79 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb79 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb8(pub u32);
    impl Huffsymb8 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb8 {
        #[inline(always)]
        fn default() -> Huffsymb8 {
            Huffsymb8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb8")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb8 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb80(pub u32);
    impl Huffsymb80 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb80 {
        #[inline(always)]
        fn default() -> Huffsymb80 {
            Huffsymb80(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb80 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb80")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb80 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb80 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb81(pub u32);
    impl Huffsymb81 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb81 {
        #[inline(always)]
        fn default() -> Huffsymb81 {
            Huffsymb81(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb81 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb81")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb81 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb81 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb82(pub u32);
    impl Huffsymb82 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb82 {
        #[inline(always)]
        fn default() -> Huffsymb82 {
            Huffsymb82(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb82 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb82")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb82 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb82 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb83(pub u32);
    impl Huffsymb83 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb83 {
        #[inline(always)]
        fn default() -> Huffsymb83 {
            Huffsymb83(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb83 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb83")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb83 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb83 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb9(pub u32);
    impl Huffsymb9 {
        #[doc = "DHTSymb RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb9 {
        #[inline(always)]
        fn default() -> Huffsymb9 {
            Huffsymb9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Huffsymb9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Huffsymb9")
                .field("huff_symb_ram", &self.huff_symb_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Huffsymb9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Huffsymb9 {{ huff_symb_ram: {=u32:?} }}", self.huff_symb_ram())
        }
    }
    #[doc = "JPEG clear flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegCfr(pub u32);
    impl JpegCfr {
        #[doc = "Clear End of Conversion Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ceocf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clear End of Conversion Flag"]
        #[inline(always)]
        pub const fn set_ceocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clear Header Parsing Done Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn chpdf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Header Parsing Done Flag"]
        #[inline(always)]
        pub const fn set_chpdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for JpegCfr {
        #[inline(always)]
        fn default() -> JpegCfr {
            JpegCfr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegCfr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegCfr")
                .field("ceocf", &self.ceocf())
                .field("chpdf", &self.chpdf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegCfr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegCfr {{ ceocf: {=bool:?}, chpdf: {=bool:?} }}",
                self.ceocf(),
                self.chpdf()
            )
        }
    }
    #[doc = "JPEG codec configuration register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr0(pub u32);
    impl JpegConfr0 {
        #[doc = "Start"]
        #[must_use]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start"]
        #[inline(always)]
        pub const fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for JpegConfr0 {
        #[inline(always)]
        fn default() -> JpegConfr0 {
            JpegConfr0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegConfr0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr0").field("start", &self.start()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegConfr0 {{ start: {=bool:?} }}", self.start())
        }
    }
    #[doc = "JPEG codec configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr1(pub u32);
    impl JpegConfr1 {
        #[doc = "Number of color components"]
        #[must_use]
        #[inline(always)]
        pub const fn nf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Number of color components"]
        #[inline(always)]
        pub const fn set_nf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Decoding Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn de(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Decoding Enable"]
        #[inline(always)]
        pub const fn set_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Color Space"]
        #[must_use]
        #[inline(always)]
        pub const fn colorspace(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Color Space"]
        #[inline(always)]
        pub const fn set_colorspace(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Number of components for Scan"]
        #[must_use]
        #[inline(always)]
        pub const fn ns(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Number of components for Scan"]
        #[inline(always)]
        pub const fn set_ns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Header Processing"]
        #[must_use]
        #[inline(always)]
        pub const fn hdr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Header Processing"]
        #[inline(always)]
        pub const fn set_hdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Y Size"]
        #[must_use]
        #[inline(always)]
        pub const fn ysize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Y Size"]
        #[inline(always)]
        pub const fn set_ysize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for JpegConfr1 {
        #[inline(always)]
        fn default() -> JpegConfr1 {
            JpegConfr1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegConfr1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr1")
                .field("nf", &self.nf())
                .field("de", &self.de())
                .field("colorspace", &self.colorspace())
                .field("ns", &self.ns())
                .field("hdr", &self.hdr())
                .field("ysize", &self.ysize())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr1 {{ nf: {=u8:?}, de: {=bool:?}, colorspace: {=u8:?}, ns: {=u8:?}, hdr: {=bool:?}, ysize: {=u16:?} }}",
                self.nf(),
                self.de(),
                self.colorspace(),
                self.ns(),
                self.hdr(),
                self.ysize()
            )
        }
    }
    #[doc = "JPEG codec configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr2(pub u32);
    impl JpegConfr2 {
        #[doc = "Number of MCU"]
        #[must_use]
        #[inline(always)]
        pub const fn nmcu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Number of MCU"]
        #[inline(always)]
        pub const fn set_nmcu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for JpegConfr2 {
        #[inline(always)]
        fn default() -> JpegConfr2 {
            JpegConfr2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegConfr2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr2").field("nmcu", &self.nmcu()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegConfr2 {{ nmcu: {=u32:?} }}", self.nmcu())
        }
    }
    #[doc = "JPEG codec configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr3(pub u32);
    impl JpegConfr3 {
        #[doc = "X size"]
        #[must_use]
        #[inline(always)]
        pub const fn xsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "X size"]
        #[inline(always)]
        pub const fn set_xsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for JpegConfr3 {
        #[inline(always)]
        fn default() -> JpegConfr3 {
            JpegConfr3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegConfr3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr3").field("xsize", &self.xsize()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegConfr3 {{ xsize: {=u16:?} }}", self.xsize())
        }
    }
    #[doc = "JPEG codec configuration register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr4(pub u32);
    impl JpegConfr4 {
        #[doc = "Huffman DC"]
        #[must_use]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub const fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Huffman AC"]
        #[must_use]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub const fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Quantization Table"]
        #[must_use]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub const fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Number of Block"]
        #[must_use]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub const fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Vertical Sampling Factor"]
        #[must_use]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub const fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Horizontal Sampling Factor"]
        #[must_use]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub const fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr4 {
        #[inline(always)]
        fn default() -> JpegConfr4 {
            JpegConfr4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegConfr4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr4")
                .field("hd", &self.hd())
                .field("ha", &self.ha())
                .field("qt", &self.qt())
                .field("nb", &self.nb())
                .field("vsf", &self.vsf())
                .field("hsf", &self.hsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr4 {{ hd: {=bool:?}, ha: {=bool:?}, qt: {=u8:?}, nb: {=u8:?}, vsf: {=u8:?}, hsf: {=u8:?} }}",
                self.hd(),
                self.ha(),
                self.qt(),
                self.nb(),
                self.vsf(),
                self.hsf()
            )
        }
    }
    #[doc = "JPEG codec configuration register 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr5(pub u32);
    impl JpegConfr5 {
        #[doc = "Huffman DC"]
        #[must_use]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub const fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Huffman AC"]
        #[must_use]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub const fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Quantization Table"]
        #[must_use]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub const fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Number of Block"]
        #[must_use]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub const fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Vertical Sampling Factor"]
        #[must_use]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub const fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Horizontal Sampling Factor"]
        #[must_use]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub const fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr5 {
        #[inline(always)]
        fn default() -> JpegConfr5 {
            JpegConfr5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegConfr5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr5")
                .field("hd", &self.hd())
                .field("ha", &self.ha())
                .field("qt", &self.qt())
                .field("nb", &self.nb())
                .field("vsf", &self.vsf())
                .field("hsf", &self.hsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr5 {{ hd: {=bool:?}, ha: {=bool:?}, qt: {=u8:?}, nb: {=u8:?}, vsf: {=u8:?}, hsf: {=u8:?} }}",
                self.hd(),
                self.ha(),
                self.qt(),
                self.nb(),
                self.vsf(),
                self.hsf()
            )
        }
    }
    #[doc = "JPEG codec configuration register 6"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr6(pub u32);
    impl JpegConfr6 {
        #[doc = "Huffman DC"]
        #[must_use]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub const fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Huffman AC"]
        #[must_use]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub const fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Quantization Table"]
        #[must_use]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub const fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Number of Block"]
        #[must_use]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub const fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Vertical Sampling Factor"]
        #[must_use]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub const fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Horizontal Sampling Factor"]
        #[must_use]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub const fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr6 {
        #[inline(always)]
        fn default() -> JpegConfr6 {
            JpegConfr6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegConfr6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr6")
                .field("hd", &self.hd())
                .field("ha", &self.ha())
                .field("qt", &self.qt())
                .field("nb", &self.nb())
                .field("vsf", &self.vsf())
                .field("hsf", &self.hsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr6 {{ hd: {=bool:?}, ha: {=bool:?}, qt: {=u8:?}, nb: {=u8:?}, vsf: {=u8:?}, hsf: {=u8:?} }}",
                self.hd(),
                self.ha(),
                self.qt(),
                self.nb(),
                self.vsf(),
                self.hsf()
            )
        }
    }
    #[doc = "JPEG codec configuration register 7"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr7(pub u32);
    impl JpegConfr7 {
        #[doc = "Huffman DC"]
        #[must_use]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub const fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Huffman AC"]
        #[must_use]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub const fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Quantization Table"]
        #[must_use]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub const fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Number of Block"]
        #[must_use]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub const fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Vertical Sampling Factor"]
        #[must_use]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub const fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Horizontal Sampling Factor"]
        #[must_use]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub const fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr7 {
        #[inline(always)]
        fn default() -> JpegConfr7 {
            JpegConfr7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegConfr7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegConfr7")
                .field("hd", &self.hd())
                .field("ha", &self.ha())
                .field("qt", &self.qt())
                .field("nb", &self.nb())
                .field("vsf", &self.vsf())
                .field("hsf", &self.hsf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegConfr7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegConfr7 {{ hd: {=bool:?}, ha: {=bool:?}, qt: {=u8:?}, nb: {=u8:?}, vsf: {=u8:?}, hsf: {=u8:?} }}",
                self.hd(),
                self.ha(),
                self.qt(),
                self.nb(),
                self.vsf(),
                self.hsf()
            )
        }
    }
    #[doc = "JPEG control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegCr(pub u32);
    impl JpegCr {
        #[doc = "JPEG Core Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn jcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG Core Enable"]
        #[inline(always)]
        pub const fn set_jcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Input FIFO Threshold Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn iftie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Threshold Interrupt Enable"]
        #[inline(always)]
        pub const fn set_iftie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Input FIFO Not Full Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ifnfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Not Full Interrupt Enable"]
        #[inline(always)]
        pub const fn set_ifnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Output FIFO Threshold Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn oftie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Threshold Interrupt Enable"]
        #[inline(always)]
        pub const fn set_oftie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Output FIFO Not Empty Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn ofneie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Not Empty Interrupt Enable"]
        #[inline(always)]
        pub const fn set_ofneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of Conversion Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of Conversion Interrupt Enable"]
        #[inline(always)]
        pub const fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Header Parsing Done Interrupt Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn hpdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Header Parsing Done Interrupt Enable"]
        #[inline(always)]
        pub const fn set_hpdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Input DMA Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn idmaen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Input DMA Enable"]
        #[inline(always)]
        pub const fn set_idmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Output DMA Enable"]
        #[must_use]
        #[inline(always)]
        pub const fn odmaen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Output DMA Enable"]
        #[inline(always)]
        pub const fn set_odmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Input FIFO Flush"]
        #[must_use]
        #[inline(always)]
        pub const fn iff(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Flush"]
        #[inline(always)]
        pub const fn set_iff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Output FIFO Flush"]
        #[must_use]
        #[inline(always)]
        pub const fn off(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Flush"]
        #[inline(always)]
        pub const fn set_off(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for JpegCr {
        #[inline(always)]
        fn default() -> JpegCr {
            JpegCr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegCr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegCr")
                .field("jcen", &self.jcen())
                .field("iftie", &self.iftie())
                .field("ifnfie", &self.ifnfie())
                .field("oftie", &self.oftie())
                .field("ofneie", &self.ofneie())
                .field("eocie", &self.eocie())
                .field("hpdie", &self.hpdie())
                .field("idmaen", &self.idmaen())
                .field("odmaen", &self.odmaen())
                .field("iff", &self.iff())
                .field("off", &self.off())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegCr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegCr {{ jcen: {=bool:?}, iftie: {=bool:?}, ifnfie: {=bool:?}, oftie: {=bool:?}, ofneie: {=bool:?}, eocie: {=bool:?}, hpdie: {=bool:?}, idmaen: {=bool:?}, odmaen: {=bool:?}, iff: {=bool:?}, off: {=bool:?} }}",
                self.jcen(),
                self.iftie(),
                self.ifnfie(),
                self.oftie(),
                self.ofneie(),
                self.eocie(),
                self.hpdie(),
                self.idmaen(),
                self.odmaen(),
                self.iff(),
                self.off()
            )
        }
    }
    #[doc = "JPEG data input register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegDir(pub u32);
    impl JpegDir {
        #[doc = "Data Input FIFO"]
        #[must_use]
        #[inline(always)]
        pub const fn datain(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data Input FIFO"]
        #[inline(always)]
        pub const fn set_datain(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for JpegDir {
        #[inline(always)]
        fn default() -> JpegDir {
            JpegDir(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegDir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegDir").field("datain", &self.datain()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegDir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegDir {{ datain: {=u32:?} }}", self.datain())
        }
    }
    #[doc = "JPEG data output register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegDor(pub u32);
    impl JpegDor {
        #[doc = "Data Output FIFO"]
        #[must_use]
        #[inline(always)]
        pub const fn dataout(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data Output FIFO"]
        #[inline(always)]
        pub const fn set_dataout(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for JpegDor {
        #[inline(always)]
        fn default() -> JpegDor {
            JpegDor(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegDor {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegDor").field("dataout", &self.dataout()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegDor {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "JpegDor {{ dataout: {=u32:?} }}", self.dataout())
        }
    }
    #[doc = "JPEG status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegSr(pub u32);
    impl JpegSr {
        #[doc = "Input FIFO Threshold Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn iftf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Threshold Flag"]
        #[inline(always)]
        pub const fn set_iftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Input FIFO Not Full Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ifnff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Not Full Flag"]
        #[inline(always)]
        pub const fn set_ifnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Output FIFO Threshold Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn oftf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Threshold Flag"]
        #[inline(always)]
        pub const fn set_oftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Output FIFO Not Empty Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn ofnef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Not Empty Flag"]
        #[inline(always)]
        pub const fn set_ofnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of Conversion Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn eocf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of Conversion Flag"]
        #[inline(always)]
        pub const fn set_eocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Header Parsing Done Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn hpdf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Header Parsing Done Flag"]
        #[inline(always)]
        pub const fn set_hpdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Codec Operation Flag"]
        #[must_use]
        #[inline(always)]
        pub const fn cof(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Codec Operation Flag"]
        #[inline(always)]
        pub const fn set_cof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for JpegSr {
        #[inline(always)]
        fn default() -> JpegSr {
            JpegSr(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for JpegSr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("JpegSr")
                .field("iftf", &self.iftf())
                .field("ifnff", &self.ifnff())
                .field("oftf", &self.oftf())
                .field("ofnef", &self.ofnef())
                .field("eocf", &self.eocf())
                .field("hpdf", &self.hpdf())
                .field("cof", &self.cof())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for JpegSr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "JpegSr {{ iftf: {=bool:?}, ifnff: {=bool:?}, oftf: {=bool:?}, ofnef: {=bool:?}, eocf: {=bool:?}, hpdf: {=bool:?}, cof: {=bool:?} }}",
                self.iftf(),
                self.ifnff(),
                self.oftf(),
                self.ofnef(),
                self.eocf(),
                self.hpdf(),
                self.cof()
            )
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_0(pub u32);
    impl Qmem0_0 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_0 {
        #[inline(always)]
        fn default() -> Qmem0_0 {
            Qmem0_0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_0").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_0 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_1(pub u32);
    impl Qmem0_1 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_1 {
        #[inline(always)]
        fn default() -> Qmem0_1 {
            Qmem0_1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_1").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_1 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_10(pub u32);
    impl Qmem0_10 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_10 {
        #[inline(always)]
        fn default() -> Qmem0_10 {
            Qmem0_10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_10")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_10 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_11(pub u32);
    impl Qmem0_11 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_11 {
        #[inline(always)]
        fn default() -> Qmem0_11 {
            Qmem0_11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_11")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_11 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_12(pub u32);
    impl Qmem0_12 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_12 {
        #[inline(always)]
        fn default() -> Qmem0_12 {
            Qmem0_12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_12")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_12 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_13(pub u32);
    impl Qmem0_13 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_13 {
        #[inline(always)]
        fn default() -> Qmem0_13 {
            Qmem0_13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_13")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_13 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_14(pub u32);
    impl Qmem0_14 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_14 {
        #[inline(always)]
        fn default() -> Qmem0_14 {
            Qmem0_14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_14")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_14 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_15(pub u32);
    impl Qmem0_15 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_15 {
        #[inline(always)]
        fn default() -> Qmem0_15 {
            Qmem0_15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_15")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_15 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_2(pub u32);
    impl Qmem0_2 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_2 {
        #[inline(always)]
        fn default() -> Qmem0_2 {
            Qmem0_2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_2").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_2 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_3(pub u32);
    impl Qmem0_3 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_3 {
        #[inline(always)]
        fn default() -> Qmem0_3 {
            Qmem0_3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_3").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_3 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_4(pub u32);
    impl Qmem0_4 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_4 {
        #[inline(always)]
        fn default() -> Qmem0_4 {
            Qmem0_4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_4").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_4 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_5(pub u32);
    impl Qmem0_5 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_5 {
        #[inline(always)]
        fn default() -> Qmem0_5 {
            Qmem0_5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_5").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_5 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_6(pub u32);
    impl Qmem0_6 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_6 {
        #[inline(always)]
        fn default() -> Qmem0_6 {
            Qmem0_6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_6").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_6 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_7(pub u32);
    impl Qmem0_7 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_7 {
        #[inline(always)]
        fn default() -> Qmem0_7 {
            Qmem0_7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_7").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_7 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_8(pub u32);
    impl Qmem0_8 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_8 {
        #[inline(always)]
        fn default() -> Qmem0_8 {
            Qmem0_8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_8").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_8 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem0_9(pub u32);
    impl Qmem0_9 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem0_9 {
        #[inline(always)]
        fn default() -> Qmem0_9 {
            Qmem0_9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem0_9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem0_9").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem0_9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem0_9 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_0(pub u32);
    impl Qmem1_0 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_0 {
        #[inline(always)]
        fn default() -> Qmem1_0 {
            Qmem1_0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_0").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_0 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_1(pub u32);
    impl Qmem1_1 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_1 {
        #[inline(always)]
        fn default() -> Qmem1_1 {
            Qmem1_1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_1").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_1 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_10(pub u32);
    impl Qmem1_10 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_10 {
        #[inline(always)]
        fn default() -> Qmem1_10 {
            Qmem1_10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_10")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_10 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_11(pub u32);
    impl Qmem1_11 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_11 {
        #[inline(always)]
        fn default() -> Qmem1_11 {
            Qmem1_11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_11")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_11 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_12(pub u32);
    impl Qmem1_12 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_12 {
        #[inline(always)]
        fn default() -> Qmem1_12 {
            Qmem1_12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_12")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_12 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_13(pub u32);
    impl Qmem1_13 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_13 {
        #[inline(always)]
        fn default() -> Qmem1_13 {
            Qmem1_13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_13")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_13 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_14(pub u32);
    impl Qmem1_14 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_14 {
        #[inline(always)]
        fn default() -> Qmem1_14 {
            Qmem1_14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_14")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_14 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_15(pub u32);
    impl Qmem1_15 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_15 {
        #[inline(always)]
        fn default() -> Qmem1_15 {
            Qmem1_15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_15")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_15 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_2(pub u32);
    impl Qmem1_2 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_2 {
        #[inline(always)]
        fn default() -> Qmem1_2 {
            Qmem1_2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_2").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_2 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_3(pub u32);
    impl Qmem1_3 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_3 {
        #[inline(always)]
        fn default() -> Qmem1_3 {
            Qmem1_3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_3").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_3 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_4(pub u32);
    impl Qmem1_4 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_4 {
        #[inline(always)]
        fn default() -> Qmem1_4 {
            Qmem1_4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_4").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_4 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_5(pub u32);
    impl Qmem1_5 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_5 {
        #[inline(always)]
        fn default() -> Qmem1_5 {
            Qmem1_5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_5").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_5 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_6(pub u32);
    impl Qmem1_6 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_6 {
        #[inline(always)]
        fn default() -> Qmem1_6 {
            Qmem1_6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_6").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_6 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_7(pub u32);
    impl Qmem1_7 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_7 {
        #[inline(always)]
        fn default() -> Qmem1_7 {
            Qmem1_7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_7").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_7 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_8(pub u32);
    impl Qmem1_8 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_8 {
        #[inline(always)]
        fn default() -> Qmem1_8 {
            Qmem1_8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_8").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_8 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem1_9(pub u32);
    impl Qmem1_9 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem1_9 {
        #[inline(always)]
        fn default() -> Qmem1_9 {
            Qmem1_9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem1_9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem1_9").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem1_9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem1_9 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_0(pub u32);
    impl Qmem2_0 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_0 {
        #[inline(always)]
        fn default() -> Qmem2_0 {
            Qmem2_0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_0").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_0 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_1(pub u32);
    impl Qmem2_1 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_1 {
        #[inline(always)]
        fn default() -> Qmem2_1 {
            Qmem2_1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_1").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_1 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_10(pub u32);
    impl Qmem2_10 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_10 {
        #[inline(always)]
        fn default() -> Qmem2_10 {
            Qmem2_10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_10")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_10 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_11(pub u32);
    impl Qmem2_11 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_11 {
        #[inline(always)]
        fn default() -> Qmem2_11 {
            Qmem2_11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_11")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_11 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_12(pub u32);
    impl Qmem2_12 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_12 {
        #[inline(always)]
        fn default() -> Qmem2_12 {
            Qmem2_12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_12")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_12 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_13(pub u32);
    impl Qmem2_13 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_13 {
        #[inline(always)]
        fn default() -> Qmem2_13 {
            Qmem2_13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_13")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_13 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_14(pub u32);
    impl Qmem2_14 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_14 {
        #[inline(always)]
        fn default() -> Qmem2_14 {
            Qmem2_14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_14")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_14 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_15(pub u32);
    impl Qmem2_15 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_15 {
        #[inline(always)]
        fn default() -> Qmem2_15 {
            Qmem2_15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_15")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_15 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_2(pub u32);
    impl Qmem2_2 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_2 {
        #[inline(always)]
        fn default() -> Qmem2_2 {
            Qmem2_2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_2").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_2 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_3(pub u32);
    impl Qmem2_3 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_3 {
        #[inline(always)]
        fn default() -> Qmem2_3 {
            Qmem2_3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_3").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_3 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_4(pub u32);
    impl Qmem2_4 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_4 {
        #[inline(always)]
        fn default() -> Qmem2_4 {
            Qmem2_4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_4").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_4 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_5(pub u32);
    impl Qmem2_5 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_5 {
        #[inline(always)]
        fn default() -> Qmem2_5 {
            Qmem2_5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_5").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_5 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_6(pub u32);
    impl Qmem2_6 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_6 {
        #[inline(always)]
        fn default() -> Qmem2_6 {
            Qmem2_6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_6").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_6 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_7(pub u32);
    impl Qmem2_7 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_7 {
        #[inline(always)]
        fn default() -> Qmem2_7 {
            Qmem2_7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_7").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_7 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_8(pub u32);
    impl Qmem2_8 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_8 {
        #[inline(always)]
        fn default() -> Qmem2_8 {
            Qmem2_8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_8").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_8 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem2_9(pub u32);
    impl Qmem2_9 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem2_9 {
        #[inline(always)]
        fn default() -> Qmem2_9 {
            Qmem2_9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem2_9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem2_9").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem2_9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem2_9 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_0(pub u32);
    impl Qmem3_0 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_0 {
        #[inline(always)]
        fn default() -> Qmem3_0 {
            Qmem3_0(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_0 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_0").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_0 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_0 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_1(pub u32);
    impl Qmem3_1 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_1 {
        #[inline(always)]
        fn default() -> Qmem3_1 {
            Qmem3_1(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_1 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_1").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_1 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_1 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_10(pub u32);
    impl Qmem3_10 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_10 {
        #[inline(always)]
        fn default() -> Qmem3_10 {
            Qmem3_10(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_10 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_10")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_10 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_10 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_11(pub u32);
    impl Qmem3_11 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_11 {
        #[inline(always)]
        fn default() -> Qmem3_11 {
            Qmem3_11(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_11 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_11")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_11 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_12(pub u32);
    impl Qmem3_12 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_12 {
        #[inline(always)]
        fn default() -> Qmem3_12 {
            Qmem3_12(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_12 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_12")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_12 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_13(pub u32);
    impl Qmem3_13 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_13 {
        #[inline(always)]
        fn default() -> Qmem3_13 {
            Qmem3_13(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_13 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_13")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_13 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_13 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_14(pub u32);
    impl Qmem3_14 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_14 {
        #[inline(always)]
        fn default() -> Qmem3_14 {
            Qmem3_14(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_14 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_14")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_14 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_14 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_15(pub u32);
    impl Qmem3_15 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_15 {
        #[inline(always)]
        fn default() -> Qmem3_15 {
            Qmem3_15(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_15 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_15")
                .field("q_mem_ram", &self.q_mem_ram())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_15 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_15 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_2(pub u32);
    impl Qmem3_2 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_2 {
        #[inline(always)]
        fn default() -> Qmem3_2 {
            Qmem3_2(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_2 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_2").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_2 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_2 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_3(pub u32);
    impl Qmem3_3 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_3 {
        #[inline(always)]
        fn default() -> Qmem3_3 {
            Qmem3_3(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_3 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_3").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_3 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_4(pub u32);
    impl Qmem3_4 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_4 {
        #[inline(always)]
        fn default() -> Qmem3_4 {
            Qmem3_4(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_4 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_4").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_4 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_4 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_5(pub u32);
    impl Qmem3_5 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_5 {
        #[inline(always)]
        fn default() -> Qmem3_5 {
            Qmem3_5(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_5 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_5").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_5 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_5 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_6(pub u32);
    impl Qmem3_6 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_6 {
        #[inline(always)]
        fn default() -> Qmem3_6 {
            Qmem3_6(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_6 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_6").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_6 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_7(pub u32);
    impl Qmem3_7 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_7 {
        #[inline(always)]
        fn default() -> Qmem3_7 {
            Qmem3_7(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_7 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_7").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_7 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_8(pub u32);
    impl Qmem3_8 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_8 {
        #[inline(always)]
        fn default() -> Qmem3_8 {
            Qmem3_8(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_8 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_8").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_8 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem3_9(pub u32);
    impl Qmem3_9 {
        #[doc = "QMem RAM"]
        #[must_use]
        #[inline(always)]
        pub const fn q_mem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn set_q_mem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem3_9 {
        #[inline(always)]
        fn default() -> Qmem3_9 {
            Qmem3_9(0)
        }
    }
    #[cfg(feature = "debug")]
    impl core::fmt::Debug for Qmem3_9 {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Qmem3_9").field("q_mem_ram", &self.q_mem_ram()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Qmem3_9 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Qmem3_9 {{ q_mem_ram: {=u32:?} }}", self.q_mem_ram())
        }
    }
}
