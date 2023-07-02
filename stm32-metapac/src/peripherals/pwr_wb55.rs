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
    #[doc = "Power Port A pull-up control register"]
    #[inline(always)]
    pub const fn pucra(self) -> crate::common::Reg<regs::Pucra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Power Port A pull-down control register"]
    #[inline(always)]
    pub const fn pdcra(self) -> crate::common::Reg<regs::Pdcra, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Power Port B pull-up control register"]
    #[inline(always)]
    pub const fn pucrb(self) -> crate::common::Reg<regs::Pucrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Power Port B pull-down control register"]
    #[inline(always)]
    pub const fn pdcrb(self) -> crate::common::Reg<regs::Pdcrb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Power Port C pull-up control register"]
    #[inline(always)]
    pub const fn pucrc(self) -> crate::common::Reg<regs::Pucrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Power Port C pull-down control register"]
    #[inline(always)]
    pub const fn pdcrc(self) -> crate::common::Reg<regs::Pdcrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Power Port D pull-up control register"]
    #[inline(always)]
    pub const fn pucrd(self) -> crate::common::Reg<regs::Pucrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Power Port D pull-down control register"]
    #[inline(always)]
    pub const fn pdcrd(self) -> crate::common::Reg<regs::Pdcrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Power Port E pull-up control register"]
    #[inline(always)]
    pub const fn pucre(self) -> crate::common::Reg<regs::Pucre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Power Port E pull-down control register"]
    #[inline(always)]
    pub const fn pdcre(self) -> crate::common::Reg<regs::Pdcre, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Power Port H pull-up control register"]
    #[inline(always)]
    pub const fn pucrh(self) -> crate::common::Reg<regs::Pucrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Power Port H pull-down control register"]
    #[inline(always)]
    pub const fn pdcrh(self) -> crate::common::Reg<regs::Pdcrh, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "CPU2 Power control register 1"]
    #[inline(always)]
    pub const fn c2cr1(self) -> crate::common::Reg<regs::C2cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "CPU2 Power control register 3"]
    #[inline(always)]
    pub const fn c2cr3(self) -> crate::common::Reg<regs::C2cr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Power status clear register"]
    #[inline(always)]
    pub const fn extscr(self) -> crate::common::Reg<regs::Extscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
}
pub mod regs {
    #[doc = "CPU2 Power control register 1"]
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
        #[doc = "Flash power down mode during LPRun for CPU2"]
        #[inline(always)]
        pub const fn fpdr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flash power down mode during LPRun for CPU2"]
        #[inline(always)]
        pub fn set_fpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Flash power down mode during LPSleep for CPU2"]
        #[inline(always)]
        pub const fn fpds(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Flash power down mode during LPSleep for CPU2"]
        #[inline(always)]
        pub fn set_fpds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "BLE external wakeup signal"]
        #[inline(always)]
        pub const fn bleewkup(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "BLE external wakeup signal"]
        #[inline(always)]
        pub fn set_bleewkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "802.15.4 external wakeup signal"]
        #[inline(always)]
        pub const fn _802ewkup(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "802.15.4 external wakeup signal"]
        #[inline(always)]
        pub fn set__802ewkup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for C2cr1 {
        #[inline(always)]
        fn default() -> C2cr1 {
            C2cr1(0)
        }
    }
    #[doc = "CPU2 Power control register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct C2cr3(pub u32);
    impl C2cr3 {
        #[doc = "Enable Wakeup pin WKUP1 for CPU2"]
        #[inline(always)]
        pub const fn ewup1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP1 for CPU2"]
        #[inline(always)]
        pub fn set_ewup1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Wakeup pin WKUP2 for CPU2"]
        #[inline(always)]
        pub const fn ewup2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP2 for CPU2"]
        #[inline(always)]
        pub fn set_ewup2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Wakeup pin WKUP3 for CPU2"]
        #[inline(always)]
        pub const fn ewup3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP3 for CPU2"]
        #[inline(always)]
        pub fn set_ewup3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable Wakeup pin WKUP4 for CPU2"]
        #[inline(always)]
        pub const fn ewup4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP4 for CPU2"]
        #[inline(always)]
        pub fn set_ewup4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Wakeup pin WKUP5 for CPU2"]
        #[inline(always)]
        pub const fn ewup5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP5 for CPU2"]
        #[inline(always)]
        pub fn set_ewup5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable BLE host wakeup interrupt for CPU2"]
        #[inline(always)]
        pub const fn eblewup(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Enable BLE host wakeup interrupt for CPU2"]
        #[inline(always)]
        pub fn set_eblewup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable 802.15.4 host wakeup interrupt for CPU2"]
        #[inline(always)]
        pub const fn e802wup(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Enable 802.15.4 host wakeup interrupt for CPU2"]
        #[inline(always)]
        pub fn set_e802wup(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Apply pull-up and pull-down configuration for CPU2"]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration for CPU2"]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
        pub const fn lpms(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Low-power mode selection for CPU1"]
        #[inline(always)]
        pub fn set_lpms(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Flash power down mode during LPRun for CPU1"]
        #[inline(always)]
        pub const fn fpdr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Flash power down mode during LPRun for CPU1"]
        #[inline(always)]
        pub fn set_fpdr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Flash power down mode during LPsSleep for CPU1"]
        #[inline(always)]
        pub const fn fpds(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Flash power down mode during LPsSleep for CPU1"]
        #[inline(always)]
        pub fn set_fpds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub const fn dbp(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Disable backup domain write protection"]
        #[inline(always)]
        pub fn set_dbp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub const fn vos(&self) -> u8 {
            let val = (self.0 >> 9usize) & 0x03;
            val as u8
        }
        #[doc = "Voltage scaling range selection"]
        #[inline(always)]
        pub fn set_vos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 9usize)) | (((val as u32) & 0x03) << 9usize);
        }
        #[doc = "Low-power run"]
        #[inline(always)]
        pub const fn lpr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power run"]
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
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub const fn pvde(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Power voltage detector enable"]
        #[inline(always)]
        pub fn set_pvde(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Power voltage detector level selection"]
        #[inline(always)]
        pub const fn pls(&self) -> u8 {
            let val = (self.0 >> 1usize) & 0x07;
            val as u8
        }
        #[doc = "Power voltage detector level selection"]
        #[inline(always)]
        pub fn set_pls(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 1usize)) | (((val as u32) & 0x07) << 1usize);
        }
        #[doc = "Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
        #[inline(always)]
        pub const fn pvme1(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V"]
        #[inline(always)]
        pub fn set_pvme1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
        #[inline(always)]
        pub const fn pvme3(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V"]
        #[inline(always)]
        pub fn set_pvme3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "VDDUSB USB supply valid"]
        #[inline(always)]
        pub const fn usv(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "VDDUSB USB supply valid"]
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
        #[doc = "Enable Wakeup pin WKUP1"]
        #[inline(always)]
        pub const fn ewup1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP1"]
        #[inline(always)]
        pub fn set_ewup1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Enable Wakeup pin WKUP2"]
        #[inline(always)]
        pub const fn ewup2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP2"]
        #[inline(always)]
        pub fn set_ewup2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Enable Wakeup pin WKUP3"]
        #[inline(always)]
        pub const fn ewup3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP3"]
        #[inline(always)]
        pub fn set_ewup3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Enable Wakeup pin WKUP4"]
        #[inline(always)]
        pub const fn ewup4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP4"]
        #[inline(always)]
        pub fn set_ewup4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Enable Wakeup pin WKUP5"]
        #[inline(always)]
        pub const fn ewup5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Wakeup pin WKUP5"]
        #[inline(always)]
        pub fn set_ewup5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Enable BORH and Step Down counverter forced in Bypass interrups for CPU1"]
        #[inline(always)]
        pub const fn eborhsdfb(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Enable BORH and Step Down counverter forced in Bypass interrups for CPU1"]
        #[inline(always)]
        pub fn set_eborhsdfb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "SRAM2a retention in Standby mode"]
        #[inline(always)]
        pub const fn rrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2a retention in Standby mode"]
        #[inline(always)]
        pub fn set_rrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Apply pull-up and pull-down configuration"]
        #[inline(always)]
        pub const fn apc(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Apply pull-up and pull-down configuration"]
        #[inline(always)]
        pub fn set_apc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable BLE end of activity interrupt for CPU1"]
        #[inline(always)]
        pub const fn eblea(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable BLE end of activity interrupt for CPU1"]
        #[inline(always)]
        pub fn set_eblea(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Enable critical radio phase end of activity interrupt for CPU1"]
        #[inline(always)]
        pub const fn ecrpe(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Enable critical radio phase end of activity interrupt for CPU1"]
        #[inline(always)]
        pub fn set_ecrpe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Enable end of activity interrupt for CPU1"]
        #[inline(always)]
        pub const fn e802a(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Enable end of activity interrupt for CPU1"]
        #[inline(always)]
        pub fn set_e802a(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Enable CPU2 Hold interrupt for CPU1"]
        #[inline(always)]
        pub const fn ec2h(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable CPU2 Hold interrupt for CPU1"]
        #[inline(always)]
        pub fn set_ec2h(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable internal wakeup line for CPU1"]
        #[inline(always)]
        pub const fn eiwul(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable internal wakeup line for CPU1"]
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
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub const fn wp1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP1 polarity"]
        #[inline(always)]
        pub fn set_wp1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wakeup pin WKUP2 polarity"]
        #[inline(always)]
        pub const fn wp2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP2 polarity"]
        #[inline(always)]
        pub fn set_wp2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup pin WKUP3 polarity"]
        #[inline(always)]
        pub const fn wp3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP3 polarity"]
        #[inline(always)]
        pub fn set_wp3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup pin WKUP4 polarity"]
        #[inline(always)]
        pub const fn wp4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP4 polarity"]
        #[inline(always)]
        pub fn set_wp4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wakeup pin WKUP5 polarity"]
        #[inline(always)]
        pub const fn wp5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup pin WKUP5 polarity"]
        #[inline(always)]
        pub fn set_wp5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "VBAT battery charging enable"]
        #[inline(always)]
        pub const fn vbe(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT battery charging enable"]
        #[inline(always)]
        pub fn set_vbe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VBAT battery charging resistor selection"]
        #[inline(always)]
        pub const fn vbrs(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VBAT battery charging resistor selection"]
        #[inline(always)]
        pub fn set_vbrs(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "BOOT CPU2 after reset or wakeup from Stop or Standby modes"]
        #[inline(always)]
        pub const fn c2boot(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "BOOT CPU2 after reset or wakeup from Stop or Standby modes"]
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
        #[doc = "Step Down converter voltage output scaling"]
        #[inline(always)]
        pub const fn sdvos(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Step Down converter voltage output scaling"]
        #[inline(always)]
        pub fn set_sdvos(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Step Down converter supplt startup current selection"]
        #[inline(always)]
        pub const fn sdsc(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x07;
            val as u8
        }
        #[doc = "Step Down converter supplt startup current selection"]
        #[inline(always)]
        pub fn set_sdsc(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
        }
        #[doc = "BORH configuration selection"]
        #[inline(always)]
        pub const fn borhc(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "BORH configuration selection"]
        #[inline(always)]
        pub fn set_borhc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "VOS configuration selection (non user)"]
        #[inline(always)]
        pub const fn smpscfg(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "VOS configuration selection (non user)"]
        #[inline(always)]
        pub fn set_smpscfg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Enable Step Down converter Bypass mode enabled"]
        #[inline(always)]
        pub const fn sdben(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Step Down converter Bypass mode enabled"]
        #[inline(always)]
        pub fn set_sdben(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Enable Step Down converter SMPS mode enabled"]
        #[inline(always)]
        pub const fn sdeb(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Enable Step Down converter SMPS mode enabled"]
        #[inline(always)]
        pub fn set_sdeb(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
    }
    impl Default for Cr5 {
        #[inline(always)]
        fn default() -> Cr5 {
            Cr5(0)
        }
    }
    #[doc = "Power status clear register"]
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
        #[doc = "Clear CPU2 Stop Standby flags"]
        #[inline(always)]
        pub const fn c2cssf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear CPU2 Stop Standby flags"]
        #[inline(always)]
        pub fn set_c2cssf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear Critical Radio system phase"]
        #[inline(always)]
        pub const fn ccrpf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear Critical Radio system phase"]
        #[inline(always)]
        pub fn set_ccrpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "System Standby flag for CPU1"]
        #[inline(always)]
        pub const fn c1sbf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "System Standby flag for CPU1"]
        #[inline(always)]
        pub fn set_c1sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "System Stop flag for CPU1"]
        #[inline(always)]
        pub const fn c1stopf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "System Stop flag for CPU1"]
        #[inline(always)]
        pub fn set_c1stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "System Standby flag for CPU2"]
        #[inline(always)]
        pub const fn c2sbf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "System Standby flag for CPU2"]
        #[inline(always)]
        pub fn set_c2sbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "System Stop flag for CPU2"]
        #[inline(always)]
        pub const fn c2stopf(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "System Stop flag for CPU2"]
        #[inline(always)]
        pub fn set_c2stopf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Critical Radio system phase"]
        #[inline(always)]
        pub const fn crpf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Critical Radio system phase"]
        #[inline(always)]
        pub fn set_crpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CPU1 deepsleep mode"]
        #[inline(always)]
        pub const fn c1ds(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CPU1 deepsleep mode"]
        #[inline(always)]
        pub fn set_c1ds(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "CPU2 deepsleep mode"]
        #[inline(always)]
        pub const fn c2ds(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "CPU2 deepsleep mode"]
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
    #[doc = "Power Port A pull-down control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcra(pub u32);
    impl Pdcra {
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
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
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-down bit y (y=0..15)"]
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
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-down bit y (y=0..15)"]
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
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pd4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-down bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pd4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Pdcre {
        #[inline(always)]
        fn default() -> Pdcre {
            Pdcre(0)
        }
    }
    #[doc = "Power Port H pull-down control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdcrh(pub u32);
    impl Pdcrh {
        #[doc = "Port H pull-down bit y (y=0..1)"]
        #[inline(always)]
        pub const fn pd0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port H pull-down bit y (y=0..1)"]
        #[inline(always)]
        pub fn set_pd0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port H pull-down bit y (y=0..1)"]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port H pull-down bit y (y=0..1)"]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port H pull-down bit y (y=0..1)"]
        #[inline(always)]
        pub const fn pd3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port H pull-down bit y (y=0..1)"]
        #[inline(always)]
        pub fn set_pd3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Pdcrh {
        #[inline(always)]
        fn default() -> Pdcrh {
            Pdcrh(0)
        }
    }
    #[doc = "Power Port A pull-up control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucra(pub u32);
    impl Pucra {
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port A pull-up bit y (y=0..15)"]
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
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port B pull-up bit y (y=0..15)"]
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
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port C pull-up bit y (y=0..15)"]
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
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Port D pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
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
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub const fn pu4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Port E pull-up bit y (y=0..15)"]
        #[inline(always)]
        pub fn set_pu4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
    }
    impl Default for Pucre {
        #[inline(always)]
        fn default() -> Pucre {
            Pucre(0)
        }
    }
    #[doc = "Power Port H pull-up control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pucrh(pub u32);
    impl Pucrh {
        #[doc = "Port H pull-up bit y (y=0..1)"]
        #[inline(always)]
        pub const fn pu0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Port H pull-up bit y (y=0..1)"]
        #[inline(always)]
        pub fn set_pu0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Port H pull-up bit y (y=0..1)"]
        #[inline(always)]
        pub const fn pu1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Port H pull-up bit y (y=0..1)"]
        #[inline(always)]
        pub fn set_pu1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Port H pull-up bit y (y=0..1)"]
        #[inline(always)]
        pub const fn pu3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Port H pull-up bit y (y=0..1)"]
        #[inline(always)]
        pub fn set_pu3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
    }
    impl Default for Pucrh {
        #[inline(always)]
        fn default() -> Pucrh {
            Pucrh(0)
        }
    }
    #[doc = "Power status clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Scr(pub u32);
    impl Scr {
        #[doc = "Clear wakeup flag 1"]
        #[inline(always)]
        pub const fn cwuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag 1"]
        #[inline(always)]
        pub fn set_cwuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear wakeup flag 2"]
        #[inline(always)]
        pub const fn cwuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag 2"]
        #[inline(always)]
        pub fn set_cwuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Clear wakeup flag 3"]
        #[inline(always)]
        pub const fn cwuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag 3"]
        #[inline(always)]
        pub fn set_cwuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Clear wakeup flag 4"]
        #[inline(always)]
        pub const fn cwuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag 4"]
        #[inline(always)]
        pub fn set_cwuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Clear wakeup flag 5"]
        #[inline(always)]
        pub const fn cwuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Clear wakeup flag 5"]
        #[inline(always)]
        pub fn set_cwuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Clear SMPS Step Down converter forced in Bypass interrupt flag"]
        #[inline(always)]
        pub const fn csmpsfbf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Clear SMPS Step Down converter forced in Bypass interrupt flag"]
        #[inline(always)]
        pub fn set_csmpsfbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Clear BORH interrupt flag"]
        #[inline(always)]
        pub const fn cborhf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Clear BORH interrupt flag"]
        #[inline(always)]
        pub fn set_cborhf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Clear BLE wakeup interrupt flag"]
        #[inline(always)]
        pub const fn cblewuf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Clear BLE wakeup interrupt flag"]
        #[inline(always)]
        pub fn set_cblewuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Clear 802.15.4 wakeup interrupt flag"]
        #[inline(always)]
        pub const fn c802wuf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Clear 802.15.4 wakeup interrupt flag"]
        #[inline(always)]
        pub fn set_c802wuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Clear critical radio phase end of activity interrupt flag"]
        #[inline(always)]
        pub const fn ccrpef(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Clear critical radio phase end of activity interrupt flag"]
        #[inline(always)]
        pub fn set_ccrpef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Clear BLE end of activity interrupt flag"]
        #[inline(always)]
        pub const fn cbleaf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Clear BLE end of activity interrupt flag"]
        #[inline(always)]
        pub fn set_cbleaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Clear 802.15.4 end of activity interrupt flag"]
        #[inline(always)]
        pub const fn c802af(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Clear 802.15.4 end of activity interrupt flag"]
        #[inline(always)]
        pub fn set_c802af(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Clear CPU2 Hold interrupt flag"]
        #[inline(always)]
        pub const fn cc2hf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Clear CPU2 Hold interrupt flag"]
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
    #[doc = "Power status register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr1(pub u32);
    impl Sr1 {
        #[doc = "Wakeup flag 1"]
        #[inline(always)]
        pub const fn cwuf1(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 1"]
        #[inline(always)]
        pub fn set_cwuf1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Wakeup flag 2"]
        #[inline(always)]
        pub const fn cwuf2(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 2"]
        #[inline(always)]
        pub fn set_cwuf2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Wakeup flag 3"]
        #[inline(always)]
        pub const fn cwuf3(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 3"]
        #[inline(always)]
        pub fn set_cwuf3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Wakeup flag 4"]
        #[inline(always)]
        pub const fn cwuf4(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 4"]
        #[inline(always)]
        pub fn set_cwuf4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Wakeup flag 5"]
        #[inline(always)]
        pub const fn cwuf5(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Wakeup flag 5"]
        #[inline(always)]
        pub fn set_cwuf5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Step Down converter forced in Bypass interrupt flag"]
        #[inline(always)]
        pub const fn sdfbf(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Step Down converter forced in Bypass interrupt flag"]
        #[inline(always)]
        pub fn set_sdfbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "BORH interrupt flag"]
        #[inline(always)]
        pub const fn borhf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "BORH interrupt flag"]
        #[inline(always)]
        pub fn set_borhf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "BLE wakeup interrupt flag"]
        #[inline(always)]
        pub const fn blewuf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "BLE wakeup interrupt flag"]
        #[inline(always)]
        pub fn set_blewuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "802.15.4 wakeup interrupt flag"]
        #[inline(always)]
        pub const fn _802wuf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "802.15.4 wakeup interrupt flag"]
        #[inline(always)]
        pub fn set__802wuf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Enable critical radio phase end of activity interrupt flag"]
        #[inline(always)]
        pub const fn crpef(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Enable critical radio phase end of activity interrupt flag"]
        #[inline(always)]
        pub fn set_crpef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "BLE end of activity interrupt flag"]
        #[inline(always)]
        pub const fn bleaf(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "BLE end of activity interrupt flag"]
        #[inline(always)]
        pub fn set_bleaf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "802.15.4 end of activity interrupt flag"]
        #[inline(always)]
        pub const fn af802(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "802.15.4 end of activity interrupt flag"]
        #[inline(always)]
        pub fn set_af802(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "CPU2 Hold interrupt flag"]
        #[inline(always)]
        pub const fn c2hf(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "CPU2 Hold interrupt flag"]
        #[inline(always)]
        pub fn set_c2hf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Internal Wakeup interrupt flag"]
        #[inline(always)]
        pub const fn wufi(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Internal Wakeup interrupt flag"]
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
        #[doc = "Step Down converter Bypass mode flag"]
        #[inline(always)]
        pub const fn sdbf(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Step Down converter Bypass mode flag"]
        #[inline(always)]
        pub fn set_sdbf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Step Down converter SMPS mode flag"]
        #[inline(always)]
        pub const fn sdsmpsf(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Step Down converter SMPS mode flag"]
        #[inline(always)]
        pub fn set_sdsmpsf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Low-power regulator started"]
        #[inline(always)]
        pub const fn reglps(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power regulator started"]
        #[inline(always)]
        pub fn set_reglps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power regulator flag"]
        #[inline(always)]
        pub const fn reglpf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power regulator flag"]
        #[inline(always)]
        pub fn set_reglpf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Voltage scaling flag"]
        #[inline(always)]
        pub const fn vosf(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Voltage scaling flag"]
        #[inline(always)]
        pub fn set_vosf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Power voltage detector output"]
        #[inline(always)]
        pub const fn pvdo(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Power voltage detector output"]
        #[inline(always)]
        pub fn set_pvdo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
        #[inline(always)]
        pub const fn pvmo1(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: VDDUSB vs. 1.2 V"]
        #[inline(always)]
        pub fn set_pvmo1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
        #[inline(always)]
        pub const fn pvmo3(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Peripheral voltage monitoring output: VDDA vs. 1.62 V"]
        #[inline(always)]
        pub fn set_pvmo3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Sr2 {
        #[inline(always)]
        fn default() -> Sr2 {
            Sr2(0)
        }
    }
}
