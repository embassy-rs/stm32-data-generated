#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Layer {
    ptr: *mut u8,
}
unsafe impl Send for Layer {}
unsafe impl Sync for Layer {}
impl Layer {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Layerx Control Register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Layerx Window Horizontal Position Configuration Register"]
    #[inline(always)]
    pub const fn whpcr(self) -> crate::common::Reg<regs::Whpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Layerx Window Vertical Position Configuration Register"]
    #[inline(always)]
    pub const fn wvpcr(self) -> crate::common::Reg<regs::Wvpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Layerx Color Keying Configuration Register"]
    #[inline(always)]
    pub const fn ckcr(self) -> crate::common::Reg<regs::Ckcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Layerx Pixel Format Configuration Register"]
    #[inline(always)]
    pub const fn pfcr(self) -> crate::common::Reg<regs::Pfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Layerx Constant Alpha Configuration Register"]
    #[inline(always)]
    pub const fn cacr(self) -> crate::common::Reg<regs::Cacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Layerx Default Color Configuration Register"]
    #[inline(always)]
    pub const fn dccr(self) -> crate::common::Reg<regs::Dccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Layerx Blending Factors Configuration Register"]
    #[inline(always)]
    pub const fn bfcr(self) -> crate::common::Reg<regs::Bfcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Layerx Color Frame Buffer Address Register"]
    #[inline(always)]
    pub const fn cfbar(self) -> crate::common::Reg<regs::Cfbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Layerx Color Frame Buffer Length Register"]
    #[inline(always)]
    pub const fn cfblr(self) -> crate::common::Reg<regs::Cfblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Layerx ColorFrame Buffer Line Number Register"]
    #[inline(always)]
    pub const fn cfblnr(self) -> crate::common::Reg<regs::Cfblnr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Layerx CLUT Write Register"]
    #[inline(always)]
    pub const fn clutwr(self) -> crate::common::Reg<regs::Clutwr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
}
#[doc = "LCD-TFT Controller"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ltdc {
    ptr: *mut u8,
}
unsafe impl Send for Ltdc {}
unsafe impl Sync for Ltdc {}
impl Ltdc {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Synchronization Size Configuration Register"]
    #[inline(always)]
    pub const fn sscr(self) -> crate::common::Reg<regs::Sscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Back Porch Configuration Register"]
    #[inline(always)]
    pub const fn bpcr(self) -> crate::common::Reg<regs::Bpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Active Width Configuration Register"]
    #[inline(always)]
    pub const fn awcr(self) -> crate::common::Reg<regs::Awcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Total Width Configuration Register"]
    #[inline(always)]
    pub const fn twcr(self) -> crate::common::Reg<regs::Twcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Global Control Register"]
    #[inline(always)]
    pub const fn gcr(self) -> crate::common::Reg<regs::Gcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Shadow Reload Configuration Register"]
    #[inline(always)]
    pub const fn srcr(self) -> crate::common::Reg<regs::Srcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Background Color Configuration Register"]
    #[inline(always)]
    pub const fn bccr(self) -> crate::common::Reg<regs::Bccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(self) -> crate::common::Reg<regs::Ier, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Line Interrupt Position Configuration Register"]
    #[inline(always)]
    pub const fn lipcr(self) -> crate::common::Reg<regs::Lipcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Current Position Status Register"]
    #[inline(always)]
    pub const fn cpsr(self) -> crate::common::Reg<regs::Cpsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Current Display Status Register"]
    #[inline(always)]
    pub const fn cdsr(self) -> crate::common::Reg<regs::Cdsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Cluster LAYER%s, containing L?CR, L?WHPCR, L?WVPCR, L?CKCR, L?PFCR, L?CACR, L?DCCR, L?BFCR, L?CFBAR, L?CFBLR, L?CFBLNR, L?CLUTWR"]
    #[inline(always)]
    pub const fn layer(self, n: usize) -> Layer {
        assert!(n < 2usize);
        unsafe { Layer::from_ptr(self.ptr.add(0x84usize + n * 128usize) as _) }
    }
}
pub mod regs {
    #[doc = "Active Width Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Awcr(pub u32);
    impl Awcr {
        #[doc = "Accumulated Active Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub const fn aah(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Accumulated Active Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub fn set_aah(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Accumulated Active Width (in units of pixel clock period)"]
        #[inline(always)]
        pub const fn aaw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Accumulated Active Width (in units of pixel clock period)"]
        #[inline(always)]
        pub fn set_aaw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Awcr {
        #[inline(always)]
        fn default() -> Awcr {
            Awcr(0)
        }
    }
    #[doc = "Background Color Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bccr(pub u32);
    impl Bccr {
        #[doc = "Background color blue value"]
        #[inline(always)]
        pub const fn bcblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Background color blue value"]
        #[inline(always)]
        pub fn set_bcblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Background color green value"]
        #[inline(always)]
        pub const fn bcgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Background color green value"]
        #[inline(always)]
        pub fn set_bcgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Background color red value"]
        #[inline(always)]
        pub const fn bcred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Background color red value"]
        #[inline(always)]
        pub fn set_bcred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Bccr {
        #[inline(always)]
        fn default() -> Bccr {
            Bccr(0)
        }
    }
    #[doc = "Layerx Blending Factors Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bfcr(pub u32);
    impl Bfcr {
        #[doc = "Blending Factor 2"]
        #[inline(always)]
        pub const fn bf2(&self) -> super::vals::Bf2 {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Bf2::from_bits(val as u8)
        }
        #[doc = "Blending Factor 2"]
        #[inline(always)]
        pub fn set_bf2(&mut self, val: super::vals::Bf2) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Blending Factor 1"]
        #[inline(always)]
        pub const fn bf1(&self) -> super::vals::Bf1 {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::Bf1::from_bits(val as u8)
        }
        #[doc = "Blending Factor 1"]
        #[inline(always)]
        pub fn set_bf1(&mut self, val: super::vals::Bf1) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
    }
    impl Default for Bfcr {
        #[inline(always)]
        fn default() -> Bfcr {
            Bfcr(0)
        }
    }
    #[doc = "Back Porch Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bpcr(pub u32);
    impl Bpcr {
        #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)"]
        #[inline(always)]
        pub const fn avbp(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Accumulated Vertical back porch (in units of horizontal scan line)"]
        #[inline(always)]
        pub fn set_avbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)"]
        #[inline(always)]
        pub const fn ahbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Accumulated Horizontal back porch (in units of pixel clock period)"]
        #[inline(always)]
        pub fn set_ahbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Bpcr {
        #[inline(always)]
        fn default() -> Bpcr {
            Bpcr(0)
        }
    }
    #[doc = "Layerx Constant Alpha Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cacr(pub u32);
    impl Cacr {
        #[doc = "Constant Alpha"]
        #[inline(always)]
        pub const fn consta(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Constant Alpha"]
        #[inline(always)]
        pub fn set_consta(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Cacr {
        #[inline(always)]
        fn default() -> Cacr {
            Cacr(0)
        }
    }
    #[doc = "Current Display Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cdsr(pub u32);
    impl Cdsr {
        #[doc = "Vertical Data Enable display Status"]
        #[inline(always)]
        pub const fn vdes(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Data Enable display Status"]
        #[inline(always)]
        pub fn set_vdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Horizontal Data Enable display Status"]
        #[inline(always)]
        pub const fn hdes(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Data Enable display Status"]
        #[inline(always)]
        pub fn set_hdes(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Vertical Synchronization display Status"]
        #[inline(always)]
        pub const fn vsyncs(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Vertical Synchronization display Status"]
        #[inline(always)]
        pub fn set_vsyncs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Horizontal Synchronization display Status"]
        #[inline(always)]
        pub const fn hsyncs(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Horizontal Synchronization display Status"]
        #[inline(always)]
        pub fn set_hsyncs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Cdsr {
        #[inline(always)]
        fn default() -> Cdsr {
            Cdsr(0)
        }
    }
    #[doc = "Layerx Color Frame Buffer Address Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfbar(pub u32);
    impl Cfbar {
        #[doc = "Color Frame Buffer Start Address"]
        #[inline(always)]
        pub const fn cfbadd(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Color Frame Buffer Start Address"]
        #[inline(always)]
        pub fn set_cfbadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Cfbar {
        #[inline(always)]
        fn default() -> Cfbar {
            Cfbar(0)
        }
    }
    #[doc = "Layerx ColorFrame Buffer Line Number Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfblnr(pub u32);
    impl Cfblnr {
        #[doc = "Frame Buffer Line Number"]
        #[inline(always)]
        pub const fn cfblnbr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Frame Buffer Line Number"]
        #[inline(always)]
        pub fn set_cfblnbr(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Cfblnr {
        #[inline(always)]
        fn default() -> Cfblnr {
            Cfblnr(0)
        }
    }
    #[doc = "Layerx Color Frame Buffer Length Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfblr(pub u32);
    impl Cfblr {
        #[doc = "Color Frame Buffer Line Length"]
        #[inline(always)]
        pub const fn cfbll(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "Color Frame Buffer Line Length"]
        #[inline(always)]
        pub fn set_cfbll(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "Color Frame Buffer Pitch in bytes"]
        #[inline(always)]
        pub const fn cfbp(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "Color Frame Buffer Pitch in bytes"]
        #[inline(always)]
        pub fn set_cfbp(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Cfblr {
        #[inline(always)]
        fn default() -> Cfblr {
            Cfblr(0)
        }
    }
    #[doc = "Layerx Color Keying Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ckcr(pub u32);
    impl Ckcr {
        #[doc = "Color Key Blue value"]
        #[inline(always)]
        pub const fn ckblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Blue value"]
        #[inline(always)]
        pub fn set_ckblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Color Key Green value"]
        #[inline(always)]
        pub const fn ckgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Green value"]
        #[inline(always)]
        pub fn set_ckgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Color Key Red value"]
        #[inline(always)]
        pub const fn ckred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Color Key Red value"]
        #[inline(always)]
        pub fn set_ckred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for Ckcr {
        #[inline(always)]
        fn default() -> Ckcr {
            Ckcr(0)
        }
    }
    #[doc = "Layerx CLUT Write Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Clutwr(pub u32);
    impl Clutwr {
        #[doc = "Blue value"]
        #[inline(always)]
        pub const fn blue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Blue value"]
        #[inline(always)]
        pub fn set_blue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Green value"]
        #[inline(always)]
        pub const fn green(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Green value"]
        #[inline(always)]
        pub fn set_green(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Red value"]
        #[inline(always)]
        pub const fn red(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Red value"]
        #[inline(always)]
        pub fn set_red(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "CLUT Address"]
        #[inline(always)]
        pub const fn clutadd(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "CLUT Address"]
        #[inline(always)]
        pub fn set_clutadd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Clutwr {
        #[inline(always)]
        fn default() -> Clutwr {
            Clutwr(0)
        }
    }
    #[doc = "Current Position Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cpsr(pub u32);
    impl Cpsr {
        #[doc = "Current Y Position"]
        #[inline(always)]
        pub const fn cypos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Current Y Position"]
        #[inline(always)]
        pub fn set_cypos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Current X Position"]
        #[inline(always)]
        pub const fn cxpos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Current X Position"]
        #[inline(always)]
        pub fn set_cxpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for Cpsr {
        #[inline(always)]
        fn default() -> Cpsr {
            Cpsr(0)
        }
    }
    #[doc = "Layerx Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Layer Enable"]
        #[inline(always)]
        pub const fn len(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Layer Enable"]
        #[inline(always)]
        pub fn set_len(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Color Keying Enable"]
        #[inline(always)]
        pub const fn colken(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Color Keying Enable"]
        #[inline(always)]
        pub fn set_colken(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Color Look-Up Table Enable"]
        #[inline(always)]
        pub const fn cluten(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Color Look-Up Table Enable"]
        #[inline(always)]
        pub fn set_cluten(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    #[doc = "Layerx Default Color Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dccr(pub u32);
    impl Dccr {
        #[doc = "Default Color Blue"]
        #[inline(always)]
        pub const fn dcblue(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Blue"]
        #[inline(always)]
        pub fn set_dcblue(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
        #[doc = "Default Color Green"]
        #[inline(always)]
        pub const fn dcgreen(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Green"]
        #[inline(always)]
        pub fn set_dcgreen(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
        }
        #[doc = "Default Color Red"]
        #[inline(always)]
        pub const fn dcred(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Red"]
        #[inline(always)]
        pub fn set_dcred(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "Default Color Alpha"]
        #[inline(always)]
        pub const fn dcalpha(&self) -> u8 {
            let val = (self.0 >> 24usize) & 0xff;
            val as u8
        }
        #[doc = "Default Color Alpha"]
        #[inline(always)]
        pub fn set_dcalpha(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
        }
    }
    impl Default for Dccr {
        #[inline(always)]
        fn default() -> Dccr {
            Dccr(0)
        }
    }
    #[doc = "Global Control Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Gcr(pub u32);
    impl Gcr {
        #[doc = "LCD-TFT controller enable bit"]
        #[inline(always)]
        pub const fn ltdcen(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "LCD-TFT controller enable bit"]
        #[inline(always)]
        pub fn set_ltdcen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Dither Blue Width"]
        #[inline(always)]
        pub const fn dbw(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Dither Blue Width"]
        #[inline(always)]
        pub fn set_dbw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "Dither Green Width"]
        #[inline(always)]
        pub const fn dgw(&self) -> u8 {
            let val = (self.0 >> 8usize) & 0x07;
            val as u8
        }
        #[doc = "Dither Green Width"]
        #[inline(always)]
        pub fn set_dgw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
        }
        #[doc = "Dither Red Width"]
        #[inline(always)]
        pub const fn drw(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x07;
            val as u8
        }
        #[doc = "Dither Red Width"]
        #[inline(always)]
        pub fn set_drw(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
        }
        #[doc = "Dither Enable"]
        #[inline(always)]
        pub const fn den(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Dither Enable"]
        #[inline(always)]
        pub fn set_den(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Pixel Clock Polarity"]
        #[inline(always)]
        pub const fn pcpol(&self) -> super::vals::Pcpol {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Pcpol::from_bits(val as u8)
        }
        #[doc = "Pixel Clock Polarity"]
        #[inline(always)]
        pub fn set_pcpol(&mut self, val: super::vals::Pcpol) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "Data Enable Polarity"]
        #[inline(always)]
        pub const fn depol(&self) -> super::vals::Depol {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Depol::from_bits(val as u8)
        }
        #[doc = "Data Enable Polarity"]
        #[inline(always)]
        pub fn set_depol(&mut self, val: super::vals::Depol) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Vertical Synchronization Polarity"]
        #[inline(always)]
        pub const fn vspol(&self) -> super::vals::Vspol {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Vspol::from_bits(val as u8)
        }
        #[doc = "Vertical Synchronization Polarity"]
        #[inline(always)]
        pub fn set_vspol(&mut self, val: super::vals::Vspol) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Horizontal Synchronization Polarity"]
        #[inline(always)]
        pub const fn hspol(&self) -> super::vals::Hspol {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Hspol::from_bits(val as u8)
        }
        #[doc = "Horizontal Synchronization Polarity"]
        #[inline(always)]
        pub fn set_hspol(&mut self, val: super::vals::Hspol) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Gcr {
        #[inline(always)]
        fn default() -> Gcr {
            Gcr(0)
        }
    }
    #[doc = "Interrupt Clear Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Icr(pub u32);
    impl Icr {
        #[doc = "Clears the Line Interrupt Flag"]
        #[inline(always)]
        pub const fn clif(&self) -> super::vals::Clif {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Clif::from_bits(val as u8)
        }
        #[doc = "Clears the Line Interrupt Flag"]
        #[inline(always)]
        pub fn set_clif(&mut self, val: super::vals::Clif) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Clears the FIFO Underrun Interrupt flag"]
        #[inline(always)]
        pub const fn cfuif(&self) -> super::vals::Cfuif {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Cfuif::from_bits(val as u8)
        }
        #[doc = "Clears the FIFO Underrun Interrupt flag"]
        #[inline(always)]
        pub fn set_cfuif(&mut self, val: super::vals::Cfuif) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Clears the Transfer Error Interrupt Flag"]
        #[inline(always)]
        pub const fn cterrif(&self) -> super::vals::Cterrif {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Cterrif::from_bits(val as u8)
        }
        #[doc = "Clears the Transfer Error Interrupt Flag"]
        #[inline(always)]
        pub fn set_cterrif(&mut self, val: super::vals::Cterrif) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "Clears Register Reload Interrupt Flag"]
        #[inline(always)]
        pub const fn crrif(&self) -> super::vals::Crrif {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Crrif::from_bits(val as u8)
        }
        #[doc = "Clears Register Reload Interrupt Flag"]
        #[inline(always)]
        pub fn set_crrif(&mut self, val: super::vals::Crrif) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Icr {
        #[inline(always)]
        fn default() -> Icr {
            Icr(0)
        }
    }
    #[doc = "Interrupt Enable Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ier(pub u32);
    impl Ier {
        #[doc = "Line Interrupt Enable"]
        #[inline(always)]
        pub const fn lie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt Enable"]
        #[inline(always)]
        pub fn set_lie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO Underrun Interrupt Enable"]
        #[inline(always)]
        pub const fn fuie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Underrun Interrupt Enable"]
        #[inline(always)]
        pub fn set_fuie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer Error Interrupt Enable"]
        #[inline(always)]
        pub const fn terrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error Interrupt Enable"]
        #[inline(always)]
        pub fn set_terrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Register Reload interrupt enable"]
        #[inline(always)]
        pub const fn rrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload interrupt enable"]
        #[inline(always)]
        pub fn set_rrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Ier {
        #[inline(always)]
        fn default() -> Ier {
            Ier(0)
        }
    }
    #[doc = "Interrupt Status Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Isr(pub u32);
    impl Isr {
        #[doc = "Line Interrupt flag"]
        #[inline(always)]
        pub const fn lif(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Line Interrupt flag"]
        #[inline(always)]
        pub fn set_lif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "FIFO Underrun Interrupt flag"]
        #[inline(always)]
        pub const fn fuif(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "FIFO Underrun Interrupt flag"]
        #[inline(always)]
        pub fn set_fuif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Transfer Error interrupt flag"]
        #[inline(always)]
        pub const fn terrif(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Transfer Error interrupt flag"]
        #[inline(always)]
        pub fn set_terrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Register Reload Interrupt Flag"]
        #[inline(always)]
        pub const fn rrif(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Register Reload Interrupt Flag"]
        #[inline(always)]
        pub fn set_rrif(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Isr {
        #[inline(always)]
        fn default() -> Isr {
            Isr(0)
        }
    }
    #[doc = "Line Interrupt Position Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Lipcr(pub u32);
    impl Lipcr {
        #[doc = "Line Interrupt Position"]
        #[inline(always)]
        pub const fn lipos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Line Interrupt Position"]
        #[inline(always)]
        pub fn set_lipos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
    }
    impl Default for Lipcr {
        #[inline(always)]
        fn default() -> Lipcr {
            Lipcr(0)
        }
    }
    #[doc = "Layerx Pixel Format Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pfcr(pub u32);
    impl Pfcr {
        #[doc = "Pixel Format"]
        #[inline(always)]
        pub const fn pf(&self) -> super::vals::Pf {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Pf::from_bits(val as u8)
        }
        #[doc = "Pixel Format"]
        #[inline(always)]
        pub fn set_pf(&mut self, val: super::vals::Pf) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
    }
    impl Default for Pfcr {
        #[inline(always)]
        fn default() -> Pfcr {
            Pfcr(0)
        }
    }
    #[doc = "Shadow Reload Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Srcr(pub u32);
    impl Srcr {
        #[doc = "Immediate Reload"]
        #[inline(always)]
        pub const fn imr(&self) -> super::vals::Imr {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Imr::from_bits(val as u8)
        }
        #[doc = "Immediate Reload"]
        #[inline(always)]
        pub fn set_imr(&mut self, val: super::vals::Imr) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Vertical Blanking Reload"]
        #[inline(always)]
        pub const fn vbr(&self) -> super::vals::Vbr {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Vbr::from_bits(val as u8)
        }
        #[doc = "Vertical Blanking Reload"]
        #[inline(always)]
        pub fn set_vbr(&mut self, val: super::vals::Vbr) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Srcr {
        #[inline(always)]
        fn default() -> Srcr {
            Srcr(0)
        }
    }
    #[doc = "Synchronization Size Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sscr(pub u32);
    impl Sscr {
        #[doc = "Vertical Synchronization Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub const fn vsh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Vertical Synchronization Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub fn set_vsh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Horizontal Synchronization Width (in units of pixel clock period)"]
        #[inline(always)]
        pub const fn hsw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Horizontal Synchronization Width (in units of pixel clock period)"]
        #[inline(always)]
        pub fn set_hsw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Sscr {
        #[inline(always)]
        fn default() -> Sscr {
            Sscr(0)
        }
    }
    #[doc = "Total Width Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Twcr(pub u32);
    impl Twcr {
        #[doc = "Total Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub const fn totalh(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Total Height (in units of horizontal scan line)"]
        #[inline(always)]
        pub fn set_totalh(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Total Width (in units of pixel clock period)"]
        #[inline(always)]
        pub const fn totalw(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Total Width (in units of pixel clock period)"]
        #[inline(always)]
        pub fn set_totalw(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Twcr {
        #[inline(always)]
        fn default() -> Twcr {
            Twcr(0)
        }
    }
    #[doc = "Layerx Window Horizontal Position Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Whpcr(pub u32);
    impl Whpcr {
        #[doc = "Window Horizontal Start Position"]
        #[inline(always)]
        pub const fn whstpos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x0fff;
            val as u16
        }
        #[doc = "Window Horizontal Start Position"]
        #[inline(always)]
        pub fn set_whstpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
        }
        #[doc = "Window Horizontal Stop Position"]
        #[inline(always)]
        pub const fn whsppos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x0fff;
            val as u16
        }
        #[doc = "Window Horizontal Stop Position"]
        #[inline(always)]
        pub fn set_whsppos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
        }
    }
    impl Default for Whpcr {
        #[inline(always)]
        fn default() -> Whpcr {
            Whpcr(0)
        }
    }
    #[doc = "Layerx Window Vertical Position Configuration Register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wvpcr(pub u32);
    impl Wvpcr {
        #[doc = "Window Vertical Start Position"]
        #[inline(always)]
        pub const fn wvstpos(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x07ff;
            val as u16
        }
        #[doc = "Window Vertical Start Position"]
        #[inline(always)]
        pub fn set_wvstpos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
        }
        #[doc = "Window Vertical Stop Position"]
        #[inline(always)]
        pub const fn wvsppos(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Window Vertical Stop Position"]
        #[inline(always)]
        pub fn set_wvsppos(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
    }
    impl Default for Wvpcr {
        #[inline(always)]
        fn default() -> Wvpcr {
            Wvpcr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Bf1 {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "BF1 = constant alpha"]
        CONSTANT = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "BF1 = pixel alpha * constant alpha"]
        PIXEL = 0x07,
    }
    impl Bf1 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bf1 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bf1 {
        #[inline(always)]
        fn from(val: u8) -> Bf1 {
            Bf1::from_bits(val)
        }
    }
    impl From<Bf1> for u8 {
        #[inline(always)]
        fn from(val: Bf1) -> u8 {
            Bf1::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Bf2 {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        #[doc = "BF2 = 1 - constant alpha"]
        CONSTANT = 0x05,
        _RESERVED_6 = 0x06,
        #[doc = "BF2 = 1 - pixel alpha * constant alpha"]
        PIXEL = 0x07,
    }
    impl Bf2 {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bf2 {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bf2 {
        #[inline(always)]
        fn from(val: u8) -> Bf2 {
            Bf2::from_bits(val)
        }
    }
    impl From<Bf2> for u8 {
        #[inline(always)]
        fn from(val: Bf2) -> u8 {
            Bf2::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cfuif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears the FUIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Cfuif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cfuif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cfuif {
        #[inline(always)]
        fn from(val: u8) -> Cfuif {
            Cfuif::from_bits(val)
        }
    }
    impl From<Cfuif> for u8 {
        #[inline(always)]
        fn from(val: Cfuif) -> u8 {
            Cfuif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Clif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears the LIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Clif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Clif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Clif {
        #[inline(always)]
        fn from(val: u8) -> Clif {
            Clif::from_bits(val)
        }
    }
    impl From<Clif> for u8 {
        #[inline(always)]
        fn from(val: Clif) -> u8 {
            Clif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Crrif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears the RRIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Crrif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Crrif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Crrif {
        #[inline(always)]
        fn from(val: u8) -> Crrif {
            Crrif::from_bits(val)
        }
    }
    impl From<Crrif> for u8 {
        #[inline(always)]
        fn from(val: Crrif) -> u8 {
            Crrif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cterrif {
        _RESERVED_0 = 0x0,
        #[doc = "Clears the TERRIF flag in the ISR register"]
        CLEAR = 0x01,
    }
    impl Cterrif {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cterrif {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cterrif {
        #[inline(always)]
        fn from(val: u8) -> Cterrif {
            Cterrif::from_bits(val)
        }
    }
    impl From<Cterrif> for u8 {
        #[inline(always)]
        fn from(val: Cterrif) -> u8 {
            Cterrif::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Depol {
        #[doc = "Data enable polarity is active low"]
        ACTIVELOW = 0x0,
        #[doc = "Data enable polarity is active high"]
        ACTIVEHIGH = 0x01,
    }
    impl Depol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Depol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Depol {
        #[inline(always)]
        fn from(val: u8) -> Depol {
            Depol::from_bits(val)
        }
    }
    impl From<Depol> for u8 {
        #[inline(always)]
        fn from(val: Depol) -> u8 {
            Depol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Hspol {
        #[doc = "Horizontal synchronization polarity is active low"]
        ACTIVELOW = 0x0,
        #[doc = "Horizontal synchronization polarity is active high"]
        ACTIVEHIGH = 0x01,
    }
    impl Hspol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Hspol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Hspol {
        #[inline(always)]
        fn from(val: u8) -> Hspol {
            Hspol::from_bits(val)
        }
    }
    impl From<Hspol> for u8 {
        #[inline(always)]
        fn from(val: Hspol) -> u8 {
            Hspol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Imr {
        #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
        NOEFFECT = 0x0,
        #[doc = "The shadow registers are reloaded immediately. This bit is set by software and cleared only by hardware after reload"]
        RELOAD = 0x01,
    }
    impl Imr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Imr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Imr {
        #[inline(always)]
        fn from(val: u8) -> Imr {
            Imr::from_bits(val)
        }
    }
    impl From<Imr> for u8 {
        #[inline(always)]
        fn from(val: Imr) -> u8 {
            Imr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pcpol {
        #[doc = "Pixel clock on rising edge"]
        RISINGEDGE = 0x0,
        #[doc = "Pixel clock on falling edge"]
        FALLINGEDGE = 0x01,
    }
    impl Pcpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pcpol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pcpol {
        #[inline(always)]
        fn from(val: u8) -> Pcpol {
            Pcpol::from_bits(val)
        }
    }
    impl From<Pcpol> for u8 {
        #[inline(always)]
        fn from(val: Pcpol) -> u8 {
            Pcpol::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pf {
        #[doc = "ARGB8888"]
        ARGB8888 = 0x0,
        #[doc = "RGB888"]
        RGB888 = 0x01,
        #[doc = "RGB565"]
        RGB565 = 0x02,
        #[doc = "ARGB1555"]
        ARGB1555 = 0x03,
        #[doc = "ARGB4444"]
        ARGB4444 = 0x04,
        #[doc = "L8 (8-bit luminance)"]
        L8 = 0x05,
        #[doc = "AL44 (4-bit alpha, 4-bit luminance)"]
        AL44 = 0x06,
        #[doc = "AL88 (8-bit alpha, 8-bit luminance)"]
        AL88 = 0x07,
    }
    impl Pf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pf {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pf {
        #[inline(always)]
        fn from(val: u8) -> Pf {
            Pf::from_bits(val)
        }
    }
    impl From<Pf> for u8 {
        #[inline(always)]
        fn from(val: Pf) -> u8 {
            Pf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Vbr {
        #[doc = "This bit is set by software and cleared only by hardware after reload (it cannot be cleared through register write once it is set)"]
        NOEFFECT = 0x0,
        #[doc = "The shadow registers are reloaded during the vertical blanking period (at the beginning of the first line after the active display area)."]
        RELOAD = 0x01,
    }
    impl Vbr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbr {
        #[inline(always)]
        fn from(val: u8) -> Vbr {
            Vbr::from_bits(val)
        }
    }
    impl From<Vbr> for u8 {
        #[inline(always)]
        fn from(val: Vbr) -> u8 {
            Vbr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Vspol {
        #[doc = "Vertical synchronization polarity is active low"]
        ACTIVELOW = 0x0,
        #[doc = "Vertical synchronization polarity is active high"]
        ACTIVEHIGH = 0x01,
    }
    impl Vspol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vspol {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vspol {
        #[inline(always)]
        fn from(val: u8) -> Vspol {
            Vspol::from_bits(val)
        }
    }
    impl From<Vspol> for u8 {
        #[inline(always)]
        fn from(val: Vspol) -> u8 {
            Vspol::to_bits(val)
        }
    }
}
