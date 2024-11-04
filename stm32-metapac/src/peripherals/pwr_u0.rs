#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "PWR register block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr {
    ptr: *mut u8,
}
unsafe impl Send for Pwr {}
unsafe impl Sync for Pwr {}
impl Pwr {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Power control register 1"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Power control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Power control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Power control register 4"]
    #[inline(always)]
    pub const fn cr4(self) -> crate::common::Reg<regs::Cr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Power status register 1"]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Power status register 2"]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Power status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Power Port A pull-up control register"]
    #[inline(always)]
    pub const fn pucra(self) -> crate::common::Reg<regs::Pucra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Power Port A pull-down control register"]
    #[inline(always)]
    pub const fn pdcra(self) -> crate::common::Reg<regs::Pdcra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Power Port B pull-up control register"]
    #[inline(always)]
    pub const fn pucrb(self) -> crate::common::Reg<regs::Pucrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Power Port B pull-down control register"]
    #[inline(always)]
    pub const fn pdcrb(self) -> crate::common::Reg<regs::Pdcrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Power Port C pull-up control register"]
    #[inline(always)]
    pub const fn pucrc(self) -> crate::common::Reg<regs::Pucrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Power Port C pull-down control register"]
    #[inline(always)]
    pub const fn pdcrc(self) -> crate::common::Reg<regs::Pdcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Power Port D pull-up control register"]
    #[inline(always)]
    pub const fn pucrd(self) -> crate::common::Reg<regs::Pucrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Power Port D pull-down control register"]
    #[inline(always)]
    pub const fn pdcrd(self) -> crate::common::Reg<regs::Pdcrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Power Port E pull-up control register"]
    #[inline(always)]
    pub const fn pucre(self) -> crate::common::Reg<regs::Pucre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Power Port E pull-down control register"]
    #[inline(always)]
    pub const fn pdcre(self) -> crate::common::Reg<regs::Pdcre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Power Port F pull-up control register"]
    #[inline(always)]
    pub const fn pucrf(self) -> crate::common::Reg<regs::Pucrf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Power Port F pull-down control register"]
    #[inline(always)]
    pub const fn pdcrf(self) -> crate::common::Reg<regs::Pdcrf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Power control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: If LPR bit is set, Stop 2 mode cannot be selected and Stop 1 mode shall be entered instead of Stop 2. Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "Low-power mode selection These bits select the low-power mode entered when CPU enters the deepsleep mode. 1xx: Shutdown mode Note: If LPR bit is set, Stop 2 mode cannot be selected and Stop 1 mode shall be entered instead of Stop 2. Note: In Standby mode, SRAM2 can be preserved or not, depending on RRS bit configuration in PWR_CR3."]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash memory powered down during Stop mode. This bit determines whether the flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
        #[inline(always)]
        pub const fn fpd_stop(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory powered down during Stop mode. This bit determines whether the flash memory is put in power-down mode or remains in idle mode when the device enters Stop mode."]
        #[inline(always)]
        pub fn set_fpd_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Flash memory powered down during Low-power run mode. This bit determines whether the flash memory is put in power-down mode or remains in idle mode when the device enters Low-power sleep mode."]
        #[inline(always)]
        pub const fn fpd_lprun(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory powered down during Low-power run mode. This bit determines whether the flash memory is put in power-down mode or remains in idle mode when the device enters Low-power sleep mode."]
        #[inline(always)]
        pub fn set_fpd_lprun(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Flash memory powered down during Low-power sleep mode. This bit determines whether the flash memory is put in power-down mode or remains in idle mode when the device enters Low-power sleep mode."]
        #[inline(always)]
        pub const fn fpd_lpslp(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory powered down during Low-power sleep mode. This bit determines whether the flash memory is put in power-down mode or remains in idle mode when the device enters Low-power sleep mode."]
        #[inline(always)]
        pub fn set_fpd_lpslp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection In reset state, the RTC and backup registers are protected against parasitic write access. This bit must be set to enable write access to these registers."]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub const fn vos(&self) -> super::vals::Vos {
            let val = (self.0 >> 9usize) & 0x03;
            super::vals::Vos::from_bits(val as u8)
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub fn set_vos(&mut self, val: super::vals::Vos) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
        }
        #[doc = "Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR). Note: Stop 2 mode cannot be entered when LPR bit is set. Stop 1 is entered instead."]
        #[inline(always)]
        pub const fn lpr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power run When this bit is set, the regulator is switched from main mode (MR) to low-power mode (LPR). Note: Stop 2 mode cannot be entered when LPR bit is set. Stop 1 is entered instead."]
        #[inline(always)]
        pub fn set_lpr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Cr1 {
        #[inline(always)]
        fn default() -> Cr1 {
            Cr1(0)
        }
    }
    #[doc = "Power control register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr2(pub u32);
    impl Cr2 {
        #[doc = "Programmable voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset."]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable voltage detector enable Note: This bit is write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: This bit is reset only by a system reset."]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Programmable voltage detector level selection. These bits select the voltage threshold detected by the programmable voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset."]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::Pls::from_bits(val as u8)
        }
        #[doc = "Programmable voltage detector level selection. These bits select the voltage threshold detected by the programmable voltage detector: Note: These bits are write-protected when the bit PVDL (PVD Lock) is set in the SYSCFG_CBR register. Note: These bits are reset only by a system reset."]
        #[inline(always)]
        pub fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "Peripheral voltage monitoring 1 enable: V<sub>DDUSB</sub> vs. 1.21V"]
        #[inline(always)]
        pub const fn pvme1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 1 enable: V<sub>DDUSB</sub> vs. 1.21V"]
        #[inline(always)]
        pub fn set_pvme1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. 1.621V"]
        #[inline(always)]
        pub const fn pvme3(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 3 enable: V<sub>DDA</sub> vs. 1.621V"]
        #[inline(always)]
        pub fn set_pvme3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. 1.861V"]
        #[inline(always)]
        pub const fn pvme4(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 4 enable: V<sub>DDA</sub> vs. 1.861V"]
        #[inline(always)]
        pub fn set_pvme4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "V<sub>DDUSB</sub> USB supply valid This bit is used to validate the V<sub>DDUSB</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB FS peripheral. If V<sub>DDUSB</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not."]
        #[inline(always)]
        pub const fn usv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>DDUSB</sub> USB supply valid This bit is used to validate the V<sub>DDUSB</sub> supply for electrical and logical isolation purpose. Setting this bit is mandatory to use the USB FS peripheral. If V<sub>DDUSB</sub> is not always present in the application, the PVM can be used to determine whether this supply is ready or not."]
        #[inline(always)]
        pub fn set_usv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Cr2 {
        #[inline(always)]
        fn default() -> Cr2 {
            Cr2(0)
        }
    }
    #[doc = "Power control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr3(pub u32);
    impl Cr3 {
        #[doc = "Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub const fn ewup1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wake-up pin WKUP1 When this bit is set, the external wake-up pin WKUP1 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP1 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub fn set_ewup1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub const fn ewup2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wake-up pin WKUP2 When this bit is set, the external wake-up pin WKUP2 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP2 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub fn set_ewup2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub const fn ewup3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wake-up pin WKUP3 When this bit is set, the external wake-up pin WKUP3 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP3 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub fn set_ewup3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub const fn ewup4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wake-up pin WKUP4 When this bit is set, the external wake-up pin WKUP4 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs. The active edge is configured via the WP4 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub fn set_ewup4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub const fn ewup5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wake-up pin WKUP5 When this bit is set, the external wake-up pin WKUP5 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP5 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub fn set_ewup5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub const fn ewup7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wake-up pin WKUP7. When this bit is set, the external wake-up pin WKUP7 is enabled and triggers a wake-up from Standby or Shutdown event when a rising or a falling edge occurs.The active edge is configured via the WP7 bit in the PWR_CR4 register."]
        #[inline(always)]
        pub fn set_ewup7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "SRAM2 retention in Standby mode"]
        #[inline(always)]
        pub const fn rrs(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 retention in Standby mode"]
        #[inline(always)]
        pub fn set_rrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes."]
        #[inline(always)]
        pub const fn enulp(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable ULP sampling When this bit is set, the BORL, BORH and PVD are periodically sampled instead continuous monitoring to reduce power consumption. Fast supply drop between two sample/compare phases is not detected in this mode. This bit has impact only on STOP2, Standby and shutdown low power modes."]
        #[inline(always)]
        pub fn set_enulp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode."]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration When this bit is set, the I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied. When this bit is cleared, the PWR_PUCRx and PWR_PDCRx registers are not applied to the I/Os, instead the I/Os are in floating mode during Standby or configured according GPIO controller GPIOx_PUPDR register during RUN mode."]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable internal wake-up line"]
        #[inline(always)]
        pub const fn eiwul(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable internal wake-up line"]
        #[inline(always)]
        pub fn set_eiwul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr3 {
        #[inline(always)]
        fn default() -> Cr3 {
            Cr3(0)
        }
    }
    #[doc = "Power control register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr4(pub u32);
    impl Cr4 {
        #[doc = "Wake-up pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
        #[inline(always)]
        pub const fn wp1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up pin WKUP1 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP1"]
        #[inline(always)]
        pub fn set_wp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wake-up pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
        #[inline(always)]
        pub const fn wp2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up pin WKUP2 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP2"]
        #[inline(always)]
        pub fn set_wp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wake-up pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
        #[inline(always)]
        pub const fn wp3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up pin WKUP3 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP3"]
        #[inline(always)]
        pub fn set_wp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wake-up pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
        #[inline(always)]
        pub const fn wp4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up pin WKUP4 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP4"]
        #[inline(always)]
        pub fn set_wp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wake-up pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
        #[inline(always)]
        pub const fn wp5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up pin WKUP5 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP5"]
        #[inline(always)]
        pub fn set_wp5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Wake-up pin WKUP7 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP7"]
        #[inline(always)]
        pub const fn wp7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up pin WKUP7 polarity This bit defines the polarity used for an event detection on external wake-up pin, WKUP7"]
        #[inline(always)]
        pub fn set_wp7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "V<sub>BAT</sub> battery charging enable"]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>BAT</sub> battery charging enable"]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "V<sub>BAT</sub> battery charging resistor selection"]
        #[inline(always)]
        pub const fn vbrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "V<sub>BAT</sub> battery charging resistor selection"]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Cr4 {
        #[inline(always)]
        fn default() -> Cr4 {
            Cr4(0)
        }
    }
    #[doc = "Power Port A pull-down control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcra(pub u32);
    impl Pdcra {
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y When set, this bit activates the pull-down on PA\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pdcra {
        #[inline(always)]
        fn default() -> Pdcra {
            Pdcra(0)
        }
    }
    #[doc = "Power Port B pull-down control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrb(pub u32);
    impl Pdcrb {
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y When set, this bit activates the pull-down on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pdcrb {
        #[inline(always)]
        fn default() -> Pdcrb {
            Pdcrb(0)
        }
    }
    #[doc = "Power Port C pull-down control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrc(pub u32);
    impl Pdcrc {
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y When set, this bit activates the pull-down on PC\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pdcrc {
        #[inline(always)]
        fn default() -> Pdcrc {
            Pdcrc(0)
        }
    }
    #[doc = "Power Port D pull-down control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrd(pub u32);
    impl Pdcrd {
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y When set, this bit activates the pull-down on PD\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Pdcrd {
        #[inline(always)]
        fn default() -> Pdcrd {
            Pdcrd(0)
        }
    }
    #[doc = "Power Port E pull-down control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcre(pub u32);
    impl Pdcre {
        #[doc = "Port E pull-down bit 3 When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-down bit 3 When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-down bit y When set, this bit activates the pull-down on PE\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Pdcre {
        #[inline(always)]
        fn default() -> Pdcre {
            Pdcre(0)
        }
    }
    #[doc = "Power Port F pull-down control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrf(pub u32);
    impl Pdcrf {
        #[doc = "Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port F pull-down bit y When set, this bit activates the pull-down on PH\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Pdcrf {
        #[inline(always)]
        fn default() -> Pdcrf {
            Pdcrf(0)
        }
    }
    #[doc = "Power Port A pull-up control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucra(pub u32);
    impl Pucra {
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y1=115 to 0) When set, this bit activates the pull-up on PA\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pucra {
        #[inline(always)]
        fn default() -> Pucra {
            Pucra(0)
        }
    }
    #[doc = "Power Port B pull-up control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrb(pub u32);
    impl Pucrb {
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y When set, this bit activates the pull-up on PB\\[y\\]
when APC bit is set in PWR_CR3 register."]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pucrb {
        #[inline(always)]
        fn default() -> Pucrb {
            Pucrb(0)
        }
    }
    #[doc = "Power Port C pull-up control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrc(pub u32);
    impl Pucrc {
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y When set, this bit activates the pull-up on PC\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Pucrc {
        #[inline(always)]
        fn default() -> Pucrc {
            Pucrc(0)
        }
    }
    #[doc = "Power Port D pull-up control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrd(pub u32);
    impl Pucrd {
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y When set, this bit activates the pull-up on PD\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
    }
    impl Default for Pucrd {
        #[inline(always)]
        fn default() -> Pucrd {
            Pucrd(0)
        }
    }
    #[doc = "Power Port E pull-up control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucre(pub u32);
    impl Pucre {
        #[doc = "Port E pull-up bit 3 When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-up bit 3 When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-up bit y When set, this bit activates the pull-up on PE\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Pucre {
        #[inline(always)]
        fn default() -> Pucre {
            Pucre(0)
        }
    }
    #[doc = "Power Port F pull-up control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrf(pub u32);
    impl Pucrf {
        #[doc = "Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port F pull-up bit y When set, this bit activates the pull-up on PH\\[y\\]
when APC bit is set in PWR_CR3 register. If the corresponding PDy bit is also set, the pull-up is not activated and the pull-down is activated instead with highest priority."]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Pucrf {
        #[inline(always)]
        fn default() -> Pucrf {
            Pucrf(0)
        }
    }
    #[doc = "Power status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear wake-up flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub const fn cwuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wake-up flag 1 Setting this bit clears the WUF1 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub fn set_cwuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear wake-up flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub const fn cwuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wake-up flag 2 Setting this bit clears the WUF2 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub fn set_cwuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear wake-up flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub const fn cwuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wake-up flag 3 Setting this bit clears the WUF3 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub fn set_cwuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear wake-up flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub const fn cwuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wake-up flag 4 Setting this bit clears the WUF4 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub fn set_cwuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear wake-up flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub const fn cwuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wake-up flag 5 Setting this bit clears the WUF5 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub fn set_cwuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear wake-up flag 7 Setting this bit clears the WUF7 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub const fn cwuf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wake-up flag 7 Setting this bit clears the WUF7 flag in the PWR_SR1 register."]
        #[inline(always)]
        pub fn set_cwuf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
        #[inline(always)]
        pub const fn csbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear standby flag Setting this bit clears the SBF flag in the PWR_SR1 register."]
        #[inline(always)]
        pub fn set_csbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    #[doc = "Power status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Wake-up flag 1 This bit is set when a wake-up event is detected on wake-up pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register."]
        #[inline(always)]
        pub const fn wuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up flag 1 This bit is set when a wake-up event is detected on wake-up pin, WKUP1. It is cleared by writing 1 in the CWUF1 bit of the PWR_SCR register."]
        #[inline(always)]
        pub fn set_wuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wake-up flag 2 This bit is set when a wake-up event is detected on wake-up pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register."]
        #[inline(always)]
        pub const fn wuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up flag 2 This bit is set when a wake-up event is detected on wake-up pin, WKUP2. It is cleared by writing 1 in the CWUF2 bit of the PWR_SCR register."]
        #[inline(always)]
        pub fn set_wuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wake-up flag 3 This bit is set when a wake-up event is detected on wake-up pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register."]
        #[inline(always)]
        pub const fn wuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up flag 3 This bit is set when a wake-up event is detected on wake-up pin, WKUP3. It is cleared by writing 1 in the CWUF3 bit of the PWR_SCR register."]
        #[inline(always)]
        pub fn set_wuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wake-up flag 4 This bit is set when a wake-up event is detected on wake-up pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register."]
        #[inline(always)]
        pub const fn wuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up flag 4 This bit is set when a wake-up event is detected on wake-up pin,WKUP4. It is cleared by writing 1 in the CWUF4 bit of the PWR_SCR register."]
        #[inline(always)]
        pub fn set_wuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wake-up flag 5 This bit is set when a wake-up event is detected on wake-up pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register."]
        #[inline(always)]
        pub const fn wuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up flag 5 This bit is set when a wake-up event is detected on wake-up pin, WKUP5. It is cleared by writing 1 in the CWUF5 bit of the PWR_SCR register."]
        #[inline(always)]
        pub fn set_wuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Wake-up flag 7 This bit is set when a wake-up event is detected on wake-up pin, WKUP7. It is cleared by writing 1 in the CWUF7 bit of the PWR_SCR register."]
        #[inline(always)]
        pub const fn wuf7(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up flag 7 This bit is set when a wake-up event is detected on wake-up pin, WKUP7. It is cleared by writing 1 in the CWUF7 bit of the PWR_SCR register."]
        #[inline(always)]
        pub fn set_wuf7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub const fn sbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Standby flag This bit is set by hardware when the device enters the Standby mode and is cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub fn set_sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Stop Flags These bits are set by hardware when the device enters any stop mode and are cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub const fn stopf(&self) -> super::vals::Stopf {
            let val = (self.0 >> 9usize) & 0x07;
            super::vals::Stopf::from_bits(val as u8)
        }
        #[doc = "Stop Flags These bits are set by hardware when the device enters any stop mode and are cleared by setting the CSBF bit in the PWR_SCR register, or by a power-on reset. It is not cleared by the system reset."]
        #[inline(always)]
        pub fn set_stopf(&mut self, val: super::vals::Stopf) {
            self.0 = (self.0 & !(0x07 << 9usize)) | (((val.to_bits() as u32) & 0x07) << 9usize);
        }
        #[doc = "Wake-up flag internal This bit is set when a wake-up is detected on the internal wake-up line. It is cleared when all internal wake-up sources are cleared."]
        #[inline(always)]
        pub const fn wufi(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Wake-up flag internal This bit is set when a wake-up is detected on the internal wake-up line. It is cleared when all internal wake-up sources are cleared."]
        #[inline(always)]
        pub fn set_wufi(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Sr1 {
        #[inline(always)]
        fn default() -> Sr1 {
            Sr1(0)
        }
    }
    #[doc = "Power status register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr2(pub u32);
    impl Sr2 {
        #[doc = "Flash ready flag This bit is set by hardware to indicate when the flash memory is readey to be accessed after wake-up from power-down. To place the flash memory in power-down, set either FPD_LPRUN, FPD_LPSLP or FPD_STP bits. Note : If the system boots from SRAM, the user application must wait until the FLASH_RDY bit is set, prior to jumping to flash memory."]
        #[inline(always)]
        pub const fn flash_rdy(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Flash ready flag This bit is set by hardware to indicate when the flash memory is readey to be accessed after wake-up from power-down. To place the flash memory in power-down, set either FPD_LPRUN, FPD_LPSLP or FPD_STP bits. Note : If the system boots from SRAM, the user application must wait until the FLASH_RDY bit is set, prior to jumping to flash memory."]
        #[inline(always)]
        pub fn set_flash_rdy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wake-up from Standby mode time may be increased."]
        #[inline(always)]
        pub const fn reglps(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power regulator started This bit provides the information whether the low-power regulator is ready after a power-on reset or a Standby/Shutdown. If the Standby mode is entered while REGLPS bit is still cleared, the wake-up from Standby mode time may be increased."]
        #[inline(always)]
        pub fn set_reglps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits from the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready."]
        #[inline(always)]
        pub const fn reglpf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power regulator flag This bit is set by hardware when the MCU is in Low-power run mode. When the MCU exits from the Low-power run mode, this bit remains at 1 until the regulator is ready in main mode. A polling on this bit must be done before increasing the product frequency. This bit is cleared by hardware when the regulator is ready."]
        #[inline(always)]
        pub fn set_reglpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register."]
        #[inline(always)]
        pub const fn vosf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage scaling flag A delay is required for the internal regulator to be ready after the voltage scaling has been changed. VOSF indicates that the regulator reached the voltage level defined with VOS bits of the PWR_CR1 register."]
        #[inline(always)]
        pub fn set_vosf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Programmable voltage detector output"]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Programmable voltage detector output"]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Peripheral voltage monitoring output: V<sub>DDUSB</sub> vs. 1.2 V Note: PVMO1 is cleared when PVM1 is disabled (PVME1 = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wake-up time."]
        #[inline(always)]
        pub const fn pvmo1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: V<sub>DDUSB</sub> vs. 1.2 V Note: PVMO1 is cleared when PVM1 is disabled (PVME1 = 0). After enabling PVM1, the PVM1 output is valid after the PVM1 wake-up time."]
        #[inline(always)]
        pub fn set_pvmo1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.621V Note: PVMO3 is cleared when PVM3 is disabled (PVME3 = 0). After enabling PVM3, the PVM3 output is valid after the PVM3 wake-up time."]
        #[inline(always)]
        pub const fn pvmo3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 1.621V Note: PVMO3 is cleared when PVM3 is disabled (PVME3 = 0). After enabling PVM3, the PVM3 output is valid after the PVM3 wake-up time."]
        #[inline(always)]
        pub fn set_pvmo3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 2.21V Note: PVMO4 is cleared when PVM4 is disabled (PVME4 = 0). After enabling PVM4, the PVM4 output is valid after the PVM4 wake-up time."]
        #[inline(always)]
        pub const fn pvmo4(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: V<sub>DDA</sub> vs. 2.21V Note: PVMO4 is cleared when PVM4 is disabled (PVME4 = 0). After enabling PVM4, the PVM4 output is valid after the PVM4 wake-up time."]
        #[inline(always)]
        pub fn set_pvmo4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Sr2 {
        #[inline(always)]
        fn default() -> Sr2 {
            Sr2(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lpms {
        #[doc = "Stop 0 mode"]
        STOP0 = 0x0,
        #[doc = "Stop 1 mode"]
        STOP1 = 0x01,
        #[doc = "Stop 2 mode"]
        STOP2 = 0x02,
        #[doc = "Standby mode"]
        STANDBY = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Lpms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpms {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpms {
        #[inline(always)]
        fn from(val: u8) -> Lpms {
            Lpms::from_bits(val)
        }
    }
    impl From<Lpms> for u8 {
        #[inline(always)]
        fn from(val: Lpms) -> u8 {
            Lpms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pls {
        #[doc = "V<sub>PVD0</sub> around 2.01V"]
        B_0X0 = 0x0,
        #[doc = "V<sub>PVD1</sub> around 2.21V"]
        B_0X1 = 0x01,
        #[doc = "V<sub>PVD2</sub> around 2.41V"]
        B_0X2 = 0x02,
        #[doc = "V<sub>PVD3</sub> around 2.51V"]
        B_0X3 = 0x03,
        #[doc = "V<sub>PVD4</sub> around 2.61V"]
        B_0X4 = 0x04,
        #[doc = "V<sub>PVD5</sub> around 2.81V"]
        B_0X5 = 0x05,
        #[doc = "V<sub>PVD6</sub> around 2.91V"]
        B_0X6 = 0x06,
        #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
        B_0X7 = 0x07,
    }
    impl Pls {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pls {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pls {
        #[inline(always)]
        fn from(val: u8) -> Pls {
            Pls::from_bits(val)
        }
    }
    impl From<Pls> for u8 {
        #[inline(always)]
        fn from(val: Pls) -> u8 {
            Pls::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Stopf {
        #[doc = "The device did not enter any Stop mode."]
        NONE = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        #[doc = "The device entered in Stop 0 mode."]
        STOP0 = 0x04,
        #[doc = "The device entered in Stop 1 mode."]
        STOP1 = 0x05,
        #[doc = "The device entered in Stop 2 mode."]
        STOP2 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl Stopf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Stopf {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Stopf {
        #[inline(always)]
        fn from(val: u8) -> Stopf {
            Stopf::from_bits(val)
        }
    }
    impl From<Stopf> for u8 {
        #[inline(always)]
        fn from(val: Stopf) -> u8 {
            Stopf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Vos {
        _RESERVED_0 = 0x0,
        #[doc = "Range 1"]
        RANGE1 = 0x01,
        #[doc = "Range 2"]
        RANGE2 = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Vos {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vos {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vos {
        #[inline(always)]
        fn from(val: u8) -> Vos {
            Vos::from_bits(val)
        }
    }
    impl From<Vos> for u8 {
        #[inline(always)]
        fn from(val: Vos) -> u8 {
            Vos::to_bits(val)
        }
    }
}
