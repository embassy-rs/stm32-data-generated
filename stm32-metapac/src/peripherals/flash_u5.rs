#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Flash"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flash {
    ptr: *mut u8,
}
unsafe impl Send for Flash {}
unsafe impl Sync for Flash {}
impl Flash {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FLASH access control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "FLASH non-secure key register"]
    #[inline(always)]
    pub const fn nskeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "FLASH secure key register"]
    #[inline(always)]
    pub const fn seckeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "FLASH bank 1 power-down key register"]
    #[inline(always)]
    pub const fn pdkey1r(self) -> crate::common::Reg<regs::Pdkey1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FLASH bank 2 power-down key register"]
    #[inline(always)]
    pub const fn pdkey2r(self) -> crate::common::Reg<regs::Pdkey2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "FLASH non-secure status register"]
    #[inline(always)]
    pub const fn nssr(self) -> crate::common::Reg<regs::Nssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "FLASH secure status register"]
    #[inline(always)]
    pub const fn secsr(self) -> crate::common::Reg<regs::Secsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "FLASH non-secure control register"]
    #[inline(always)]
    pub const fn nscr(self) -> crate::common::Reg<regs::Nscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FLASH secure control register"]
    #[inline(always)]
    pub const fn seccr(self) -> crate::common::Reg<regs::Seccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "FLASH ECC register"]
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<regs::Eccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "FLASH operation status register"]
    #[inline(always)]
    pub const fn opsr(self) -> crate::common::Reg<regs::Opsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "FLASH option register"]
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<regs::Optr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "FLASH non-secure boot address 0 register"]
    #[inline(always)]
    pub const fn nsbootadd0r(self) -> crate::common::Reg<regs::Nsbootadd0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "FLASH non-secure boot address 1 register"]
    #[inline(always)]
    pub const fn nsbootadd1r(self) -> crate::common::Reg<regs::Nsbootadd1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "FLASH secure boot address 0 register"]
    #[inline(always)]
    pub const fn secbootadd0r(self) -> crate::common::Reg<regs::Secbootadd0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "FLASH secure watermark1 register 1"]
    #[inline(always)]
    pub const fn secwm1r1(self) -> crate::common::Reg<regs::Secwm1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FLASH secure watermark1 register 2"]
    #[inline(always)]
    pub const fn secwm1r2(self) -> crate::common::Reg<regs::Secwm1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "FLASH WRP1 area A address register"]
    #[inline(always)]
    pub const fn wrp1ar(self) -> crate::common::Reg<regs::Wrp1ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "FLASH WRP1 area B address register"]
    #[inline(always)]
    pub const fn wrp1br(self) -> crate::common::Reg<regs::Wrp1br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "FLASH secure watermark2 register 1"]
    #[inline(always)]
    pub const fn secwm2r1(self) -> crate::common::Reg<regs::Secwm2r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "FLASH secure watermark2 register 2"]
    #[inline(always)]
    pub const fn secwm2r2(self) -> crate::common::Reg<regs::Secwm2r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "FLASH WPR2 area A address register"]
    #[inline(always)]
    pub const fn wrp2ar(self) -> crate::common::Reg<regs::Wrp2ar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "FLASH WPR2 area B address register"]
    #[inline(always)]
    pub const fn wrp2br(self) -> crate::common::Reg<regs::Wrp2br, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "FLASH OEM1 key register 1"]
    #[inline(always)]
    pub const fn oem1keyr1(self) -> crate::common::Reg<regs::Oem1keyr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "FLASH OEM1 key register 2"]
    #[inline(always)]
    pub const fn oem1keyr2(self) -> crate::common::Reg<regs::Oem1keyr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "FLASH OEM2 key register 1"]
    #[inline(always)]
    pub const fn oem2keyr1(self) -> crate::common::Reg<regs::Oem2keyr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "FLASH OEM2 key register 2"]
    #[inline(always)]
    pub const fn oem2keyr2(self) -> crate::common::Reg<regs::Oem2keyr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 1"]
    #[inline(always)]
    pub const fn sec1bbr1(self) -> crate::common::Reg<regs::Sec1bbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 2"]
    #[inline(always)]
    pub const fn sec1bbr2(self) -> crate::common::Reg<regs::Sec1bbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 3"]
    #[inline(always)]
    pub const fn sec1bbr3(self) -> crate::common::Reg<regs::Sec1bbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "FLASH secure block based bank 1 register 4"]
    #[inline(always)]
    pub const fn sec1bbr4(self) -> crate::common::Reg<regs::Sec1bbr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 1"]
    #[inline(always)]
    pub const fn sec2bbr1(self) -> crate::common::Reg<regs::Sec2bbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 2"]
    #[inline(always)]
    pub const fn sec2bbr2(self) -> crate::common::Reg<regs::Sec2bbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 3"]
    #[inline(always)]
    pub const fn sec2bbr3(self) -> crate::common::Reg<regs::Sec2bbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "FLASH secure block based bank 2 register 4"]
    #[inline(always)]
    pub const fn sec2bbr4(self) -> crate::common::Reg<regs::Sec2bbr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "FLASH secure HDP control register"]
    #[inline(always)]
    pub const fn sechdpcr(self) -> crate::common::Reg<regs::Sechdpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "FLASH privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 1 register 1"]
    #[inline(always)]
    pub const fn priv1bbr1(self) -> crate::common::Reg<regs::Priv1bbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 1 register 2"]
    #[inline(always)]
    pub const fn priv1bbr2(self) -> crate::common::Reg<regs::Priv1bbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd4usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 1 register 3"]
    #[inline(always)]
    pub const fn priv1bbr3(self) -> crate::common::Reg<regs::Priv1bbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 1 register 4"]
    #[inline(always)]
    pub const fn priv1bbr4(self) -> crate::common::Reg<regs::Priv1bbr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 1"]
    #[inline(always)]
    pub const fn priv2bbr1(self) -> crate::common::Reg<regs::Priv2bbr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 2"]
    #[inline(always)]
    pub const fn priv2bbr2(self) -> crate::common::Reg<regs::Priv2bbr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 3"]
    #[inline(always)]
    pub const fn priv2bbr3(self) -> crate::common::Reg<regs::Priv2bbr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "FLASH privilege block based bank 2 register 4"]
    #[inline(always)]
    pub const fn priv2bbr4(self) -> crate::common::Reg<regs::Priv2bbr4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
}
pub mod regs {
    #[doc = "FLASH access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ..."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Latency These bits represent the ratio between the HCLK (AHB clock) period and the Flash memory access time. ..."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory."]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable This bit enables the prefetch buffer in the embedded Flash memory."]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power read mode This bit puts the Flash memory in low-power read mode."]
        #[inline(always)]
        pub const fn lpm(&self) -> super::vals::Lpm {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Lpm::from_bits(val as u8)
        }
        #[doc = "Low-power read mode This bit puts the Flash memory in low-power read mode."]
        #[inline(always)]
        pub fn set_lpm(&mut self, val: super::vals::Lpm) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked."]
        #[inline(always)]
        pub const fn pdreq1(&self) -> super::vals::Pdreq {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::Pdreq::from_bits(val as u8)
        }
        #[doc = "Bank 1 power-down mode request This bit is write-protected with FLASH_PDKEY1R. This bit requests bank 1 to enter power-down mode. When bank 1 enters power-down mode, this bit is cleared by hardware and the PDKEY1R is locked."]
        #[inline(always)]
        pub fn set_pdreq1(&mut self, val: super::vals::Pdreq) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked."]
        #[inline(always)]
        pub const fn pdreq2(&self) -> super::vals::Pdreq {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::Pdreq::from_bits(val as u8)
        }
        #[doc = "Bank 2 power-down mode request This bit is write-protected with FLASH_PDKEY2R. This bit requests bank 2 to enter power-down mode. When bank 2 enters power-down mode, this bit is cleared by hardware and the PDKEY2R is locked."]
        #[inline(always)]
        pub fn set_pdreq2(&mut self, val: super::vals::Pdreq) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going."]
        #[inline(always)]
        pub const fn sleep_pd(&self) -> super::vals::SleepPd {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::SleepPd::from_bits(val as u8)
        }
        #[doc = "Flash memory power-down mode during Sleep mode This bit determines whether the Flash memory is in power-down mode or Idle mode when the device is in Sleep mode. The Flash must not be put in power-down while a program or an erase operation is on-going."]
        #[inline(always)]
        pub fn set_sleep_pd(&mut self, val: super::vals::SleepPd) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    #[doc = "FLASH ECC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr(pub u32);
    impl Eccr {
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "ECC fail address"]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "ECC fail bank"]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> super::vals::BkEcc {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::BkEcc::from_bits(val as u8)
        }
        #[doc = "ECC fail bank"]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: super::vals::BkEcc) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "System Flash memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system Flash memory."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set."]
        #[inline(always)]
        pub const fn eccie(&self) -> super::vals::Eccie {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::Eccie::from_bits(val as u8)
        }
        #[doc = "ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is set."]
        #[inline(always)]
        pub fn set_eccie(&mut self, val: super::vals::Eccie) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction This bit is set by hardware when one ECC error has been detected and corrected (only if ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection This bit is set by hardware when two ECC errors have been detected (only if ECCC and ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccr {
        #[inline(always)]
        fn default() -> Eccr {
            Eccr(0)
        }
    }
    #[doc = "FLASH non-secure boot address 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootadd0r(pub u32);
    impl Nsbootadd0r {
        #[doc = "Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
        #[inline(always)]
        pub const fn nsbootadd0(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Non-secure boot base address 0 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD0\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD0\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD0\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
        #[inline(always)]
        pub fn set_nsbootadd0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Nsbootadd0r {
        #[inline(always)]
        fn default() -> Nsbootadd0r {
            Nsbootadd0r(0)
        }
    }
    #[doc = "FLASH non-secure boot address 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootadd1r(pub u32);
    impl Nsbootadd1r {
        #[doc = "Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
        #[inline(always)]
        pub const fn nsbootadd1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Non-secure boot address 1 The non-secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from non-secure Flash memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F200: Boot from system memory bootloader (0x0BF9 0000) NSBOOTADD1\\[24:0\\]
= 0x0400000: Boot from non-secure SRAM1 on S-Bus (0x2000 0000)"]
        #[inline(always)]
        pub fn set_nsbootadd1(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Nsbootadd1r {
        #[inline(always)]
        fn default() -> Nsbootadd1r {
            Nsbootadd1r(0)
        }
    }
    #[doc = "FLASH non-secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr(pub u32);
    impl Nscr {
        #[doc = "Non-secure programming"]
        #[inline(always)]
        pub const fn pg(&self) -> super::vals::NscrPg {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::NscrPg::from_bits(val as u8)
        }
        #[doc = "Non-secure programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: super::vals::NscrPg) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-secure page erase"]
        #[inline(always)]
        pub const fn per(&self) -> super::vals::NscrPer {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::NscrPer::from_bits(val as u8)
        }
        #[doc = "Non-secure page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: super::vals::NscrPer) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set."]
        #[inline(always)]
        pub const fn mer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure bank 1 mass erase This bit triggers the bank 1 non-secure mass erase (all bank 1 user pages) when set."]
        #[inline(always)]
        pub fn set_mer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Non-secure page number selection These bits select the page to erase. ..."]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x7f;
            val as u8
        }
        #[doc = "Non-secure page number selection These bits select the page to erase. ..."]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
        }
        #[doc = "Non-secure bank selection for page erase"]
        #[inline(always)]
        pub const fn bker(&self) -> super::vals::NscrBker {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::NscrBker::from_bits(val as u8)
        }
        #[doc = "Non-secure bank selection for page erase"]
        #[inline(always)]
        pub fn set_bker(&mut self, val: super::vals::NscrBker) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Non-secure burst write programming mode When set, this bit selects the burst write programming mode."]
        #[inline(always)]
        pub const fn bwr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure burst write programming mode When set, this bit selects the burst write programming mode."]
        #[inline(always)]
        pub fn set_bwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set."]
        #[inline(always)]
        pub const fn mer2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure bank 2 mass erase This bit triggers the bank 2 non-secure mass erase (all bank 2 user pages) when set."]
        #[inline(always)]
        pub fn set_mer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure start This bit triggers a non-secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR bit in FLASH_NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_NSSR."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR."]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Options modification start This bit triggers an options operation when set. It can not be written if OPTLOCK bit is set. This bit is set only by software, and is cleared when the BSY bit is cleared in FLASH_NSSR."]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1."]
        #[inline(always)]
        pub const fn eopie(&self) -> super::vals::NscrEopie {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::NscrEopie::from_bits(val as u8)
        }
        #[doc = "Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_NSSR is set to 1."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: super::vals::NscrEopie) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1."]
        #[inline(always)]
        pub const fn errie(&self) -> super::vals::NscrErrie {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::NscrErrie::from_bits(val as u8)
        }
        #[doc = "Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the FLASH_NSSR is set to 1."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: super::vals::NscrErrie) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set."]
        #[inline(always)]
        pub const fn obl_launch(&self) -> super::vals::OblLaunch {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::OblLaunch::from_bits(val as u8)
        }
        #[doc = "Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. It cannot be written if OPTLOCK is set."]
        #[inline(always)]
        pub fn set_obl_launch(&mut self, val: super::vals::OblLaunch) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
        #[doc = "Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Option lock This bit is set only. When set, all bits concerning user options in FLASH_NSCR register are locked. This bit is cleared by hardware after detecting the unlock sequence. The LOCK bit in the FLASH_NSCR must be cleared before doing the unlock sequence for OPTLOCK bit. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure lock This bit is set only. When set, the FLASH_NSCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_NSKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Nscr {
        #[inline(always)]
        fn default() -> Nscr {
            Nscr(0)
        }
    }
    #[doc = "FLASH non-secure status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nssr(pub u32);
    impl Nssr {
        #[doc = "Non-secure end of operation"]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure end of operation"]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-secure operation error"]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure operation error"]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure programming error This bit is set by hardware when a non-secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure write protection error This bit is set by hardware when an non-secure address to be erased/programmed belongs to a write-protected part (by WRP, HDP or RDP level 1) of the Flash memory. This bit is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure size error This bit is set by hardware when the size of the access is a byte or half-word during a non-secure program sequence. Only quad-word programming is allowed by means of successive word accesses. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn optwerr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Option write error This bit is set by hardware when the options bytes are written with an invalid configuration. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_optwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Non-secure busy This indicates that a Flash memory secure or non-secure operation is in progress. This bit is set at the beginning of a Flash operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure busy This indicates that a Flash memory secure or non-secure operation is in progress. This bit is set at the beginning of a Flash operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Non-secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory."]
        #[inline(always)]
        pub const fn wdw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory."]
        #[inline(always)]
        pub fn set_wdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "OEM1 lock This bit indicates that the OEM1 RDP key read during the OBL is not virgin. When set, the OEM1 RDP lock mechanism is active."]
        #[inline(always)]
        pub const fn oem1lock(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "OEM1 lock This bit indicates that the OEM1 RDP key read during the OBL is not virgin. When set, the OEM1 RDP lock mechanism is active."]
        #[inline(always)]
        pub fn set_oem1lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "OEM2 lock This bit indicates that the OEM2 RDP key read during the OBL is not virgin. When set, the OEM2 RDP lock mechanism is active."]
        #[inline(always)]
        pub const fn oem2lock(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OEM2 lock This bit indicates that the OEM2 RDP key read during the OBL is not virgin. When set, the OEM2 RDP lock mechanism is active."]
        #[inline(always)]
        pub fn set_oem2lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "Bank 1 in power-down mode This bit indicates that the Flash memory bank 1 is in power-down state. It is reset when bank 1 is in normal mode or being awaken."]
        #[inline(always)]
        pub const fn pd1(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 1 in power-down mode This bit indicates that the Flash memory bank 1 is in power-down state. It is reset when bank 1 is in normal mode or being awaken."]
        #[inline(always)]
        pub fn set_pd1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Bank 2 in power-down mode This bit indicates that the Flash memory bank 2 is in power-down state. It is reset when bank 2 is in normal mode or being awaken."]
        #[inline(always)]
        pub const fn pd2(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Bank 2 in power-down mode This bit indicates that the Flash memory bank 2 is in power-down state. It is reset when bank 2 is in normal mode or being awaken."]
        #[inline(always)]
        pub fn set_pd2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Nssr {
        #[inline(always)]
        fn default() -> Nssr {
            Nssr(0)
        }
    }
    #[doc = "FLASH OEM1 key register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem1keyr1(pub u32);
    impl Oem1keyr1 {
        #[doc = "OEM1 least significant bytes key"]
        #[inline(always)]
        pub const fn oem1key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM1 least significant bytes key"]
        #[inline(always)]
        pub fn set_oem1key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem1keyr1 {
        #[inline(always)]
        fn default() -> Oem1keyr1 {
            Oem1keyr1(0)
        }
    }
    #[doc = "FLASH OEM1 key register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem1keyr2(pub u32);
    impl Oem1keyr2 {
        #[doc = "OEM1 most significant bytes key"]
        #[inline(always)]
        pub const fn oem1key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM1 most significant bytes key"]
        #[inline(always)]
        pub fn set_oem1key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem1keyr2 {
        #[inline(always)]
        fn default() -> Oem1keyr2 {
            Oem1keyr2(0)
        }
    }
    #[doc = "FLASH OEM2 key register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem2keyr1(pub u32);
    impl Oem2keyr1 {
        #[doc = "OEM2 least significant bytes key"]
        #[inline(always)]
        pub const fn oem2key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM2 least significant bytes key"]
        #[inline(always)]
        pub fn set_oem2key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem2keyr1 {
        #[inline(always)]
        fn default() -> Oem2keyr1 {
            Oem2keyr1(0)
        }
    }
    #[doc = "FLASH OEM2 key register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Oem2keyr2(pub u32);
    impl Oem2keyr2 {
        #[doc = "OEM2 most significant bytes key"]
        #[inline(always)]
        pub const fn oem2key(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OEM2 most significant bytes key"]
        #[inline(always)]
        pub fn set_oem2key(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Oem2keyr2 {
        #[inline(always)]
        fn default() -> Oem2keyr2 {
            Oem2keyr2(0)
        }
    }
    #[doc = "FLASH operation status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opsr(pub u32);
    impl Opsr {
        #[doc = "Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given by bank from address 0x0 0000 to 0xF FFF0."]
        #[inline(always)]
        pub const fn addr_op(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupted operation address This field indicates which address in the Flash memory was accessed when reset occurred. The address is given by bank from address 0x0 0000 to 0xF FFF0."]
        #[inline(always)]
        pub fn set_addr_op(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Interrupted operation bank This bit indicates which Flash memory bank was accessed when reset occurred"]
        #[inline(always)]
        pub const fn bk_op(&self) -> super::vals::BkOp {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::BkOp::from_bits(val as u8)
        }
        #[doc = "Interrupted operation bank This bit indicates which Flash memory bank was accessed when reset occurred"]
        #[inline(always)]
        pub fn set_bk_op(&mut self, val: super::vals::BkOp) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory."]
        #[inline(always)]
        pub const fn sysf_op(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Operation in system Flash memory interrupted This bit indicates that the reset occurred during an operation in the system Flash memory."]
        #[inline(always)]
        pub fn set_sysf_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:"]
        #[inline(always)]
        pub const fn code_op(&self) -> super::vals::CodeOp {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::CodeOp::from_bits(val as u8)
        }
        #[doc = "Flash memory operation code This field indicates which Flash memory operation has been interrupted by a system reset:"]
        #[inline(always)]
        pub fn set_code_op(&mut self, val: super::vals::CodeOp) {
            self.0 = (self.0 & !(0x07 << 29usize)) | (((val.to_bits() as u32) & 0x07) << 29usize);
        }
    }
    impl Default for Opsr {
        #[inline(always)]
        fn default() -> Opsr {
            Opsr(0)
        }
    }
    #[doc = "FLASH option register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optr(pub u32);
    impl Optr {
        #[doc = "Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details."]
        #[inline(always)]
        pub const fn rdp(&self) -> super::vals::Rdp {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Rdp::from_bits(val as u8)
        }
        #[doc = "Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to for more details."]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: super::vals::Rdp) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset."]
        #[inline(always)]
        pub const fn bor_lev(&self) -> super::vals::BorLev {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::BorLev::from_bits(val as u8)
        }
        #[doc = "BOR reset level These bits contain the VDD supply level threshold that activates/releases the reset."]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: super::vals::BorLev) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Reset generation in Stop mode"]
        #[inline(always)]
        pub const fn n_rst_stop(&self) -> super::vals::NRstStop {
            let val = (self.0 >> 12usize) & 0x01;
            super::vals::NRstStop::from_bits(val as u8)
        }
        #[doc = "Reset generation in Stop mode"]
        #[inline(always)]
        pub fn set_n_rst_stop(&mut self, val: super::vals::NRstStop) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset generation in Standby mode"]
        #[inline(always)]
        pub const fn n_rst_stdby(&self) -> super::vals::NRstStdby {
            let val = (self.0 >> 13usize) & 0x01;
            super::vals::NRstStdby::from_bits(val as u8)
        }
        #[doc = "Reset generation in Standby mode"]
        #[inline(always)]
        pub fn set_n_rst_stdby(&mut self, val: super::vals::NRstStdby) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
        }
        #[doc = "Reset generation in Shutdown mode"]
        #[inline(always)]
        pub const fn n_rst_shdw(&self) -> super::vals::NRstShdw {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::NRstShdw::from_bits(val as u8)
        }
        #[doc = "Reset generation in Shutdown mode"]
        #[inline(always)]
        pub fn set_n_rst_shdw(&mut self, val: super::vals::NRstShdw) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "SRAM1, SRAM3 and SRAM4 erase upon system reset"]
        #[inline(always)]
        pub const fn sram1345_rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1, SRAM3 and SRAM4 erase upon system reset"]
        #[inline(always)]
        pub fn set_sram1345_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> super::vals::IwdgSw {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::IwdgSw::from_bits(val as u8)
        }
        #[doc = "Independent watchdog selection"]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: super::vals::IwdgSw) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> super::vals::IwdgStop {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::IwdgStop::from_bits(val as u8)
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: super::vals::IwdgStop) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> super::vals::IwdgStdby {
            let val = (self.0 >> 18usize) & 0x01;
            super::vals::IwdgStdby::from_bits(val as u8)
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: super::vals::IwdgStdby) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> super::vals::WwdgSw {
            let val = (self.0 >> 19usize) & 0x01;
            super::vals::WwdgSw::from_bits(val as u8)
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: super::vals::WwdgSw) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
        }
        #[doc = "Swap banks"]
        #[inline(always)]
        pub const fn swap_bank(&self) -> super::vals::SwapBank {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::SwapBank::from_bits(val as u8)
        }
        #[doc = "Swap banks"]
        #[inline(always)]
        pub fn set_swap_bank(&mut self, val: super::vals::SwapBank) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices"]
        #[inline(always)]
        pub const fn dualbank(&self) -> super::vals::Dualbank {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::Dualbank::from_bits(val as u8)
        }
        #[doc = "Dual-bank on 1-Mbyte and 512-Kbyte Flash memory devices"]
        #[inline(always)]
        pub fn set_dualbank(&mut self, val: super::vals::Dualbank) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Backup RAM ECC detection and correction enable"]
        #[inline(always)]
        pub const fn bkpsram_ecc(&self) -> super::vals::BkpsramEcc {
            let val = (self.0 >> 22usize) & 0x01;
            super::vals::BkpsramEcc::from_bits(val as u8)
        }
        #[doc = "Backup RAM ECC detection and correction enable"]
        #[inline(always)]
        pub fn set_bkpsram_ecc(&mut self, val: super::vals::BkpsramEcc) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
        }
        #[doc = "SRAM3 ECC detection and correction enable"]
        #[inline(always)]
        pub const fn sram3_ecc(&self) -> super::vals::SramEcc {
            let val = (self.0 >> 23usize) & 0x01;
            super::vals::SramEcc::from_bits(val as u8)
        }
        #[doc = "SRAM3 ECC detection and correction enable"]
        #[inline(always)]
        pub fn set_sram3_ecc(&mut self, val: super::vals::SramEcc) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
        }
        #[doc = "SRAM2 ECC detection and correction enable"]
        #[inline(always)]
        pub const fn sram2_ecc(&self) -> super::vals::SramEcc {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::SramEcc::from_bits(val as u8)
        }
        #[doc = "SRAM2 ECC detection and correction enable"]
        #[inline(always)]
        pub fn set_sram2_ecc(&mut self, val: super::vals::SramEcc) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "SRAM2 erase when system reset"]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase when system reset"]
        #[inline(always)]
        pub fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Software BOOT0"]
        #[inline(always)]
        pub const fn n_swboot0(&self) -> super::vals::NSwboot {
            let val = (self.0 >> 26usize) & 0x01;
            super::vals::NSwboot::from_bits(val as u8)
        }
        #[doc = "Software BOOT0"]
        #[inline(always)]
        pub fn set_n_swboot0(&mut self, val: super::vals::NSwboot) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
        }
        #[doc = "nBOOT0 option bit"]
        #[inline(always)]
        pub const fn n_boot0(&self) -> super::vals::NBoot {
            let val = (self.0 >> 27usize) & 0x01;
            super::vals::NBoot::from_bits(val as u8)
        }
        #[doc = "nBOOT0 option bit"]
        #[inline(always)]
        pub fn set_n_boot0(&mut self, val: super::vals::NBoot) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
        }
        #[doc = "PA15 pull-up enable"]
        #[inline(always)]
        pub const fn pa15_pupen(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "PA15 pull-up enable"]
        #[inline(always)]
        pub fn set_pa15_pupen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V"]
        #[inline(always)]
        pub const fn io_vdd_hslv(&self) -> super::vals::IoVddHslv {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::IoVddHslv::from_bits(val as u8)
        }
        #[doc = "High-speed IO at low VDD voltage configuration bit This bit can be set only with VDD below 2.5V"]
        #[inline(always)]
        pub fn set_io_vdd_hslv(&mut self, val: super::vals::IoVddHslv) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V."]
        #[inline(always)]
        pub const fn io_vddio2_hslv(&self) -> super::vals::IoVddioHslv {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::IoVddioHslv::from_bits(val as u8)
        }
        #[doc = "High-speed IO at low VDDIO2 voltage configuration bit This bit can be set only with VDDIO2 below 2.5 V."]
        #[inline(always)]
        pub fn set_io_vddio2_hslv(&mut self, val: super::vals::IoVddioHslv) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Global TrustZone security enable"]
        #[inline(always)]
        pub const fn tzen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Global TrustZone security enable"]
        #[inline(always)]
        pub fn set_tzen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optr {
        #[inline(always)]
        fn default() -> Optr {
            Optr(0)
        }
    }
    #[doc = "FLASH bank 1 power-down key register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdkey1r(pub u32);
    impl Pdkey1r {
        #[doc = "Bank 1 power-down key"]
        #[inline(always)]
        pub const fn pdkey1(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bank 1 power-down key"]
        #[inline(always)]
        pub fn set_pdkey1(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pdkey1r {
        #[inline(always)]
        fn default() -> Pdkey1r {
            Pdkey1r(0)
        }
    }
    #[doc = "FLASH bank 2 power-down key register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pdkey2r(pub u32);
    impl Pdkey2r {
        #[doc = "Bank 2 power-down key"]
        #[inline(always)]
        pub const fn pdkey2(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bank 2 power-down key"]
        #[inline(always)]
        pub fn set_pdkey2(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Pdkey2r {
        #[inline(always)]
        fn default() -> Pdkey2r {
            Pdkey2r(0)
        }
    }
    #[doc = "FLASH privilege block based bank 1 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv1bbr1(pub u32);
    impl Priv1bbr1 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv1bbr1 {
        #[inline(always)]
        fn default() -> Priv1bbr1 {
            Priv1bbr1(0)
        }
    }
    #[doc = "FLASH privilege block based bank 1 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv1bbr2(pub u32);
    impl Priv1bbr2 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv1bbr2 {
        #[inline(always)]
        fn default() -> Priv1bbr2 {
            Priv1bbr2(0)
        }
    }
    #[doc = "FLASH privilege block based bank 1 register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv1bbr3(pub u32);
    impl Priv1bbr3 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv1bbr3 {
        #[inline(always)]
        fn default() -> Priv1bbr3 {
            Priv1bbr3(0)
        }
    }
    #[doc = "FLASH privilege block based bank 1 register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv1bbr4(pub u32);
    impl Priv1bbr4 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv1bbr4 {
        #[inline(always)]
        fn default() -> Priv1bbr4 {
            Priv1bbr4(0)
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv2bbr1(pub u32);
    impl Priv2bbr1 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv2bbr1 {
        #[inline(always)]
        fn default() -> Priv2bbr1 {
            Priv2bbr1(0)
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv2bbr2(pub u32);
    impl Priv2bbr2 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv2bbr2 {
        #[inline(always)]
        fn default() -> Priv2bbr2 {
            Priv2bbr2(0)
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv2bbr3(pub u32);
    impl Priv2bbr3 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv2bbr3 {
        #[inline(always)]
        fn default() -> Priv2bbr3 {
            Priv2bbr3(0)
        }
    }
    #[doc = "FLASH privilege block based bank 2 register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Priv2bbr4(pub u32);
    impl Priv2bbr4 {
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub const fn priv2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page privileged/unprivileged attribution"]
        #[inline(always)]
        pub fn set_priv2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Priv2bbr4 {
        #[inline(always)]
        fn default() -> Priv2bbr4 {
            Priv2bbr4(0)
        }
    }
    #[doc = "FLASH privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored."]
        #[inline(always)]
        pub const fn spriv(&self) -> super::vals::Spriv {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::Spriv::from_bits(val as u8)
        }
        #[doc = "Privileged protection for secure registers This bit can be accessed only when TrustZone is enabled (TZEN = 1). This bit can be read by both privileged or unprivileged, secure and non-secure access. The SPRIV bit can be written only by a secure privileged access. A non-secure write access on SPRIV bit is ignored. A secure unprivileged write access on SPRIV bit is ignored."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: super::vals::Spriv) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored."]
        #[inline(always)]
        pub const fn nspriv(&self) -> super::vals::Nspriv {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Nspriv::from_bits(val as u8)
        }
        #[doc = "Privileged protection for non-secure registers This bit can be read by both privileged or unprivileged, secure and non-secure access. The NSPRIV bit can be written by a secure or non-secure privileged access. A secure or non-secure unprivileged write access on NSPRIV bit is ignored."]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: super::vals::Nspriv) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    #[doc = "FLASH secure block based bank 1 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec1bbr1(pub u32);
    impl Sec1bbr1 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec1bbr1 {
        #[inline(always)]
        fn default() -> Sec1bbr1 {
            Sec1bbr1(0)
        }
    }
    #[doc = "FLASH secure block based bank 1 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec1bbr2(pub u32);
    impl Sec1bbr2 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec1bbr2 {
        #[inline(always)]
        fn default() -> Sec1bbr2 {
            Sec1bbr2(0)
        }
    }
    #[doc = "FLASH secure block based bank 1 register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec1bbr3(pub u32);
    impl Sec1bbr3 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec1bbr3 {
        #[inline(always)]
        fn default() -> Sec1bbr3 {
            Sec1bbr3(0)
        }
    }
    #[doc = "FLASH secure block based bank 1 register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec1bbr4(pub u32);
    impl Sec1bbr4 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec1bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec1bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec1bbr4 {
        #[inline(always)]
        fn default() -> Sec1bbr4 {
            Sec1bbr4(0)
        }
    }
    #[doc = "FLASH secure block based bank 2 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec2bbr1(pub u32);
    impl Sec2bbr1 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec2bbr1 {
        #[inline(always)]
        fn default() -> Sec2bbr1 {
            Sec2bbr1(0)
        }
    }
    #[doc = "FLASH secure block based bank 2 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec2bbr2(pub u32);
    impl Sec2bbr2 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec2bbr2 {
        #[inline(always)]
        fn default() -> Sec2bbr2 {
            Sec2bbr2(0)
        }
    }
    #[doc = "FLASH secure block based bank 2 register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec2bbr3(pub u32);
    impl Sec2bbr3 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec2bbr3 {
        #[inline(always)]
        fn default() -> Sec2bbr3 {
            Sec2bbr3(0)
        }
    }
    #[doc = "FLASH secure block based bank 2 register 4"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sec2bbr4(pub u32);
    impl Sec2bbr4 {
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb0(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb1(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb2(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb3(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb4(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb4(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb5(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb5(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb6(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb6(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb7(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb7(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb8(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb8(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb9(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb9(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb10(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb10(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb11(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb11(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb12(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb12(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb13(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb13(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb14(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb14(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb15(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb15(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb16(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb16(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb17(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb17(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb18(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb18(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb19(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb19(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb20(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb20(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb21(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb21(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb22(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb22(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb23(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb23(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb24(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb24(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb25(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb25(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb26(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb26(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb27(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb27(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb28(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb28(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb29(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb29(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb30(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb30(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub const fn sec2bb31(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "page secure/non-secure attribution"]
        #[inline(always)]
        pub fn set_sec2bb31(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Sec2bbr4 {
        #[inline(always)]
        fn default() -> Sec2bbr4 {
            Sec2bbr4(0)
        }
    }
    #[doc = "FLASH secure boot address 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbootadd0r(pub u32);
    impl Secbootadd0r {
        #[doc = "Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\\[24:0\\]
option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0."]
        #[inline(always)]
        pub const fn boot_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Boot lock When set, the boot is always forced to base address value programmed in SECBOOTADD0\\[24:0\\]
option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP at level 0."]
        #[inline(always)]
        pub fn set_boot_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \\[31:7\\]
The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\\[24:0\\]
= 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\\[24:0\\]
= 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\\[24:0\\]
= 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)"]
        #[inline(always)]
        pub const fn secbootadd0(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Secure boot base address 0 The secure boot memory address can be programmed to any address in the valid address range with a granularity of 128 bytes. This bits correspond to address \\[31:7\\]
The SECBOOTADD0 option bytes are selected following the BOOT0 pin or nSWBOOT0 state. Examples: SECBOOTADD0\\[24:0\\]
= 0x018 0000: Boot from secure Flash memory (0x0C00 0000) SECBOOTADD0\\[24:0\\]
= 0x01F F000: Boot from RSS (0x0FF8 0000) SECBOOTADD0\\[24:0\\]
= 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)"]
        #[inline(always)]
        pub fn set_secbootadd0(&mut self, val: u32) {
            self.0 = (self.0 & !(0x01ff_ffff << 7usize)) | (((val as u32) & 0x01ff_ffff) << 7usize);
        }
    }
    impl Default for Secbootadd0r {
        #[inline(always)]
        fn default() -> Secbootadd0r {
            Secbootadd0r(0)
        }
    }
    #[doc = "FLASH secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccr(pub u32);
    impl Seccr {
        #[doc = "Secure programming"]
        #[inline(always)]
        pub const fn pg(&self) -> super::vals::SeccrPg {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::SeccrPg::from_bits(val as u8)
        }
        #[doc = "Secure programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: super::vals::SeccrPg) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure page erase"]
        #[inline(always)]
        pub const fn per(&self) -> super::vals::SeccrPer {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::SeccrPer::from_bits(val as u8)
        }
        #[doc = "Secure page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: super::vals::SeccrPer) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set."]
        #[inline(always)]
        pub const fn mer1(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Secure bank 1 mass erase This bit triggers the bank 1 secure mass erase (all bank 1 user pages) when set."]
        #[inline(always)]
        pub fn set_mer1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Secure page number selection These bits select the page to erase: ..."]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x7f;
            val as u8
        }
        #[doc = "Secure page number selection These bits select the page to erase: ..."]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
        }
        #[doc = "Secure bank selection for page erase"]
        #[inline(always)]
        pub const fn bker(&self) -> super::vals::SeccrBker {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::SeccrBker::from_bits(val as u8)
        }
        #[doc = "Secure bank selection for page erase"]
        #[inline(always)]
        pub fn set_bker(&mut self, val: super::vals::SeccrBker) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "Secure burst write programming mode When set, this bit selects the burst write programming mode."]
        #[inline(always)]
        pub const fn bwr(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Secure burst write programming mode When set, this bit selects the burst write programming mode."]
        #[inline(always)]
        pub fn set_bwr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set."]
        #[inline(always)]
        pub const fn mer2(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Secure bank 2 mass erase This bit triggers the bank 2 secure mass erase (all bank 2 user pages) when set."]
        #[inline(always)]
        pub fn set_mer2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure start This bit triggers a secure erase operation when set. If MER1, MER2 and PER bits are reset and the STRT bit is set, the PGSERR in the FLASH_SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in FLASH_SECSR."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1."]
        #[inline(always)]
        pub const fn eopie(&self) -> super::vals::SeccrEopie {
            let val = (self.0 >> 24usize) & 0x01;
            super::vals::SeccrEopie::from_bits(val as u8)
        }
        #[doc = "Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the FLASH_SECSR is set to 1."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: super::vals::SeccrEopie) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
        }
        #[doc = "Secure error interrupt enable"]
        #[inline(always)]
        pub const fn errie(&self) -> super::vals::SeccrErrie {
            let val = (self.0 >> 25usize) & 0x01;
            super::vals::SeccrErrie::from_bits(val as u8)
        }
        #[doc = "Secure error interrupt enable"]
        #[inline(always)]
        pub fn set_errie(&mut self, val: super::vals::SeccrErrie) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
        }
        #[doc = "Secure PCROP read error interrupt enable"]
        #[inline(always)]
        pub const fn rderrie(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Secure PCROP read error interrupt enable"]
        #[inline(always)]
        pub fn set_rderrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Flash memory security state invert This bit inverts the Flash memory security state."]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Flash memory security state invert This bit inverts the Flash memory security state."]
        #[inline(always)]
        pub fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Secure lock This bit is set only. When set, the FLASH_SECCR register is locked. It is cleared by hardware after detecting the unlock sequence in FLASH_SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccr {
        #[inline(always)]
        fn default() -> Seccr {
            Seccr(0)
        }
    }
    #[doc = "FLASH secure HDP control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sechdpcr(pub u32);
    impl Sechdpcr {
        #[doc = "HDP1 area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub const fn hdp1_accdis(&self) -> super::vals::HdpAccdis {
            let val = (self.0 >> 0usize) & 0x01;
            super::vals::HdpAccdis::from_bits(val as u8)
        }
        #[doc = "HDP1 area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub fn set_hdp1_accdis(&mut self, val: super::vals::HdpAccdis) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
        }
        #[doc = "HDP2 area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub const fn hdp2_accdis(&self) -> super::vals::HdpAccdis {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::HdpAccdis::from_bits(val as u8)
        }
        #[doc = "HDP2 area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub fn set_hdp2_accdis(&mut self, val: super::vals::HdpAccdis) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Sechdpcr {
        #[inline(always)]
        fn default() -> Sechdpcr {
            Sechdpcr(0)
        }
    }
    #[doc = "FLASH secure status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secsr(pub u32);
    impl Secsr {
        #[doc = "Secure end of operation This bit is set by hardware when one or more Flash memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_SECCR). This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure end of operation This bit is set by hardware when one or more Flash memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in FLASH_SECCR). This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure operation error This bit is set by hardware when a Flash memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure operation error This bit is set by hardware when a Flash memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_operr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure programming error This bit is set by hardware when a secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn progerr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming error This bit is set by hardware when a secure quad-word address to be programmed contains a value different from all 1 before programming, except if the data to write is all 0. This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_progerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory.This bit is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP, PCROP, HDP or RDP level 1) of the Flash memory.This bit is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address.This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn pgaerr(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming alignment error This bit is set by hardware when the first word to be programmed is not aligned with a quad-word address, or the second, third or forth word does not belong to the same quad-word address.This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_pgaerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Secure size error This bit is set by hardware when the size of the access is a byte or half-word during a secure program sequence. Only quad-word programming is allowed by means of successive word accesses.This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn sizerr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Secure size error This bit is set by hardware when the size of the access is a byte or half-word during a secure program sequence. Only quad-word programming is allowed by means of successive word accesses.This bit is cleared by writing 1."]
        #[inline(always)]
        pub fn set_sizerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure busy This bit indicates that a Flash memory secure or non-secure operation is in progress. This is set on the beginning of a Flash operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure busy This bit indicates that a Flash memory secure or non-secure operation is in progress. This is set on the beginning of a Flash operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory."]
        #[inline(always)]
        pub const fn wdw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Secure wait data to write This bit indicates that the Flash memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the Flash memory."]
        #[inline(always)]
        pub fn set_wdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
    }
    impl Default for Secsr {
        #[inline(always)]
        fn default() -> Secsr {
            Secsr(0)
        }
    }
    #[doc = "FLASH secure watermark1 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm1r1(pub u32);
    impl Secwm1r1 {
        #[doc = "Start page of first secure area This field contains the first page of the secure area in bank 1."]
        #[inline(always)]
        pub const fn secwm1_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Start page of first secure area This field contains the first page of the secure area in bank 1."]
        #[inline(always)]
        pub fn set_secwm1_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "End page of first secure area This field contains the last page of the secure area in bank 1."]
        #[inline(always)]
        pub const fn secwm1_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of first secure area This field contains the last page of the secure area in bank 1."]
        #[inline(always)]
        pub fn set_secwm1_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Secwm1r1 {
        #[inline(always)]
        fn default() -> Secwm1r1 {
            Secwm1r1(0)
        }
    }
    #[doc = "FLASH secure watermark1 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm1r2(pub u32);
    impl Secwm1r2 {
        #[doc = "End page of first hide protection area This field contains the last page of the HDP area in bank 1."]
        #[inline(always)]
        pub const fn hdp1_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of first hide protection area This field contains the last page of the HDP area in bank 1."]
        #[inline(always)]
        pub fn set_hdp1_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Hide protection first area enable"]
        #[inline(always)]
        pub const fn hdp1en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Hide protection first area enable"]
        #[inline(always)]
        pub fn set_hdp1en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secwm1r2 {
        #[inline(always)]
        fn default() -> Secwm1r2 {
            Secwm1r2(0)
        }
    }
    #[doc = "FLASH secure watermark2 register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm2r1(pub u32);
    impl Secwm2r1 {
        #[doc = "Start page of second secure area This field contains the first page of the secure area in bank 2."]
        #[inline(always)]
        pub const fn secwm2_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Start page of second secure area This field contains the first page of the secure area in bank 2."]
        #[inline(always)]
        pub fn set_secwm2_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "End page of second secure area This field contains the last page of the secure area in bank 2."]
        #[inline(always)]
        pub const fn secwm2_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of second secure area This field contains the last page of the secure area in bank 2."]
        #[inline(always)]
        pub fn set_secwm2_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Secwm2r1 {
        #[inline(always)]
        fn default() -> Secwm2r1 {
            Secwm2r1(0)
        }
    }
    #[doc = "FLASH secure watermark2 register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwm2r2(pub u32);
    impl Secwm2r2 {
        #[doc = "End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2."]
        #[inline(always)]
        pub const fn hdp2_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of hide protection second area HDP2_PEND contains the last page of the HDP area in bank 2."]
        #[inline(always)]
        pub fn set_hdp2_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Hide protection second area enable"]
        #[inline(always)]
        pub const fn hdp2en(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Hide protection second area enable"]
        #[inline(always)]
        pub fn set_hdp2en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secwm2r2 {
        #[inline(always)]
        fn default() -> Secwm2r2 {
            Secwm2r2(0)
        }
    }
    #[doc = "FLASH WRP1 area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1ar(pub u32);
    impl Wrp1ar {
        #[doc = "bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1."]
        #[inline(always)]
        pub const fn wrp1a_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "bank 1 WPR first area A start page This field contains the first page of the first WPR area for bank 1."]
        #[inline(always)]
        pub fn set_wrp1a_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1."]
        #[inline(always)]
        pub const fn wrp1a_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WPR first area A end page This field contains the last page of the first WPR area in bank 1."]
        #[inline(always)]
        pub fn set_wrp1a_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 1 WPR first area A unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> super::vals::WrparUnlock {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::WrparUnlock::from_bits(val as u8)
        }
        #[doc = "Bank 1 WPR first area A unlock"]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: super::vals::WrparUnlock) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrp1ar {
        #[inline(always)]
        fn default() -> Wrp1ar {
            Wrp1ar(0)
        }
    }
    #[doc = "FLASH WRP1 area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp1br(pub u32);
    impl Wrp1br {
        #[doc = "Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1."]
        #[inline(always)]
        pub const fn wrp1b_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B start page This field contains the first page of the second WRP area for bank 1."]
        #[inline(always)]
        pub fn set_wrp1b_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1."]
        #[inline(always)]
        pub const fn wrp1b_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 1 WRP second area B end page This field contains the last page of the second WRP area in bank 1."]
        #[inline(always)]
        pub fn set_wrp1b_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 1 WPR second area B unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> super::vals::WrpbrUnlock {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::WrpbrUnlock::from_bits(val as u8)
        }
        #[doc = "Bank 1 WPR second area B unlock"]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: super::vals::WrpbrUnlock) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrp1br {
        #[inline(always)]
        fn default() -> Wrp1br {
            Wrp1br(0)
        }
    }
    #[doc = "FLASH WPR2 area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2ar(pub u32);
    impl Wrp2ar {
        #[doc = "Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2."]
        #[inline(always)]
        pub const fn wrp2a_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR first area A start page This field contains the first page of the first WRP area for bank 2."]
        #[inline(always)]
        pub fn set_wrp2a_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2."]
        #[inline(always)]
        pub const fn wrp2a_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR first area A end page This field contains the last page of the first WRP area in bank 2."]
        #[inline(always)]
        pub fn set_wrp2a_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 2 WPR first area A unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> super::vals::WrparUnlock {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::WrparUnlock::from_bits(val as u8)
        }
        #[doc = "Bank 2 WPR first area A unlock"]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: super::vals::WrparUnlock) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrp2ar {
        #[inline(always)]
        fn default() -> Wrp2ar {
            Wrp2ar(0)
        }
    }
    #[doc = "FLASH WPR2 area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp2br(pub u32);
    impl Wrp2br {
        #[doc = "Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2."]
        #[inline(always)]
        pub const fn wrp2b_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR second area B start page This field contains the first page of the second WRP area for bank 2."]
        #[inline(always)]
        pub fn set_wrp2b_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2."]
        #[inline(always)]
        pub const fn wrp2b_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "Bank 2 WPR second area B end page This field contains the last page of the second WRP area in bank 2."]
        #[inline(always)]
        pub fn set_wrp2b_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Bank 2 WPR second area B unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> super::vals::WrpbrUnlock {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::WrpbrUnlock::from_bits(val as u8)
        }
        #[doc = "Bank 2 WPR second area B unlock"]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: super::vals::WrpbrUnlock) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrp2br {
        #[inline(always)]
        fn default() -> Wrp2br {
            Wrp2br(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BkEcc {
        #[doc = "Bank 1"]
        B_0X0 = 0x0,
        #[doc = "Bank 2"]
        B_0X1 = 0x01,
    }
    impl BkEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BkEcc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BkEcc {
        #[inline(always)]
        fn from(val: u8) -> BkEcc {
            BkEcc::from_bits(val)
        }
    }
    impl From<BkEcc> for u8 {
        #[inline(always)]
        fn from(val: BkEcc) -> u8 {
            BkEcc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BkOp {
        #[doc = "Bank 1"]
        B_0X0 = 0x0,
        #[doc = "Bank 2"]
        B_0X1 = 0x01,
    }
    impl BkOp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BkOp {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BkOp {
        #[inline(always)]
        fn from(val: u8) -> BkOp {
            BkOp::from_bits(val)
        }
    }
    impl From<BkOp> for u8 {
        #[inline(always)]
        fn from(val: BkOp) -> u8 {
            BkOp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BkpsramEcc {
        #[doc = "Backup RAM ECC check enabled"]
        B_0X0 = 0x0,
        #[doc = "Backup RAM ECC check disabled"]
        B_0X1 = 0x01,
    }
    impl BkpsramEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BkpsramEcc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BkpsramEcc {
        #[inline(always)]
        fn from(val: u8) -> BkpsramEcc {
            BkpsramEcc::from_bits(val)
        }
    }
    impl From<BkpsramEcc> for u8 {
        #[inline(always)]
        fn from(val: BkpsramEcc) -> u8 {
            BkpsramEcc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BorLev {
        #[doc = "BOR level 0 (reset level threshold around 1.7 V)"]
        B_0X0 = 0x0,
        #[doc = "BOR level 1 (reset level threshold around 2.0 V)"]
        B_0X1 = 0x01,
        #[doc = "BOR level 2 (reset level threshold around 2.2 V)"]
        B_0X2 = 0x02,
        #[doc = "BOR level 3 (reset level threshold around 2.5 V)"]
        B_0X3 = 0x03,
        #[doc = "BOR level 4 (reset level threshold around 2.8 V)"]
        B_0X4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl BorLev {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> BorLev {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for BorLev {
        #[inline(always)]
        fn from(val: u8) -> BorLev {
            BorLev::from_bits(val)
        }
    }
    impl From<BorLev> for u8 {
        #[inline(always)]
        fn from(val: BorLev) -> u8 {
            BorLev::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CodeOp {
        #[doc = "No Flash operation interrupted by previous reset"]
        B_0X0 = 0x0,
        #[doc = "Single write operation interrupted"]
        B_0X1 = 0x01,
        #[doc = "Burst write operation interrupted"]
        B_0X2 = 0x02,
        #[doc = "Page erase operation interrupted"]
        B_0X3 = 0x03,
        #[doc = "Bank erase operation interrupted"]
        B_0X4 = 0x04,
        #[doc = "Mass erase operation interrupted"]
        B_0X5 = 0x05,
        #[doc = "Option change operation interrupted"]
        B_0X6 = 0x06,
        _RESERVED_7 = 0x07,
    }
    impl CodeOp {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> CodeOp {
            unsafe { core::mem::transmute(val & 0x07) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for CodeOp {
        #[inline(always)]
        fn from(val: u8) -> CodeOp {
            CodeOp::from_bits(val)
        }
    }
    impl From<CodeOp> for u8 {
        #[inline(always)]
        fn from(val: CodeOp) -> u8 {
            CodeOp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dualbank {
        #[doc = "Single bank Flash with contiguous address in bank 1"]
        B_0X0 = 0x0,
        #[doc = "Dual-bank Flash with contiguous addresses"]
        B_0X1 = 0x01,
    }
    impl Dualbank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dualbank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dualbank {
        #[inline(always)]
        fn from(val: u8) -> Dualbank {
            Dualbank::from_bits(val)
        }
    }
    impl From<Dualbank> for u8 {
        #[inline(always)]
        fn from(val: Dualbank) -> u8 {
            Dualbank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Eccie {
        #[doc = "ECCC interrupt disabled"]
        B_0X0 = 0x0,
        #[doc = "ECCC interrupt enabled."]
        B_0X1 = 0x01,
    }
    impl Eccie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Eccie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Eccie {
        #[inline(always)]
        fn from(val: u8) -> Eccie {
            Eccie::from_bits(val)
        }
    }
    impl From<Eccie> for u8 {
        #[inline(always)]
        fn from(val: Eccie) -> u8 {
            Eccie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum HdpAccdis {
        #[doc = "Access to HDP2 area granted"]
        B_0X0 = 0x0,
        #[doc = "Access to HDP2 area denied (SECWM2Ry option bytes modification bocked -refer to )"]
        B_0X1 = 0x01,
    }
    impl HdpAccdis {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> HdpAccdis {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for HdpAccdis {
        #[inline(always)]
        fn from(val: u8) -> HdpAccdis {
            HdpAccdis::from_bits(val)
        }
    }
    impl From<HdpAccdis> for u8 {
        #[inline(always)]
        fn from(val: HdpAccdis) -> u8 {
            HdpAccdis::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum IoVddHslv {
        #[doc = "High-speed IO at low VDD voltage feature disabled (VDD can exceed 2.5 V)"]
        B_0X0 = 0x0,
        #[doc = "High-speed IO at low VDD voltage feature enabled (VDD remains below 2.5 V)"]
        B_0X1 = 0x01,
    }
    impl IoVddHslv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IoVddHslv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IoVddHslv {
        #[inline(always)]
        fn from(val: u8) -> IoVddHslv {
            IoVddHslv::from_bits(val)
        }
    }
    impl From<IoVddHslv> for u8 {
        #[inline(always)]
        fn from(val: IoVddHslv) -> u8 {
            IoVddHslv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum IoVddioHslv {
        #[doc = "High-speed IO at low VDDIO2 voltage feature disabled (VDDIO2 can exceed 2.5 V)"]
        B_0X0 = 0x0,
        #[doc = "High-speed IO at low VDDIO2 voltage feature enabled (VDDIO2 remains below 2.5 V)"]
        B_0X1 = 0x01,
    }
    impl IoVddioHslv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IoVddioHslv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IoVddioHslv {
        #[inline(always)]
        fn from(val: u8) -> IoVddioHslv {
            IoVddioHslv::from_bits(val)
        }
    }
    impl From<IoVddioHslv> for u8 {
        #[inline(always)]
        fn from(val: IoVddioHslv) -> u8 {
            IoVddioHslv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum IwdgStdby {
        #[doc = "Independent watchdog counter frozen in Standby mode"]
        B_0X0 = 0x0,
        #[doc = "Independent watchdog counter running in Standby mode"]
        B_0X1 = 0x01,
    }
    impl IwdgStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IwdgStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IwdgStdby {
        #[inline(always)]
        fn from(val: u8) -> IwdgStdby {
            IwdgStdby::from_bits(val)
        }
    }
    impl From<IwdgStdby> for u8 {
        #[inline(always)]
        fn from(val: IwdgStdby) -> u8 {
            IwdgStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum IwdgStop {
        #[doc = "Independent watchdog counter frozen in Stop mode"]
        B_0X0 = 0x0,
        #[doc = "Independent watchdog counter running in Stop mode"]
        B_0X1 = 0x01,
    }
    impl IwdgStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IwdgStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IwdgStop {
        #[inline(always)]
        fn from(val: u8) -> IwdgStop {
            IwdgStop::from_bits(val)
        }
    }
    impl From<IwdgStop> for u8 {
        #[inline(always)]
        fn from(val: IwdgStop) -> u8 {
            IwdgStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum IwdgSw {
        #[doc = "Hardware independent watchdog selected"]
        B_0X0 = 0x0,
        #[doc = "Software independent watchdog selected"]
        B_0X1 = 0x01,
    }
    impl IwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> IwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for IwdgSw {
        #[inline(always)]
        fn from(val: u8) -> IwdgSw {
            IwdgSw::from_bits(val)
        }
    }
    impl From<IwdgSw> for u8 {
        #[inline(always)]
        fn from(val: IwdgSw) -> u8 {
            IwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lpm {
        #[doc = "Flash not in low-power read mode"]
        B_0X0 = 0x0,
        #[doc = "Flash in low-power read mode"]
        B_0X1 = 0x01,
    }
    impl Lpm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lpm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lpm {
        #[inline(always)]
        fn from(val: u8) -> Lpm {
            Lpm::from_bits(val)
        }
    }
    impl From<Lpm> for u8 {
        #[inline(always)]
        fn from(val: Lpm) -> u8 {
            Lpm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NBoot {
        #[doc = "nBOOT0 = 0"]
        B_0X0 = 0x0,
        #[doc = "nBOOT0 = 1"]
        B_0X1 = 0x01,
    }
    impl NBoot {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NBoot {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NBoot {
        #[inline(always)]
        fn from(val: u8) -> NBoot {
            NBoot::from_bits(val)
        }
    }
    impl From<NBoot> for u8 {
        #[inline(always)]
        fn from(val: NBoot) -> u8 {
            NBoot::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NRstShdw {
        #[doc = "Reset generated when entering the Shutdown mode"]
        B_0X0 = 0x0,
        #[doc = "No reset generated when entering the Shutdown mode"]
        B_0X1 = 0x01,
    }
    impl NRstShdw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NRstShdw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NRstShdw {
        #[inline(always)]
        fn from(val: u8) -> NRstShdw {
            NRstShdw::from_bits(val)
        }
    }
    impl From<NRstShdw> for u8 {
        #[inline(always)]
        fn from(val: NRstShdw) -> u8 {
            NRstShdw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NRstStdby {
        #[doc = "Reset generated when entering the Standby mode"]
        B_0X0 = 0x0,
        #[doc = "No reset generate when entering the Standby mode"]
        B_0X1 = 0x01,
    }
    impl NRstStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NRstStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NRstStdby {
        #[inline(always)]
        fn from(val: u8) -> NRstStdby {
            NRstStdby::from_bits(val)
        }
    }
    impl From<NRstStdby> for u8 {
        #[inline(always)]
        fn from(val: NRstStdby) -> u8 {
            NRstStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NRstStop {
        #[doc = "Reset generated when entering the Stop mode"]
        B_0X0 = 0x0,
        #[doc = "No reset generated when entering the Stop mode"]
        B_0X1 = 0x01,
    }
    impl NRstStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NRstStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NRstStop {
        #[inline(always)]
        fn from(val: u8) -> NRstStop {
            NRstStop::from_bits(val)
        }
    }
    impl From<NRstStop> for u8 {
        #[inline(always)]
        fn from(val: NRstStop) -> u8 {
            NRstStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NSwboot {
        #[doc = "BOOT0 taken from the option bit nBOOT0"]
        B_0X0 = 0x0,
        #[doc = "BOOT0 taken from PH3/BOOT0 pin"]
        B_0X1 = 0x01,
    }
    impl NSwboot {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NSwboot {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NSwboot {
        #[inline(always)]
        fn from(val: u8) -> NSwboot {
            NSwboot::from_bits(val)
        }
    }
    impl From<NSwboot> for u8 {
        #[inline(always)]
        fn from(val: NSwboot) -> u8 {
            NSwboot::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NscrBker {
        #[doc = "Bank 1 selected for non-secure page erase"]
        B_0X0 = 0x0,
        #[doc = "Bank 2 selected for non-secure page erase"]
        B_0X1 = 0x01,
    }
    impl NscrBker {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NscrBker {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NscrBker {
        #[inline(always)]
        fn from(val: u8) -> NscrBker {
            NscrBker::from_bits(val)
        }
    }
    impl From<NscrBker> for u8 {
        #[inline(always)]
        fn from(val: NscrBker) -> u8 {
            NscrBker::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NscrEopie {
        #[doc = "Non-secure EOP Interrupt disabled"]
        B_0X0 = 0x0,
        #[doc = "Non-secure EOP Interrupt enabled"]
        B_0X1 = 0x01,
    }
    impl NscrEopie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NscrEopie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NscrEopie {
        #[inline(always)]
        fn from(val: u8) -> NscrEopie {
            NscrEopie::from_bits(val)
        }
    }
    impl From<NscrEopie> for u8 {
        #[inline(always)]
        fn from(val: NscrEopie) -> u8 {
            NscrEopie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NscrErrie {
        #[doc = "Non-secure OPERR error interrupt disabled"]
        B_0X0 = 0x0,
        #[doc = "Non-secure OPERR error interrupt enabled"]
        B_0X1 = 0x01,
    }
    impl NscrErrie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NscrErrie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NscrErrie {
        #[inline(always)]
        fn from(val: u8) -> NscrErrie {
            NscrErrie::from_bits(val)
        }
    }
    impl From<NscrErrie> for u8 {
        #[inline(always)]
        fn from(val: NscrErrie) -> u8 {
            NscrErrie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NscrPer {
        #[doc = "Non-secure page erase disabled"]
        B_0X0 = 0x0,
        #[doc = "Non-secure page erase enabled"]
        B_0X1 = 0x01,
    }
    impl NscrPer {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NscrPer {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NscrPer {
        #[inline(always)]
        fn from(val: u8) -> NscrPer {
            NscrPer::from_bits(val)
        }
    }
    impl From<NscrPer> for u8 {
        #[inline(always)]
        fn from(val: NscrPer) -> u8 {
            NscrPer::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum NscrPg {
        #[doc = "Non-secure Flash programming disabled"]
        B_0X0 = 0x0,
        #[doc = "Non-secure Flash programming enabled"]
        B_0X1 = 0x01,
    }
    impl NscrPg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> NscrPg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for NscrPg {
        #[inline(always)]
        fn from(val: u8) -> NscrPg {
            NscrPg::from_bits(val)
        }
    }
    impl From<NscrPg> for u8 {
        #[inline(always)]
        fn from(val: NscrPg) -> u8 {
            NscrPg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Nspriv {
        #[doc = "Non-secure Flash registers can be read and written by privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "Non-secure Flash registers can be read and written by privileged access only."]
        B_0X1 = 0x01,
    }
    impl Nspriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Nspriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Nspriv {
        #[inline(always)]
        fn from(val: u8) -> Nspriv {
            Nspriv::from_bits(val)
        }
    }
    impl From<Nspriv> for u8 {
        #[inline(always)]
        fn from(val: Nspriv) -> u8 {
            Nspriv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OblLaunch {
        #[doc = "Option byte loading complete"]
        B_0X0 = 0x0,
        #[doc = "Option byte loading requested"]
        B_0X1 = 0x01,
    }
    impl OblLaunch {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OblLaunch {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OblLaunch {
        #[inline(always)]
        fn from(val: u8) -> OblLaunch {
            OblLaunch::from_bits(val)
        }
    }
    impl From<OblLaunch> for u8 {
        #[inline(always)]
        fn from(val: OblLaunch) -> u8 {
            OblLaunch::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pdreq {
        #[doc = "No request for bank 2 to enter power-down mode"]
        B_0X0 = 0x0,
        #[doc = "Bank 2 requested to enter power-down mode"]
        B_0X1 = 0x01,
    }
    impl Pdreq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pdreq {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pdreq {
        #[inline(always)]
        fn from(val: u8) -> Pdreq {
            Pdreq::from_bits(val)
        }
    }
    impl From<Pdreq> for u8 {
        #[inline(always)]
        fn from(val: Pdreq) -> u8 {
            Pdreq::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdp(pub u8);
    impl Rdp {
        #[doc = "Level 0.5 (readout protection not active, only non-secure debug access is possible). Only available when TrustZone is active (TZEN=1)"]
        pub const B_0X55: Self = Self(0x55);
        #[doc = "Level 0 (readout protection not active)"]
        pub const B_0XAA: Self = Self(0xaa);
        #[doc = "Level 2 (chip readout protection active)"]
        pub const B_0XCC: Self = Self(0xcc);
    }
    impl Rdp {
        pub const fn from_bits(val: u8) -> Rdp {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl From<u8> for Rdp {
        #[inline(always)]
        fn from(val: u8) -> Rdp {
            Rdp::from_bits(val)
        }
    }
    impl From<Rdp> for u8 {
        #[inline(always)]
        fn from(val: Rdp) -> u8 {
            Rdp::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SeccrBker {
        #[doc = "Bank 1 selected for secure page erase"]
        B_0X0 = 0x0,
        #[doc = "Bank 2 selected for secure page erase"]
        B_0X1 = 0x01,
    }
    impl SeccrBker {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SeccrBker {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SeccrBker {
        #[inline(always)]
        fn from(val: u8) -> SeccrBker {
            SeccrBker::from_bits(val)
        }
    }
    impl From<SeccrBker> for u8 {
        #[inline(always)]
        fn from(val: SeccrBker) -> u8 {
            SeccrBker::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SeccrEopie {
        #[doc = "Secure EOP Interrupt disabled"]
        B_0X0 = 0x0,
        #[doc = "Secure EOP Interrupt enabled"]
        B_0X1 = 0x01,
    }
    impl SeccrEopie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SeccrEopie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SeccrEopie {
        #[inline(always)]
        fn from(val: u8) -> SeccrEopie {
            SeccrEopie::from_bits(val)
        }
    }
    impl From<SeccrEopie> for u8 {
        #[inline(always)]
        fn from(val: SeccrEopie) -> u8 {
            SeccrEopie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SeccrErrie {
        #[doc = "Secure OPERR error interrupt disabled"]
        B_0X0 = 0x0,
        #[doc = "Secure OPERR error interrupt enabled"]
        B_0X1 = 0x01,
    }
    impl SeccrErrie {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SeccrErrie {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SeccrErrie {
        #[inline(always)]
        fn from(val: u8) -> SeccrErrie {
            SeccrErrie::from_bits(val)
        }
    }
    impl From<SeccrErrie> for u8 {
        #[inline(always)]
        fn from(val: SeccrErrie) -> u8 {
            SeccrErrie::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SeccrPer {
        #[doc = "Secure page erase disabled"]
        B_0X0 = 0x0,
        #[doc = "Secure page erase enabled"]
        B_0X1 = 0x01,
    }
    impl SeccrPer {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SeccrPer {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SeccrPer {
        #[inline(always)]
        fn from(val: u8) -> SeccrPer {
            SeccrPer::from_bits(val)
        }
    }
    impl From<SeccrPer> for u8 {
        #[inline(always)]
        fn from(val: SeccrPer) -> u8 {
            SeccrPer::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SeccrPg {
        #[doc = "Secure Flash programming disabled"]
        B_0X0 = 0x0,
        #[doc = "Secure Flash programming enabled"]
        B_0X1 = 0x01,
    }
    impl SeccrPg {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SeccrPg {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SeccrPg {
        #[inline(always)]
        fn from(val: u8) -> SeccrPg {
            SeccrPg::from_bits(val)
        }
    }
    impl From<SeccrPg> for u8 {
        #[inline(always)]
        fn from(val: SeccrPg) -> u8 {
            SeccrPg::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SleepPd {
        #[doc = "Flash in Idle mode during Sleep mode"]
        B_0X0 = 0x0,
        #[doc = "Flash in power-down mode during Sleep mode"]
        B_0X1 = 0x01,
    }
    impl SleepPd {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SleepPd {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SleepPd {
        #[inline(always)]
        fn from(val: u8) -> SleepPd {
            SleepPd::from_bits(val)
        }
    }
    impl From<SleepPd> for u8 {
        #[inline(always)]
        fn from(val: SleepPd) -> u8 {
            SleepPd::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Spriv {
        #[doc = "Secure Flash registers can be read and written by privileged or unprivileged access."]
        B_0X0 = 0x0,
        #[doc = "Secure Flash registers can be read and written by privileged access only."]
        B_0X1 = 0x01,
    }
    impl Spriv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Spriv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Spriv {
        #[inline(always)]
        fn from(val: u8) -> Spriv {
            Spriv::from_bits(val)
        }
    }
    impl From<Spriv> for u8 {
        #[inline(always)]
        fn from(val: Spriv) -> u8 {
            Spriv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SramEcc {
        #[doc = "SRAM3 ECC check enabled"]
        B_0X0 = 0x0,
        #[doc = "SRAM3 ECC check disabled"]
        B_0X1 = 0x01,
    }
    impl SramEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SramEcc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SramEcc {
        #[inline(always)]
        fn from(val: u8) -> SramEcc {
            SramEcc::from_bits(val)
        }
    }
    impl From<SramEcc> for u8 {
        #[inline(always)]
        fn from(val: SramEcc) -> u8 {
            SramEcc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum SwapBank {
        #[doc = "Bank 1 and bank 2 addresses not swapped"]
        B_0X0 = 0x0,
        #[doc = "Bank 1 and bank 2 addresses swapped"]
        B_0X1 = 0x01,
    }
    impl SwapBank {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> SwapBank {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for SwapBank {
        #[inline(always)]
        fn from(val: u8) -> SwapBank {
            SwapBank::from_bits(val)
        }
    }
    impl From<SwapBank> for u8 {
        #[inline(always)]
        fn from(val: SwapBank) -> u8 {
            SwapBank::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum WrparUnlock {
        #[doc = "WRP2A start and end pages locked"]
        B_0X0 = 0x0,
        #[doc = "WRP2A start and end pages unlocked"]
        B_0X1 = 0x01,
    }
    impl WrparUnlock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WrparUnlock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WrparUnlock {
        #[inline(always)]
        fn from(val: u8) -> WrparUnlock {
            WrparUnlock::from_bits(val)
        }
    }
    impl From<WrparUnlock> for u8 {
        #[inline(always)]
        fn from(val: WrparUnlock) -> u8 {
            WrparUnlock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum WrpbrUnlock {
        #[doc = "WRP2B start and end pages locked"]
        B_0X0 = 0x0,
        #[doc = "WRP2B start and end pages unlocked"]
        B_0X1 = 0x01,
    }
    impl WrpbrUnlock {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WrpbrUnlock {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WrpbrUnlock {
        #[inline(always)]
        fn from(val: u8) -> WrpbrUnlock {
            WrpbrUnlock::from_bits(val)
        }
    }
    impl From<WrpbrUnlock> for u8 {
        #[inline(always)]
        fn from(val: WrpbrUnlock) -> u8 {
            WrpbrUnlock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum WwdgSw {
        #[doc = "Hardware window watchdog selected"]
        B_0X0 = 0x0,
        #[doc = "Software window watchdog selected"]
        B_0X1 = 0x01,
    }
    impl WwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> WwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for WwdgSw {
        #[inline(always)]
        fn from(val: u8) -> WwdgSw {
            WwdgSw::from_bits(val)
        }
    }
    impl From<WwdgSw> for u8 {
        #[inline(always)]
        fn from(val: WwdgSw) -> u8 {
            WwdgSw::to_bits(val)
        }
    }
}
