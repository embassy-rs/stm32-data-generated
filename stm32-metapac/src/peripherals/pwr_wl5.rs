#[doc = "Power control"]
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Power control register 2"]
    #[inline(always)]
    pub const fn cr2(self) -> crate::common::Reg<regs::Cr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Power control register 3"]
    #[inline(always)]
    pub const fn cr3(self) -> crate::common::Reg<regs::Cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Power control register 4"]
    #[inline(always)]
    pub const fn cr4(self) -> crate::common::Reg<regs::Cr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Power status register 1"]
    #[inline(always)]
    pub const fn sr1(self) -> crate::common::Reg<regs::Sr1, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Power status register 2"]
    #[inline(always)]
    pub const fn sr2(self) -> crate::common::Reg<regs::Sr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Power status clear register"]
    #[inline(always)]
    pub const fn scr(self) -> crate::common::Reg<regs::Scr, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Power control register 5"]
    #[inline(always)]
    pub const fn cr5(self) -> crate::common::Reg<regs::Cr5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Power Port pull-up control register"]
    #[inline(always)]
    pub const fn pucr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize + n * 8usize) as _) }
    }
    #[doc = "Power Port pull-down control register"]
    #[inline(always)]
    pub const fn pdcr(self, n: usize) -> crate::common::Reg<regs::Pcr, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize + n * 8usize) as _) }
    }
    #[doc = "Power CPU2 control register 1 \\[dual core device only\\]"]
    #[inline(always)]
    pub const fn c2cr1(self) -> crate::common::Reg<regs::C2cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Power CPU2 control register 3 \\[dual core device only\\]"]
    #[inline(always)]
    pub const fn c2cr3(self) -> crate::common::Reg<regs::C2cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Power extended status and status clear register"]
    #[inline(always)]
    pub const fn extscr(self) -> crate::common::Reg<regs::Extscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Power security configuration register \\[dual core device only\\]"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "Power SPI3 control register"]
    #[inline(always)]
    pub const fn subghzspicr(self) -> crate::common::Reg<regs::Subghzspicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "RSS Command register \\[dual core device only\\]"]
    #[inline(always)]
    pub const fn rsscmdr(self) -> crate::common::Reg<regs::Rsscmdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
}
pub mod regs {
    #[doc = "Power CPU2 control register 1 \\[dual core device only\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2cr1(pub u32);
    impl C2cr1 {
        #[doc = "Low-power mode selection for CPU2"]
        #[inline(always)]
        pub const fn lpms(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Low-power mode selection for CPU2"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash memory power down mode during LPRun for CPU2"]
        #[inline(always)]
        pub const fn fpdr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory power down mode during LPRun for CPU2"]
        #[inline(always)]
        pub fn set_fpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Flash memory power down mode during LPSleep for CPU2"]
        #[inline(always)]
        pub const fn fpds(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory power down mode during LPSleep for CPU2"]
        #[inline(always)]
        pub fn set_fpds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for C2cr1 {
        #[inline(always)]
        fn default() -> C2cr1 {
            C2cr1(0)
        }
    }
    #[doc = "Power CPU2 control register 3 \\[dual core device only\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2cr3(pub u32);
    impl C2cr3 {
        #[doc = "Enable Wakeup pin WKUP1 for CPU2"]
        #[inline(always)]
        pub const fn ewup(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP1 for CPU2"]
        #[inline(always)]
        pub fn set_ewup(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Enable wakeup PVD for CPU2"]
        #[inline(always)]
        pub const fn ewpvd(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable wakeup PVD for CPU2"]
        #[inline(always)]
        pub fn set_ewpvd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Apply pull-up and pull-down configuration for CPU2"]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration for CPU2"]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "EWRFBUSY"]
        #[inline(always)]
        pub const fn ewrfbusy(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "EWRFBUSY"]
        #[inline(always)]
        pub fn set_ewrfbusy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "akeup for CPU2"]
        #[inline(always)]
        pub const fn ewrfirq(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "akeup for CPU2"]
        #[inline(always)]
        pub fn set_ewrfirq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable internal wakeup line for CPU2"]
        #[inline(always)]
        pub const fn eiwul(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable internal wakeup line for CPU2"]
        #[inline(always)]
        pub fn set_eiwul(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for C2cr3 {
        #[inline(always)]
        fn default() -> C2cr3 {
            C2cr3(0)
        }
    }
    #[doc = "Power control register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr1(pub u32);
    impl Cr1 {
        #[doc = "Low-power mode selection for CPU1"]
        #[inline(always)]
        pub const fn lpms(&self) -> super::vals::Lpms {
            let val = (self.0 >> 0usize) & 0x07;
            super::vals::Lpms::from_bits(val as u8)
        }
        #[doc = "Low-power mode selection for CPU1"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: super::vals::Lpms) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
        }
        #[doc = "sub-GHz SPI NSS source select"]
        #[inline(always)]
        pub const fn subghzspinsssel(&self) -> super::vals::Subghzspinsssel {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Subghzspinsssel::from_bits(val as u8)
        }
        #[doc = "sub-GHz SPI NSS source select"]
        #[inline(always)]
        pub fn set_subghzspinsssel(&mut self, val: super::vals::Subghzspinsssel) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "Flash memory power down mode during LPRun for CPU1"]
        #[inline(always)]
        pub const fn fpdr(&self) -> super::vals::Fpdr {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Fpdr::from_bits(val as u8)
        }
        #[doc = "Flash memory power down mode during LPRun for CPU1"]
        #[inline(always)]
        pub fn set_fpdr(&mut self, val: super::vals::Fpdr) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Flash memory power down mode during LPSleep for CPU1"]
        #[inline(always)]
        pub const fn fpds(&self) -> super::vals::Fpds {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Fpds::from_bits(val as u8)
        }
        #[doc = "Flash memory power down mode during LPSleep for CPU1"]
        #[inline(always)]
        pub fn set_fpds(&mut self, val: super::vals::Fpds) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub const fn dbp(&self) -> super::vals::Dbp {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Dbp::from_bits(val as u8)
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: super::vals::Dbp) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
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
        #[doc = "Low-power run"]
        #[inline(always)]
        pub const fn lpr(&self) -> super::vals::Lpr {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Lpr::from_bits(val as u8)
        }
        #[doc = "Low-power run"]
        #[inline(always)]
        pub fn set_lpr(&mut self, val: super::vals::Lpr) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
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
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub const fn pvde(&self) -> super::vals::Pvde {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Pvde::from_bits(val as u8)
        }
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: super::vals::Pvde) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Power voltage detector level selection."]
        #[inline(always)]
        pub const fn pls(&self) -> super::vals::Pls {
            let val = (self.0 >> 1usize) & 0x07;
            super::vals::Pls::from_bits(val as u8)
        }
        #[doc = "Power voltage detector level selection."]
        #[inline(always)]
        pub fn set_pls(&mut self, val: super::vals::Pls) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
        }
        #[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
        #[inline(always)]
        pub const fn pvme(&self) -> super::vals::Pvme {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Pvme::from_bits(val as u8)
        }
        #[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
        #[inline(always)]
        pub fn set_pvme(&mut self, val: super::vals::Pvme) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
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
        #[doc = "Enable Wakeup pin WKUP1 for CPU1"]
        #[inline(always)]
        pub const fn ewup(&self, n: usize) -> super::vals::Ewup {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Ewup::from_bits(val as u8)
        }
        #[doc = "Enable Wakeup pin WKUP1 for CPU1"]
        #[inline(always)]
        pub fn set_ewup(&mut self, n: usize, val: super::vals::Ewup) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Ultra-low-power enable"]
        #[inline(always)]
        pub const fn eulpen(&self) -> super::vals::Eulpen {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Eulpen::from_bits(val as u8)
        }
        #[doc = "Ultra-low-power enable"]
        #[inline(always)]
        pub fn set_eulpen(&mut self, val: super::vals::Eulpen) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Enable wakeup PVD for CPU1"]
        #[inline(always)]
        pub const fn ewpvd(&self) -> super::vals::Ewpvd {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Ewpvd::from_bits(val as u8)
        }
        #[doc = "Enable wakeup PVD for CPU1"]
        #[inline(always)]
        pub fn set_ewpvd(&mut self, val: super::vals::Ewpvd) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM2 retention in Standby mode"]
        #[inline(always)]
        pub const fn rrs(&self) -> super::vals::Rrs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Rrs::from_bits(val as u8)
        }
        #[doc = "SRAM2 retention in Standby mode"]
        #[inline(always)]
        pub fn set_rrs(&mut self, val: super::vals::Rrs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Apply pull-up and pull-down configuration from CPU1"]
        #[inline(always)]
        pub const fn apc(&self) -> super::vals::Apc {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Apc::from_bits(val as u8)
        }
        #[doc = "Apply pull-up and pull-down configuration from CPU1"]
        #[inline(always)]
        pub fn set_apc(&mut self, val: super::vals::Apc) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable Radio BUSY Wakeup from Standby for CPU1"]
        #[inline(always)]
        pub const fn ewrfbusy(&self) -> super::vals::Ewrfbusy {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Ewrfbusy::from_bits(val as u8)
        }
        #[doc = "Enable Radio BUSY Wakeup from Standby for CPU1"]
        #[inline(always)]
        pub fn set_ewrfbusy(&mut self, val: super::vals::Ewrfbusy) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "akeup for CPU1"]
        #[inline(always)]
        pub const fn ewrfirq(&self) -> super::vals::Ewrfirq {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Ewrfirq::from_bits(val as u8)
        }
        #[doc = "akeup for CPU1"]
        #[inline(always)]
        pub fn set_ewrfirq(&mut self, val: super::vals::Ewrfirq) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "nable CPU2 Hold interrupt for CPU1"]
        #[inline(always)]
        pub const fn ec2h(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "nable CPU2 Hold interrupt for CPU1"]
        #[inline(always)]
        pub fn set_ec2h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable internal wakeup line for CPU1"]
        #[inline(always)]
        pub const fn eiwul(&self) -> super::vals::Eiwul {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Eiwul::from_bits(val as u8)
        }
        #[doc = "Enable internal wakeup line for CPU1"]
        #[inline(always)]
        pub fn set_eiwul(&mut self, val: super::vals::Eiwul) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
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
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub const fn wp(&self, n: usize) -> super::vals::Wp {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Wp::from_bits(val as u8)
        }
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub fn set_wp(&mut self, n: usize, val: super::vals::Wp) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "VBAT battery charging enable"]
        #[inline(always)]
        pub const fn vbe(&self) -> super::vals::Vbe {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Vbe::from_bits(val as u8)
        }
        #[doc = "VBAT battery charging enable"]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: super::vals::Vbe) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "VBAT battery charging resistor selection"]
        #[inline(always)]
        pub const fn vbrs(&self) -> super::vals::Vbrs {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Vbrs::from_bits(val as u8)
        }
        #[doc = "VBAT battery charging resistor selection"]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: super::vals::Vbrs) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Wakeup Radio BUSY polarity"]
        #[inline(always)]
        pub const fn wrfbusyp(&self) -> super::vals::Wrfbusyp {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wrfbusyp::from_bits(val as u8)
        }
        #[doc = "Wakeup Radio BUSY polarity"]
        #[inline(always)]
        pub fn set_wrfbusyp(&mut self, val: super::vals::Wrfbusyp) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "oot CPU2 after reset or wakeup from Stop or Standby modes."]
        #[inline(always)]
        pub const fn c2boot(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "oot CPU2 after reset or wakeup from Stop or Standby modes."]
        #[inline(always)]
        pub fn set_c2boot(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr4 {
        #[inline(always)]
        fn default() -> Cr4 {
            Cr4(0)
        }
    }
    #[doc = "Power control register 5"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr5(pub u32);
    impl Cr5 {
        #[doc = "Enable Radio End Of Life detector enabled"]
        #[inline(always)]
        pub const fn rfeolen(&self) -> super::vals::Rfeolen {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Rfeolen::from_bits(val as u8)
        }
        #[doc = "Enable Radio End Of Life detector enabled"]
        #[inline(always)]
        pub fn set_rfeolen(&mut self, val: super::vals::Rfeolen) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable SMPS Step Down converter SMPS mode enabled."]
        #[inline(always)]
        pub const fn smpsen(&self) -> super::vals::Smpsen {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Smpsen::from_bits(val as u8)
        }
        #[doc = "Enable SMPS Step Down converter SMPS mode enabled."]
        #[inline(always)]
        pub fn set_smpsen(&mut self, val: super::vals::Smpsen) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr5 {
        #[inline(always)]
        fn default() -> Cr5 {
            Cr5(0)
        }
    }
    #[doc = "Power extended status and status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Extscr(pub u32);
    impl Extscr {
        #[doc = "Clear CPU1 Stop Standby flags"]
        #[inline(always)]
        pub const fn c1cssf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear CPU1 Stop Standby flags"]
        #[inline(always)]
        pub fn set_c1cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "lear CPU2 Stop Standby flags"]
        #[inline(always)]
        pub const fn c2cssf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "lear CPU2 Stop Standby flags"]
        #[inline(always)]
        pub fn set_c2cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "System Standby flag for CPU1. (no core states retained)"]
        #[inline(always)]
        pub const fn c1sbf(&self) -> super::vals::Csbf {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Csbf::from_bits(val as u8)
        }
        #[doc = "System Standby flag for CPU1. (no core states retained)"]
        #[inline(always)]
        pub fn set_c1sbf(&mut self, val: super::vals::Csbf) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "System Stop2 flag for CPU1. (partial core states retained)"]
        #[inline(always)]
        pub const fn c1stop2f(&self) -> super::vals::Cstopf {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Cstopf::from_bits(val as u8)
        }
        #[doc = "System Stop2 flag for CPU1. (partial core states retained)"]
        #[inline(always)]
        pub fn set_c1stop2f(&mut self, val: super::vals::Cstopf) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "System Stop0, 1 flag for CPU1. (All core states retained)"]
        #[inline(always)]
        pub const fn c1stopf(&self) -> super::vals::Cstopf {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Cstopf::from_bits(val as u8)
        }
        #[doc = "System Stop0, 1 flag for CPU1. (All core states retained)"]
        #[inline(always)]
        pub fn set_c1stopf(&mut self, val: super::vals::Cstopf) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "ystem Standby flag for CPU2. (no core states retained)"]
        #[inline(always)]
        pub const fn c2sbf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "ystem Standby flag for CPU2. (no core states retained)"]
        #[inline(always)]
        pub fn set_c2sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "ystem Stop2 flag for CPU2. (partial core states retained)"]
        #[inline(always)]
        pub const fn c2stop2f(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "ystem Stop2 flag for CPU2. (partial core states retained)"]
        #[inline(always)]
        pub fn set_c2stop2f(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "ystem Stop0, 1 flag for CPU2. (All core states retained)"]
        #[inline(always)]
        pub const fn c2stopf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "ystem Stop0, 1 flag for CPU2. (All core states retained)"]
        #[inline(always)]
        pub fn set_c2stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CPU1 deepsleep mode"]
        #[inline(always)]
        pub const fn c1ds(&self) -> super::vals::Cds {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Cds::from_bits(val as u8)
        }
        #[doc = "CPU1 deepsleep mode"]
        #[inline(always)]
        pub fn set_c1ds(&mut self, val: super::vals::Cds) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "PU2 deepsleep mode"]
        #[inline(always)]
        pub const fn c2ds(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "PU2 deepsleep mode"]
        #[inline(always)]
        pub fn set_c2ds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Extscr {
        #[inline(always)]
        fn default() -> Extscr {
            Extscr(0)
        }
    }
    #[doc = "Power Port pull control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pcr(pub u32);
    impl Pcr {
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub const fn p(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Port pull bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_p(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Pcr {
        #[inline(always)]
        fn default() -> Pcr {
            Pcr(0)
        }
    }
    #[doc = "RSS Command register \\[dual core device only\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rsscmdr(pub u32);
    impl Rsscmdr {
        #[doc = "RSS command"]
        #[inline(always)]
        pub const fn rsscmd(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "RSS command"]
        #[inline(always)]
        pub fn set_rsscmd(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Rsscmdr {
        #[inline(always)]
        fn default() -> Rsscmdr {
            Rsscmdr(0)
        }
    }
    #[doc = "Power status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear wakeup flag 1"]
        #[inline(always)]
        pub const fn cwuf(&self, n: usize) -> bool {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag 1"]
        #[inline(always)]
        pub fn set_cwuf(&mut self, n: usize, val: bool) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
        #[doc = "Clear wakeup PVD interrupt flag"]
        #[inline(always)]
        pub const fn cwpvdf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup PVD interrupt flag"]
        #[inline(always)]
        pub fn set_cwpvdf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clear wakeup Radio BUSY flag"]
        #[inline(always)]
        pub const fn cwrfbusyf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup Radio BUSY flag"]
        #[inline(always)]
        pub fn set_cwrfbusyf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "lear CPU2 Hold interrupt flag"]
        #[inline(always)]
        pub const fn cc2hf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "lear CPU2 Hold interrupt flag"]
        #[inline(always)]
        pub fn set_cc2hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Scr {
        #[inline(always)]
        fn default() -> Scr {
            Scr(0)
        }
    }
    #[doc = "Power security configuration register \\[dual core device only\\]"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "wakeup on CPU2 illegal access interrupt enable"]
        #[inline(always)]
        pub const fn c2ewila(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "wakeup on CPU2 illegal access interrupt enable"]
        #[inline(always)]
        pub fn set_c2ewila(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
    #[doc = "Power status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Wakeup flag 1"]
        #[inline(always)]
        pub const fn wuf(&self, n: usize) -> super::vals::Wuf {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            super::vals::Wuf::from_bits(val as u8)
        }
        #[doc = "Wakeup flag 1"]
        #[inline(always)]
        pub fn set_wuf(&mut self, n: usize, val: super::vals::Wuf) {
            assert!(n < 3usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val.to_bits() as u32) & 0x01) << offs);
        }
        #[doc = "Wakeup PVD flag"]
        #[inline(always)]
        pub const fn wpvdf(&self) -> super::vals::Wpvdf {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Wpvdf::from_bits(val as u8)
        }
        #[doc = "Wakeup PVD flag"]
        #[inline(always)]
        pub fn set_wpvdf(&mut self, val: super::vals::Wpvdf) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "Radio BUSY wakeup flag"]
        #[inline(always)]
        pub const fn wrfbusyf(&self) -> super::vals::Wrfbusyf {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Wrfbusyf::from_bits(val as u8)
        }
        #[doc = "Radio BUSY wakeup flag"]
        #[inline(always)]
        pub fn set_wrfbusyf(&mut self, val: super::vals::Wrfbusyf) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "PU2 Hold interrupt flag"]
        #[inline(always)]
        pub const fn c2hf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "PU2 Hold interrupt flag"]
        #[inline(always)]
        pub fn set_c2hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Internal wakeup interrupt flag"]
        #[inline(always)]
        pub const fn wufi(&self) -> super::vals::Wufi {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Wufi::from_bits(val as u8)
        }
        #[doc = "Internal wakeup interrupt flag"]
        #[inline(always)]
        pub fn set_wufi(&mut self, val: super::vals::Wufi) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
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
        #[doc = "PU2 boot/wakeup request source information"]
        #[inline(always)]
        pub const fn c2boots(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "PU2 boot/wakeup request source information"]
        #[inline(always)]
        pub fn set_c2boots(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Radio BUSY signal status"]
        #[inline(always)]
        pub const fn rfbusys(&self) -> super::vals::Rfbusys {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Rfbusys::from_bits(val as u8)
        }
        #[doc = "Radio BUSY signal status"]
        #[inline(always)]
        pub fn set_rfbusys(&mut self, val: super::vals::Rfbusys) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Radio BUSY masked signal status"]
        #[inline(always)]
        pub const fn rfbusyms(&self) -> super::vals::Rfbusyms {
            let val = (self.0 >> 2usize) & 0x01;
            super::vals::Rfbusyms::from_bits(val as u8)
        }
        #[doc = "Radio BUSY masked signal status"]
        #[inline(always)]
        pub fn set_rfbusyms(&mut self, val: super::vals::Rfbusyms) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
        }
        #[doc = "SMPS ready flag"]
        #[inline(always)]
        pub const fn smpsrdy(&self) -> super::vals::Smpsrdy {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::Smpsrdy::from_bits(val as u8)
        }
        #[doc = "SMPS ready flag"]
        #[inline(always)]
        pub fn set_smpsrdy(&mut self, val: super::vals::Smpsrdy) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "LDO ready flag"]
        #[inline(always)]
        pub const fn ldordy(&self) -> super::vals::Ldordy {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::Ldordy::from_bits(val as u8)
        }
        #[doc = "LDO ready flag"]
        #[inline(always)]
        pub fn set_ldordy(&mut self, val: super::vals::Ldordy) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Radio end of life flag"]
        #[inline(always)]
        pub const fn rfeolf(&self) -> super::vals::Rfeolf {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::Rfeolf::from_bits(val as u8)
        }
        #[doc = "Radio end of life flag"]
        #[inline(always)]
        pub fn set_rfeolf(&mut self, val: super::vals::Rfeolf) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "regulator2 low power flag"]
        #[inline(always)]
        pub const fn regmrs(&self) -> super::vals::Regmrs {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::Regmrs::from_bits(val as u8)
        }
        #[doc = "regulator2 low power flag"]
        #[inline(always)]
        pub fn set_regmrs(&mut self, val: super::vals::Regmrs) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Flash ready"]
        #[inline(always)]
        pub const fn flashrdy(&self) -> super::vals::Flashrdy {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::Flashrdy::from_bits(val as u8)
        }
        #[doc = "Flash ready"]
        #[inline(always)]
        pub fn set_flashrdy(&mut self, val: super::vals::Flashrdy) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "regulator1 started"]
        #[inline(always)]
        pub const fn reglps(&self) -> super::vals::Reglps {
            let val = (self.0 >> 8usize) & 0x01;
            super::vals::Reglps::from_bits(val as u8)
        }
        #[doc = "regulator1 started"]
        #[inline(always)]
        pub fn set_reglps(&mut self, val: super::vals::Reglps) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
        }
        #[doc = "regulator1 low power flag"]
        #[inline(always)]
        pub const fn reglpf(&self) -> super::vals::Reglpf {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Reglpf::from_bits(val as u8)
        }
        #[doc = "regulator1 low power flag"]
        #[inline(always)]
        pub fn set_reglpf(&mut self, val: super::vals::Reglpf) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "Voltage scaling flag"]
        #[inline(always)]
        pub const fn vosf(&self) -> super::vals::Vosf {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Vosf::from_bits(val as u8)
        }
        #[doc = "Voltage scaling flag"]
        #[inline(always)]
        pub fn set_vosf(&mut self, val: super::vals::Vosf) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Power voltage detector output"]
        #[inline(always)]
        pub const fn pvdo(&self) -> super::vals::Pvdo {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Pvdo::from_bits(val as u8)
        }
        #[doc = "Power voltage detector output"]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: super::vals::Pvdo) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
        #[inline(always)]
        pub const fn pvmo(&self) -> super::vals::Pvmo {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Pvmo::from_bits(val as u8)
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
        #[inline(always)]
        pub fn set_pvmo(&mut self, val: super::vals::Pvmo) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Sr2 {
        #[inline(always)]
        fn default() -> Sr2 {
            Sr2(0)
        }
    }
    #[doc = "Power SPI3 control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Subghzspicr(pub u32);
    impl Subghzspicr {
        #[doc = "sub-GHz SPI NSS control"]
        #[inline(always)]
        pub const fn nss(&self) -> super::vals::Nss {
            let val = (self.0 >> 15usize) & 0x01;
            super::vals::Nss::from_bits(val as u8)
        }
        #[doc = "sub-GHz SPI NSS control"]
        #[inline(always)]
        pub fn set_nss(&mut self, val: super::vals::Nss) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Subghzspicr {
        #[inline(always)]
        fn default() -> Subghzspicr {
            Subghzspicr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Apc {
        #[doc = "I/O pull-up and pull-down configurations defined in the PWR_PUCRx and PWR_PDCRx registers are applied"]
        DISABLED = 0,
        #[doc = "PWR_PUCRx and PWR_PDCRx registers are NOT applied to the I/Os"]
        ENABLED = 0x01,
    }
    impl Apc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Apc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Apc {
        #[inline(always)]
        fn from(val: u8) -> Apc {
            Apc::from_bits(val)
        }
    }
    impl From<Apc> for u8 {
        #[inline(always)]
        fn from(val: Apc) -> u8 {
            Apc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cds {
        #[doc = "CPU is running or in sleep"]
        RUNNINGORSLEEP = 0,
        #[doc = "CPU is in Deep-Sleep"]
        DEEPSLEEP = 0x01,
    }
    impl Cds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cds {
        #[inline(always)]
        fn from(val: u8) -> Cds {
            Cds::from_bits(val)
        }
    }
    impl From<Cds> for u8 {
        #[inline(always)]
        fn from(val: Cds) -> u8 {
            Cds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Csbf {
        #[doc = "System has not been in Standby mode"]
        NOSTANDBY = 0,
        #[doc = "System has been in Standby mode"]
        STANDBY = 0x01,
    }
    impl Csbf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Csbf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Csbf {
        #[inline(always)]
        fn from(val: u8) -> Csbf {
            Csbf::from_bits(val)
        }
    }
    impl From<Csbf> for u8 {
        #[inline(always)]
        fn from(val: Csbf) -> u8 {
            Csbf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Cstopf {
        #[doc = "System has not been in Stop 2 mode"]
        NOSTOP = 0,
        #[doc = "System has been in Stop 2 mode"]
        STOP = 0x01,
    }
    impl Cstopf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Cstopf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Cstopf {
        #[inline(always)]
        fn from(val: u8) -> Cstopf {
            Cstopf::from_bits(val)
        }
    }
    impl From<Cstopf> for u8 {
        #[inline(always)]
        fn from(val: Cstopf) -> u8 {
            Cstopf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dbp {
        #[doc = "Access to RTC and backup registers disabled"]
        DISABLED = 0,
        #[doc = "Access to RTC and backup registers enabled"]
        ENABLED = 0x01,
    }
    impl Dbp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dbp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dbp {
        #[inline(always)]
        fn from(val: u8) -> Dbp {
            Dbp::from_bits(val)
        }
    }
    impl From<Dbp> for u8 {
        #[inline(always)]
        fn from(val: Dbp) -> u8 {
            Dbp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Eiwul {
        #[doc = "Internal wakeup line interrupt to CPU disabled"]
        DISABLED = 0,
        #[doc = "Internal wakeup line interrupt to CPU enabled"]
        ENABLED = 0x01,
    }
    impl Eiwul {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eiwul {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eiwul {
        #[inline(always)]
        fn from(val: u8) -> Eiwul {
            Eiwul::from_bits(val)
        }
    }
    impl From<Eiwul> for u8 {
        #[inline(always)]
        fn from(val: Eiwul) -> u8 {
            Eiwul::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Eulpen {
        #[doc = "Disable (the supply voltage is monitored continuously)"]
        DISABLED = 0,
        #[doc = "Enable, when set, the supply voltage is sampled for PDR/BOR reset condition only periodically"]
        ENABLED = 0x01,
    }
    impl Eulpen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eulpen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eulpen {
        #[inline(always)]
        fn from(val: u8) -> Eulpen {
            Eulpen::from_bits(val)
        }
    }
    impl From<Eulpen> for u8 {
        #[inline(always)]
        fn from(val: Eulpen) -> u8 {
            Eulpen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ewpvd {
        #[doc = "PVD not enabled by the sub-GHz radio active state"]
        DISABLED = 0,
        #[doc = "PVD enabled while the sub-GHz radio is active"]
        ENABLED = 0x01,
    }
    impl Ewpvd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ewpvd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ewpvd {
        #[inline(always)]
        fn from(val: u8) -> Ewpvd {
            Ewpvd::from_bits(val)
        }
    }
    impl From<Ewpvd> for u8 {
        #[inline(always)]
        fn from(val: Ewpvd) -> u8 {
            Ewpvd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ewrfbusy {
        #[doc = "Radio Busy is disabled and does not trigger a wakeup from Standby event to CPUwhen a rising or a falling edge occurs"]
        DISABLED = 0,
        #[doc = "Radio Busy is enabled and triggers a wakeup from Standby event to CPUwhen a rising or a falling edge occurs. The active edge is configured via the WRFBUSYP bit in PWR_CR4"]
        ENABLED = 0x01,
    }
    impl Ewrfbusy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ewrfbusy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ewrfbusy {
        #[inline(always)]
        fn from(val: u8) -> Ewrfbusy {
            Ewrfbusy::from_bits(val)
        }
    }
    impl From<Ewrfbusy> for u8 {
        #[inline(always)]
        fn from(val: Ewrfbusy) -> u8 {
            Ewrfbusy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ewrfirq {
        #[doc = "Radio IRQ\\[2:0\\]
is disabled and does not trigger a wakeup from Standby event to CPU."]
        DISABLED = 0,
        #[doc = "Radio IRQ\\[2:0\\]
is enabled and triggers a wakeup from Standby event to CPU."]
        ENABLED = 0x01,
    }
    impl Ewrfirq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ewrfirq {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ewrfirq {
        #[inline(always)]
        fn from(val: u8) -> Ewrfirq {
            Ewrfirq::from_bits(val)
        }
    }
    impl From<Ewrfirq> for u8 {
        #[inline(always)]
        fn from(val: Ewrfirq) -> u8 {
            Ewrfirq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ewup {
        #[doc = "WKUP pin 3 is used for general purpose I/Os. An event on the WKUP pin 3 does not wakeup the device from Standby mode"]
        DISABLED = 0,
        #[doc = "WKUP pin 3 is used for wakeup from Standby mode and forced in input pull down configuration (rising edge on WKUP pin 3wakes-up the system from Standby mode)"]
        ENABLED = 0x01,
    }
    impl Ewup {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ewup {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ewup {
        #[inline(always)]
        fn from(val: u8) -> Ewup {
            Ewup::from_bits(val)
        }
    }
    impl From<Ewup> for u8 {
        #[inline(always)]
        fn from(val: Ewup) -> u8 {
            Ewup::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Flashrdy {
        #[doc = "Flash memory not ready to be accessed"]
        NOTREADY = 0,
        #[doc = "Flash memory ready to be accessed"]
        READY = 0x01,
    }
    impl Flashrdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Flashrdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Flashrdy {
        #[inline(always)]
        fn from(val: u8) -> Flashrdy {
            Flashrdy::from_bits(val)
        }
    }
    impl From<Flashrdy> for u8 {
        #[inline(always)]
        fn from(val: Flashrdy) -> u8 {
            Flashrdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fpdr {
        #[doc = "Flash memory in Idle mode when system is in LPRun mode"]
        IDLE = 0,
        #[doc = "Flash memory in Power-down mode when system is in LPRun mode"]
        POWERDOWN = 0x01,
    }
    impl Fpdr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fpdr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fpdr {
        #[inline(always)]
        fn from(val: u8) -> Fpdr {
            Fpdr::from_bits(val)
        }
    }
    impl From<Fpdr> for u8 {
        #[inline(always)]
        fn from(val: Fpdr) -> u8 {
            Fpdr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Fpds {
        #[doc = "Flash memory in Idle mode when system is in LPSleep mode"]
        IDLE = 0,
        #[doc = "Flash memory in Power-down mode when system is in LPSleep mode"]
        POWERDOWN = 0x01,
    }
    impl Fpds {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Fpds {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Fpds {
        #[inline(always)]
        fn from(val: u8) -> Fpds {
            Fpds::from_bits(val)
        }
    }
    impl From<Fpds> for u8 {
        #[inline(always)]
        fn from(val: Fpds) -> u8 {
            Fpds::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ldordy {
        #[doc = "LDO not ready or off"]
        NOTREADY = 0,
        #[doc = "LDO ready"]
        READY = 0x01,
    }
    impl Ldordy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ldordy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ldordy {
        #[inline(always)]
        fn from(val: u8) -> Ldordy {
            Ldordy::from_bits(val)
        }
    }
    impl From<Ldordy> for u8 {
        #[inline(always)]
        fn from(val: Ldordy) -> u8 {
            Ldordy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lpms {
        #[doc = "Stop 0 mode"]
        STOP0 = 0,
        #[doc = "Stop 1 mode"]
        STOP1 = 0x01,
        #[doc = "Stop 2 mode"]
        STOP2 = 0x02,
        #[doc = "Standby mode"]
        STANDBY = 0x03,
        #[doc = "Shutdown mode"]
        SHUTDOWN = 0x04,
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
    pub enum Lpr {
        #[doc = "Voltage regulator in Main mode in Low-power run mode"]
        MAINMODE = 0,
        #[doc = "Voltage regulator in low-power mode in Low-power run mode"]
        LOWPOWERMODE = 0x01,
    }
    impl Lpr {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpr {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpr {
        #[inline(always)]
        fn from(val: u8) -> Lpr {
            Lpr::from_bits(val)
        }
    }
    impl From<Lpr> for u8 {
        #[inline(always)]
        fn from(val: Lpr) -> u8 {
            Lpr::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Nss {
        #[doc = "Sub-GHz SPI NSS signal at level low"]
        LOW = 0,
        #[doc = "Sub-GHz SPI NSS signal is at level high"]
        HIGH = 0x01,
    }
    impl Nss {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nss {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nss {
        #[inline(always)]
        fn from(val: u8) -> Nss {
            Nss::from_bits(val)
        }
    }
    impl From<Nss> for u8 {
        #[inline(always)]
        fn from(val: Nss) -> u8 {
            Nss::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pls {
        #[doc = "2.0V"]
        V2_0 = 0,
        #[doc = "2.2V"]
        V2_2 = 0x01,
        #[doc = "2.4V"]
        V2_4 = 0x02,
        #[doc = "2.5V"]
        V2_5 = 0x03,
        #[doc = "2.6V"]
        V2_6 = 0x04,
        #[doc = "2.8V"]
        V2_8 = 0x05,
        #[doc = "2.9V"]
        V2_9 = 0x06,
        #[doc = "External input analog voltage PVD_IN (compared internally to VREFINT)"]
        EXTERNAL = 0x07,
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
    pub enum Pvde {
        #[doc = "PVD Disabled"]
        DISABLED = 0,
        #[doc = "PVD Enabled"]
        ENABLED = 0x01,
    }
    impl Pvde {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvde {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvde {
        #[inline(always)]
        fn from(val: u8) -> Pvde {
            Pvde::from_bits(val)
        }
    }
    impl From<Pvde> for u8 {
        #[inline(always)]
        fn from(val: Pvde) -> u8 {
            Pvde::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pvdo {
        #[doc = "VDD or voltage level on PVD_IN above the selected PVD threshold"]
        ABOVE = 0,
        #[doc = "VDD or voltage level on PVD_IN below the selected PVD threshold"]
        BELOW = 0x01,
    }
    impl Pvdo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvdo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvdo {
        #[inline(always)]
        fn from(val: u8) -> Pvdo {
            Pvdo::from_bits(val)
        }
    }
    impl From<Pvdo> for u8 {
        #[inline(always)]
        fn from(val: Pvdo) -> u8 {
            Pvdo::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pvme {
        #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) disable"]
        DISABLED = 0,
        #[doc = "PVM3 (VDDA monitoring versus 1.62 V threshold) enable"]
        ENABLED = 0x01,
    }
    impl Pvme {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvme {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvme {
        #[inline(always)]
        fn from(val: u8) -> Pvme {
            Pvme::from_bits(val)
        }
    }
    impl From<Pvme> for u8 {
        #[inline(always)]
        fn from(val: Pvme) -> u8 {
            Pvme::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pvmo {
        #[doc = "VDDA voltage above PVM3 threshold (around 1.62 V)"]
        ABOVE = 0,
        #[doc = "VDDA voltage below PVM3 threshold (around 1.62 V)"]
        BELOW = 0x01,
    }
    impl Pvmo {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pvmo {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pvmo {
        #[inline(always)]
        fn from(val: u8) -> Pvmo {
            Pvmo::from_bits(val)
        }
    }
    impl From<Pvmo> for u8 {
        #[inline(always)]
        fn from(val: Pvmo) -> u8 {
            Pvmo::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Reglpf {
        #[doc = "Main regulator (MR) ready and used"]
        MAIN = 0,
        #[doc = "Low-power regulator (LPR) used"]
        LOWPOWER = 0x01,
    }
    impl Reglpf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Reglpf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Reglpf {
        #[inline(always)]
        fn from(val: u8) -> Reglpf {
            Reglpf::from_bits(val)
        }
    }
    impl From<Reglpf> for u8 {
        #[inline(always)]
        fn from(val: Reglpf) -> u8 {
            Reglpf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Reglps {
        #[doc = "LPR not ready"]
        NOTREADY = 0,
        #[doc = "LPR ready"]
        READY = 0x01,
    }
    impl Reglps {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Reglps {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Reglps {
        #[inline(always)]
        fn from(val: u8) -> Reglps {
            Reglps::from_bits(val)
        }
    }
    impl From<Reglps> for u8 {
        #[inline(always)]
        fn from(val: Reglps) -> u8 {
            Reglps::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Regmrs {
        #[doc = "Main regulator supplied directly from VDD"]
        V_DD = 0,
        #[doc = "Main regulator supplied through LDO or SMPS"]
        LDO_SMPS = 0x01,
    }
    impl Regmrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Regmrs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Regmrs {
        #[inline(always)]
        fn from(val: u8) -> Regmrs {
            Regmrs::from_bits(val)
        }
    }
    impl From<Regmrs> for u8 {
        #[inline(always)]
        fn from(val: Regmrs) -> u8 {
            Regmrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rfbusyms {
        #[doc = "radio busy masked signal low (not busy)"]
        NOTBUSY = 0,
        #[doc = "radio busy masked signal high (busy)"]
        BUSY = 0x01,
    }
    impl Rfbusyms {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rfbusyms {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rfbusyms {
        #[inline(always)]
        fn from(val: u8) -> Rfbusyms {
            Rfbusyms::from_bits(val)
        }
    }
    impl From<Rfbusyms> for u8 {
        #[inline(always)]
        fn from(val: Rfbusyms) -> u8 {
            Rfbusyms::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rfbusys {
        #[doc = "radio busy signal low (not busy)"]
        NOTBUSY = 0,
        #[doc = "radio busy signal high (busy)"]
        BUSY = 0x01,
    }
    impl Rfbusys {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rfbusys {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rfbusys {
        #[inline(always)]
        fn from(val: u8) -> Rfbusys {
            Rfbusys::from_bits(val)
        }
    }
    impl From<Rfbusys> for u8 {
        #[inline(always)]
        fn from(val: Rfbusys) -> u8 {
            Rfbusys::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rfeolen {
        #[doc = "Radio end-of-life detector disabled"]
        DISABLED = 0,
        #[doc = "Radio end-of-life detector enabled"]
        ENABLED = 0x01,
    }
    impl Rfeolen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rfeolen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rfeolen {
        #[inline(always)]
        fn from(val: u8) -> Rfeolen {
            Rfeolen::from_bits(val)
        }
    }
    impl From<Rfeolen> for u8 {
        #[inline(always)]
        fn from(val: Rfeolen) -> u8 {
            Rfeolen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rfeolf {
        #[doc = "Supply voltage above radio end-of-life operating low level"]
        ABOVE = 0,
        #[doc = "Supply voltage below radio end-of-life operating low level"]
        BELOW = 0x01,
    }
    impl Rfeolf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rfeolf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rfeolf {
        #[inline(always)]
        fn from(val: u8) -> Rfeolf {
            Rfeolf::from_bits(val)
        }
    }
    impl From<Rfeolf> for u8 {
        #[inline(always)]
        fn from(val: Rfeolf) -> u8 {
            Rfeolf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Rrs {
        #[doc = "SRAM2 powered off in Standby mode (SRAM2 content lost)"]
        POWEROFF = 0,
        #[doc = "SRAM2 powered by the low-power regulator in Standby mode (SRAM2 content kept)"]
        ONLPR = 0x01,
    }
    impl Rrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Rrs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Rrs {
        #[inline(always)]
        fn from(val: u8) -> Rrs {
            Rrs::from_bits(val)
        }
    }
    impl From<Rrs> for u8 {
        #[inline(always)]
        fn from(val: Rrs) -> u8 {
            Rrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Smpsen {
        #[doc = "SMPS step-down converter SMPS mode disabled (LDO mode enabled)"]
        DISABLED = 0,
        #[doc = "SMPS step-down converter SMPS mode enabled"]
        ENABLED = 0x01,
    }
    impl Smpsen {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpsen {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpsen {
        #[inline(always)]
        fn from(val: u8) -> Smpsen {
            Smpsen::from_bits(val)
        }
    }
    impl From<Smpsen> for u8 {
        #[inline(always)]
        fn from(val: Smpsen) -> u8 {
            Smpsen::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Smpsrdy {
        #[doc = "SMPS step-down converter not ready or off"]
        NOTREADY = 0,
        #[doc = "SMPS step-down converter ready"]
        READY = 0x01,
    }
    impl Smpsrdy {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Smpsrdy {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Smpsrdy {
        #[inline(always)]
        fn from(val: u8) -> Smpsrdy {
            Smpsrdy::from_bits(val)
        }
    }
    impl From<Smpsrdy> for u8 {
        #[inline(always)]
        fn from(val: Smpsrdy) -> u8 {
            Smpsrdy::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Subghzspinsssel {
        #[doc = "sub-GHz SPI NSS signal driven from PWR_SUBGHZSPICR.NSS (RFBUSYMS functionality enabled)"]
        SUBGHZSPICR = 0,
        #[doc = "sub-GHz SPI NSS signal driven from LPTIM3_OUT (RFBUSYMS functionality disabled)"]
        LPTIM3 = 0x01,
    }
    impl Subghzspinsssel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Subghzspinsssel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Subghzspinsssel {
        #[inline(always)]
        fn from(val: u8) -> Subghzspinsssel {
            Subghzspinsssel::from_bits(val)
        }
    }
    impl From<Subghzspinsssel> for u8 {
        #[inline(always)]
        fn from(val: Subghzspinsssel) -> u8 {
            Subghzspinsssel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Vbe {
        #[doc = "VBAT battery charging disabled"]
        DISABLED = 0,
        #[doc = "VBAT battery charging enabled"]
        ENABLED = 0x01,
    }
    impl Vbe {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbe {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbe {
        #[inline(always)]
        fn from(val: u8) -> Vbe {
            Vbe::from_bits(val)
        }
    }
    impl From<Vbe> for u8 {
        #[inline(always)]
        fn from(val: Vbe) -> u8 {
            Vbe::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Vbrs {
        #[doc = "VBAT charging through a 5 kΩ resistor"]
        R5K = 0,
        #[doc = "VBAT charging through a 1.5 kΩ resistor"]
        R1_5K = 0x01,
    }
    impl Vbrs {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vbrs {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vbrs {
        #[inline(always)]
        fn from(val: u8) -> Vbrs {
            Vbrs::from_bits(val)
        }
    }
    impl From<Vbrs> for u8 {
        #[inline(always)]
        fn from(val: Vbrs) -> u8 {
            Vbrs::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Vos {
        _RESERVED_0 = 0,
        #[doc = "1.2 V (range 1)"]
        V1_2 = 0x01,
        #[doc = "1.0 V (range 2)"]
        V1_0 = 0x02,
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
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Vosf {
        #[doc = "Regulator ready in the selected voltage range"]
        READY = 0,
        #[doc = "Regulator output voltage changed to the required voltage level"]
        CHANGE = 0x01,
    }
    impl Vosf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Vosf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Vosf {
        #[inline(always)]
        fn from(val: u8) -> Vosf {
            Vosf::from_bits(val)
        }
    }
    impl From<Vosf> for u8 {
        #[inline(always)]
        fn from(val: Vosf) -> u8 {
            Vosf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wp {
        #[doc = "Detection on high level (rising edge)"]
        RISINGEDGE = 0,
        #[doc = "Detection on low level (falling edge)"]
        FALLINGEDGE = 0x01,
    }
    impl Wp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wp {
        #[inline(always)]
        fn from(val: u8) -> Wp {
            Wp::from_bits(val)
        }
    }
    impl From<Wp> for u8 {
        #[inline(always)]
        fn from(val: Wp) -> u8 {
            Wp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wpvdf {
        #[doc = "No wakeup event detected on PVD"]
        CLEAR = 0,
        #[doc = "Wakeup event detected on PVD"]
        WAKEUP = 0x01,
    }
    impl Wpvdf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wpvdf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wpvdf {
        #[inline(always)]
        fn from(val: u8) -> Wpvdf {
            Wpvdf::from_bits(val)
        }
    }
    impl From<Wpvdf> for u8 {
        #[inline(always)]
        fn from(val: Wpvdf) -> u8 {
            Wpvdf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wrfbusyf {
        #[doc = "No wakeup event detected on radio busy"]
        CLEAR = 0,
        #[doc = "Wakeup event detected on radio busy"]
        WAKEUP = 0x01,
    }
    impl Wrfbusyf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wrfbusyf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wrfbusyf {
        #[inline(always)]
        fn from(val: u8) -> Wrfbusyf {
            Wrfbusyf::from_bits(val)
        }
    }
    impl From<Wrfbusyf> for u8 {
        #[inline(always)]
        fn from(val: Wrfbusyf) -> u8 {
            Wrfbusyf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wrfbusyp {
        #[doc = "Detection on high level (rising edge)"]
        RISINGEDGE = 0,
        #[doc = "Detection on low level (falling edge)"]
        FALLINGEDGE = 0x01,
    }
    impl Wrfbusyp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wrfbusyp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wrfbusyp {
        #[inline(always)]
        fn from(val: u8) -> Wrfbusyp {
            Wrfbusyp::from_bits(val)
        }
    }
    impl From<Wrfbusyp> for u8 {
        #[inline(always)]
        fn from(val: Wrfbusyp) -> u8 {
            Wrfbusyp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wuf {
        #[doc = "No wakeup event detected on WKUP3"]
        CLEAR = 0,
        #[doc = "Wakeup event detected on WKUP3"]
        WAKEUP = 0x01,
    }
    impl Wuf {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wuf {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wuf {
        #[inline(always)]
        fn from(val: u8) -> Wuf {
            Wuf::from_bits(val)
        }
    }
    impl From<Wuf> for u8 {
        #[inline(always)]
        fn from(val: Wuf) -> u8 {
            Wuf::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Wufi {
        #[doc = "All internal wakeup sources are cleared"]
        CLEAR = 0,
        #[doc = "wakeup is detected on the internal wakeup line"]
        WAKEUP = 0x01,
    }
    impl Wufi {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Wufi {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Wufi {
        #[inline(always)]
        fn from(val: u8) -> Wufi {
            Wufi::from_bits(val)
        }
    }
    impl From<Wufi> for u8 {
        #[inline(always)]
        fn from(val: Wufi) -> u8 {
            Wufi::to_bits(val)
        }
    }
}
