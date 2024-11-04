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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "JPEG codec configuration register 1"]
    #[inline(always)]
    pub const fn jpeg_confr1(self) -> crate::common::Reg<regs::JpegConfr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "JPEG codec configuration register 2"]
    #[inline(always)]
    pub const fn jpeg_confr2(self) -> crate::common::Reg<regs::JpegConfr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "JPEG codec configuration register 3"]
    #[inline(always)]
    pub const fn jpeg_confr3(self) -> crate::common::Reg<regs::JpegConfr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "JPEG codec configuration register 4"]
    #[inline(always)]
    pub const fn jpeg_confr4(self) -> crate::common::Reg<regs::JpegConfr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "JPEG codec configuration register 5"]
    #[inline(always)]
    pub const fn jpeg_confr5(self) -> crate::common::Reg<regs::JpegConfr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "JPEG codec configuration register 6"]
    #[inline(always)]
    pub const fn jpeg_confr6(self) -> crate::common::Reg<regs::JpegConfr6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "JPEG codec configuration register 7"]
    #[inline(always)]
    pub const fn jpeg_confr7(self) -> crate::common::Reg<regs::JpegConfr7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "JPEG control register"]
    #[inline(always)]
    pub const fn jpeg_cr(self) -> crate::common::Reg<regs::JpegCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "JPEG status register"]
    #[inline(always)]
    pub const fn jpeg_sr(self) -> crate::common::Reg<regs::JpegSr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "JPEG clear flag register"]
    #[inline(always)]
    pub const fn jpeg_cfr(self) -> crate::common::Reg<regs::JpegCfr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "JPEG data input register"]
    #[inline(always)]
    pub const fn jpeg_dir(self) -> crate::common::Reg<regs::JpegDir, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "JPEG data output register"]
    #[inline(always)]
    pub const fn jpeg_dor(self) -> crate::common::Reg<regs::JpegDor, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_0(self) -> crate::common::Reg<regs::Qmem00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_1(self) -> crate::common::Reg<regs::Qmem01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_2(self) -> crate::common::Reg<regs::Qmem02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_3(self) -> crate::common::Reg<regs::Qmem03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_4(self) -> crate::common::Reg<regs::Qmem04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_5(self) -> crate::common::Reg<regs::Qmem05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_6(self) -> crate::common::Reg<regs::Qmem06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_7(self) -> crate::common::Reg<regs::Qmem07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_8(self) -> crate::common::Reg<regs::Qmem08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_9(self) -> crate::common::Reg<regs::Qmem09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_10(self) -> crate::common::Reg<regs::Qmem010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_11(self) -> crate::common::Reg<regs::Qmem011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_12(self) -> crate::common::Reg<regs::Qmem012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_13(self) -> crate::common::Reg<regs::Qmem013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_14(self) -> crate::common::Reg<regs::Qmem014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem0_15(self) -> crate::common::Reg<regs::Qmem015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_0(self) -> crate::common::Reg<regs::Qmem10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_1(self) -> crate::common::Reg<regs::Qmem11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_2(self) -> crate::common::Reg<regs::Qmem12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_3(self) -> crate::common::Reg<regs::Qmem13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_4(self) -> crate::common::Reg<regs::Qmem14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_5(self) -> crate::common::Reg<regs::Qmem15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_6(self) -> crate::common::Reg<regs::Qmem16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_7(self) -> crate::common::Reg<regs::Qmem17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_8(self) -> crate::common::Reg<regs::Qmem18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_9(self) -> crate::common::Reg<regs::Qmem19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_10(self) -> crate::common::Reg<regs::Qmem110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_11(self) -> crate::common::Reg<regs::Qmem111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_12(self) -> crate::common::Reg<regs::Qmem112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_13(self) -> crate::common::Reg<regs::Qmem113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_14(self) -> crate::common::Reg<regs::Qmem114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem1_15(self) -> crate::common::Reg<regs::Qmem115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xccusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_0(self) -> crate::common::Reg<regs::Qmem20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_1(self) -> crate::common::Reg<regs::Qmem21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_2(self) -> crate::common::Reg<regs::Qmem22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_3(self) -> crate::common::Reg<regs::Qmem23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_4(self) -> crate::common::Reg<regs::Qmem24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_5(self) -> crate::common::Reg<regs::Qmem25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_6(self) -> crate::common::Reg<regs::Qmem26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_7(self) -> crate::common::Reg<regs::Qmem27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_8(self) -> crate::common::Reg<regs::Qmem28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_9(self) -> crate::common::Reg<regs::Qmem29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_10(self) -> crate::common::Reg<regs::Qmem210, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_11(self) -> crate::common::Reg<regs::Qmem211, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_12(self) -> crate::common::Reg<regs::Qmem212, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_13(self) -> crate::common::Reg<regs::Qmem213, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_14(self) -> crate::common::Reg<regs::Qmem214, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem2_15(self) -> crate::common::Reg<regs::Qmem215, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_0(self) -> crate::common::Reg<regs::Qmem30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_1(self) -> crate::common::Reg<regs::Qmem31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_2(self) -> crate::common::Reg<regs::Qmem32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_3(self) -> crate::common::Reg<regs::Qmem33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_4(self) -> crate::common::Reg<regs::Qmem34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_5(self) -> crate::common::Reg<regs::Qmem35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_6(self) -> crate::common::Reg<regs::Qmem36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_7(self) -> crate::common::Reg<regs::Qmem37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_8(self) -> crate::common::Reg<regs::Qmem38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_9(self) -> crate::common::Reg<regs::Qmem39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_10(self) -> crate::common::Reg<regs::Qmem310, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_11(self) -> crate::common::Reg<regs::Qmem311, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_12(self) -> crate::common::Reg<regs::Qmem312, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_13(self) -> crate::common::Reg<regs::Qmem313, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_14(self) -> crate::common::Reg<regs::Qmem314, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "JPEG quantization tables"]
    #[inline(always)]
    pub const fn qmem3_15(self) -> crate::common::Reg<regs::Qmem315, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_0(self) -> crate::common::Reg<regs::Huffmin0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_1(self) -> crate::common::Reg<regs::Huffmin1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_2(self) -> crate::common::Reg<regs::Huffmin2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_3(self) -> crate::common::Reg<regs::Huffmin3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_4(self) -> crate::common::Reg<regs::Huffmin4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_5(self) -> crate::common::Reg<regs::Huffmin5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_6(self) -> crate::common::Reg<regs::Huffmin6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_7(self) -> crate::common::Reg<regs::Huffmin7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_8(self) -> crate::common::Reg<regs::Huffmin8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_9(self) -> crate::common::Reg<regs::Huffmin9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0174usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_10(self) -> crate::common::Reg<regs::Huffmin10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_11(self) -> crate::common::Reg<regs::Huffmin11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_12(self) -> crate::common::Reg<regs::Huffmin12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_13(self) -> crate::common::Reg<regs::Huffmin13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_14(self) -> crate::common::Reg<regs::Huffmin14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "JPEG HuffMin tables"]
    #[inline(always)]
    pub const fn huffmin_15(self) -> crate::common::Reg<regs::Huffmin15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase0(self) -> crate::common::Reg<regs::Huffbase0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase1(self) -> crate::common::Reg<regs::Huffbase1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0194usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase2(self) -> crate::common::Reg<regs::Huffbase2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0198usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase3(self) -> crate::common::Reg<regs::Huffbase3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x019cusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase4(self) -> crate::common::Reg<regs::Huffbase4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase5(self) -> crate::common::Reg<regs::Huffbase5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase6(self) -> crate::common::Reg<regs::Huffbase6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase7(self) -> crate::common::Reg<regs::Huffbase7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01acusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase8(self) -> crate::common::Reg<regs::Huffbase8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase9(self) -> crate::common::Reg<regs::Huffbase9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase10(self) -> crate::common::Reg<regs::Huffbase10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase11(self) -> crate::common::Reg<regs::Huffbase11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase12(self) -> crate::common::Reg<regs::Huffbase12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase13(self) -> crate::common::Reg<regs::Huffbase13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase14(self) -> crate::common::Reg<regs::Huffbase14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase15(self) -> crate::common::Reg<regs::Huffbase15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase16(self) -> crate::common::Reg<regs::Huffbase16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase17(self) -> crate::common::Reg<regs::Huffbase17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase18(self) -> crate::common::Reg<regs::Huffbase18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase19(self) -> crate::common::Reg<regs::Huffbase19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01dcusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase20(self) -> crate::common::Reg<regs::Huffbase20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase21(self) -> crate::common::Reg<regs::Huffbase21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase22(self) -> crate::common::Reg<regs::Huffbase22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase23(self) -> crate::common::Reg<regs::Huffbase23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase24(self) -> crate::common::Reg<regs::Huffbase24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f0usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase25(self) -> crate::common::Reg<regs::Huffbase25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f4usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase26(self) -> crate::common::Reg<regs::Huffbase26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f8usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase27(self) -> crate::common::Reg<regs::Huffbase27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase28(self) -> crate::common::Reg<regs::Huffbase28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase29(self) -> crate::common::Reg<regs::Huffbase29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0204usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase30(self) -> crate::common::Reg<regs::Huffbase30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize) as _) }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[inline(always)]
    pub const fn huffbase31(self) -> crate::common::Reg<regs::Huffbase31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x020cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb0(self) -> crate::common::Reg<regs::Huffsymb0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb1(self) -> crate::common::Reg<regs::Huffsymb1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0214usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb2(self) -> crate::common::Reg<regs::Huffsymb2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb3(self) -> crate::common::Reg<regs::Huffsymb3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x021cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb4(self) -> crate::common::Reg<regs::Huffsymb4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb5(self) -> crate::common::Reg<regs::Huffsymb5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0224usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb6(self) -> crate::common::Reg<regs::Huffsymb6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb7(self) -> crate::common::Reg<regs::Huffsymb7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x022cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb8(self) -> crate::common::Reg<regs::Huffsymb8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb9(self) -> crate::common::Reg<regs::Huffsymb9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0234usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb10(self) -> crate::common::Reg<regs::Huffsymb10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0238usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb11(self) -> crate::common::Reg<regs::Huffsymb11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x023cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb12(self) -> crate::common::Reg<regs::Huffsymb12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0240usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb13(self) -> crate::common::Reg<regs::Huffsymb13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0244usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb14(self) -> crate::common::Reg<regs::Huffsymb14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0248usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb15(self) -> crate::common::Reg<regs::Huffsymb15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x024cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb16(self) -> crate::common::Reg<regs::Huffsymb16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0250usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb17(self) -> crate::common::Reg<regs::Huffsymb17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0254usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb18(self) -> crate::common::Reg<regs::Huffsymb18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0258usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb19(self) -> crate::common::Reg<regs::Huffsymb19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x025cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb20(self) -> crate::common::Reg<regs::Huffsymb20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0260usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb21(self) -> crate::common::Reg<regs::Huffsymb21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0264usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb22(self) -> crate::common::Reg<regs::Huffsymb22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0268usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb23(self) -> crate::common::Reg<regs::Huffsymb23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x026cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb24(self) -> crate::common::Reg<regs::Huffsymb24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0270usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb25(self) -> crate::common::Reg<regs::Huffsymb25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0274usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb26(self) -> crate::common::Reg<regs::Huffsymb26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0278usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb27(self) -> crate::common::Reg<regs::Huffsymb27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x027cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb28(self) -> crate::common::Reg<regs::Huffsymb28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0280usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb29(self) -> crate::common::Reg<regs::Huffsymb29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0284usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb30(self) -> crate::common::Reg<regs::Huffsymb30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0288usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb31(self) -> crate::common::Reg<regs::Huffsymb31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x028cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb32(self) -> crate::common::Reg<regs::Huffsymb32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0290usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb33(self) -> crate::common::Reg<regs::Huffsymb33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0294usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb34(self) -> crate::common::Reg<regs::Huffsymb34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0298usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb35(self) -> crate::common::Reg<regs::Huffsymb35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x029cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb36(self) -> crate::common::Reg<regs::Huffsymb36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb37(self) -> crate::common::Reg<regs::Huffsymb37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb38(self) -> crate::common::Reg<regs::Huffsymb38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02a8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb39(self) -> crate::common::Reg<regs::Huffsymb39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02acusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb40(self) -> crate::common::Reg<regs::Huffsymb40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb41(self) -> crate::common::Reg<regs::Huffsymb41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb42(self) -> crate::common::Reg<regs::Huffsymb42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02b8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb43(self) -> crate::common::Reg<regs::Huffsymb43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02bcusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb44(self) -> crate::common::Reg<regs::Huffsymb44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb45(self) -> crate::common::Reg<regs::Huffsymb45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb46(self) -> crate::common::Reg<regs::Huffsymb46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02c8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb47(self) -> crate::common::Reg<regs::Huffsymb47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02ccusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb48(self) -> crate::common::Reg<regs::Huffsymb48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb49(self) -> crate::common::Reg<regs::Huffsymb49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb50(self) -> crate::common::Reg<regs::Huffsymb50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02d8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb51(self) -> crate::common::Reg<regs::Huffsymb51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02dcusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb52(self) -> crate::common::Reg<regs::Huffsymb52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb53(self) -> crate::common::Reg<regs::Huffsymb53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb54(self) -> crate::common::Reg<regs::Huffsymb54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02e8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb55(self) -> crate::common::Reg<regs::Huffsymb55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02ecusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb56(self) -> crate::common::Reg<regs::Huffsymb56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f0usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb57(self) -> crate::common::Reg<regs::Huffsymb57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f4usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb58(self) -> crate::common::Reg<regs::Huffsymb58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02f8usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb59(self) -> crate::common::Reg<regs::Huffsymb59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x02fcusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb60(self) -> crate::common::Reg<regs::Huffsymb60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0300usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb61(self) -> crate::common::Reg<regs::Huffsymb61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0304usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb62(self) -> crate::common::Reg<regs::Huffsymb62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0308usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb63(self) -> crate::common::Reg<regs::Huffsymb63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x030cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb64(self) -> crate::common::Reg<regs::Huffsymb64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0310usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb65(self) -> crate::common::Reg<regs::Huffsymb65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0314usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb66(self) -> crate::common::Reg<regs::Huffsymb66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0318usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb67(self) -> crate::common::Reg<regs::Huffsymb67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x031cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb68(self) -> crate::common::Reg<regs::Huffsymb68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0320usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb69(self) -> crate::common::Reg<regs::Huffsymb69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0324usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb70(self) -> crate::common::Reg<regs::Huffsymb70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0328usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb71(self) -> crate::common::Reg<regs::Huffsymb71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x032cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb72(self) -> crate::common::Reg<regs::Huffsymb72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0330usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb73(self) -> crate::common::Reg<regs::Huffsymb73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0334usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb74(self) -> crate::common::Reg<regs::Huffsymb74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0338usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb75(self) -> crate::common::Reg<regs::Huffsymb75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x033cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb76(self) -> crate::common::Reg<regs::Huffsymb76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0340usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb77(self) -> crate::common::Reg<regs::Huffsymb77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0344usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb78(self) -> crate::common::Reg<regs::Huffsymb78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0348usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb79(self) -> crate::common::Reg<regs::Huffsymb79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x034cusize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb80(self) -> crate::common::Reg<regs::Huffsymb80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0350usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb81(self) -> crate::common::Reg<regs::Huffsymb81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0354usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb82(self) -> crate::common::Reg<regs::Huffsymb82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0358usize) as _) }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[inline(always)]
    pub const fn huffsymb83(self) -> crate::common::Reg<regs::Huffsymb83, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x035cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem0(self) -> crate::common::Reg<regs::Dhtmem0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0360usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem2(self) -> crate::common::Reg<regs::Dhtmem2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0364usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem3(self) -> crate::common::Reg<regs::Dhtmem3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0368usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem4(self) -> crate::common::Reg<regs::Dhtmem4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x036cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem5(self) -> crate::common::Reg<regs::Dhtmem5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0370usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem6(self) -> crate::common::Reg<regs::Dhtmem6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0374usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem7(self) -> crate::common::Reg<regs::Dhtmem7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0378usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem8(self) -> crate::common::Reg<regs::Dhtmem8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x037cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem9(self) -> crate::common::Reg<regs::Dhtmem9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0380usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem10(self) -> crate::common::Reg<regs::Dhtmem10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0384usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem11(self) -> crate::common::Reg<regs::Dhtmem11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0388usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem12(self) -> crate::common::Reg<regs::Dhtmem12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x038cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem13(self) -> crate::common::Reg<regs::Dhtmem13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0390usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem14(self) -> crate::common::Reg<regs::Dhtmem14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0394usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem15(self) -> crate::common::Reg<regs::Dhtmem15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0398usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem16(self) -> crate::common::Reg<regs::Dhtmem16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x039cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem17(self) -> crate::common::Reg<regs::Dhtmem17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem18(self) -> crate::common::Reg<regs::Dhtmem18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem19(self) -> crate::common::Reg<regs::Dhtmem19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03a8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem20(self) -> crate::common::Reg<regs::Dhtmem20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03acusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem21(self) -> crate::common::Reg<regs::Dhtmem21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem22(self) -> crate::common::Reg<regs::Dhtmem22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem23(self) -> crate::common::Reg<regs::Dhtmem23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03b8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem24(self) -> crate::common::Reg<regs::Dhtmem24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03bcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem25(self) -> crate::common::Reg<regs::Dhtmem25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem26(self) -> crate::common::Reg<regs::Dhtmem26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem27(self) -> crate::common::Reg<regs::Dhtmem27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03c8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem28(self) -> crate::common::Reg<regs::Dhtmem28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ccusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem29(self) -> crate::common::Reg<regs::Dhtmem29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem30(self) -> crate::common::Reg<regs::Dhtmem30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem31(self) -> crate::common::Reg<regs::Dhtmem31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03d8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem32(self) -> crate::common::Reg<regs::Dhtmem32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03dcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem33(self) -> crate::common::Reg<regs::Dhtmem33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem34(self) -> crate::common::Reg<regs::Dhtmem34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem35(self) -> crate::common::Reg<regs::Dhtmem35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03e8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem36(self) -> crate::common::Reg<regs::Dhtmem36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03ecusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem37(self) -> crate::common::Reg<regs::Dhtmem37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem38(self) -> crate::common::Reg<regs::Dhtmem38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem39(self) -> crate::common::Reg<regs::Dhtmem39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03f8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem40(self) -> crate::common::Reg<regs::Dhtmem40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x03fcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem41(self) -> crate::common::Reg<regs::Dhtmem41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem42(self) -> crate::common::Reg<regs::Dhtmem42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem43(self) -> crate::common::Reg<regs::Dhtmem43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem44(self) -> crate::common::Reg<regs::Dhtmem44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem45(self) -> crate::common::Reg<regs::Dhtmem45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0410usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem46(self) -> crate::common::Reg<regs::Dhtmem46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0414usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem47(self) -> crate::common::Reg<regs::Dhtmem47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0418usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem48(self) -> crate::common::Reg<regs::Dhtmem48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x041cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem49(self) -> crate::common::Reg<regs::Dhtmem49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0420usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem50(self) -> crate::common::Reg<regs::Dhtmem50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0424usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem51(self) -> crate::common::Reg<regs::Dhtmem51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0428usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem52(self) -> crate::common::Reg<regs::Dhtmem52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x042cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem53(self) -> crate::common::Reg<regs::Dhtmem53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0430usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem54(self) -> crate::common::Reg<regs::Dhtmem54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0434usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem55(self) -> crate::common::Reg<regs::Dhtmem55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0438usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem56(self) -> crate::common::Reg<regs::Dhtmem56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x043cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem57(self) -> crate::common::Reg<regs::Dhtmem57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem58(self) -> crate::common::Reg<regs::Dhtmem58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0444usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem59(self) -> crate::common::Reg<regs::Dhtmem59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0448usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem60(self) -> crate::common::Reg<regs::Dhtmem60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x044cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem61(self) -> crate::common::Reg<regs::Dhtmem61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0450usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem62(self) -> crate::common::Reg<regs::Dhtmem62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0454usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem63(self) -> crate::common::Reg<regs::Dhtmem63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0458usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem64(self) -> crate::common::Reg<regs::Dhtmem64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x045cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem65(self) -> crate::common::Reg<regs::Dhtmem65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0460usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem66(self) -> crate::common::Reg<regs::Dhtmem66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0464usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem67(self) -> crate::common::Reg<regs::Dhtmem67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0468usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem68(self) -> crate::common::Reg<regs::Dhtmem68, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x046cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem69(self) -> crate::common::Reg<regs::Dhtmem69, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0470usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem70(self) -> crate::common::Reg<regs::Dhtmem70, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0474usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem71(self) -> crate::common::Reg<regs::Dhtmem71, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0478usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem72(self) -> crate::common::Reg<regs::Dhtmem72, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x047cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem73(self) -> crate::common::Reg<regs::Dhtmem73, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem74(self) -> crate::common::Reg<regs::Dhtmem74, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0484usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem75(self) -> crate::common::Reg<regs::Dhtmem75, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0488usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem76(self) -> crate::common::Reg<regs::Dhtmem76, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x048cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem77(self) -> crate::common::Reg<regs::Dhtmem77, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0490usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem78(self) -> crate::common::Reg<regs::Dhtmem78, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0494usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem79(self) -> crate::common::Reg<regs::Dhtmem79, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0498usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem80(self) -> crate::common::Reg<regs::Dhtmem80, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x049cusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem81(self) -> crate::common::Reg<regs::Dhtmem81, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem82(self) -> crate::common::Reg<regs::Dhtmem82, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem83(self) -> crate::common::Reg<regs::Dhtmem83, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04a8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem84(self) -> crate::common::Reg<regs::Dhtmem84, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04acusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem85(self) -> crate::common::Reg<regs::Dhtmem85, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem86(self) -> crate::common::Reg<regs::Dhtmem86, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem87(self) -> crate::common::Reg<regs::Dhtmem87, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04b8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem88(self) -> crate::common::Reg<regs::Dhtmem88, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04bcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem89(self) -> crate::common::Reg<regs::Dhtmem89, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem90(self) -> crate::common::Reg<regs::Dhtmem90, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem91(self) -> crate::common::Reg<regs::Dhtmem91, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem92(self) -> crate::common::Reg<regs::Dhtmem92, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04ccusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem93(self) -> crate::common::Reg<regs::Dhtmem93, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem94(self) -> crate::common::Reg<regs::Dhtmem94, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem95(self) -> crate::common::Reg<regs::Dhtmem95, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem96(self) -> crate::common::Reg<regs::Dhtmem96, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04dcusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem97(self) -> crate::common::Reg<regs::Dhtmem97, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem98(self) -> crate::common::Reg<regs::Dhtmem98, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem99(self) -> crate::common::Reg<regs::Dhtmem99, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04e8usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem100(self) -> crate::common::Reg<regs::Dhtmem100, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04ecusize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem101(self) -> crate::common::Reg<regs::Dhtmem101, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04f0usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem102(self) -> crate::common::Reg<regs::Dhtmem102, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04f4usize) as _) }
    }
    #[doc = "JPEG DHTMem tables"]
    #[inline(always)]
    pub const fn dhtmem103(self) -> crate::common::Reg<regs::Dhtmem103, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04f8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_0(self) -> crate::common::Reg<regs::HuffencAc00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_1(self) -> crate::common::Reg<regs::HuffencAc01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_2(self) -> crate::common::Reg<regs::HuffencAc02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_3(self) -> crate::common::Reg<regs::HuffencAc03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x050cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_4(self) -> crate::common::Reg<regs::HuffencAc04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0510usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_5(self) -> crate::common::Reg<regs::HuffencAc05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0514usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_6(self) -> crate::common::Reg<regs::HuffencAc06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0518usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_7(self) -> crate::common::Reg<regs::HuffencAc07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x051cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_8(self) -> crate::common::Reg<regs::HuffencAc08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0520usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_9(self) -> crate::common::Reg<regs::HuffencAc09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0524usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_10(self) -> crate::common::Reg<regs::HuffencAc010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0528usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_11(self) -> crate::common::Reg<regs::HuffencAc011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x052cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_12(self) -> crate::common::Reg<regs::HuffencAc012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0530usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_13(self) -> crate::common::Reg<regs::HuffencAc013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0534usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_14(self) -> crate::common::Reg<regs::HuffencAc014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0538usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_15(self) -> crate::common::Reg<regs::HuffencAc015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x053cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_16(self) -> crate::common::Reg<regs::HuffencAc016, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0540usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_17(self) -> crate::common::Reg<regs::HuffencAc017, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0544usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_18(self) -> crate::common::Reg<regs::HuffencAc018, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0548usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_19(self) -> crate::common::Reg<regs::HuffencAc019, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x054cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_20(self) -> crate::common::Reg<regs::HuffencAc020, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0550usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_21(self) -> crate::common::Reg<regs::HuffencAc021, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0554usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_22(self) -> crate::common::Reg<regs::HuffencAc022, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0558usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_23(self) -> crate::common::Reg<regs::HuffencAc023, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x055cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_24(self) -> crate::common::Reg<regs::HuffencAc024, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0560usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_25(self) -> crate::common::Reg<regs::HuffencAc025, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0564usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_26(self) -> crate::common::Reg<regs::HuffencAc026, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0568usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_27(self) -> crate::common::Reg<regs::HuffencAc027, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x056cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_28(self) -> crate::common::Reg<regs::HuffencAc028, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0570usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_29(self) -> crate::common::Reg<regs::HuffencAc029, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0574usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_30(self) -> crate::common::Reg<regs::HuffencAc030, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0578usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_31(self) -> crate::common::Reg<regs::HuffencAc031, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x057cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_32(self) -> crate::common::Reg<regs::HuffencAc032, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0580usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_33(self) -> crate::common::Reg<regs::HuffencAc033, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0584usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_34(self) -> crate::common::Reg<regs::HuffencAc034, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0588usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_35(self) -> crate::common::Reg<regs::HuffencAc035, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x058cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_36(self) -> crate::common::Reg<regs::HuffencAc036, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0590usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_37(self) -> crate::common::Reg<regs::HuffencAc037, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0594usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_38(self) -> crate::common::Reg<regs::HuffencAc038, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0598usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_39(self) -> crate::common::Reg<regs::HuffencAc039, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x059cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_40(self) -> crate::common::Reg<regs::HuffencAc040, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_41(self) -> crate::common::Reg<regs::HuffencAc041, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_42(self) -> crate::common::Reg<regs::HuffencAc042, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05a8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_43(self) -> crate::common::Reg<regs::HuffencAc043, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05acusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_44(self) -> crate::common::Reg<regs::HuffencAc044, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_45(self) -> crate::common::Reg<regs::HuffencAc045, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_46(self) -> crate::common::Reg<regs::HuffencAc046, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05b8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_47(self) -> crate::common::Reg<regs::HuffencAc047, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05bcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_48(self) -> crate::common::Reg<regs::HuffencAc048, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_49(self) -> crate::common::Reg<regs::HuffencAc049, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_50(self) -> crate::common::Reg<regs::HuffencAc050, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05c8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_51(self) -> crate::common::Reg<regs::HuffencAc051, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05ccusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_52(self) -> crate::common::Reg<regs::HuffencAc052, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_53(self) -> crate::common::Reg<regs::HuffencAc053, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_54(self) -> crate::common::Reg<regs::HuffencAc054, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05d8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_55(self) -> crate::common::Reg<regs::HuffencAc055, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05dcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_56(self) -> crate::common::Reg<regs::HuffencAc056, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_57(self) -> crate::common::Reg<regs::HuffencAc057, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_58(self) -> crate::common::Reg<regs::HuffencAc058, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05e8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_59(self) -> crate::common::Reg<regs::HuffencAc059, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05ecusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_60(self) -> crate::common::Reg<regs::HuffencAc060, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_61(self) -> crate::common::Reg<regs::HuffencAc061, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_62(self) -> crate::common::Reg<regs::HuffencAc062, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05f8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_63(self) -> crate::common::Reg<regs::HuffencAc063, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x05fcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_64(self) -> crate::common::Reg<regs::HuffencAc064, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0600usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_65(self) -> crate::common::Reg<regs::HuffencAc065, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0604usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_66(self) -> crate::common::Reg<regs::HuffencAc066, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0608usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_67(self) -> crate::common::Reg<regs::HuffencAc067, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x060cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_68(self) -> crate::common::Reg<regs::HuffencAc068, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0610usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_69(self) -> crate::common::Reg<regs::HuffencAc069, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0614usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_70(self) -> crate::common::Reg<regs::HuffencAc070, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0618usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_71(self) -> crate::common::Reg<regs::HuffencAc071, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x061cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_72(self) -> crate::common::Reg<regs::HuffencAc072, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0620usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_73(self) -> crate::common::Reg<regs::HuffencAc073, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0624usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_74(self) -> crate::common::Reg<regs::HuffencAc074, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0628usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_75(self) -> crate::common::Reg<regs::HuffencAc075, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x062cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_76(self) -> crate::common::Reg<regs::HuffencAc076, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0630usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_77(self) -> crate::common::Reg<regs::HuffencAc077, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0634usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_78(self) -> crate::common::Reg<regs::HuffencAc078, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0638usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_79(self) -> crate::common::Reg<regs::HuffencAc079, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x063cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_80(self) -> crate::common::Reg<regs::HuffencAc080, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0640usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_81(self) -> crate::common::Reg<regs::HuffencAc081, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0644usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_82(self) -> crate::common::Reg<regs::HuffencAc082, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0648usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_83(self) -> crate::common::Reg<regs::HuffencAc083, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x064cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_84(self) -> crate::common::Reg<regs::HuffencAc084, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0650usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_85(self) -> crate::common::Reg<regs::HuffencAc085, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0654usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_86(self) -> crate::common::Reg<regs::HuffencAc086, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0658usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_ac0_87(self) -> crate::common::Reg<regs::HuffencAc087, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x065cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_0(self) -> crate::common::Reg<regs::HuffencAc10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0660usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_1(self) -> crate::common::Reg<regs::HuffencAc11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0664usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_2(self) -> crate::common::Reg<regs::HuffencAc12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0668usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_3(self) -> crate::common::Reg<regs::HuffencAc13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x066cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_4(self) -> crate::common::Reg<regs::HuffencAc14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0670usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_5(self) -> crate::common::Reg<regs::HuffencAc15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0674usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_6(self) -> crate::common::Reg<regs::HuffencAc16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0678usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_7(self) -> crate::common::Reg<regs::HuffencAc17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x067cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_8(self) -> crate::common::Reg<regs::HuffencAc18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0680usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_9(self) -> crate::common::Reg<regs::HuffencAc19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0684usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_10(self) -> crate::common::Reg<regs::HuffencAc110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0688usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_11(self) -> crate::common::Reg<regs::HuffencAc111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x068cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_12(self) -> crate::common::Reg<regs::HuffencAc112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0690usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_13(self) -> crate::common::Reg<regs::HuffencAc113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0694usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_14(self) -> crate::common::Reg<regs::HuffencAc114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0698usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_15(self) -> crate::common::Reg<regs::HuffencAc115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x069cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_16(self) -> crate::common::Reg<regs::HuffencAc116, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_17(self) -> crate::common::Reg<regs::HuffencAc117, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_18(self) -> crate::common::Reg<regs::HuffencAc118, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06a8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_19(self) -> crate::common::Reg<regs::HuffencAc119, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06acusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_20(self) -> crate::common::Reg<regs::HuffencAc120, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_21(self) -> crate::common::Reg<regs::HuffencAc121, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_22(self) -> crate::common::Reg<regs::HuffencAc122, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06b8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_23(self) -> crate::common::Reg<regs::HuffencAc123, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06bcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_24(self) -> crate::common::Reg<regs::HuffencAc124, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_25(self) -> crate::common::Reg<regs::HuffencAc125, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_26(self) -> crate::common::Reg<regs::HuffencAc126, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06c8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_27(self) -> crate::common::Reg<regs::HuffencAc127, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06ccusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_28(self) -> crate::common::Reg<regs::HuffencAc128, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_29(self) -> crate::common::Reg<regs::HuffencAc129, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_30(self) -> crate::common::Reg<regs::HuffencAc130, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06d8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_31(self) -> crate::common::Reg<regs::HuffencAc131, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06dcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_32(self) -> crate::common::Reg<regs::HuffencAc132, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_33(self) -> crate::common::Reg<regs::HuffencAc133, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_34(self) -> crate::common::Reg<regs::HuffencAc134, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06e8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_35(self) -> crate::common::Reg<regs::HuffencAc135, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06ecusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_36(self) -> crate::common::Reg<regs::HuffencAc136, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_37(self) -> crate::common::Reg<regs::HuffencAc137, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_38(self) -> crate::common::Reg<regs::HuffencAc138, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06f8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_39(self) -> crate::common::Reg<regs::HuffencAc139, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x06fcusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_40(self) -> crate::common::Reg<regs::HuffencAc140, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0700usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_41(self) -> crate::common::Reg<regs::HuffencAc141, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0704usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_42(self) -> crate::common::Reg<regs::HuffencAc142, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0708usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_43(self) -> crate::common::Reg<regs::HuffencAc143, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x070cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_44(self) -> crate::common::Reg<regs::HuffencAc144, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0710usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_45(self) -> crate::common::Reg<regs::HuffencAc145, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0714usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_46(self) -> crate::common::Reg<regs::HuffencAc146, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0718usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_47(self) -> crate::common::Reg<regs::HuffencAc147, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x071cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_48(self) -> crate::common::Reg<regs::HuffencAc148, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0720usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_49(self) -> crate::common::Reg<regs::HuffencAc149, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0724usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_50(self) -> crate::common::Reg<regs::HuffencAc150, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0728usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_51(self) -> crate::common::Reg<regs::HuffencAc151, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x072cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_52(self) -> crate::common::Reg<regs::HuffencAc152, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0730usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_53(self) -> crate::common::Reg<regs::HuffencAc153, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0734usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_54(self) -> crate::common::Reg<regs::HuffencAc154, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0738usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_55(self) -> crate::common::Reg<regs::HuffencAc155, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x073cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_56(self) -> crate::common::Reg<regs::HuffencAc156, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0740usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_57(self) -> crate::common::Reg<regs::HuffencAc157, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0744usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_58(self) -> crate::common::Reg<regs::HuffencAc158, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0748usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_59(self) -> crate::common::Reg<regs::HuffencAc159, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x074cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_60(self) -> crate::common::Reg<regs::HuffencAc160, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0750usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_61(self) -> crate::common::Reg<regs::HuffencAc161, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0754usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_62(self) -> crate::common::Reg<regs::HuffencAc162, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0758usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_63(self) -> crate::common::Reg<regs::HuffencAc163, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x075cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_64(self) -> crate::common::Reg<regs::HuffencAc164, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0760usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_65(self) -> crate::common::Reg<regs::HuffencAc165, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0764usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_66(self) -> crate::common::Reg<regs::HuffencAc166, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0768usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_67(self) -> crate::common::Reg<regs::HuffencAc167, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x076cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_68(self) -> crate::common::Reg<regs::HuffencAc168, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0770usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_69(self) -> crate::common::Reg<regs::HuffencAc169, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0774usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_70(self) -> crate::common::Reg<regs::HuffencAc170, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0778usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_71(self) -> crate::common::Reg<regs::HuffencAc171, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x077cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_72(self) -> crate::common::Reg<regs::HuffencAc172, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0780usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_73(self) -> crate::common::Reg<regs::HuffencAc173, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0784usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_74(self) -> crate::common::Reg<regs::HuffencAc174, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0788usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_75(self) -> crate::common::Reg<regs::HuffencAc175, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x078cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_76(self) -> crate::common::Reg<regs::HuffencAc176, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0790usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_77(self) -> crate::common::Reg<regs::HuffencAc177, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0794usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_78(self) -> crate::common::Reg<regs::HuffencAc178, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0798usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_79(self) -> crate::common::Reg<regs::HuffencAc179, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x079cusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_80(self) -> crate::common::Reg<regs::HuffencAc180, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_81(self) -> crate::common::Reg<regs::HuffencAc181, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_82(self) -> crate::common::Reg<regs::HuffencAc182, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07a8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_83(self) -> crate::common::Reg<regs::HuffencAc183, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07acusize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_84(self) -> crate::common::Reg<regs::HuffencAc184, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07b0usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_85(self) -> crate::common::Reg<regs::HuffencAc185, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07b4usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_86(self) -> crate::common::Reg<regs::HuffencAc186, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07b8usize) as _) }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_ac1_87(self) -> crate::common::Reg<regs::HuffencAc187, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07bcusize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_0(self) -> crate::common::Reg<regs::HuffencDc00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07c0usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_1(self) -> crate::common::Reg<regs::HuffencDc01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07c4usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_2(self) -> crate::common::Reg<regs::HuffencDc02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07c8usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_3(self) -> crate::common::Reg<regs::HuffencDc03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07ccusize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_4(self) -> crate::common::Reg<regs::HuffencDc04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07d0usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_5(self) -> crate::common::Reg<regs::HuffencDc05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07d4usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_6(self) -> crate::common::Reg<regs::HuffencDc06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07d8usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[inline(always)]
    pub const fn huffenc_dc0_7(self) -> crate::common::Reg<regs::HuffencDc07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07dcusize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_0(self) -> crate::common::Reg<regs::HuffencDc10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07e0usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_1(self) -> crate::common::Reg<regs::HuffencDc11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07e4usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_2(self) -> crate::common::Reg<regs::HuffencDc12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07e8usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_3(self) -> crate::common::Reg<regs::HuffencDc13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07ecusize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_4(self) -> crate::common::Reg<regs::HuffencDc14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f0usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_5(self) -> crate::common::Reg<regs::HuffencDc15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f4usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_6(self) -> crate::common::Reg<regs::HuffencDc16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07f8usize) as _) }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[inline(always)]
    pub const fn huffenc_dc1_7(self) -> crate::common::Reg<regs::HuffencDc17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x07fcusize) as _) }
    }
}
pub mod regs {
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem0(pub u32);
    impl Dhtmem0 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem0 {
        #[inline(always)]
        fn default() -> Dhtmem0 {
            Dhtmem0(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem10(pub u32);
    impl Dhtmem10 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem10 {
        #[inline(always)]
        fn default() -> Dhtmem10 {
            Dhtmem10(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem100(pub u32);
    impl Dhtmem100 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem100 {
        #[inline(always)]
        fn default() -> Dhtmem100 {
            Dhtmem100(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem101(pub u32);
    impl Dhtmem101 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem101 {
        #[inline(always)]
        fn default() -> Dhtmem101 {
            Dhtmem101(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem102(pub u32);
    impl Dhtmem102 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem102 {
        #[inline(always)]
        fn default() -> Dhtmem102 {
            Dhtmem102(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem103(pub u32);
    impl Dhtmem103 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem103 {
        #[inline(always)]
        fn default() -> Dhtmem103 {
            Dhtmem103(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem11(pub u32);
    impl Dhtmem11 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem11 {
        #[inline(always)]
        fn default() -> Dhtmem11 {
            Dhtmem11(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem12(pub u32);
    impl Dhtmem12 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem12 {
        #[inline(always)]
        fn default() -> Dhtmem12 {
            Dhtmem12(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem13(pub u32);
    impl Dhtmem13 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem13 {
        #[inline(always)]
        fn default() -> Dhtmem13 {
            Dhtmem13(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem14(pub u32);
    impl Dhtmem14 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem14 {
        #[inline(always)]
        fn default() -> Dhtmem14 {
            Dhtmem14(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem15(pub u32);
    impl Dhtmem15 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem15 {
        #[inline(always)]
        fn default() -> Dhtmem15 {
            Dhtmem15(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem16(pub u32);
    impl Dhtmem16 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem16 {
        #[inline(always)]
        fn default() -> Dhtmem16 {
            Dhtmem16(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem17(pub u32);
    impl Dhtmem17 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem17 {
        #[inline(always)]
        fn default() -> Dhtmem17 {
            Dhtmem17(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem18(pub u32);
    impl Dhtmem18 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem18 {
        #[inline(always)]
        fn default() -> Dhtmem18 {
            Dhtmem18(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem19(pub u32);
    impl Dhtmem19 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem19 {
        #[inline(always)]
        fn default() -> Dhtmem19 {
            Dhtmem19(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem2(pub u32);
    impl Dhtmem2 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem2 {
        #[inline(always)]
        fn default() -> Dhtmem2 {
            Dhtmem2(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem20(pub u32);
    impl Dhtmem20 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem20 {
        #[inline(always)]
        fn default() -> Dhtmem20 {
            Dhtmem20(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem21(pub u32);
    impl Dhtmem21 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem21 {
        #[inline(always)]
        fn default() -> Dhtmem21 {
            Dhtmem21(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem22(pub u32);
    impl Dhtmem22 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem22 {
        #[inline(always)]
        fn default() -> Dhtmem22 {
            Dhtmem22(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem23(pub u32);
    impl Dhtmem23 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem23 {
        #[inline(always)]
        fn default() -> Dhtmem23 {
            Dhtmem23(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem24(pub u32);
    impl Dhtmem24 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem24 {
        #[inline(always)]
        fn default() -> Dhtmem24 {
            Dhtmem24(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem25(pub u32);
    impl Dhtmem25 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem25 {
        #[inline(always)]
        fn default() -> Dhtmem25 {
            Dhtmem25(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem26(pub u32);
    impl Dhtmem26 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem26 {
        #[inline(always)]
        fn default() -> Dhtmem26 {
            Dhtmem26(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem27(pub u32);
    impl Dhtmem27 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem27 {
        #[inline(always)]
        fn default() -> Dhtmem27 {
            Dhtmem27(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem28(pub u32);
    impl Dhtmem28 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem28 {
        #[inline(always)]
        fn default() -> Dhtmem28 {
            Dhtmem28(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem29(pub u32);
    impl Dhtmem29 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem29 {
        #[inline(always)]
        fn default() -> Dhtmem29 {
            Dhtmem29(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem3(pub u32);
    impl Dhtmem3 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem3 {
        #[inline(always)]
        fn default() -> Dhtmem3 {
            Dhtmem3(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem30(pub u32);
    impl Dhtmem30 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem30 {
        #[inline(always)]
        fn default() -> Dhtmem30 {
            Dhtmem30(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem31(pub u32);
    impl Dhtmem31 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem31 {
        #[inline(always)]
        fn default() -> Dhtmem31 {
            Dhtmem31(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem32(pub u32);
    impl Dhtmem32 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem32 {
        #[inline(always)]
        fn default() -> Dhtmem32 {
            Dhtmem32(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem33(pub u32);
    impl Dhtmem33 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem33 {
        #[inline(always)]
        fn default() -> Dhtmem33 {
            Dhtmem33(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem34(pub u32);
    impl Dhtmem34 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem34 {
        #[inline(always)]
        fn default() -> Dhtmem34 {
            Dhtmem34(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem35(pub u32);
    impl Dhtmem35 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem35 {
        #[inline(always)]
        fn default() -> Dhtmem35 {
            Dhtmem35(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem36(pub u32);
    impl Dhtmem36 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem36 {
        #[inline(always)]
        fn default() -> Dhtmem36 {
            Dhtmem36(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem37(pub u32);
    impl Dhtmem37 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem37 {
        #[inline(always)]
        fn default() -> Dhtmem37 {
            Dhtmem37(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem38(pub u32);
    impl Dhtmem38 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem38 {
        #[inline(always)]
        fn default() -> Dhtmem38 {
            Dhtmem38(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem39(pub u32);
    impl Dhtmem39 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem39 {
        #[inline(always)]
        fn default() -> Dhtmem39 {
            Dhtmem39(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem4(pub u32);
    impl Dhtmem4 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem4 {
        #[inline(always)]
        fn default() -> Dhtmem4 {
            Dhtmem4(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem40(pub u32);
    impl Dhtmem40 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem40 {
        #[inline(always)]
        fn default() -> Dhtmem40 {
            Dhtmem40(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem41(pub u32);
    impl Dhtmem41 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem41 {
        #[inline(always)]
        fn default() -> Dhtmem41 {
            Dhtmem41(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem42(pub u32);
    impl Dhtmem42 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem42 {
        #[inline(always)]
        fn default() -> Dhtmem42 {
            Dhtmem42(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem43(pub u32);
    impl Dhtmem43 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem43 {
        #[inline(always)]
        fn default() -> Dhtmem43 {
            Dhtmem43(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem44(pub u32);
    impl Dhtmem44 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem44 {
        #[inline(always)]
        fn default() -> Dhtmem44 {
            Dhtmem44(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem45(pub u32);
    impl Dhtmem45 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem45 {
        #[inline(always)]
        fn default() -> Dhtmem45 {
            Dhtmem45(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem46(pub u32);
    impl Dhtmem46 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem46 {
        #[inline(always)]
        fn default() -> Dhtmem46 {
            Dhtmem46(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem47(pub u32);
    impl Dhtmem47 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem47 {
        #[inline(always)]
        fn default() -> Dhtmem47 {
            Dhtmem47(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem48(pub u32);
    impl Dhtmem48 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem48 {
        #[inline(always)]
        fn default() -> Dhtmem48 {
            Dhtmem48(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem49(pub u32);
    impl Dhtmem49 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem49 {
        #[inline(always)]
        fn default() -> Dhtmem49 {
            Dhtmem49(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem5(pub u32);
    impl Dhtmem5 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem5 {
        #[inline(always)]
        fn default() -> Dhtmem5 {
            Dhtmem5(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem50(pub u32);
    impl Dhtmem50 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem50 {
        #[inline(always)]
        fn default() -> Dhtmem50 {
            Dhtmem50(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem51(pub u32);
    impl Dhtmem51 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem51 {
        #[inline(always)]
        fn default() -> Dhtmem51 {
            Dhtmem51(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem52(pub u32);
    impl Dhtmem52 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem52 {
        #[inline(always)]
        fn default() -> Dhtmem52 {
            Dhtmem52(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem53(pub u32);
    impl Dhtmem53 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem53 {
        #[inline(always)]
        fn default() -> Dhtmem53 {
            Dhtmem53(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem54(pub u32);
    impl Dhtmem54 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem54 {
        #[inline(always)]
        fn default() -> Dhtmem54 {
            Dhtmem54(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem55(pub u32);
    impl Dhtmem55 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem55 {
        #[inline(always)]
        fn default() -> Dhtmem55 {
            Dhtmem55(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem56(pub u32);
    impl Dhtmem56 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem56 {
        #[inline(always)]
        fn default() -> Dhtmem56 {
            Dhtmem56(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem57(pub u32);
    impl Dhtmem57 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem57 {
        #[inline(always)]
        fn default() -> Dhtmem57 {
            Dhtmem57(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem58(pub u32);
    impl Dhtmem58 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem58 {
        #[inline(always)]
        fn default() -> Dhtmem58 {
            Dhtmem58(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem59(pub u32);
    impl Dhtmem59 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem59 {
        #[inline(always)]
        fn default() -> Dhtmem59 {
            Dhtmem59(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem6(pub u32);
    impl Dhtmem6 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem6 {
        #[inline(always)]
        fn default() -> Dhtmem6 {
            Dhtmem6(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem60(pub u32);
    impl Dhtmem60 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem60 {
        #[inline(always)]
        fn default() -> Dhtmem60 {
            Dhtmem60(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem61(pub u32);
    impl Dhtmem61 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem61 {
        #[inline(always)]
        fn default() -> Dhtmem61 {
            Dhtmem61(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem62(pub u32);
    impl Dhtmem62 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem62 {
        #[inline(always)]
        fn default() -> Dhtmem62 {
            Dhtmem62(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem63(pub u32);
    impl Dhtmem63 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem63 {
        #[inline(always)]
        fn default() -> Dhtmem63 {
            Dhtmem63(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem64(pub u32);
    impl Dhtmem64 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem64 {
        #[inline(always)]
        fn default() -> Dhtmem64 {
            Dhtmem64(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem65(pub u32);
    impl Dhtmem65 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem65 {
        #[inline(always)]
        fn default() -> Dhtmem65 {
            Dhtmem65(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem66(pub u32);
    impl Dhtmem66 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem66 {
        #[inline(always)]
        fn default() -> Dhtmem66 {
            Dhtmem66(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem67(pub u32);
    impl Dhtmem67 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem67 {
        #[inline(always)]
        fn default() -> Dhtmem67 {
            Dhtmem67(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem68(pub u32);
    impl Dhtmem68 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem68 {
        #[inline(always)]
        fn default() -> Dhtmem68 {
            Dhtmem68(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem69(pub u32);
    impl Dhtmem69 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem69 {
        #[inline(always)]
        fn default() -> Dhtmem69 {
            Dhtmem69(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem7(pub u32);
    impl Dhtmem7 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem7 {
        #[inline(always)]
        fn default() -> Dhtmem7 {
            Dhtmem7(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem70(pub u32);
    impl Dhtmem70 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem70 {
        #[inline(always)]
        fn default() -> Dhtmem70 {
            Dhtmem70(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem71(pub u32);
    impl Dhtmem71 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem71 {
        #[inline(always)]
        fn default() -> Dhtmem71 {
            Dhtmem71(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem72(pub u32);
    impl Dhtmem72 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem72 {
        #[inline(always)]
        fn default() -> Dhtmem72 {
            Dhtmem72(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem73(pub u32);
    impl Dhtmem73 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem73 {
        #[inline(always)]
        fn default() -> Dhtmem73 {
            Dhtmem73(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem74(pub u32);
    impl Dhtmem74 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem74 {
        #[inline(always)]
        fn default() -> Dhtmem74 {
            Dhtmem74(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem75(pub u32);
    impl Dhtmem75 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem75 {
        #[inline(always)]
        fn default() -> Dhtmem75 {
            Dhtmem75(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem76(pub u32);
    impl Dhtmem76 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem76 {
        #[inline(always)]
        fn default() -> Dhtmem76 {
            Dhtmem76(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem77(pub u32);
    impl Dhtmem77 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem77 {
        #[inline(always)]
        fn default() -> Dhtmem77 {
            Dhtmem77(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem78(pub u32);
    impl Dhtmem78 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem78 {
        #[inline(always)]
        fn default() -> Dhtmem78 {
            Dhtmem78(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem79(pub u32);
    impl Dhtmem79 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem79 {
        #[inline(always)]
        fn default() -> Dhtmem79 {
            Dhtmem79(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem8(pub u32);
    impl Dhtmem8 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem8 {
        #[inline(always)]
        fn default() -> Dhtmem8 {
            Dhtmem8(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem80(pub u32);
    impl Dhtmem80 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem80 {
        #[inline(always)]
        fn default() -> Dhtmem80 {
            Dhtmem80(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem81(pub u32);
    impl Dhtmem81 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem81 {
        #[inline(always)]
        fn default() -> Dhtmem81 {
            Dhtmem81(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem82(pub u32);
    impl Dhtmem82 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem82 {
        #[inline(always)]
        fn default() -> Dhtmem82 {
            Dhtmem82(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem83(pub u32);
    impl Dhtmem83 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem83 {
        #[inline(always)]
        fn default() -> Dhtmem83 {
            Dhtmem83(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem84(pub u32);
    impl Dhtmem84 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem84 {
        #[inline(always)]
        fn default() -> Dhtmem84 {
            Dhtmem84(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem85(pub u32);
    impl Dhtmem85 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem85 {
        #[inline(always)]
        fn default() -> Dhtmem85 {
            Dhtmem85(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem86(pub u32);
    impl Dhtmem86 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem86 {
        #[inline(always)]
        fn default() -> Dhtmem86 {
            Dhtmem86(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem87(pub u32);
    impl Dhtmem87 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem87 {
        #[inline(always)]
        fn default() -> Dhtmem87 {
            Dhtmem87(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem88(pub u32);
    impl Dhtmem88 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem88 {
        #[inline(always)]
        fn default() -> Dhtmem88 {
            Dhtmem88(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem89(pub u32);
    impl Dhtmem89 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem89 {
        #[inline(always)]
        fn default() -> Dhtmem89 {
            Dhtmem89(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem9(pub u32);
    impl Dhtmem9 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem9 {
        #[inline(always)]
        fn default() -> Dhtmem9 {
            Dhtmem9(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem90(pub u32);
    impl Dhtmem90 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem90 {
        #[inline(always)]
        fn default() -> Dhtmem90 {
            Dhtmem90(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem91(pub u32);
    impl Dhtmem91 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem91 {
        #[inline(always)]
        fn default() -> Dhtmem91 {
            Dhtmem91(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem92(pub u32);
    impl Dhtmem92 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem92 {
        #[inline(always)]
        fn default() -> Dhtmem92 {
            Dhtmem92(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem93(pub u32);
    impl Dhtmem93 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem93 {
        #[inline(always)]
        fn default() -> Dhtmem93 {
            Dhtmem93(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem94(pub u32);
    impl Dhtmem94 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem94 {
        #[inline(always)]
        fn default() -> Dhtmem94 {
            Dhtmem94(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem95(pub u32);
    impl Dhtmem95 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem95 {
        #[inline(always)]
        fn default() -> Dhtmem95 {
            Dhtmem95(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem96(pub u32);
    impl Dhtmem96 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem96 {
        #[inline(always)]
        fn default() -> Dhtmem96 {
            Dhtmem96(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem97(pub u32);
    impl Dhtmem97 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem97 {
        #[inline(always)]
        fn default() -> Dhtmem97 {
            Dhtmem97(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem98(pub u32);
    impl Dhtmem98 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem98 {
        #[inline(always)]
        fn default() -> Dhtmem98 {
            Dhtmem98(0)
        }
    }
    #[doc = "JPEG DHTMem tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dhtmem99(pub u32);
    impl Dhtmem99 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Dhtmem99 {
        #[inline(always)]
        fn default() -> Dhtmem99 {
            Dhtmem99(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase0(pub u32);
    impl Huffbase0 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase0 {
        #[inline(always)]
        fn default() -> Huffbase0 {
            Huffbase0(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase1(pub u32);
    impl Huffbase1 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase1 {
        #[inline(always)]
        fn default() -> Huffbase1 {
            Huffbase1(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase10(pub u32);
    impl Huffbase10 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase10 {
        #[inline(always)]
        fn default() -> Huffbase10 {
            Huffbase10(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase11(pub u32);
    impl Huffbase11 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase11 {
        #[inline(always)]
        fn default() -> Huffbase11 {
            Huffbase11(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase12(pub u32);
    impl Huffbase12 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase12 {
        #[inline(always)]
        fn default() -> Huffbase12 {
            Huffbase12(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase13(pub u32);
    impl Huffbase13 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase13 {
        #[inline(always)]
        fn default() -> Huffbase13 {
            Huffbase13(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase14(pub u32);
    impl Huffbase14 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase14 {
        #[inline(always)]
        fn default() -> Huffbase14 {
            Huffbase14(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase15(pub u32);
    impl Huffbase15 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase15 {
        #[inline(always)]
        fn default() -> Huffbase15 {
            Huffbase15(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase16(pub u32);
    impl Huffbase16 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase16 {
        #[inline(always)]
        fn default() -> Huffbase16 {
            Huffbase16(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase17(pub u32);
    impl Huffbase17 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase17 {
        #[inline(always)]
        fn default() -> Huffbase17 {
            Huffbase17(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase18(pub u32);
    impl Huffbase18 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase18 {
        #[inline(always)]
        fn default() -> Huffbase18 {
            Huffbase18(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase19(pub u32);
    impl Huffbase19 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase19 {
        #[inline(always)]
        fn default() -> Huffbase19 {
            Huffbase19(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase2(pub u32);
    impl Huffbase2 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase2 {
        #[inline(always)]
        fn default() -> Huffbase2 {
            Huffbase2(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase20(pub u32);
    impl Huffbase20 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase20 {
        #[inline(always)]
        fn default() -> Huffbase20 {
            Huffbase20(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase21(pub u32);
    impl Huffbase21 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase21 {
        #[inline(always)]
        fn default() -> Huffbase21 {
            Huffbase21(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase22(pub u32);
    impl Huffbase22 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase22 {
        #[inline(always)]
        fn default() -> Huffbase22 {
            Huffbase22(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase23(pub u32);
    impl Huffbase23 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase23 {
        #[inline(always)]
        fn default() -> Huffbase23 {
            Huffbase23(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase24(pub u32);
    impl Huffbase24 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase24 {
        #[inline(always)]
        fn default() -> Huffbase24 {
            Huffbase24(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase25(pub u32);
    impl Huffbase25 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase25 {
        #[inline(always)]
        fn default() -> Huffbase25 {
            Huffbase25(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase26(pub u32);
    impl Huffbase26 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase26 {
        #[inline(always)]
        fn default() -> Huffbase26 {
            Huffbase26(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase27(pub u32);
    impl Huffbase27 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase27 {
        #[inline(always)]
        fn default() -> Huffbase27 {
            Huffbase27(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase28(pub u32);
    impl Huffbase28 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase28 {
        #[inline(always)]
        fn default() -> Huffbase28 {
            Huffbase28(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase29(pub u32);
    impl Huffbase29 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase29 {
        #[inline(always)]
        fn default() -> Huffbase29 {
            Huffbase29(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase3(pub u32);
    impl Huffbase3 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase3 {
        #[inline(always)]
        fn default() -> Huffbase3 {
            Huffbase3(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase30(pub u32);
    impl Huffbase30 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase30 {
        #[inline(always)]
        fn default() -> Huffbase30 {
            Huffbase30(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase31(pub u32);
    impl Huffbase31 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase31 {
        #[inline(always)]
        fn default() -> Huffbase31 {
            Huffbase31(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase4(pub u32);
    impl Huffbase4 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase4 {
        #[inline(always)]
        fn default() -> Huffbase4 {
            Huffbase4(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase5(pub u32);
    impl Huffbase5 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase5 {
        #[inline(always)]
        fn default() -> Huffbase5 {
            Huffbase5(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase6(pub u32);
    impl Huffbase6 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase6 {
        #[inline(always)]
        fn default() -> Huffbase6 {
            Huffbase6(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase7(pub u32);
    impl Huffbase7 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase7 {
        #[inline(always)]
        fn default() -> Huffbase7 {
            Huffbase7(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase8(pub u32);
    impl Huffbase8 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase8 {
        #[inline(always)]
        fn default() -> Huffbase8 {
            Huffbase8(0)
        }
    }
    #[doc = "JPEG HuffSymb tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffbase9(pub u32);
    impl Huffbase9 {
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_0(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_0(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub const fn huff_base_ram_1(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x01ff;
            val as u16
        }
        #[doc = "HuffBase RAM"]
        #[inline(always)]
        pub fn set_huff_base_ram_1(&mut self, val: u16) {
            self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
        }
    }
    impl Default for Huffbase9 {
        #[inline(always)]
        fn default() -> Huffbase9 {
            Huffbase9(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc00(pub u32);
    impl HuffencAc00 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc00 {
        #[inline(always)]
        fn default() -> HuffencAc00 {
            HuffencAc00(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc01(pub u32);
    impl HuffencAc01 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc01 {
        #[inline(always)]
        fn default() -> HuffencAc01 {
            HuffencAc01(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc010(pub u32);
    impl HuffencAc010 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc010 {
        #[inline(always)]
        fn default() -> HuffencAc010 {
            HuffencAc010(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc011(pub u32);
    impl HuffencAc011 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc011 {
        #[inline(always)]
        fn default() -> HuffencAc011 {
            HuffencAc011(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc012(pub u32);
    impl HuffencAc012 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc012 {
        #[inline(always)]
        fn default() -> HuffencAc012 {
            HuffencAc012(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc013(pub u32);
    impl HuffencAc013 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc013 {
        #[inline(always)]
        fn default() -> HuffencAc013 {
            HuffencAc013(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc014(pub u32);
    impl HuffencAc014 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc014 {
        #[inline(always)]
        fn default() -> HuffencAc014 {
            HuffencAc014(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc015(pub u32);
    impl HuffencAc015 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc015 {
        #[inline(always)]
        fn default() -> HuffencAc015 {
            HuffencAc015(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc016(pub u32);
    impl HuffencAc016 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc016 {
        #[inline(always)]
        fn default() -> HuffencAc016 {
            HuffencAc016(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc017(pub u32);
    impl HuffencAc017 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc017 {
        #[inline(always)]
        fn default() -> HuffencAc017 {
            HuffencAc017(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc018(pub u32);
    impl HuffencAc018 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc018 {
        #[inline(always)]
        fn default() -> HuffencAc018 {
            HuffencAc018(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc019(pub u32);
    impl HuffencAc019 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc019 {
        #[inline(always)]
        fn default() -> HuffencAc019 {
            HuffencAc019(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc02(pub u32);
    impl HuffencAc02 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc02 {
        #[inline(always)]
        fn default() -> HuffencAc02 {
            HuffencAc02(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc020(pub u32);
    impl HuffencAc020 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc020 {
        #[inline(always)]
        fn default() -> HuffencAc020 {
            HuffencAc020(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc021(pub u32);
    impl HuffencAc021 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc021 {
        #[inline(always)]
        fn default() -> HuffencAc021 {
            HuffencAc021(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc022(pub u32);
    impl HuffencAc022 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc022 {
        #[inline(always)]
        fn default() -> HuffencAc022 {
            HuffencAc022(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc023(pub u32);
    impl HuffencAc023 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc023 {
        #[inline(always)]
        fn default() -> HuffencAc023 {
            HuffencAc023(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc024(pub u32);
    impl HuffencAc024 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc024 {
        #[inline(always)]
        fn default() -> HuffencAc024 {
            HuffencAc024(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc025(pub u32);
    impl HuffencAc025 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc025 {
        #[inline(always)]
        fn default() -> HuffencAc025 {
            HuffencAc025(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc026(pub u32);
    impl HuffencAc026 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc026 {
        #[inline(always)]
        fn default() -> HuffencAc026 {
            HuffencAc026(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc027(pub u32);
    impl HuffencAc027 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc027 {
        #[inline(always)]
        fn default() -> HuffencAc027 {
            HuffencAc027(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc028(pub u32);
    impl HuffencAc028 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc028 {
        #[inline(always)]
        fn default() -> HuffencAc028 {
            HuffencAc028(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc029(pub u32);
    impl HuffencAc029 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc029 {
        #[inline(always)]
        fn default() -> HuffencAc029 {
            HuffencAc029(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc03(pub u32);
    impl HuffencAc03 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc03 {
        #[inline(always)]
        fn default() -> HuffencAc03 {
            HuffencAc03(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc030(pub u32);
    impl HuffencAc030 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc030 {
        #[inline(always)]
        fn default() -> HuffencAc030 {
            HuffencAc030(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc031(pub u32);
    impl HuffencAc031 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc031 {
        #[inline(always)]
        fn default() -> HuffencAc031 {
            HuffencAc031(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc032(pub u32);
    impl HuffencAc032 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc032 {
        #[inline(always)]
        fn default() -> HuffencAc032 {
            HuffencAc032(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc033(pub u32);
    impl HuffencAc033 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc033 {
        #[inline(always)]
        fn default() -> HuffencAc033 {
            HuffencAc033(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc034(pub u32);
    impl HuffencAc034 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc034 {
        #[inline(always)]
        fn default() -> HuffencAc034 {
            HuffencAc034(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc035(pub u32);
    impl HuffencAc035 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc035 {
        #[inline(always)]
        fn default() -> HuffencAc035 {
            HuffencAc035(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc036(pub u32);
    impl HuffencAc036 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc036 {
        #[inline(always)]
        fn default() -> HuffencAc036 {
            HuffencAc036(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc037(pub u32);
    impl HuffencAc037 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc037 {
        #[inline(always)]
        fn default() -> HuffencAc037 {
            HuffencAc037(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc038(pub u32);
    impl HuffencAc038 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc038 {
        #[inline(always)]
        fn default() -> HuffencAc038 {
            HuffencAc038(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc039(pub u32);
    impl HuffencAc039 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc039 {
        #[inline(always)]
        fn default() -> HuffencAc039 {
            HuffencAc039(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc04(pub u32);
    impl HuffencAc04 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc04 {
        #[inline(always)]
        fn default() -> HuffencAc04 {
            HuffencAc04(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc040(pub u32);
    impl HuffencAc040 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc040 {
        #[inline(always)]
        fn default() -> HuffencAc040 {
            HuffencAc040(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc041(pub u32);
    impl HuffencAc041 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc041 {
        #[inline(always)]
        fn default() -> HuffencAc041 {
            HuffencAc041(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc042(pub u32);
    impl HuffencAc042 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc042 {
        #[inline(always)]
        fn default() -> HuffencAc042 {
            HuffencAc042(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc043(pub u32);
    impl HuffencAc043 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc043 {
        #[inline(always)]
        fn default() -> HuffencAc043 {
            HuffencAc043(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc044(pub u32);
    impl HuffencAc044 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc044 {
        #[inline(always)]
        fn default() -> HuffencAc044 {
            HuffencAc044(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc045(pub u32);
    impl HuffencAc045 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc045 {
        #[inline(always)]
        fn default() -> HuffencAc045 {
            HuffencAc045(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc046(pub u32);
    impl HuffencAc046 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc046 {
        #[inline(always)]
        fn default() -> HuffencAc046 {
            HuffencAc046(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc047(pub u32);
    impl HuffencAc047 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc047 {
        #[inline(always)]
        fn default() -> HuffencAc047 {
            HuffencAc047(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc048(pub u32);
    impl HuffencAc048 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc048 {
        #[inline(always)]
        fn default() -> HuffencAc048 {
            HuffencAc048(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc049(pub u32);
    impl HuffencAc049 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc049 {
        #[inline(always)]
        fn default() -> HuffencAc049 {
            HuffencAc049(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc05(pub u32);
    impl HuffencAc05 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc05 {
        #[inline(always)]
        fn default() -> HuffencAc05 {
            HuffencAc05(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc050(pub u32);
    impl HuffencAc050 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc050 {
        #[inline(always)]
        fn default() -> HuffencAc050 {
            HuffencAc050(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc051(pub u32);
    impl HuffencAc051 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc051 {
        #[inline(always)]
        fn default() -> HuffencAc051 {
            HuffencAc051(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc052(pub u32);
    impl HuffencAc052 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc052 {
        #[inline(always)]
        fn default() -> HuffencAc052 {
            HuffencAc052(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc053(pub u32);
    impl HuffencAc053 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc053 {
        #[inline(always)]
        fn default() -> HuffencAc053 {
            HuffencAc053(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc054(pub u32);
    impl HuffencAc054 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc054 {
        #[inline(always)]
        fn default() -> HuffencAc054 {
            HuffencAc054(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc055(pub u32);
    impl HuffencAc055 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc055 {
        #[inline(always)]
        fn default() -> HuffencAc055 {
            HuffencAc055(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc056(pub u32);
    impl HuffencAc056 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc056 {
        #[inline(always)]
        fn default() -> HuffencAc056 {
            HuffencAc056(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc057(pub u32);
    impl HuffencAc057 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc057 {
        #[inline(always)]
        fn default() -> HuffencAc057 {
            HuffencAc057(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc058(pub u32);
    impl HuffencAc058 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc058 {
        #[inline(always)]
        fn default() -> HuffencAc058 {
            HuffencAc058(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc059(pub u32);
    impl HuffencAc059 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc059 {
        #[inline(always)]
        fn default() -> HuffencAc059 {
            HuffencAc059(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc06(pub u32);
    impl HuffencAc06 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc06 {
        #[inline(always)]
        fn default() -> HuffencAc06 {
            HuffencAc06(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc060(pub u32);
    impl HuffencAc060 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc060 {
        #[inline(always)]
        fn default() -> HuffencAc060 {
            HuffencAc060(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc061(pub u32);
    impl HuffencAc061 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc061 {
        #[inline(always)]
        fn default() -> HuffencAc061 {
            HuffencAc061(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc062(pub u32);
    impl HuffencAc062 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc062 {
        #[inline(always)]
        fn default() -> HuffencAc062 {
            HuffencAc062(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc063(pub u32);
    impl HuffencAc063 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc063 {
        #[inline(always)]
        fn default() -> HuffencAc063 {
            HuffencAc063(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc064(pub u32);
    impl HuffencAc064 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc064 {
        #[inline(always)]
        fn default() -> HuffencAc064 {
            HuffencAc064(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc065(pub u32);
    impl HuffencAc065 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc065 {
        #[inline(always)]
        fn default() -> HuffencAc065 {
            HuffencAc065(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc066(pub u32);
    impl HuffencAc066 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc066 {
        #[inline(always)]
        fn default() -> HuffencAc066 {
            HuffencAc066(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc067(pub u32);
    impl HuffencAc067 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc067 {
        #[inline(always)]
        fn default() -> HuffencAc067 {
            HuffencAc067(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc068(pub u32);
    impl HuffencAc068 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc068 {
        #[inline(always)]
        fn default() -> HuffencAc068 {
            HuffencAc068(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc069(pub u32);
    impl HuffencAc069 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc069 {
        #[inline(always)]
        fn default() -> HuffencAc069 {
            HuffencAc069(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc07(pub u32);
    impl HuffencAc07 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc07 {
        #[inline(always)]
        fn default() -> HuffencAc07 {
            HuffencAc07(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc070(pub u32);
    impl HuffencAc070 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc070 {
        #[inline(always)]
        fn default() -> HuffencAc070 {
            HuffencAc070(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc071(pub u32);
    impl HuffencAc071 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc071 {
        #[inline(always)]
        fn default() -> HuffencAc071 {
            HuffencAc071(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc072(pub u32);
    impl HuffencAc072 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc072 {
        #[inline(always)]
        fn default() -> HuffencAc072 {
            HuffencAc072(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc073(pub u32);
    impl HuffencAc073 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc073 {
        #[inline(always)]
        fn default() -> HuffencAc073 {
            HuffencAc073(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc074(pub u32);
    impl HuffencAc074 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc074 {
        #[inline(always)]
        fn default() -> HuffencAc074 {
            HuffencAc074(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc075(pub u32);
    impl HuffencAc075 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc075 {
        #[inline(always)]
        fn default() -> HuffencAc075 {
            HuffencAc075(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc076(pub u32);
    impl HuffencAc076 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc076 {
        #[inline(always)]
        fn default() -> HuffencAc076 {
            HuffencAc076(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc077(pub u32);
    impl HuffencAc077 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc077 {
        #[inline(always)]
        fn default() -> HuffencAc077 {
            HuffencAc077(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc078(pub u32);
    impl HuffencAc078 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc078 {
        #[inline(always)]
        fn default() -> HuffencAc078 {
            HuffencAc078(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc079(pub u32);
    impl HuffencAc079 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc079 {
        #[inline(always)]
        fn default() -> HuffencAc079 {
            HuffencAc079(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc08(pub u32);
    impl HuffencAc08 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc08 {
        #[inline(always)]
        fn default() -> HuffencAc08 {
            HuffencAc08(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc080(pub u32);
    impl HuffencAc080 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc080 {
        #[inline(always)]
        fn default() -> HuffencAc080 {
            HuffencAc080(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc081(pub u32);
    impl HuffencAc081 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc081 {
        #[inline(always)]
        fn default() -> HuffencAc081 {
            HuffencAc081(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc082(pub u32);
    impl HuffencAc082 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc082 {
        #[inline(always)]
        fn default() -> HuffencAc082 {
            HuffencAc082(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc083(pub u32);
    impl HuffencAc083 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc083 {
        #[inline(always)]
        fn default() -> HuffencAc083 {
            HuffencAc083(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc084(pub u32);
    impl HuffencAc084 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc084 {
        #[inline(always)]
        fn default() -> HuffencAc084 {
            HuffencAc084(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc085(pub u32);
    impl HuffencAc085 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc085 {
        #[inline(always)]
        fn default() -> HuffencAc085 {
            HuffencAc085(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc086(pub u32);
    impl HuffencAc086 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc086 {
        #[inline(always)]
        fn default() -> HuffencAc086 {
            HuffencAc086(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc087(pub u32);
    impl HuffencAc087 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc087 {
        #[inline(always)]
        fn default() -> HuffencAc087 {
            HuffencAc087(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc09(pub u32);
    impl HuffencAc09 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc09 {
        #[inline(always)]
        fn default() -> HuffencAc09 {
            HuffencAc09(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc10(pub u32);
    impl HuffencAc10 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc10 {
        #[inline(always)]
        fn default() -> HuffencAc10 {
            HuffencAc10(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc11(pub u32);
    impl HuffencAc11 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc11 {
        #[inline(always)]
        fn default() -> HuffencAc11 {
            HuffencAc11(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc110(pub u32);
    impl HuffencAc110 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc110 {
        #[inline(always)]
        fn default() -> HuffencAc110 {
            HuffencAc110(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc111(pub u32);
    impl HuffencAc111 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc111 {
        #[inline(always)]
        fn default() -> HuffencAc111 {
            HuffencAc111(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc112(pub u32);
    impl HuffencAc112 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc112 {
        #[inline(always)]
        fn default() -> HuffencAc112 {
            HuffencAc112(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc113(pub u32);
    impl HuffencAc113 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc113 {
        #[inline(always)]
        fn default() -> HuffencAc113 {
            HuffencAc113(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc114(pub u32);
    impl HuffencAc114 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc114 {
        #[inline(always)]
        fn default() -> HuffencAc114 {
            HuffencAc114(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc115(pub u32);
    impl HuffencAc115 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc115 {
        #[inline(always)]
        fn default() -> HuffencAc115 {
            HuffencAc115(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc116(pub u32);
    impl HuffencAc116 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc116 {
        #[inline(always)]
        fn default() -> HuffencAc116 {
            HuffencAc116(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc117(pub u32);
    impl HuffencAc117 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc117 {
        #[inline(always)]
        fn default() -> HuffencAc117 {
            HuffencAc117(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc118(pub u32);
    impl HuffencAc118 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc118 {
        #[inline(always)]
        fn default() -> HuffencAc118 {
            HuffencAc118(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc119(pub u32);
    impl HuffencAc119 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc119 {
        #[inline(always)]
        fn default() -> HuffencAc119 {
            HuffencAc119(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc12(pub u32);
    impl HuffencAc12 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc12 {
        #[inline(always)]
        fn default() -> HuffencAc12 {
            HuffencAc12(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc120(pub u32);
    impl HuffencAc120 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc120 {
        #[inline(always)]
        fn default() -> HuffencAc120 {
            HuffencAc120(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc121(pub u32);
    impl HuffencAc121 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc121 {
        #[inline(always)]
        fn default() -> HuffencAc121 {
            HuffencAc121(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc122(pub u32);
    impl HuffencAc122 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc122 {
        #[inline(always)]
        fn default() -> HuffencAc122 {
            HuffencAc122(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc123(pub u32);
    impl HuffencAc123 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc123 {
        #[inline(always)]
        fn default() -> HuffencAc123 {
            HuffencAc123(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc124(pub u32);
    impl HuffencAc124 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc124 {
        #[inline(always)]
        fn default() -> HuffencAc124 {
            HuffencAc124(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc125(pub u32);
    impl HuffencAc125 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc125 {
        #[inline(always)]
        fn default() -> HuffencAc125 {
            HuffencAc125(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc126(pub u32);
    impl HuffencAc126 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc126 {
        #[inline(always)]
        fn default() -> HuffencAc126 {
            HuffencAc126(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc127(pub u32);
    impl HuffencAc127 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc127 {
        #[inline(always)]
        fn default() -> HuffencAc127 {
            HuffencAc127(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc128(pub u32);
    impl HuffencAc128 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc128 {
        #[inline(always)]
        fn default() -> HuffencAc128 {
            HuffencAc128(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc129(pub u32);
    impl HuffencAc129 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc129 {
        #[inline(always)]
        fn default() -> HuffencAc129 {
            HuffencAc129(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc13(pub u32);
    impl HuffencAc13 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc13 {
        #[inline(always)]
        fn default() -> HuffencAc13 {
            HuffencAc13(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc130(pub u32);
    impl HuffencAc130 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc130 {
        #[inline(always)]
        fn default() -> HuffencAc130 {
            HuffencAc130(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc131(pub u32);
    impl HuffencAc131 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc131 {
        #[inline(always)]
        fn default() -> HuffencAc131 {
            HuffencAc131(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc132(pub u32);
    impl HuffencAc132 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc132 {
        #[inline(always)]
        fn default() -> HuffencAc132 {
            HuffencAc132(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc133(pub u32);
    impl HuffencAc133 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc133 {
        #[inline(always)]
        fn default() -> HuffencAc133 {
            HuffencAc133(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc134(pub u32);
    impl HuffencAc134 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc134 {
        #[inline(always)]
        fn default() -> HuffencAc134 {
            HuffencAc134(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc135(pub u32);
    impl HuffencAc135 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc135 {
        #[inline(always)]
        fn default() -> HuffencAc135 {
            HuffencAc135(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc136(pub u32);
    impl HuffencAc136 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc136 {
        #[inline(always)]
        fn default() -> HuffencAc136 {
            HuffencAc136(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc137(pub u32);
    impl HuffencAc137 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc137 {
        #[inline(always)]
        fn default() -> HuffencAc137 {
            HuffencAc137(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc138(pub u32);
    impl HuffencAc138 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc138 {
        #[inline(always)]
        fn default() -> HuffencAc138 {
            HuffencAc138(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc139(pub u32);
    impl HuffencAc139 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc139 {
        #[inline(always)]
        fn default() -> HuffencAc139 {
            HuffencAc139(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc14(pub u32);
    impl HuffencAc14 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc14 {
        #[inline(always)]
        fn default() -> HuffencAc14 {
            HuffencAc14(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc140(pub u32);
    impl HuffencAc140 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc140 {
        #[inline(always)]
        fn default() -> HuffencAc140 {
            HuffencAc140(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc141(pub u32);
    impl HuffencAc141 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc141 {
        #[inline(always)]
        fn default() -> HuffencAc141 {
            HuffencAc141(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc142(pub u32);
    impl HuffencAc142 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc142 {
        #[inline(always)]
        fn default() -> HuffencAc142 {
            HuffencAc142(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc143(pub u32);
    impl HuffencAc143 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc143 {
        #[inline(always)]
        fn default() -> HuffencAc143 {
            HuffencAc143(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc144(pub u32);
    impl HuffencAc144 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc144 {
        #[inline(always)]
        fn default() -> HuffencAc144 {
            HuffencAc144(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc145(pub u32);
    impl HuffencAc145 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc145 {
        #[inline(always)]
        fn default() -> HuffencAc145 {
            HuffencAc145(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc146(pub u32);
    impl HuffencAc146 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc146 {
        #[inline(always)]
        fn default() -> HuffencAc146 {
            HuffencAc146(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc147(pub u32);
    impl HuffencAc147 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc147 {
        #[inline(always)]
        fn default() -> HuffencAc147 {
            HuffencAc147(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc148(pub u32);
    impl HuffencAc148 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc148 {
        #[inline(always)]
        fn default() -> HuffencAc148 {
            HuffencAc148(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc149(pub u32);
    impl HuffencAc149 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc149 {
        #[inline(always)]
        fn default() -> HuffencAc149 {
            HuffencAc149(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc15(pub u32);
    impl HuffencAc15 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc15 {
        #[inline(always)]
        fn default() -> HuffencAc15 {
            HuffencAc15(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc150(pub u32);
    impl HuffencAc150 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc150 {
        #[inline(always)]
        fn default() -> HuffencAc150 {
            HuffencAc150(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc151(pub u32);
    impl HuffencAc151 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc151 {
        #[inline(always)]
        fn default() -> HuffencAc151 {
            HuffencAc151(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc152(pub u32);
    impl HuffencAc152 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc152 {
        #[inline(always)]
        fn default() -> HuffencAc152 {
            HuffencAc152(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc153(pub u32);
    impl HuffencAc153 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc153 {
        #[inline(always)]
        fn default() -> HuffencAc153 {
            HuffencAc153(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc154(pub u32);
    impl HuffencAc154 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc154 {
        #[inline(always)]
        fn default() -> HuffencAc154 {
            HuffencAc154(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc155(pub u32);
    impl HuffencAc155 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc155 {
        #[inline(always)]
        fn default() -> HuffencAc155 {
            HuffencAc155(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc156(pub u32);
    impl HuffencAc156 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc156 {
        #[inline(always)]
        fn default() -> HuffencAc156 {
            HuffencAc156(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc157(pub u32);
    impl HuffencAc157 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc157 {
        #[inline(always)]
        fn default() -> HuffencAc157 {
            HuffencAc157(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc158(pub u32);
    impl HuffencAc158 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc158 {
        #[inline(always)]
        fn default() -> HuffencAc158 {
            HuffencAc158(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc159(pub u32);
    impl HuffencAc159 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc159 {
        #[inline(always)]
        fn default() -> HuffencAc159 {
            HuffencAc159(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc16(pub u32);
    impl HuffencAc16 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc16 {
        #[inline(always)]
        fn default() -> HuffencAc16 {
            HuffencAc16(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc160(pub u32);
    impl HuffencAc160 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc160 {
        #[inline(always)]
        fn default() -> HuffencAc160 {
            HuffencAc160(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc161(pub u32);
    impl HuffencAc161 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc161 {
        #[inline(always)]
        fn default() -> HuffencAc161 {
            HuffencAc161(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc162(pub u32);
    impl HuffencAc162 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc162 {
        #[inline(always)]
        fn default() -> HuffencAc162 {
            HuffencAc162(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc163(pub u32);
    impl HuffencAc163 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc163 {
        #[inline(always)]
        fn default() -> HuffencAc163 {
            HuffencAc163(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc164(pub u32);
    impl HuffencAc164 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc164 {
        #[inline(always)]
        fn default() -> HuffencAc164 {
            HuffencAc164(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc165(pub u32);
    impl HuffencAc165 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc165 {
        #[inline(always)]
        fn default() -> HuffencAc165 {
            HuffencAc165(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc166(pub u32);
    impl HuffencAc166 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc166 {
        #[inline(always)]
        fn default() -> HuffencAc166 {
            HuffencAc166(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc167(pub u32);
    impl HuffencAc167 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc167 {
        #[inline(always)]
        fn default() -> HuffencAc167 {
            HuffencAc167(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc168(pub u32);
    impl HuffencAc168 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc168 {
        #[inline(always)]
        fn default() -> HuffencAc168 {
            HuffencAc168(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc169(pub u32);
    impl HuffencAc169 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc169 {
        #[inline(always)]
        fn default() -> HuffencAc169 {
            HuffencAc169(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc17(pub u32);
    impl HuffencAc17 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc17 {
        #[inline(always)]
        fn default() -> HuffencAc17 {
            HuffencAc17(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc170(pub u32);
    impl HuffencAc170 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc170 {
        #[inline(always)]
        fn default() -> HuffencAc170 {
            HuffencAc170(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc171(pub u32);
    impl HuffencAc171 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc171 {
        #[inline(always)]
        fn default() -> HuffencAc171 {
            HuffencAc171(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc172(pub u32);
    impl HuffencAc172 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc172 {
        #[inline(always)]
        fn default() -> HuffencAc172 {
            HuffencAc172(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc173(pub u32);
    impl HuffencAc173 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc173 {
        #[inline(always)]
        fn default() -> HuffencAc173 {
            HuffencAc173(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc174(pub u32);
    impl HuffencAc174 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc174 {
        #[inline(always)]
        fn default() -> HuffencAc174 {
            HuffencAc174(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc175(pub u32);
    impl HuffencAc175 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc175 {
        #[inline(always)]
        fn default() -> HuffencAc175 {
            HuffencAc175(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc176(pub u32);
    impl HuffencAc176 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc176 {
        #[inline(always)]
        fn default() -> HuffencAc176 {
            HuffencAc176(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc177(pub u32);
    impl HuffencAc177 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc177 {
        #[inline(always)]
        fn default() -> HuffencAc177 {
            HuffencAc177(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc178(pub u32);
    impl HuffencAc178 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc178 {
        #[inline(always)]
        fn default() -> HuffencAc178 {
            HuffencAc178(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc179(pub u32);
    impl HuffencAc179 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc179 {
        #[inline(always)]
        fn default() -> HuffencAc179 {
            HuffencAc179(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc18(pub u32);
    impl HuffencAc18 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc18 {
        #[inline(always)]
        fn default() -> HuffencAc18 {
            HuffencAc18(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc180(pub u32);
    impl HuffencAc180 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc180 {
        #[inline(always)]
        fn default() -> HuffencAc180 {
            HuffencAc180(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc181(pub u32);
    impl HuffencAc181 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc181 {
        #[inline(always)]
        fn default() -> HuffencAc181 {
            HuffencAc181(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc182(pub u32);
    impl HuffencAc182 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc182 {
        #[inline(always)]
        fn default() -> HuffencAc182 {
            HuffencAc182(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc183(pub u32);
    impl HuffencAc183 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc183 {
        #[inline(always)]
        fn default() -> HuffencAc183 {
            HuffencAc183(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc184(pub u32);
    impl HuffencAc184 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc184 {
        #[inline(always)]
        fn default() -> HuffencAc184 {
            HuffencAc184(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc185(pub u32);
    impl HuffencAc185 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc185 {
        #[inline(always)]
        fn default() -> HuffencAc185 {
            HuffencAc185(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc186(pub u32);
    impl HuffencAc186 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc186 {
        #[inline(always)]
        fn default() -> HuffencAc186 {
            HuffencAc186(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc187(pub u32);
    impl HuffencAc187 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc187 {
        #[inline(always)]
        fn default() -> HuffencAc187 {
            HuffencAc187(0)
        }
    }
    #[doc = "JPEG encoder, AC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencAc19(pub u32);
    impl HuffencAc19 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencAc19 {
        #[inline(always)]
        fn default() -> HuffencAc19 {
            HuffencAc19(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc00(pub u32);
    impl HuffencDc00 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc00 {
        #[inline(always)]
        fn default() -> HuffencDc00 {
            HuffencDc00(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc01(pub u32);
    impl HuffencDc01 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc01 {
        #[inline(always)]
        fn default() -> HuffencDc01 {
            HuffencDc01(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc02(pub u32);
    impl HuffencDc02 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc02 {
        #[inline(always)]
        fn default() -> HuffencDc02 {
            HuffencDc02(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc03(pub u32);
    impl HuffencDc03 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc03 {
        #[inline(always)]
        fn default() -> HuffencDc03 {
            HuffencDc03(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc04(pub u32);
    impl HuffencDc04 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc04 {
        #[inline(always)]
        fn default() -> HuffencDc04 {
            HuffencDc04(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc05(pub u32);
    impl HuffencDc05 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc05 {
        #[inline(always)]
        fn default() -> HuffencDc05 {
            HuffencDc05(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc06(pub u32);
    impl HuffencDc06 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc06 {
        #[inline(always)]
        fn default() -> HuffencDc06 {
            HuffencDc06(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc07(pub u32);
    impl HuffencDc07 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc07 {
        #[inline(always)]
        fn default() -> HuffencDc07 {
            HuffencDc07(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc10(pub u32);
    impl HuffencDc10 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc10 {
        #[inline(always)]
        fn default() -> HuffencDc10 {
            HuffencDc10(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc11(pub u32);
    impl HuffencDc11 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc11 {
        #[inline(always)]
        fn default() -> HuffencDc11 {
            HuffencDc11(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc12(pub u32);
    impl HuffencDc12 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc12 {
        #[inline(always)]
        fn default() -> HuffencDc12 {
            HuffencDc12(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc13(pub u32);
    impl HuffencDc13 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc13 {
        #[inline(always)]
        fn default() -> HuffencDc13 {
            HuffencDc13(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc14(pub u32);
    impl HuffencDc14 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc14 {
        #[inline(always)]
        fn default() -> HuffencDc14 {
            HuffencDc14(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc15(pub u32);
    impl HuffencDc15 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc15 {
        #[inline(always)]
        fn default() -> HuffencDc15 {
            HuffencDc15(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc16(pub u32);
    impl HuffencDc16 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc16 {
        #[inline(always)]
        fn default() -> HuffencDc16 {
            HuffencDc16(0)
        }
    }
    #[doc = "JPEG encoder, DC Huffman table 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct HuffencDc17(pub u32);
    impl HuffencDc17 {
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub const fn dhtmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTMem RAM"]
        #[inline(always)]
        pub fn set_dhtmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for HuffencDc17 {
        #[inline(always)]
        fn default() -> HuffencDc17 {
            HuffencDc17(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin0(pub u32);
    impl Huffmin0 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin0 {
        #[inline(always)]
        fn default() -> Huffmin0 {
            Huffmin0(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin1(pub u32);
    impl Huffmin1 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin1 {
        #[inline(always)]
        fn default() -> Huffmin1 {
            Huffmin1(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin10(pub u32);
    impl Huffmin10 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin10 {
        #[inline(always)]
        fn default() -> Huffmin10 {
            Huffmin10(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin11(pub u32);
    impl Huffmin11 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin11 {
        #[inline(always)]
        fn default() -> Huffmin11 {
            Huffmin11(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin12(pub u32);
    impl Huffmin12 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin12 {
        #[inline(always)]
        fn default() -> Huffmin12 {
            Huffmin12(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin13(pub u32);
    impl Huffmin13 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin13 {
        #[inline(always)]
        fn default() -> Huffmin13 {
            Huffmin13(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin14(pub u32);
    impl Huffmin14 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin14 {
        #[inline(always)]
        fn default() -> Huffmin14 {
            Huffmin14(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin15(pub u32);
    impl Huffmin15 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin15 {
        #[inline(always)]
        fn default() -> Huffmin15 {
            Huffmin15(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin2(pub u32);
    impl Huffmin2 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin2 {
        #[inline(always)]
        fn default() -> Huffmin2 {
            Huffmin2(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin3(pub u32);
    impl Huffmin3 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin3 {
        #[inline(always)]
        fn default() -> Huffmin3 {
            Huffmin3(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin4(pub u32);
    impl Huffmin4 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin4 {
        #[inline(always)]
        fn default() -> Huffmin4 {
            Huffmin4(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin5(pub u32);
    impl Huffmin5 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin5 {
        #[inline(always)]
        fn default() -> Huffmin5 {
            Huffmin5(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin6(pub u32);
    impl Huffmin6 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin6 {
        #[inline(always)]
        fn default() -> Huffmin6 {
            Huffmin6(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin7(pub u32);
    impl Huffmin7 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin7 {
        #[inline(always)]
        fn default() -> Huffmin7 {
            Huffmin7(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin8(pub u32);
    impl Huffmin8 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin8 {
        #[inline(always)]
        fn default() -> Huffmin8 {
            Huffmin8(0)
        }
    }
    #[doc = "JPEG HuffMin tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffmin9(pub u32);
    impl Huffmin9 {
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub const fn huff_min_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "HuffMin RAM"]
        #[inline(always)]
        pub fn set_huff_min_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffmin9 {
        #[inline(always)]
        fn default() -> Huffmin9 {
            Huffmin9(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb0(pub u32);
    impl Huffsymb0 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb0 {
        #[inline(always)]
        fn default() -> Huffsymb0 {
            Huffsymb0(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb1(pub u32);
    impl Huffsymb1 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb1 {
        #[inline(always)]
        fn default() -> Huffsymb1 {
            Huffsymb1(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb10(pub u32);
    impl Huffsymb10 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb10 {
        #[inline(always)]
        fn default() -> Huffsymb10 {
            Huffsymb10(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb11(pub u32);
    impl Huffsymb11 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb11 {
        #[inline(always)]
        fn default() -> Huffsymb11 {
            Huffsymb11(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb12(pub u32);
    impl Huffsymb12 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb12 {
        #[inline(always)]
        fn default() -> Huffsymb12 {
            Huffsymb12(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb13(pub u32);
    impl Huffsymb13 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb13 {
        #[inline(always)]
        fn default() -> Huffsymb13 {
            Huffsymb13(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb14(pub u32);
    impl Huffsymb14 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb14 {
        #[inline(always)]
        fn default() -> Huffsymb14 {
            Huffsymb14(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb15(pub u32);
    impl Huffsymb15 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb15 {
        #[inline(always)]
        fn default() -> Huffsymb15 {
            Huffsymb15(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb16(pub u32);
    impl Huffsymb16 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb16 {
        #[inline(always)]
        fn default() -> Huffsymb16 {
            Huffsymb16(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb17(pub u32);
    impl Huffsymb17 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb17 {
        #[inline(always)]
        fn default() -> Huffsymb17 {
            Huffsymb17(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb18(pub u32);
    impl Huffsymb18 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb18 {
        #[inline(always)]
        fn default() -> Huffsymb18 {
            Huffsymb18(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb19(pub u32);
    impl Huffsymb19 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb19 {
        #[inline(always)]
        fn default() -> Huffsymb19 {
            Huffsymb19(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb2(pub u32);
    impl Huffsymb2 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb2 {
        #[inline(always)]
        fn default() -> Huffsymb2 {
            Huffsymb2(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb20(pub u32);
    impl Huffsymb20 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb20 {
        #[inline(always)]
        fn default() -> Huffsymb20 {
            Huffsymb20(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb21(pub u32);
    impl Huffsymb21 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb21 {
        #[inline(always)]
        fn default() -> Huffsymb21 {
            Huffsymb21(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb22(pub u32);
    impl Huffsymb22 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb22 {
        #[inline(always)]
        fn default() -> Huffsymb22 {
            Huffsymb22(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb23(pub u32);
    impl Huffsymb23 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb23 {
        #[inline(always)]
        fn default() -> Huffsymb23 {
            Huffsymb23(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb24(pub u32);
    impl Huffsymb24 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb24 {
        #[inline(always)]
        fn default() -> Huffsymb24 {
            Huffsymb24(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb25(pub u32);
    impl Huffsymb25 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb25 {
        #[inline(always)]
        fn default() -> Huffsymb25 {
            Huffsymb25(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb26(pub u32);
    impl Huffsymb26 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb26 {
        #[inline(always)]
        fn default() -> Huffsymb26 {
            Huffsymb26(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb27(pub u32);
    impl Huffsymb27 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb27 {
        #[inline(always)]
        fn default() -> Huffsymb27 {
            Huffsymb27(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb28(pub u32);
    impl Huffsymb28 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb28 {
        #[inline(always)]
        fn default() -> Huffsymb28 {
            Huffsymb28(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb29(pub u32);
    impl Huffsymb29 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb29 {
        #[inline(always)]
        fn default() -> Huffsymb29 {
            Huffsymb29(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb3(pub u32);
    impl Huffsymb3 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb3 {
        #[inline(always)]
        fn default() -> Huffsymb3 {
            Huffsymb3(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb30(pub u32);
    impl Huffsymb30 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb30 {
        #[inline(always)]
        fn default() -> Huffsymb30 {
            Huffsymb30(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb31(pub u32);
    impl Huffsymb31 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb31 {
        #[inline(always)]
        fn default() -> Huffsymb31 {
            Huffsymb31(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb32(pub u32);
    impl Huffsymb32 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb32 {
        #[inline(always)]
        fn default() -> Huffsymb32 {
            Huffsymb32(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb33(pub u32);
    impl Huffsymb33 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb33 {
        #[inline(always)]
        fn default() -> Huffsymb33 {
            Huffsymb33(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb34(pub u32);
    impl Huffsymb34 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb34 {
        #[inline(always)]
        fn default() -> Huffsymb34 {
            Huffsymb34(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb35(pub u32);
    impl Huffsymb35 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb35 {
        #[inline(always)]
        fn default() -> Huffsymb35 {
            Huffsymb35(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb36(pub u32);
    impl Huffsymb36 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb36 {
        #[inline(always)]
        fn default() -> Huffsymb36 {
            Huffsymb36(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb37(pub u32);
    impl Huffsymb37 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb37 {
        #[inline(always)]
        fn default() -> Huffsymb37 {
            Huffsymb37(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb38(pub u32);
    impl Huffsymb38 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb38 {
        #[inline(always)]
        fn default() -> Huffsymb38 {
            Huffsymb38(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb39(pub u32);
    impl Huffsymb39 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb39 {
        #[inline(always)]
        fn default() -> Huffsymb39 {
            Huffsymb39(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb4(pub u32);
    impl Huffsymb4 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb4 {
        #[inline(always)]
        fn default() -> Huffsymb4 {
            Huffsymb4(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb40(pub u32);
    impl Huffsymb40 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb40 {
        #[inline(always)]
        fn default() -> Huffsymb40 {
            Huffsymb40(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb41(pub u32);
    impl Huffsymb41 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb41 {
        #[inline(always)]
        fn default() -> Huffsymb41 {
            Huffsymb41(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb42(pub u32);
    impl Huffsymb42 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb42 {
        #[inline(always)]
        fn default() -> Huffsymb42 {
            Huffsymb42(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb43(pub u32);
    impl Huffsymb43 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb43 {
        #[inline(always)]
        fn default() -> Huffsymb43 {
            Huffsymb43(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb44(pub u32);
    impl Huffsymb44 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb44 {
        #[inline(always)]
        fn default() -> Huffsymb44 {
            Huffsymb44(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb45(pub u32);
    impl Huffsymb45 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb45 {
        #[inline(always)]
        fn default() -> Huffsymb45 {
            Huffsymb45(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb46(pub u32);
    impl Huffsymb46 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb46 {
        #[inline(always)]
        fn default() -> Huffsymb46 {
            Huffsymb46(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb47(pub u32);
    impl Huffsymb47 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb47 {
        #[inline(always)]
        fn default() -> Huffsymb47 {
            Huffsymb47(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb48(pub u32);
    impl Huffsymb48 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb48 {
        #[inline(always)]
        fn default() -> Huffsymb48 {
            Huffsymb48(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb49(pub u32);
    impl Huffsymb49 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb49 {
        #[inline(always)]
        fn default() -> Huffsymb49 {
            Huffsymb49(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb5(pub u32);
    impl Huffsymb5 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb5 {
        #[inline(always)]
        fn default() -> Huffsymb5 {
            Huffsymb5(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb50(pub u32);
    impl Huffsymb50 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb50 {
        #[inline(always)]
        fn default() -> Huffsymb50 {
            Huffsymb50(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb51(pub u32);
    impl Huffsymb51 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb51 {
        #[inline(always)]
        fn default() -> Huffsymb51 {
            Huffsymb51(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb52(pub u32);
    impl Huffsymb52 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb52 {
        #[inline(always)]
        fn default() -> Huffsymb52 {
            Huffsymb52(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb53(pub u32);
    impl Huffsymb53 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb53 {
        #[inline(always)]
        fn default() -> Huffsymb53 {
            Huffsymb53(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb54(pub u32);
    impl Huffsymb54 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb54 {
        #[inline(always)]
        fn default() -> Huffsymb54 {
            Huffsymb54(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb55(pub u32);
    impl Huffsymb55 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb55 {
        #[inline(always)]
        fn default() -> Huffsymb55 {
            Huffsymb55(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb56(pub u32);
    impl Huffsymb56 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb56 {
        #[inline(always)]
        fn default() -> Huffsymb56 {
            Huffsymb56(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb57(pub u32);
    impl Huffsymb57 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb57 {
        #[inline(always)]
        fn default() -> Huffsymb57 {
            Huffsymb57(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb58(pub u32);
    impl Huffsymb58 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb58 {
        #[inline(always)]
        fn default() -> Huffsymb58 {
            Huffsymb58(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb59(pub u32);
    impl Huffsymb59 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb59 {
        #[inline(always)]
        fn default() -> Huffsymb59 {
            Huffsymb59(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb6(pub u32);
    impl Huffsymb6 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb6 {
        #[inline(always)]
        fn default() -> Huffsymb6 {
            Huffsymb6(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb60(pub u32);
    impl Huffsymb60 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb60 {
        #[inline(always)]
        fn default() -> Huffsymb60 {
            Huffsymb60(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb61(pub u32);
    impl Huffsymb61 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb61 {
        #[inline(always)]
        fn default() -> Huffsymb61 {
            Huffsymb61(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb62(pub u32);
    impl Huffsymb62 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb62 {
        #[inline(always)]
        fn default() -> Huffsymb62 {
            Huffsymb62(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb63(pub u32);
    impl Huffsymb63 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb63 {
        #[inline(always)]
        fn default() -> Huffsymb63 {
            Huffsymb63(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb64(pub u32);
    impl Huffsymb64 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb64 {
        #[inline(always)]
        fn default() -> Huffsymb64 {
            Huffsymb64(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb65(pub u32);
    impl Huffsymb65 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb65 {
        #[inline(always)]
        fn default() -> Huffsymb65 {
            Huffsymb65(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb66(pub u32);
    impl Huffsymb66 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb66 {
        #[inline(always)]
        fn default() -> Huffsymb66 {
            Huffsymb66(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb67(pub u32);
    impl Huffsymb67 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb67 {
        #[inline(always)]
        fn default() -> Huffsymb67 {
            Huffsymb67(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb68(pub u32);
    impl Huffsymb68 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb68 {
        #[inline(always)]
        fn default() -> Huffsymb68 {
            Huffsymb68(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb69(pub u32);
    impl Huffsymb69 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb69 {
        #[inline(always)]
        fn default() -> Huffsymb69 {
            Huffsymb69(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb7(pub u32);
    impl Huffsymb7 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb7 {
        #[inline(always)]
        fn default() -> Huffsymb7 {
            Huffsymb7(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb70(pub u32);
    impl Huffsymb70 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb70 {
        #[inline(always)]
        fn default() -> Huffsymb70 {
            Huffsymb70(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb71(pub u32);
    impl Huffsymb71 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb71 {
        #[inline(always)]
        fn default() -> Huffsymb71 {
            Huffsymb71(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb72(pub u32);
    impl Huffsymb72 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb72 {
        #[inline(always)]
        fn default() -> Huffsymb72 {
            Huffsymb72(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb73(pub u32);
    impl Huffsymb73 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb73 {
        #[inline(always)]
        fn default() -> Huffsymb73 {
            Huffsymb73(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb74(pub u32);
    impl Huffsymb74 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb74 {
        #[inline(always)]
        fn default() -> Huffsymb74 {
            Huffsymb74(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb75(pub u32);
    impl Huffsymb75 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb75 {
        #[inline(always)]
        fn default() -> Huffsymb75 {
            Huffsymb75(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb76(pub u32);
    impl Huffsymb76 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb76 {
        #[inline(always)]
        fn default() -> Huffsymb76 {
            Huffsymb76(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb77(pub u32);
    impl Huffsymb77 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb77 {
        #[inline(always)]
        fn default() -> Huffsymb77 {
            Huffsymb77(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb78(pub u32);
    impl Huffsymb78 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb78 {
        #[inline(always)]
        fn default() -> Huffsymb78 {
            Huffsymb78(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb79(pub u32);
    impl Huffsymb79 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb79 {
        #[inline(always)]
        fn default() -> Huffsymb79 {
            Huffsymb79(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb8(pub u32);
    impl Huffsymb8 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb8 {
        #[inline(always)]
        fn default() -> Huffsymb8 {
            Huffsymb8(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb80(pub u32);
    impl Huffsymb80 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb80 {
        #[inline(always)]
        fn default() -> Huffsymb80 {
            Huffsymb80(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb81(pub u32);
    impl Huffsymb81 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb81 {
        #[inline(always)]
        fn default() -> Huffsymb81 {
            Huffsymb81(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb82(pub u32);
    impl Huffsymb82 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb82 {
        #[inline(always)]
        fn default() -> Huffsymb82 {
            Huffsymb82(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb83(pub u32);
    impl Huffsymb83 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb83 {
        #[inline(always)]
        fn default() -> Huffsymb83 {
            Huffsymb83(0)
        }
    }
    #[doc = "JPEG HUFFSYMB tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Huffsymb9(pub u32);
    impl Huffsymb9 {
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub const fn huff_symb_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "DHTSymb RAM"]
        #[inline(always)]
        pub fn set_huff_symb_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Huffsymb9 {
        #[inline(always)]
        fn default() -> Huffsymb9 {
            Huffsymb9(0)
        }
    }
    #[doc = "JPEG clear flag register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegCfr(pub u32);
    impl JpegCfr {
        #[doc = "Clear End of Conversion Flag"]
        #[inline(always)]
        pub const fn ceocf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Clear End of Conversion Flag"]
        #[inline(always)]
        pub fn set_ceocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Clear Header Parsing Done Flag"]
        #[inline(always)]
        pub const fn chpdf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Header Parsing Done Flag"]
        #[inline(always)]
        pub fn set_chpdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for JpegCfr {
        #[inline(always)]
        fn default() -> JpegCfr {
            JpegCfr(0)
        }
    }
    #[doc = "JPEG codec configuration register 0"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr0(pub u32);
    impl JpegConfr0 {
        #[doc = "Start"]
        #[inline(always)]
        pub const fn start(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start"]
        #[inline(always)]
        pub fn set_start(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for JpegConfr0 {
        #[inline(always)]
        fn default() -> JpegConfr0 {
            JpegConfr0(0)
        }
    }
    #[doc = "JPEG codec configuration register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr1(pub u32);
    impl JpegConfr1 {
        #[doc = "Number of color components"]
        #[inline(always)]
        pub const fn nf(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Number of color components"]
        #[inline(always)]
        pub fn set_nf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Decoding Enable"]
        #[inline(always)]
        pub const fn de(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Decoding Enable"]
        #[inline(always)]
        pub fn set_de(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Color Space"]
        #[inline(always)]
        pub const fn colorspace(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Color Space"]
        #[inline(always)]
        pub fn set_colorspace(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Number of components for Scan"]
        #[inline(always)]
        pub const fn ns(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x03;
            val as u8
        }
        #[doc = "Number of components for Scan"]
        #[inline(always)]
        pub fn set_ns(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
        }
        #[doc = "Header Processing"]
        #[inline(always)]
        pub const fn hdr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Header Processing"]
        #[inline(always)]
        pub fn set_hdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Y Size"]
        #[inline(always)]
        pub const fn ysize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Y Size"]
        #[inline(always)]
        pub fn set_ysize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for JpegConfr1 {
        #[inline(always)]
        fn default() -> JpegConfr1 {
            JpegConfr1(0)
        }
    }
    #[doc = "JPEG codec configuration register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr2(pub u32);
    impl JpegConfr2 {
        #[doc = "Number of MCU"]
        #[inline(always)]
        pub const fn nmcu(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x03ff_ffff;
            val as u32
        }
        #[doc = "Number of MCU"]
        #[inline(always)]
        pub fn set_nmcu(&mut self, val: u32) {
            self.0 = (self.0 & !(0x03ff_ffff << 0usize)) | (((val as u32) & 0x03ff_ffff) << 0usize);
        }
    }
    impl Default for JpegConfr2 {
        #[inline(always)]
        fn default() -> JpegConfr2 {
            JpegConfr2(0)
        }
    }
    #[doc = "JPEG codec configuration register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr3(pub u32);
    impl JpegConfr3 {
        #[doc = "X size"]
        #[inline(always)]
        pub const fn xsize(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "X size"]
        #[inline(always)]
        pub fn set_xsize(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for JpegConfr3 {
        #[inline(always)]
        fn default() -> JpegConfr3 {
            JpegConfr3(0)
        }
    }
    #[doc = "JPEG codec configuration register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr4(pub u32);
    impl JpegConfr4 {
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr4 {
        #[inline(always)]
        fn default() -> JpegConfr4 {
            JpegConfr4(0)
        }
    }
    #[doc = "JPEG codec configuration register 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr5(pub u32);
    impl JpegConfr5 {
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr5 {
        #[inline(always)]
        fn default() -> JpegConfr5 {
            JpegConfr5(0)
        }
    }
    #[doc = "JPEG codec configuration register 6"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr6(pub u32);
    impl JpegConfr6 {
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr6 {
        #[inline(always)]
        fn default() -> JpegConfr6 {
            JpegConfr6(0)
        }
    }
    #[doc = "JPEG codec configuration register 7"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegConfr7(pub u32);
    impl JpegConfr7 {
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub const fn hd(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman DC"]
        #[inline(always)]
        pub fn set_hd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub const fn ha(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Huffman AC"]
        #[inline(always)]
        pub fn set_ha(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub const fn qt(&self) -> u8 {
            let val = (self.0 >> 2usize) & 0x03;
            val as u8
        }
        #[doc = "Quantization Table"]
        #[inline(always)]
        pub fn set_qt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub const fn nb(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x0f;
            val as u8
        }
        #[doc = "Number of Block"]
        #[inline(always)]
        pub fn set_nb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub const fn vsf(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x0f;
            val as u8
        }
        #[doc = "Vertical Sampling Factor"]
        #[inline(always)]
        pub fn set_vsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub const fn hsf(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x0f;
            val as u8
        }
        #[doc = "Horizontal Sampling Factor"]
        #[inline(always)]
        pub fn set_hsf(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
        }
    }
    impl Default for JpegConfr7 {
        #[inline(always)]
        fn default() -> JpegConfr7 {
            JpegConfr7(0)
        }
    }
    #[doc = "JPEG control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegCr(pub u32);
    impl JpegCr {
        #[doc = "JPEG Core Enable"]
        #[inline(always)]
        pub const fn jcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "JPEG Core Enable"]
        #[inline(always)]
        pub fn set_jcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Input FIFO Threshold Interrupt Enable"]
        #[inline(always)]
        pub const fn iftie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Threshold Interrupt Enable"]
        #[inline(always)]
        pub fn set_iftie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Input FIFO Not Full Interrupt Enable"]
        #[inline(always)]
        pub const fn ifnfie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Not Full Interrupt Enable"]
        #[inline(always)]
        pub fn set_ifnfie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Output FIFO Threshold Interrupt Enable"]
        #[inline(always)]
        pub const fn oftie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Threshold Interrupt Enable"]
        #[inline(always)]
        pub fn set_oftie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Output FIFO Not Empty Interrupt Enable"]
        #[inline(always)]
        pub const fn ofneie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Not Empty Interrupt Enable"]
        #[inline(always)]
        pub fn set_ofneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of Conversion Interrupt Enable"]
        #[inline(always)]
        pub const fn eocie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of Conversion Interrupt Enable"]
        #[inline(always)]
        pub fn set_eocie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Header Parsing Done Interrupt Enable"]
        #[inline(always)]
        pub const fn hpdie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Header Parsing Done Interrupt Enable"]
        #[inline(always)]
        pub fn set_hpdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Input DMA Enable"]
        #[inline(always)]
        pub const fn idmaen(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Input DMA Enable"]
        #[inline(always)]
        pub fn set_idmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Output DMA Enable"]
        #[inline(always)]
        pub const fn odmaen(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Output DMA Enable"]
        #[inline(always)]
        pub fn set_odmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Input FIFO Flush"]
        #[inline(always)]
        pub const fn iff(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Flush"]
        #[inline(always)]
        pub fn set_iff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Output FIFO Flush"]
        #[inline(always)]
        pub const fn off(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Flush"]
        #[inline(always)]
        pub fn set_off(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for JpegCr {
        #[inline(always)]
        fn default() -> JpegCr {
            JpegCr(0)
        }
    }
    #[doc = "JPEG data input register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegDir(pub u32);
    impl JpegDir {
        #[doc = "Data Input FIFO"]
        #[inline(always)]
        pub const fn datain(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data Input FIFO"]
        #[inline(always)]
        pub fn set_datain(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for JpegDir {
        #[inline(always)]
        fn default() -> JpegDir {
            JpegDir(0)
        }
    }
    #[doc = "JPEG data output register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegDor(pub u32);
    impl JpegDor {
        #[doc = "Data Output FIFO"]
        #[inline(always)]
        pub const fn dataout(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Data Output FIFO"]
        #[inline(always)]
        pub fn set_dataout(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for JpegDor {
        #[inline(always)]
        fn default() -> JpegDor {
            JpegDor(0)
        }
    }
    #[doc = "JPEG status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct JpegSr(pub u32);
    impl JpegSr {
        #[doc = "Input FIFO Threshold Flag"]
        #[inline(always)]
        pub const fn iftf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Threshold Flag"]
        #[inline(always)]
        pub fn set_iftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Input FIFO Not Full Flag"]
        #[inline(always)]
        pub const fn ifnff(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Input FIFO Not Full Flag"]
        #[inline(always)]
        pub fn set_ifnff(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Output FIFO Threshold Flag"]
        #[inline(always)]
        pub const fn oftf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Threshold Flag"]
        #[inline(always)]
        pub fn set_oftf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Output FIFO Not Empty Flag"]
        #[inline(always)]
        pub const fn ofnef(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Output FIFO Not Empty Flag"]
        #[inline(always)]
        pub fn set_ofnef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "End of Conversion Flag"]
        #[inline(always)]
        pub const fn eocf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "End of Conversion Flag"]
        #[inline(always)]
        pub fn set_eocf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Header Parsing Done Flag"]
        #[inline(always)]
        pub const fn hpdf(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Header Parsing Done Flag"]
        #[inline(always)]
        pub fn set_hpdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Codec Operation Flag"]
        #[inline(always)]
        pub const fn cof(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Codec Operation Flag"]
        #[inline(always)]
        pub fn set_cof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
    }
    impl Default for JpegSr {
        #[inline(always)]
        fn default() -> JpegSr {
            JpegSr(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem00(pub u32);
    impl Qmem00 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem00 {
        #[inline(always)]
        fn default() -> Qmem00 {
            Qmem00(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem01(pub u32);
    impl Qmem01 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem01 {
        #[inline(always)]
        fn default() -> Qmem01 {
            Qmem01(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem010(pub u32);
    impl Qmem010 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem010 {
        #[inline(always)]
        fn default() -> Qmem010 {
            Qmem010(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem011(pub u32);
    impl Qmem011 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem011 {
        #[inline(always)]
        fn default() -> Qmem011 {
            Qmem011(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem012(pub u32);
    impl Qmem012 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem012 {
        #[inline(always)]
        fn default() -> Qmem012 {
            Qmem012(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem013(pub u32);
    impl Qmem013 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem013 {
        #[inline(always)]
        fn default() -> Qmem013 {
            Qmem013(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem014(pub u32);
    impl Qmem014 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem014 {
        #[inline(always)]
        fn default() -> Qmem014 {
            Qmem014(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem015(pub u32);
    impl Qmem015 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem015 {
        #[inline(always)]
        fn default() -> Qmem015 {
            Qmem015(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem02(pub u32);
    impl Qmem02 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem02 {
        #[inline(always)]
        fn default() -> Qmem02 {
            Qmem02(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem03(pub u32);
    impl Qmem03 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem03 {
        #[inline(always)]
        fn default() -> Qmem03 {
            Qmem03(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem04(pub u32);
    impl Qmem04 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem04 {
        #[inline(always)]
        fn default() -> Qmem04 {
            Qmem04(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem05(pub u32);
    impl Qmem05 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem05 {
        #[inline(always)]
        fn default() -> Qmem05 {
            Qmem05(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem06(pub u32);
    impl Qmem06 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem06 {
        #[inline(always)]
        fn default() -> Qmem06 {
            Qmem06(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem07(pub u32);
    impl Qmem07 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem07 {
        #[inline(always)]
        fn default() -> Qmem07 {
            Qmem07(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem08(pub u32);
    impl Qmem08 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem08 {
        #[inline(always)]
        fn default() -> Qmem08 {
            Qmem08(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem09(pub u32);
    impl Qmem09 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem09 {
        #[inline(always)]
        fn default() -> Qmem09 {
            Qmem09(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem10(pub u32);
    impl Qmem10 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem10 {
        #[inline(always)]
        fn default() -> Qmem10 {
            Qmem10(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem11(pub u32);
    impl Qmem11 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem11 {
        #[inline(always)]
        fn default() -> Qmem11 {
            Qmem11(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem110(pub u32);
    impl Qmem110 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem110 {
        #[inline(always)]
        fn default() -> Qmem110 {
            Qmem110(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem111(pub u32);
    impl Qmem111 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem111 {
        #[inline(always)]
        fn default() -> Qmem111 {
            Qmem111(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem112(pub u32);
    impl Qmem112 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem112 {
        #[inline(always)]
        fn default() -> Qmem112 {
            Qmem112(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem113(pub u32);
    impl Qmem113 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem113 {
        #[inline(always)]
        fn default() -> Qmem113 {
            Qmem113(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem114(pub u32);
    impl Qmem114 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem114 {
        #[inline(always)]
        fn default() -> Qmem114 {
            Qmem114(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem115(pub u32);
    impl Qmem115 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem115 {
        #[inline(always)]
        fn default() -> Qmem115 {
            Qmem115(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem12(pub u32);
    impl Qmem12 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem12 {
        #[inline(always)]
        fn default() -> Qmem12 {
            Qmem12(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem13(pub u32);
    impl Qmem13 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem13 {
        #[inline(always)]
        fn default() -> Qmem13 {
            Qmem13(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem14(pub u32);
    impl Qmem14 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem14 {
        #[inline(always)]
        fn default() -> Qmem14 {
            Qmem14(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem15(pub u32);
    impl Qmem15 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem15 {
        #[inline(always)]
        fn default() -> Qmem15 {
            Qmem15(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem16(pub u32);
    impl Qmem16 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem16 {
        #[inline(always)]
        fn default() -> Qmem16 {
            Qmem16(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem17(pub u32);
    impl Qmem17 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem17 {
        #[inline(always)]
        fn default() -> Qmem17 {
            Qmem17(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem18(pub u32);
    impl Qmem18 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem18 {
        #[inline(always)]
        fn default() -> Qmem18 {
            Qmem18(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem19(pub u32);
    impl Qmem19 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem19 {
        #[inline(always)]
        fn default() -> Qmem19 {
            Qmem19(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem20(pub u32);
    impl Qmem20 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem20 {
        #[inline(always)]
        fn default() -> Qmem20 {
            Qmem20(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem21(pub u32);
    impl Qmem21 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem21 {
        #[inline(always)]
        fn default() -> Qmem21 {
            Qmem21(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem210(pub u32);
    impl Qmem210 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem210 {
        #[inline(always)]
        fn default() -> Qmem210 {
            Qmem210(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem211(pub u32);
    impl Qmem211 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem211 {
        #[inline(always)]
        fn default() -> Qmem211 {
            Qmem211(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem212(pub u32);
    impl Qmem212 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem212 {
        #[inline(always)]
        fn default() -> Qmem212 {
            Qmem212(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem213(pub u32);
    impl Qmem213 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem213 {
        #[inline(always)]
        fn default() -> Qmem213 {
            Qmem213(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem214(pub u32);
    impl Qmem214 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem214 {
        #[inline(always)]
        fn default() -> Qmem214 {
            Qmem214(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem215(pub u32);
    impl Qmem215 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem215 {
        #[inline(always)]
        fn default() -> Qmem215 {
            Qmem215(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem22(pub u32);
    impl Qmem22 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem22 {
        #[inline(always)]
        fn default() -> Qmem22 {
            Qmem22(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem23(pub u32);
    impl Qmem23 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem23 {
        #[inline(always)]
        fn default() -> Qmem23 {
            Qmem23(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem24(pub u32);
    impl Qmem24 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem24 {
        #[inline(always)]
        fn default() -> Qmem24 {
            Qmem24(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem25(pub u32);
    impl Qmem25 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem25 {
        #[inline(always)]
        fn default() -> Qmem25 {
            Qmem25(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem26(pub u32);
    impl Qmem26 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem26 {
        #[inline(always)]
        fn default() -> Qmem26 {
            Qmem26(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem27(pub u32);
    impl Qmem27 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem27 {
        #[inline(always)]
        fn default() -> Qmem27 {
            Qmem27(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem28(pub u32);
    impl Qmem28 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem28 {
        #[inline(always)]
        fn default() -> Qmem28 {
            Qmem28(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem29(pub u32);
    impl Qmem29 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem29 {
        #[inline(always)]
        fn default() -> Qmem29 {
            Qmem29(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem30(pub u32);
    impl Qmem30 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem30 {
        #[inline(always)]
        fn default() -> Qmem30 {
            Qmem30(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem31(pub u32);
    impl Qmem31 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem31 {
        #[inline(always)]
        fn default() -> Qmem31 {
            Qmem31(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem310(pub u32);
    impl Qmem310 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem310 {
        #[inline(always)]
        fn default() -> Qmem310 {
            Qmem310(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem311(pub u32);
    impl Qmem311 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem311 {
        #[inline(always)]
        fn default() -> Qmem311 {
            Qmem311(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem312(pub u32);
    impl Qmem312 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem312 {
        #[inline(always)]
        fn default() -> Qmem312 {
            Qmem312(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem313(pub u32);
    impl Qmem313 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem313 {
        #[inline(always)]
        fn default() -> Qmem313 {
            Qmem313(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem314(pub u32);
    impl Qmem314 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem314 {
        #[inline(always)]
        fn default() -> Qmem314 {
            Qmem314(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem315(pub u32);
    impl Qmem315 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem315 {
        #[inline(always)]
        fn default() -> Qmem315 {
            Qmem315(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem32(pub u32);
    impl Qmem32 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem32 {
        #[inline(always)]
        fn default() -> Qmem32 {
            Qmem32(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem33(pub u32);
    impl Qmem33 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem33 {
        #[inline(always)]
        fn default() -> Qmem33 {
            Qmem33(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem34(pub u32);
    impl Qmem34 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem34 {
        #[inline(always)]
        fn default() -> Qmem34 {
            Qmem34(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem35(pub u32);
    impl Qmem35 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem35 {
        #[inline(always)]
        fn default() -> Qmem35 {
            Qmem35(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem36(pub u32);
    impl Qmem36 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem36 {
        #[inline(always)]
        fn default() -> Qmem36 {
            Qmem36(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem37(pub u32);
    impl Qmem37 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem37 {
        #[inline(always)]
        fn default() -> Qmem37 {
            Qmem37(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem38(pub u32);
    impl Qmem38 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem38 {
        #[inline(always)]
        fn default() -> Qmem38 {
            Qmem38(0)
        }
    }
    #[doc = "JPEG quantization tables"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Qmem39(pub u32);
    impl Qmem39 {
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub const fn qmem_ram(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "QMem RAM"]
        #[inline(always)]
        pub fn set_qmem_ram(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Qmem39 {
        #[inline(always)]
        fn default() -> Qmem39 {
            Qmem39(0)
        }
    }
}
