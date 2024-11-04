#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "FLASH address block description"]
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
    #[doc = "FLASH key register"]
    #[inline(always)]
    pub const fn nskeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "FLASH operation status register"]
    #[inline(always)]
    pub const fn opsr(self) -> crate::common::Reg<regs::Opsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "FLASH option control register"]
    #[inline(always)]
    pub const fn optcr(self) -> crate::common::Reg<regs::Optcr, crate::common::RW> {
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
    #[doc = "FLASH Non Secure control register"]
    #[inline(always)]
    pub const fn nscr(self) -> crate::common::Reg<regs::Nscr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "FLASH non-secure clear control register"]
    #[inline(always)]
    pub const fn nsccr(self) -> crate::common::Reg<regs::Nsccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "FLASH privilege configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "FLASH HDP extension register"]
    #[inline(always)]
    pub const fn hdpextr(self) -> crate::common::Reg<regs::Hdpextr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_cur(self) -> crate::common::Reg<regs::Optsr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_prg(self) -> crate::common::Reg<regs::Optsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "FLASH option status register 2"]
    #[inline(always)]
    pub const fn optsr2_cur(self) -> crate::common::Reg<regs::Optsr2, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "FLASH option status register 2"]
    #[inline(always)]
    pub const fn optsr2_prg(self) -> crate::common::Reg<regs::Optsr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "FLASH non-secure unique boot entry register"]
    #[inline(always)]
    pub const fn nsbootr_cur(self) -> crate::common::Reg<regs::Nsbootr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "FLASH non-secure unique boot entry address"]
    #[inline(always)]
    pub const fn nsbootr_prg(self) -> crate::common::Reg<regs::Nsbootr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "FLASH non-secure OTP block lock"]
    #[inline(always)]
    pub const fn otpblr_cur(self) -> crate::common::Reg<regs::Otpblr, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "FLASH non-secure OTP block lock"]
    #[inline(always)]
    pub const fn otpblr_prg(self) -> crate::common::Reg<regs::Otpblr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "FLASH privilege register for bank 1"]
    #[inline(always)]
    pub const fn privbb1r(self) -> crate::common::Reg<regs::Privbb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "FLASH write sector protection for Bank1"]
    #[inline(always)]
    pub const fn wrpsgn1r_cur(self) -> crate::common::Reg<regs::Wrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "FLASH write sector protection for Bank1"]
    #[inline(always)]
    pub const fn wrpsgn1r_prg(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "FLASH HDP Bank1 register"]
    #[inline(always)]
    pub const fn hdp1r_cur(self) -> crate::common::Reg<regs::Hdp1r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "FLASH HDP Bank1 register"]
    #[inline(always)]
    pub const fn hdp1r_prg(self) -> crate::common::Reg<regs::Hdp1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "FLASH Flash ECC correction register"]
    #[inline(always)]
    pub const fn ecccorr(self) -> crate::common::Reg<regs::Ecccorr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "FLASH ECC detection register"]
    #[inline(always)]
    pub const fn eccdetr(self) -> crate::common::Reg<regs::Eccdetr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "FLASH ECC data"]
    #[inline(always)]
    pub const fn eccdr(self) -> crate::common::Reg<regs::Eccdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "FLASH write sector protection for Bank2"]
    #[inline(always)]
    pub const fn wrpsgn2r_cur(self) -> crate::common::Reg<regs::Wrp, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
    #[doc = "FLASH write sector protection for Bank2"]
    #[inline(always)]
    pub const fn wrpsgn2r_prg(self) -> crate::common::Reg<regs::Wrp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ecusize) as _) }
    }
    #[doc = "FLASH HDP Bank2 register"]
    #[inline(always)]
    pub const fn hdp2r_cur(self) -> crate::common::Reg<regs::Hdp2r, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01f8usize) as _) }
    }
    #[doc = "FLASH HDP Bank2 register"]
    #[inline(always)]
    pub const fn hdp2r_prg(self) -> crate::common::Reg<regs::Hdp2r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01fcusize) as _) }
    }
}
pub mod regs {
    #[doc = "FLASH access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Read latency These bits are used to control the number of wait states used during read operations on both non-volatile memory banks. The application software has to program them to the correct value depending on the embedded Flash memory interface frequency and voltage conditions. ... Note: No check is performed by hardware to verify that the configuration is correct."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
        #[inline(always)]
        pub const fn wrhighfreq(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "Flash signal delay These bits are used to control the delay between non-volatile memory signals during programming operations. Application software has to program them to the correct value depending on the embedded Flash memory interface frequency. Please refer to for details. Note: No check is performed to verify that the configuration is correct. Two WRHIGHFREQ values can be selected for some frequencies."]
        #[inline(always)]
        pub fn set_wrhighfreq(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable. When bit value is modified, user must read back ACR register to be sure PRFTEN has been taken into account. Bits used to control the prefetch."]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
        #[inline(always)]
        pub const fn s_prften(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Smart prefetch enable. When bit value is modified, user must read back ACR register to be sure S_PRFTEN has been taken into account. Bits used to control the prefetch functionality."]
        #[inline(always)]
        pub fn set_s_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    #[doc = "FLASH Flash ECC correction register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ecccorr(pub u32);
    impl Ecccorr {
        #[doc = "ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECC error address When an ECC error occurs (for single correction) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "ECC bank flag for corrected ECC error It indicates which bank is concerned by ECC error"]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ECC bank flag for corrected ECC error It indicates which bank is concerned by ECC error"]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC flag for corrected ECC error in system FLASH It indicates if system Flash memory is concerned by ECC error."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ECC flag for corrected ECC error in system FLASH It indicates if system Flash memory is concerned by ECC error."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
        #[inline(always)]
        pub const fn otp_ecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTP ECC error bit This bit is set to 1 when one single ECC correction occurred during the last successful read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bitfield."]
        #[inline(always)]
        pub fn set_otp_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
        #[inline(always)]
        pub const fn ecccie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "ECC single correction error interrupt enable bit When ECCCIE bit is set to 1, an interrupt is generated when an ECC single correction error occurs during a read operation."]
        #[inline(always)]
        pub fn set_ecccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
        #[inline(always)]
        pub const fn eccc(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction set by hardware when single ECC error has been detected and corrected. Cleared by writing 1."]
        #[inline(always)]
        pub fn set_eccc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Ecccorr {
        #[inline(always)]
        fn default() -> Ecccorr {
            Ecccorr(0)
        }
    }
    #[doc = "FLASH ECC detection register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdetr(pub u32);
    impl Eccdetr {
        #[doc = "ECC error address When an ECC error occurs (double detection) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an double ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECC error address When an ECC error occurs (double detection) during a read operation, the ADDR_ECC contains the address that generated the error. ADDR_ECC is reset when the flag error is reset. The embedded Flash memory programs the address in this register only when no ECC error flags are set. This means that only the first address that generated an double ECC error is saved. The address in ADDR_ECC is relative to the Flash memory area where the error occurred (user Flash memory, system Flash memory, data area, read-only/OTP area)."]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "ECC fail bank for double ECC Error It indicates which bank is concerned by ECC error"]
        #[inline(always)]
        pub const fn bk_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail bank for double ECC Error It indicates which bank is concerned by ECC error"]
        #[inline(always)]
        pub fn set_bk_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC fail for double ECC error in system Flash memory It indicates if system Flash memory is concerned by ECC error."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "ECC fail for double ECC error in system Flash memory It indicates if system Flash memory is concerned by ECC error."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTP ECC error bit This bit is set to 1 when double ECC detection occurred during the last read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bit field."]
        #[inline(always)]
        pub const fn otp_ecc(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTP ECC error bit This bit is set to 1 when double ECC detection occurred during the last read operation from the read-only/ OTP area. The address of the ECC error is available in ADDR_ECC bit field."]
        #[inline(always)]
        pub fn set_otp_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "ECC detection set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
        #[inline(always)]
        pub const fn eccd(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "ECC detection set by hardware when two ECC error has been detected. When this bit is set, a NMI is generated. Cleared by writing 1. Needs to be cleared in order to detect subsequent double ECC errors."]
        #[inline(always)]
        pub fn set_eccd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Eccdetr {
        #[inline(always)]
        fn default() -> Eccdetr {
            Eccdetr(0)
        }
    }
    #[doc = "FLASH ECC data"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccdr(pub u32);
    impl Eccdr {
        #[doc = "ECC error data When an double detection ECC error occurs on special areas with 6-bit ECC on 16-bit of data (data area, read-only/OTP area), the failing data is read to this register. By checking if it is possible to determine whether the failure was on a real data, or due to access to uninitialized memory."]
        #[inline(always)]
        pub const fn data_ecc(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "ECC error data When an double detection ECC error occurs on special areas with 6-bit ECC on 16-bit of data (data area, read-only/OTP area), the failing data is read to this register. By checking if it is possible to determine whether the failure was on a real data, or due to access to uninitialized memory."]
        #[inline(always)]
        pub fn set_data_ecc(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
    }
    impl Default for Eccdr {
        #[inline(always)]
        fn default() -> Eccdr {
            Eccdr(0)
        }
    }
    #[doc = "FLASH HDP Bank1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp1r(pub u32);
    impl Hdp1r {
        #[doc = "HDPL barrier start set in number of 8 Kbytes sectors"]
        #[inline(always)]
        pub const fn hdp1_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "HDPL barrier start set in number of 8 Kbytes sectors"]
        #[inline(always)]
        pub fn set_hdp1_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "HDPL barrier end set in number of 8 Kbytes sectors"]
        #[inline(always)]
        pub const fn hdp1_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "HDPL barrier end set in number of 8 Kbytes sectors"]
        #[inline(always)]
        pub fn set_hdp1_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Hdp1r {
        #[inline(always)]
        fn default() -> Hdp1r {
            Hdp1r(0)
        }
    }
    #[doc = "FLASH HDP Bank2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdp2r(pub u32);
    impl Hdp2r {
        #[doc = "Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
        #[inline(always)]
        pub const fn hdp2_strt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "Bank 2 HDPL barrier start set in number of 8 Kbytes sectors"]
        #[inline(always)]
        pub fn set_hdp2_strt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
        #[inline(always)]
        pub const fn hdp2_end(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "Bank 2 HDPL barrier end set in number of 8 Kbytes sectors"]
        #[inline(always)]
        pub fn set_hdp2_end(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Hdp2r {
        #[inline(always)]
        fn default() -> Hdp2r {
            Hdp2r(0)
        }
    }
    #[doc = "FLASH HDP extension register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Hdpextr(pub u32);
    impl Hdpextr {
        #[doc = "HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
        #[inline(always)]
        pub const fn hdp1_ext(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x07;
            val as u8
        }
        #[doc = "HDP area extension in 8 Kbytes sectors in Bank1. Extension is added after the HDP1_END sector."]
        #[inline(always)]
        pub fn set_hdp1_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
        }
        #[doc = "HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
        #[inline(always)]
        pub const fn hdp2_ext(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "HDP area extension in 8 Kbytes sectors in Bank2. Extension is added after the HDP2_END sector."]
        #[inline(always)]
        pub fn set_hdp2_ext(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
    }
    impl Default for Hdpextr {
        #[inline(always)]
        fn default() -> Hdpextr {
            Hdpextr(0)
        }
    }
    #[doc = "FLASH non-secure unique boot entry register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootr(pub u32);
    impl Nsbootr {
        #[doc = "A field locking the values of SWAP_BANK, and NSBOOTADD settings."]
        #[inline(always)]
        pub const fn nsboot_lock(&self) -> super::vals::NsbootrNsbootLock {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::NsbootrNsbootLock::from_bits(val as u8)
        }
        #[doc = "A field locking the values of SWAP_BANK, and NSBOOTADD settings."]
        #[inline(always)]
        pub fn set_nsboot_lock(&mut self, val: super::vals::NsbootrNsbootLock) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "unique boot entry address These bits reflect the UBE address"]
        #[inline(always)]
        pub const fn nsbootadd(&self) -> u32 {
            let val = (self.0 >> 8usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "unique boot entry address These bits reflect the UBE address"]
        #[inline(always)]
        pub fn set_nsbootadd(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
        }
    }
    impl Default for Nsbootr {
        #[inline(always)]
        fn default() -> Nsbootr {
            Nsbootr(0)
        }
    }
    #[doc = "FLASH non-secure clear control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsccr(pub u32);
    impl Nsccr {
        #[doc = "EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "EOP flag clear bit Setting this bit to 1 resets to 0 EOP flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "WRPERR flag clear bit Setting this bit to 1 resets to 0 WRPERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "PGSERR flag clear bit Setting this bit to 1 resets to 0 PGSERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "STRBERR flag clear bit Setting this bit to 1 resets to 0 STRBERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub const fn clr_incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "INCERR flag clear bit Setting this bit to 1 resets to 0 INCERR flag in FLASH_NSSR register."]
        #[inline(always)]
        pub fn set_clr_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Clear the flag corresponding flag in FLASH_NSSR by writing this bit."]
        #[inline(always)]
        pub const fn clr_optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Clear the flag corresponding flag in FLASH_NSSR by writing this bit."]
        #[inline(always)]
        pub fn set_clr_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Nsccr {
        #[inline(always)]
        fn default() -> Nsccr {
            Nsccr(0)
        }
    }
    #[doc = "FLASH Non Secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr(pub u32);
    impl Nscr {
        #[doc = "configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "configuration lock bit This bit locks the FLASH_NSCR register. The correct write sequence to FLASH_NSKEYR register unlocks this bit. If a wrong sequence is executed, or if the unlock sequence to FLASH_NSKEYR is performed twice, this bit remains locked until the next system reset. LOCK can be set by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When LOCK changes from 0 to 1, the other bits of FLASH_NSCR register do not change."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "programming control bit PG can be programmed only when LOCK is cleared to 0. PG allows programming in Bank1 and Bank2."]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
        #[inline(always)]
        pub const fn ser(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "sector erase request Setting SER bit to 1 requests a sector erase. SER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised."]
        #[inline(always)]
        pub fn set_ser(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
        #[inline(always)]
        pub const fn ber(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "erase request Setting BER bit to 1 requests a bank erase operation (user Flash memory only). BER can be programmed only when LOCK is cleared to 0. If MER and SER are also set, a PGSERR is raised. Note: Write protection error is triggered when a bank erase is required and some sectors are protected."]
        #[inline(always)]
        pub fn set_ber(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
        #[inline(always)]
        pub const fn fw(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "write forcing control bit FW forces a write operation even if the write buffer is not full. In this case all bits not written are set to 1 by hardware. FW can be programmed only when LOCK is cleared to 0. The embedded Flash memory resets FW when the corresponding operation has been acknowledged. Note: Using a force-write operation prevents the application from updating later the missing bits with something else than 1, because it is likely that it leads to permanent ECC error. Write forcing is effective only if the write buffer is not empty (in particular, FW does not start several write operations when the force-write operations are performed consecutively). Since there is just one write buffer, FW can force a write in bank1 or bank2."]
        #[inline(always)]
        pub fn set_fw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "erase start control bit STRT bit is used to start a sector erase or a bank erase operation. STRT can be programmed only when LOCK is cleared to 0. STRT is reset at the end of the operation or when an error occurs. It cannot be reseted by software."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
        #[inline(always)]
        pub const fn snb(&self) -> u8 {
            let val = (self.0 >> 6usize) & 0x07;
            val as u8
        }
        #[doc = "sector erase selection number These bits are used to select the target sector for an erase operation (they are unused otherwise). SNB can be programmed only when LOCK is cleared to 0. ..."]
        #[inline(always)]
        pub fn set_snb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 6usize)) | (((val as u32) & 0x07) << 6usize);
        }
        #[doc = "Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "Mass erase request Setting MER bit to 1 requests a mass erase operation (user Flash memory only). MER can be programmed only when LOCK is cleared to 0. If BER or SER are both set, a PGSERR is raised. Error is triggered when a mass erase is required and some sectors are protected."]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end of operation interrupt control bit Setting EOPIE bit to 1 enables the generation of an interrupt at the end of a program or erase operation. EOPIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn wrperrie(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error interrupt enable bit When WRPERRIE bit is set to 1, an interrupt is generated when a protection error occurs during a program operation. WRPERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_wrperrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn pgserrie(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "programming sequence error interrupt enable bit When PGSERRIE bit is set to 1, an interrupt is generated when a sequence error occurs during a program operation. PGSERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_pgserrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn strberrie(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "strobe error interrupt enable bit When STRBERRIE bit is set to 1, an interrupt is generated when a strobe error occurs (the master programs several times the same byte in the write buffer) during a write operation. STRBERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_strberrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub const fn incerrie(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "inconsistency error interrupt enable bit When INCERRIE bit is set to 1, an interrupt is generated when an inconsistency error occurs during a write operation. INCERRIE can be programmed only when LOCK is cleared to 0."]
        #[inline(always)]
        pub fn set_incerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
        #[inline(always)]
        pub const fn optchangeerrie(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error interrupt enable bit OPTCHANGEERRIE bit controls if an interrupt has to be generated when an error occurs during an option byte change. This bit can be programmed only when LOCK bit is cleared to 0."]
        #[inline(always)]
        pub fn set_optchangeerrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
        #[inline(always)]
        pub const fn bksel(&self) -> super::vals::Bksel {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Bksel::from_bits(val as u8)
        }
        #[doc = "Bank selector bit BKSEL can only be programmed when LOCK is cleared to 0. The bit selects physical bank, SWAP_BANK setting is ignored."]
        #[inline(always)]
        pub fn set_bksel(&mut self, val: super::vals::Bksel) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
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
        #[doc = "busy flag BSY flag indicates that a Flash memory is busy by an operation (write, erase, option byte change). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "busy flag BSY flag indicates that a Flash memory is busy by an operation (write, erase, option byte change). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_NSCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub const fn wbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_NSCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by software writing 0 directly. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub fn set_wbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
        #[inline(always)]
        pub const fn dbne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
        #[inline(always)]
        pub fn set_dbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to 1. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_NSCCR register."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_NSCCR register clears WRPERR."]
        #[inline(always)]
        pub fn set_wrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_NSCCR register clears PGSERR."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR."]
        #[inline(always)]
        pub const fn strberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_NSCCR register clears STRBERR."]
        #[inline(always)]
        pub fn set_strberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears INCERR."]
        #[inline(always)]
        pub const fn incerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_NSCCR register clears INCERR."]
        #[inline(always)]
        pub fn set_incerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Option byte change error flag OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. Writing 1 to CLR_OPTCHANGEERR of register FLASH_CCR clears OPTCHANGEERR. Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set."]
        #[inline(always)]
        pub const fn optchangeerr(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte change error flag OPTCHANGEERR flag indicates that an error occurred during an option byte change operation. When OPTCHANGEERR is set to 1, the option byte change operation did not successfully complete. An interrupt is generated when this flag is raised if the OPTCHANGEERRIE bit of FLASH_NSCR register is set to 1. Writing 1 to CLR_OPTCHANGEERR of register FLASH_CCR clears OPTCHANGEERR. Note: The OPTSTRT bit in FLASH_OPTCR cannot be set while OPTCHANGEERR is set."]
        #[inline(always)]
        pub fn set_optchangeerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
    }
    impl Default for Nssr {
        #[inline(always)]
        fn default() -> Nssr {
            Nssr(0)
        }
    }
    #[doc = "FLASH operation status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opsr(pub u32);
    impl Opsr {
        #[doc = "Interrupted operation address."]
        #[inline(always)]
        pub const fn addr_op(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupted operation address."]
        #[inline(always)]
        pub fn set_addr_op(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Interrupted operation bank It indicates which bank was concerned by operation."]
        #[inline(always)]
        pub const fn bk_op(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupted operation bank It indicates which bank was concerned by operation."]
        #[inline(always)]
        pub fn set_bk_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "Operation in system Flash memory interrupted Indicates that reset interrupted an ongoing operation in System Flash."]
        #[inline(always)]
        pub const fn sysf_op(&self) -> bool {
            let val = (self.0 >> 23usize) & 0x01;
            val != 0
        }
        #[doc = "Operation in system Flash memory interrupted Indicates that reset interrupted an ongoing operation in System Flash."]
        #[inline(always)]
        pub fn set_sysf_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
        }
        #[doc = "OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area."]
        #[inline(always)]
        pub const fn otp_op(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "OTP operation interrupted Indicates that reset interrupted an ongoing operation in OTP area."]
        #[inline(always)]
        pub fn set_otp_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Flash memory operation code"]
        #[inline(always)]
        pub const fn code_op(&self) -> super::vals::CodeOp {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::CodeOp::from_bits(val as u8)
        }
        #[doc = "Flash memory operation code"]
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
    #[doc = "FLASH option control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optcr(pub u32);
    impl Optcr {
        #[doc = "FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "FLASH_OPTCR lock option configuration bit The OPTLOCK bit locks the FLASH_OPTCR register as well as all _PRG registers. The correct write sequence to FLASH_OPTKEYR register unlocks this bit. If a wrong sequence is executed, or the unlock sequence to FLASH_OPTKEYR is performed twice, this bit remains locked until next system reset. It is possible to set OPTLOCK by programming it to 1. When set to 1, a new unlock sequence is mandatory to unlock it. When OPTLOCK changes from 0 to 1, the others bits of FLASH_OPTCR register do not change."]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It’s set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It’s reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Option byte start change option configuration bit OPTSTRT triggers an option byte change operation. The user can set OPTSTRT only when the OPTLOCK bit is cleared to 0. It’s set only by Software and cleared when the option byte change is completed or an error occurs (PGSERR or OPTCHANGEERR). It’s reseted at the same time as BSY bit. The user application cannot modify any FLASH_XXX_PRG embedded Flash memory register until the option change operation has been completed. Before setting this bit, the user has to write the required values in the FLASH_XXX_PRG registers. The FLASH_XXX_PRG registers are locked until the option byte change operation has been executed in non-volatile memory."]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR."]
        #[inline(always)]
        pub const fn swap_bank(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank swapping option configuration bit SWAP_BANK controls whether Bank1 and Bank2 are swapped or not. This bit is loaded with the SWAP_BANK bit of FLASH_OPTSR_CUR register only after reset or POR."]
        #[inline(always)]
        pub fn set_swap_bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optcr {
        #[inline(always)]
        fn default() -> Optcr {
            Optcr(0)
        }
    }
    #[doc = "FLASH option status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr(pub u32);
    impl Optsr {
        #[doc = "Brownout level option status bit These bits reflects the power level that generates a system reset."]
        #[inline(always)]
        pub const fn bor_lev(&self) -> super::vals::OptsrBorLev {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::OptsrBorLev::from_bits(val as u8)
        }
        #[doc = "Brownout level option status bit These bits reflects the power level that generates a system reset."]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: super::vals::OptsrBorLev) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "Brownout high enable status bit"]
        #[inline(always)]
        pub const fn borh_en(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Brownout high enable status bit"]
        #[inline(always)]
        pub fn set_borh_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "IWDG control mode option status bit"]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> super::vals::OptsrIwdgSw {
            let val = (self.0 >> 3usize) & 0x01;
            super::vals::OptsrIwdgSw::from_bits(val as u8)
        }
        #[doc = "IWDG control mode option status bit"]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: super::vals::OptsrIwdgSw) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
        }
        #[doc = "WWDG control mode option status bit"]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> super::vals::OptsrWwdgSw {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OptsrWwdgSw::from_bits(val as u8)
        }
        #[doc = "WWDG control mode option status bit"]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: super::vals::OptsrWwdgSw) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "Core domain Shutdown entry reset option status bit"]
        #[inline(always)]
        pub const fn nrst_shdw(&self) -> super::vals::OptsrNrstShdw {
            let val = (self.0 >> 5usize) & 0x01;
            super::vals::OptsrNrstShdw::from_bits(val as u8)
        }
        #[doc = "Core domain Shutdown entry reset option status bit"]
        #[inline(always)]
        pub fn set_nrst_shdw(&mut self, val: super::vals::OptsrNrstShdw) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
        }
        #[doc = "Core domain Stop entry reset option status bit"]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> super::vals::OptsrNrstStop {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::OptsrNrstStop::from_bits(val as u8)
        }
        #[doc = "Core domain Stop entry reset option status bit"]
        #[inline(always)]
        pub fn set_nrst_stop(&mut self, val: super::vals::OptsrNrstStop) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "Core domain Standby entry reset option status bit"]
        #[inline(always)]
        pub const fn nrst_stdby(&self) -> super::vals::OptsrNrstStdby {
            let val = (self.0 >> 7usize) & 0x01;
            super::vals::OptsrNrstStdby::from_bits(val as u8)
        }
        #[doc = "Core domain Standby entry reset option status bit"]
        #[inline(always)]
        pub fn set_nrst_stdby(&mut self, val: super::vals::OptsrNrstStdby) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
        }
        #[doc = "Life state code (based on Hamming 8,4)."]
        #[inline(always)]
        pub const fn product_state(&self) -> super::vals::ProductState {
            let val = (self.0 >> 8usize) & 0xff;
            super::vals::ProductState::from_bits(val as u8)
        }
        #[doc = "Life state code (based on Hamming 8,4)."]
        #[inline(always)]
        pub fn set_product_state(&mut self, val: super::vals::ProductState) {
            self.0 = (self.0 & !(0xff << 8usize)) | (((val.to_bits() as u32) & 0xff) << 8usize);
        }
        #[doc = "High-speed IO at low VDD voltage status bit. This bit can be set only with VDD below 2.5 V."]
        #[inline(always)]
        pub const fn io_vdd_hslv(&self) -> super::vals::OptsrIoVddHslv {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::OptsrIoVddHslv::from_bits(val as u8)
        }
        #[doc = "High-speed IO at low VDD voltage status bit. This bit can be set only with VDD below 2.5 V."]
        #[inline(always)]
        pub fn set_io_vdd_hslv(&mut self, val: super::vals::OptsrIoVddHslv) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "High-speed IO at low VDDIO2 voltage status bit. This bit can be set only with VDDIO2 below 2.5 V."]
        #[inline(always)]
        pub const fn io_vddio2_hslv(&self) -> super::vals::OptsrIoVddioHslv {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::OptsrIoVddioHslv::from_bits(val as u8)
        }
        #[doc = "High-speed IO at low VDDIO2 voltage status bit. This bit can be set only with VDDIO2 below 2.5 V."]
        #[inline(always)]
        pub fn set_io_vddio2_hslv(&mut self, val: super::vals::OptsrIoVddioHslv) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode."]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> super::vals::OptsrIwdgStop {
            let val = (self.0 >> 20usize) & 0x01;
            super::vals::OptsrIwdgStop::from_bits(val as u8)
        }
        #[doc = "IWDG Stop mode freeze option status bit When set the independent watchdog IWDG is in system Stop mode."]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: super::vals::OptsrIwdgStop) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
        }
        #[doc = "IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode."]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> super::vals::OptsrIwdgStdby {
            let val = (self.0 >> 21usize) & 0x01;
            super::vals::OptsrIwdgStdby::from_bits(val as u8)
        }
        #[doc = "IWDG Standby mode freeze option status bit When set the independent watchdog IWDG is frozen in system Standby mode."]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: super::vals::OptsrIwdgStdby) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
        }
        #[doc = "Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset."]
        #[inline(always)]
        pub const fn swap_bank(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Bank swapping option status bit SWAP_BANK reflects whether Bank1 and Bank2 are swapped or not. SWAP_BANK is loaded to SWAP_BANK of FLASH_OPTCR after a reset."]
        #[inline(always)]
        pub fn set_swap_bank(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Optsr {
        #[inline(always)]
        fn default() -> Optsr {
            Optsr(0)
        }
    }
    #[doc = "FLASH option status register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optsr2(pub u32);
    impl Optsr2 {
        #[doc = "SRAM2 erase when system reset"]
        #[inline(always)]
        pub const fn sram2_rst(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 erase when system reset"]
        #[inline(always)]
        pub fn set_sram2_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Backup RAM ECC detection and correction disable"]
        #[inline(always)]
        pub const fn bkpram_ecc(&self) -> super::vals::OptsrBkpramEcc {
            let val = (self.0 >> 4usize) & 0x01;
            super::vals::OptsrBkpramEcc::from_bits(val as u8)
        }
        #[doc = "Backup RAM ECC detection and correction disable"]
        #[inline(always)]
        pub fn set_bkpram_ecc(&mut self, val: super::vals::OptsrBkpramEcc) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
        }
        #[doc = "SRAM2 ECC detection and correction disable"]
        #[inline(always)]
        pub const fn sram2_ecc(&self) -> super::vals::OptsrSramEcc {
            let val = (self.0 >> 6usize) & 0x01;
            super::vals::OptsrSramEcc::from_bits(val as u8)
        }
        #[doc = "SRAM2 ECC detection and correction disable"]
        #[inline(always)]
        pub fn set_sram2_ecc(&mut self, val: super::vals::OptsrSramEcc) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
        }
        #[doc = "SRAM1 erase upon system reset"]
        #[inline(always)]
        pub const fn sram1_rst(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 erase upon system reset"]
        #[inline(always)]
        pub fn set_sram1_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "SRAM1 ECC detection and correction disable"]
        #[inline(always)]
        pub const fn sram1_ecc(&self) -> super::vals::OptsrSramEcc {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::OptsrSramEcc::from_bits(val as u8)
        }
        #[doc = "SRAM1 ECC detection and correction disable"]
        #[inline(always)]
        pub fn set_sram1_ecc(&mut self, val: super::vals::OptsrSramEcc) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
    }
    impl Default for Optsr2 {
        #[inline(always)]
        fn default() -> Optsr2 {
            Optsr2(0)
        }
    }
    #[doc = "FLASH non-secure OTP block lock"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Otpblr(pub u32);
    impl Otpblr {
        #[doc = "OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared."]
        #[inline(always)]
        pub const fn lockbl(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "OTP block lock Block n corresponds to OTP 16-bit word 32 x n to 32 x n + 31. LOCKBL\\[n\\]
= 1 indicates that all OTP 16-bit words in OTP Block n are locked and attempt to program them results in WRPERR. LOCKBL\\[n\\]
= 0 indicates that all OTP 16-bit words in OTP Block n are not locked. When one block is locked, it is not possible to remove the write protection. LOCKBL bits can be set if the corresponding bit in FLASH_OTPBLR_CUR is cleared."]
        #[inline(always)]
        pub fn set_lockbl(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Otpblr {
        #[inline(always)]
        fn default() -> Otpblr {
            Otpblr(0)
        }
    }
    #[doc = "FLASH privilege register for bank 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privbb(pub u32);
    impl Privbb {
        #[doc = "Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
        #[inline(always)]
        pub const fn privbb(&self) -> super::vals::Privbb {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Privbb::from_bits(val as u8)
        }
        #[doc = "Privileged / unprivileged 8 Kbytes Flash Bank1 sector attribute (y = 0 to 7)"]
        #[inline(always)]
        pub fn set_privbb(&mut self, val: super::vals::Privbb) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Privbb {
        #[inline(always)]
        fn default() -> Privbb {
            Privbb(0)
        }
    }
    #[doc = "FLASH privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "privilege attribute for non secure registers"]
        #[inline(always)]
        pub const fn nspriv(&self) -> super::vals::Nspriv {
            let val = (self.0 >> 1usize) & 0x01;
            super::vals::Nspriv::from_bits(val as u8)
        }
        #[doc = "privilege attribute for non secure registers"]
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
    #[doc = "FLASH secure status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secsr(pub u32);
    impl Secsr {
        #[doc = "busy flag BSY flag indicates that a FLASH memory is busy by an operation (write, erase, option byte change, OBK operations, PUF operation). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
        #[inline(always)]
        pub const fn secbsy(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "busy flag BSY flag indicates that a FLASH memory is busy by an operation (write, erase, option byte change, OBK operations, PUF operation). It is set at the beginning of a Flash memory operation and cleared when the operation finishes or an error occurs."]
        #[inline(always)]
        pub fn set_secbsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub const fn secwbne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "write buffer not empty flag WBNE flag is set when the embedded Flash memory is waiting for new data to complete the write buffer. In this state, the write buffer is not empty. WBNE is reset by hardware each time the write buffer is complete or the write buffer is emptied following one of the event below: the application software forces the write operation using FW bit in FLASH_SECCR the embedded Flash memory detects an error that involves data loss This bit cannot be reset by writing 0 directly by software. To reset it, clear the write buffer by performing any of the above listed actions, or send the missing data."]
        #[inline(always)]
        pub fn set_secwbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
        #[inline(always)]
        pub const fn secdbne(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "data buffer not empty flag DBNE flag is set when the embedded Flash memory interface is processing 6-bits ECC data in dedicated buffer. This bit cannot be set to 0 by software. The hardware resets it once the buffer is free."]
        #[inline(always)]
        pub fn set_secdbne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
        #[inline(always)]
        pub const fn seceop(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "end of operation flag EOP flag is set when a operation (program/erase) completes. An interrupt is generated if the EOPIE is set to. It is not necessary to reset EOP before starting a new operation. EOP bit is cleared by writing 1 to CLR_EOP bit in FLASH_SECCCR register."]
        #[inline(always)]
        pub fn set_seceop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
        #[inline(always)]
        pub const fn secwrperr(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "write protection error flag WRPERR flag is raised when a protection error occurs during a program operation. An interrupt is also generated if the WRPERRIE is set to 1. Writing 1 to CLR_WRPERR bit in FLASH_SECCCR register clears WRPERR."]
        #[inline(always)]
        pub fn set_secwrperr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
        #[inline(always)]
        pub const fn secpgserr(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "programming sequence error flag PGSERR flag is raised when a sequence error occurs. An interrupt is generated if the PGSERRIE bit is set to 1. Writing 1 to CLR_PGSERR bit in FLASH_SECCCR register clears PGSERR."]
        #[inline(always)]
        pub fn set_secpgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
        #[inline(always)]
        pub const fn secstrberr(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "strobe error flag STRBERR flag is raised when a strobe error occurs (when the master attempts to write several times the same byte in the write buffer). An interrupt is generated if the STRBERRIE bit is set to 1. Writing 1 to CLR_STRBERR bit in FLASH_SECCCR register clears STRBERR."]
        #[inline(always)]
        pub fn set_secstrberr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
        #[inline(always)]
        pub const fn secincerr(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "inconsistency error flag INCERR flag is raised when a inconsistency error occurs. An interrupt is generated if INCERRIE is set to 1. Writing 1 to CLR_INCERR bit in the FLASH_SECCCR register clears INCERR."]
        #[inline(always)]
        pub fn set_secincerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Secsr {
        #[inline(always)]
        fn default() -> Secsr {
            Secsr(0)
        }
    }
    #[doc = "FLASH write sector protection for Bank2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrp(pub u32);
    impl Wrp {
        #[doc = "Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
        #[inline(always)]
        pub const fn wrpsg(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0xff;
            val as u8
        }
        #[doc = "Bank2 sector protection option status byte Setting WRPSG2 bits to 0 write protects the corresponding sectors in bank 2 (0: write protected; 1: not write protected)"]
        #[inline(always)]
        pub fn set_wrpsg(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
        }
    }
    impl Default for Wrp {
        #[inline(always)]
        fn default() -> Wrp {
            Wrp(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Bksel {
        #[doc = "Bank1 is selected for Bank erase / sector erase / interrupt enable"]
        BANK1 = 0x0,
        #[doc = "Bank1 is selected for Bank erase / sector erase / interrupt enable"]
        BANK2 = 0x01,
    }
    impl Bksel {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Bksel {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Bksel {
        #[inline(always)]
        fn from(val: u8) -> Bksel {
            Bksel::from_bits(val)
        }
    }
    impl From<Bksel> for u8 {
        #[inline(always)]
        fn from(val: Bksel) -> u8 {
            Bksel::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum CodeOp {
        #[doc = "No Flash operation on going during previous reset"]
        B_0X0 = 0x0,
        #[doc = "Single write operation interrupted"]
        B_0X1 = 0x01,
        _RESERVED_2 = 0x02,
        #[doc = "Sector erase operation interrupted"]
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
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct NsbootrNsbootLock(pub u8);
    impl NsbootrNsbootLock {
        #[doc = "The NSBOOTADD and SWAP_BANK are frozen."]
        pub const B_0XB4: Self = Self(0xb4);
        #[doc = "The SWAP_BANK and NSBOOTADD can still be modified following their individual rules."]
        pub const B_0XC3: Self = Self(0xc3);
    }
    impl NsbootrNsbootLock {
        pub const fn from_bits(val: u8) -> NsbootrNsbootLock {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl From<u8> for NsbootrNsbootLock {
        #[inline(always)]
        fn from(val: u8) -> NsbootrNsbootLock {
            NsbootrNsbootLock::from_bits(val)
        }
    }
    impl From<NsbootrNsbootLock> for u8 {
        #[inline(always)]
        fn from(val: NsbootrNsbootLock) -> u8 {
            NsbootrNsbootLock::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Nspriv {
        #[doc = "access to non secure registers is always granted"]
        B_0X0 = 0x0,
        #[doc = "access to non secure registers is denied in case of non privileged access."]
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
    pub enum OptsrBkpramEcc {
        #[doc = "BKPRAM ECC check enabled"]
        B_0X0 = 0x0,
        #[doc = "BKPRAM ECC check disabled"]
        B_0X1 = 0x01,
    }
    impl OptsrBkpramEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrBkpramEcc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrBkpramEcc {
        #[inline(always)]
        fn from(val: u8) -> OptsrBkpramEcc {
            OptsrBkpramEcc::from_bits(val)
        }
    }
    impl From<OptsrBkpramEcc> for u8 {
        #[inline(always)]
        fn from(val: OptsrBkpramEcc) -> u8 {
            OptsrBkpramEcc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrBorLev {
        #[doc = "BOR OFF, POR/PDR reset threshold level is applied"]
        B_0X0 = 0x0,
        #[doc = "BOR Level 1, the threshold level is low (around 2.1 V)"]
        B_0X1 = 0x01,
        #[doc = "BOR Level 2, the threshold level is medium (around 2.4 V)"]
        B_0X2 = 0x02,
        #[doc = "BOR Level 3, the threshold level is high (around 2.7 V)"]
        B_0X3 = 0x03,
    }
    impl OptsrBorLev {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrBorLev {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrBorLev {
        #[inline(always)]
        fn from(val: u8) -> OptsrBorLev {
            OptsrBorLev::from_bits(val)
        }
    }
    impl From<OptsrBorLev> for u8 {
        #[inline(always)]
        fn from(val: OptsrBorLev) -> u8 {
            OptsrBorLev::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrIoVddHslv {
        #[doc = "High-speed IO at low VDD voltage feature disabled (VDD can exceed 2.5 V)"]
        B_0X0 = 0x0,
        #[doc = "High-speed IO at low VDD voltage feature enabled (VDD remains below 2.5 V)"]
        B_0X1 = 0x01,
    }
    impl OptsrIoVddHslv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIoVddHslv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIoVddHslv {
        #[inline(always)]
        fn from(val: u8) -> OptsrIoVddHslv {
            OptsrIoVddHslv::from_bits(val)
        }
    }
    impl From<OptsrIoVddHslv> for u8 {
        #[inline(always)]
        fn from(val: OptsrIoVddHslv) -> u8 {
            OptsrIoVddHslv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrIoVddioHslv {
        #[doc = "High-speed IO at low VDDIO2 voltage feature disabled (VDDIO2 can exceed 2.5 V)"]
        B_0X0 = 0x0,
        #[doc = "High-speed IO at low VDDIO2 voltage feature enabled (VDDIO2 remains below 2.5 V)"]
        B_0X1 = 0x01,
    }
    impl OptsrIoVddioHslv {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIoVddioHslv {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIoVddioHslv {
        #[inline(always)]
        fn from(val: u8) -> OptsrIoVddioHslv {
            OptsrIoVddioHslv::from_bits(val)
        }
    }
    impl From<OptsrIoVddioHslv> for u8 {
        #[inline(always)]
        fn from(val: OptsrIoVddioHslv) -> u8 {
            OptsrIoVddioHslv::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrIwdgStdby {
        #[doc = "Independent watchdog frozen in Standby mode"]
        B_0X0 = 0x0,
        #[doc = "Independent watchdog keep running in Standby mode."]
        B_0X1 = 0x01,
    }
    impl OptsrIwdgStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgStdby {
            OptsrIwdgStdby::from_bits(val)
        }
    }
    impl From<OptsrIwdgStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgStdby) -> u8 {
            OptsrIwdgStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrIwdgStop {
        #[doc = "Independent watchdog frozen in system Stop mode"]
        B_0X0 = 0x0,
        #[doc = "Independent watchdog keep running in system Stop mode."]
        B_0X1 = 0x01,
    }
    impl OptsrIwdgStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgStop {
            OptsrIwdgStop::from_bits(val)
        }
    }
    impl From<OptsrIwdgStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgStop) -> u8 {
            OptsrIwdgStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrIwdgSw {
        #[doc = "IWDG watchdog is controlled by hardware"]
        B_0X0 = 0x0,
        #[doc = "IWDG watchdog is controlled by software"]
        B_0X1 = 0x01,
    }
    impl OptsrIwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrIwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrIwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrIwdgSw {
            OptsrIwdgSw::from_bits(val)
        }
    }
    impl From<OptsrIwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrIwdgSw) -> u8 {
            OptsrIwdgSw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrNrstShdw {
        #[doc = "a reset is generated when entering Shutdown mode on core domain"]
        B_0X0 = 0x0,
        #[doc = "no reset generated when entering Shutdown mode on core domain."]
        B_0X1 = 0x01,
    }
    impl OptsrNrstShdw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrNrstShdw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrNrstShdw {
        #[inline(always)]
        fn from(val: u8) -> OptsrNrstShdw {
            OptsrNrstShdw::from_bits(val)
        }
    }
    impl From<OptsrNrstShdw> for u8 {
        #[inline(always)]
        fn from(val: OptsrNrstShdw) -> u8 {
            OptsrNrstShdw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrNrstStdby {
        #[doc = "a reset is generated when entering Standby mode on core domain"]
        B_0X0 = 0x0,
        #[doc = "no reset generated when entering Standby mode on core domain."]
        B_0X1 = 0x01,
    }
    impl OptsrNrstStdby {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrNrstStdby {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrNrstStdby {
        #[inline(always)]
        fn from(val: u8) -> OptsrNrstStdby {
            OptsrNrstStdby::from_bits(val)
        }
    }
    impl From<OptsrNrstStdby> for u8 {
        #[inline(always)]
        fn from(val: OptsrNrstStdby) -> u8 {
            OptsrNrstStdby::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrNrstStop {
        #[doc = "a reset is generated when entering Stop mode on core domain"]
        B_0X0 = 0x0,
        #[doc = "no reset generated when entering Stop mode on core domain."]
        B_0X1 = 0x01,
    }
    impl OptsrNrstStop {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrNrstStop {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrNrstStop {
        #[inline(always)]
        fn from(val: u8) -> OptsrNrstStop {
            OptsrNrstStop::from_bits(val)
        }
    }
    impl From<OptsrNrstStop> for u8 {
        #[inline(always)]
        fn from(val: OptsrNrstStop) -> u8 {
            OptsrNrstStop::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrSramEcc {
        #[doc = "SRAM2 ECC check enabled"]
        B_0X0 = 0x0,
        #[doc = "SRAM2 ECC check disabled"]
        B_0X1 = 0x01,
    }
    impl OptsrSramEcc {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrSramEcc {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrSramEcc {
        #[inline(always)]
        fn from(val: u8) -> OptsrSramEcc {
            OptsrSramEcc::from_bits(val)
        }
    }
    impl From<OptsrSramEcc> for u8 {
        #[inline(always)]
        fn from(val: OptsrSramEcc) -> u8 {
            OptsrSramEcc::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum OptsrWwdgSw {
        #[doc = "WWDG watchdog is controlled by hardware"]
        B_0X0 = 0x0,
        #[doc = "WWDG watchdog is controlled by software"]
        B_0X1 = 0x01,
    }
    impl OptsrWwdgSw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> OptsrWwdgSw {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for OptsrWwdgSw {
        #[inline(always)]
        fn from(val: u8) -> OptsrWwdgSw {
            OptsrWwdgSw::from_bits(val)
        }
    }
    impl From<OptsrWwdgSw> for u8 {
        #[inline(always)]
        fn from(val: OptsrWwdgSw) -> u8 {
            OptsrWwdgSw::to_bits(val)
        }
    }
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Privbb(pub u8);
    impl Privbb {
        #[doc = "sectors y in bank 1 is non privileged"]
        pub const B_0X0: Self = Self(0x0);
        #[doc = "sector y in bank 1 is privileged"]
        pub const B_0X1: Self = Self(0x01);
    }
    impl Privbb {
        pub const fn from_bits(val: u8) -> Privbb {
            Self(val & 0xff)
        }
        pub const fn to_bits(self) -> u8 {
            self.0
        }
    }
    impl From<u8> for Privbb {
        #[inline(always)]
        fn from(val: u8) -> Privbb {
            Privbb::from_bits(val)
        }
    }
    impl From<Privbb> for u8 {
        #[inline(always)]
        fn from(val: Privbb) -> u8 {
            Privbb::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum ProductState {
        _RESERVED_0 = 0x0,
        _RESERVED_1 = 0x01,
        _RESERVED_2 = 0x02,
        _RESERVED_3 = 0x03,
        _RESERVED_4 = 0x04,
        _RESERVED_5 = 0x05,
        _RESERVED_6 = 0x06,
        _RESERVED_7 = 0x07,
        _RESERVED_8 = 0x08,
        _RESERVED_9 = 0x09,
        _RESERVED_a = 0x0a,
        _RESERVED_b = 0x0b,
        _RESERVED_c = 0x0c,
        _RESERVED_d = 0x0d,
        _RESERVED_e = 0x0e,
        _RESERVED_f = 0x0f,
        _RESERVED_10 = 0x10,
        _RESERVED_11 = 0x11,
        _RESERVED_12 = 0x12,
        _RESERVED_13 = 0x13,
        _RESERVED_14 = 0x14,
        _RESERVED_15 = 0x15,
        _RESERVED_16 = 0x16,
        #[doc = "Provisioning"]
        PROVISIONING = 0x17,
        _RESERVED_18 = 0x18,
        _RESERVED_19 = 0x19,
        _RESERVED_1a = 0x1a,
        _RESERVED_1b = 0x1b,
        _RESERVED_1c = 0x1c,
        _RESERVED_1d = 0x1d,
        _RESERVED_1e = 0x1e,
        _RESERVED_1f = 0x1f,
        _RESERVED_20 = 0x20,
        _RESERVED_21 = 0x21,
        _RESERVED_22 = 0x22,
        _RESERVED_23 = 0x23,
        _RESERVED_24 = 0x24,
        _RESERVED_25 = 0x25,
        _RESERVED_26 = 0x26,
        _RESERVED_27 = 0x27,
        _RESERVED_28 = 0x28,
        _RESERVED_29 = 0x29,
        _RESERVED_2a = 0x2a,
        _RESERVED_2b = 0x2b,
        _RESERVED_2c = 0x2c,
        _RESERVED_2d = 0x2d,
        #[doc = "iROT-Provisioned"]
        IROT_PROVISIONED = 0x2e,
        _RESERVED_2f = 0x2f,
        _RESERVED_30 = 0x30,
        _RESERVED_31 = 0x31,
        _RESERVED_32 = 0x32,
        _RESERVED_33 = 0x33,
        _RESERVED_34 = 0x34,
        _RESERVED_35 = 0x35,
        _RESERVED_36 = 0x36,
        _RESERVED_37 = 0x37,
        _RESERVED_38 = 0x38,
        _RESERVED_39 = 0x39,
        _RESERVED_3a = 0x3a,
        _RESERVED_3b = 0x3b,
        _RESERVED_3c = 0x3c,
        _RESERVED_3d = 0x3d,
        _RESERVED_3e = 0x3e,
        _RESERVED_3f = 0x3f,
        _RESERVED_40 = 0x40,
        _RESERVED_41 = 0x41,
        _RESERVED_42 = 0x42,
        _RESERVED_43 = 0x43,
        _RESERVED_44 = 0x44,
        _RESERVED_45 = 0x45,
        _RESERVED_46 = 0x46,
        _RESERVED_47 = 0x47,
        _RESERVED_48 = 0x48,
        _RESERVED_49 = 0x49,
        _RESERVED_4a = 0x4a,
        _RESERVED_4b = 0x4b,
        _RESERVED_4c = 0x4c,
        _RESERVED_4d = 0x4d,
        _RESERVED_4e = 0x4e,
        _RESERVED_4f = 0x4f,
        _RESERVED_50 = 0x50,
        _RESERVED_51 = 0x51,
        _RESERVED_52 = 0x52,
        _RESERVED_53 = 0x53,
        _RESERVED_54 = 0x54,
        _RESERVED_55 = 0x55,
        _RESERVED_56 = 0x56,
        _RESERVED_57 = 0x57,
        _RESERVED_58 = 0x58,
        _RESERVED_59 = 0x59,
        _RESERVED_5a = 0x5a,
        _RESERVED_5b = 0x5b,
        #[doc = "Locked"]
        LOCKED = 0x5c,
        _RESERVED_5d = 0x5d,
        _RESERVED_5e = 0x5e,
        _RESERVED_5f = 0x5f,
        _RESERVED_60 = 0x60,
        _RESERVED_61 = 0x61,
        _RESERVED_62 = 0x62,
        _RESERVED_63 = 0x63,
        _RESERVED_64 = 0x64,
        _RESERVED_65 = 0x65,
        _RESERVED_66 = 0x66,
        _RESERVED_67 = 0x67,
        _RESERVED_68 = 0x68,
        _RESERVED_69 = 0x69,
        _RESERVED_6a = 0x6a,
        _RESERVED_6b = 0x6b,
        _RESERVED_6c = 0x6c,
        _RESERVED_6d = 0x6d,
        _RESERVED_6e = 0x6e,
        _RESERVED_6f = 0x6f,
        _RESERVED_70 = 0x70,
        _RESERVED_71 = 0x71,
        #[doc = "Closed"]
        CLOSED = 0x72,
        _RESERVED_73 = 0x73,
        _RESERVED_74 = 0x74,
        _RESERVED_75 = 0x75,
        _RESERVED_76 = 0x76,
        _RESERVED_77 = 0x77,
        _RESERVED_78 = 0x78,
        _RESERVED_79 = 0x79,
        _RESERVED_7a = 0x7a,
        _RESERVED_7b = 0x7b,
        _RESERVED_7c = 0x7c,
        _RESERVED_7d = 0x7d,
        _RESERVED_7e = 0x7e,
        _RESERVED_7f = 0x7f,
        _RESERVED_80 = 0x80,
        _RESERVED_81 = 0x81,
        _RESERVED_82 = 0x82,
        _RESERVED_83 = 0x83,
        _RESERVED_84 = 0x84,
        _RESERVED_85 = 0x85,
        _RESERVED_86 = 0x86,
        _RESERVED_87 = 0x87,
        _RESERVED_88 = 0x88,
        _RESERVED_89 = 0x89,
        _RESERVED_8a = 0x8a,
        _RESERVED_8b = 0x8b,
        _RESERVED_8c = 0x8c,
        _RESERVED_8d = 0x8d,
        _RESERVED_8e = 0x8e,
        _RESERVED_8f = 0x8f,
        _RESERVED_90 = 0x90,
        _RESERVED_91 = 0x91,
        _RESERVED_92 = 0x92,
        _RESERVED_93 = 0x93,
        _RESERVED_94 = 0x94,
        _RESERVED_95 = 0x95,
        _RESERVED_96 = 0x96,
        _RESERVED_97 = 0x97,
        _RESERVED_98 = 0x98,
        _RESERVED_99 = 0x99,
        #[doc = "Regression"]
        REGRESSION = 0x9a,
        _RESERVED_9b = 0x9b,
        _RESERVED_9c = 0x9c,
        _RESERVED_9d = 0x9d,
        _RESERVED_9e = 0x9e,
        _RESERVED_9f = 0x9f,
        _RESERVED_a0 = 0xa0,
        _RESERVED_a1 = 0xa1,
        _RESERVED_a2 = 0xa2,
        _RESERVED_a3 = 0xa3,
        _RESERVED_a4 = 0xa4,
        _RESERVED_a5 = 0xa5,
        _RESERVED_a6 = 0xa6,
        _RESERVED_a7 = 0xa7,
        _RESERVED_a8 = 0xa8,
        _RESERVED_a9 = 0xa9,
        _RESERVED_aa = 0xaa,
        _RESERVED_ab = 0xab,
        _RESERVED_ac = 0xac,
        _RESERVED_ad = 0xad,
        _RESERVED_ae = 0xae,
        _RESERVED_af = 0xaf,
        _RESERVED_b0 = 0xb0,
        _RESERVED_b1 = 0xb1,
        _RESERVED_b2 = 0xb2,
        _RESERVED_b3 = 0xb3,
        _RESERVED_b4 = 0xb4,
        _RESERVED_b5 = 0xb5,
        _RESERVED_b6 = 0xb6,
        _RESERVED_b7 = 0xb7,
        _RESERVED_b8 = 0xb8,
        _RESERVED_b9 = 0xb9,
        _RESERVED_ba = 0xba,
        _RESERVED_bb = 0xbb,
        _RESERVED_bc = 0xbc,
        _RESERVED_bd = 0xbd,
        _RESERVED_be = 0xbe,
        _RESERVED_bf = 0xbf,
        _RESERVED_c0 = 0xc0,
        _RESERVED_c1 = 0xc1,
        _RESERVED_c2 = 0xc2,
        _RESERVED_c3 = 0xc3,
        _RESERVED_c4 = 0xc4,
        _RESERVED_c5 = 0xc5,
        _RESERVED_c6 = 0xc6,
        _RESERVED_c7 = 0xc7,
        _RESERVED_c8 = 0xc8,
        _RESERVED_c9 = 0xc9,
        _RESERVED_ca = 0xca,
        _RESERVED_cb = 0xcb,
        _RESERVED_cc = 0xcc,
        _RESERVED_cd = 0xcd,
        _RESERVED_ce = 0xce,
        _RESERVED_cf = 0xcf,
        _RESERVED_d0 = 0xd0,
        _RESERVED_d1 = 0xd1,
        _RESERVED_d2 = 0xd2,
        _RESERVED_d3 = 0xd3,
        _RESERVED_d4 = 0xd4,
        _RESERVED_d5 = 0xd5,
        _RESERVED_d6 = 0xd6,
        _RESERVED_d7 = 0xd7,
        _RESERVED_d8 = 0xd8,
        _RESERVED_d9 = 0xd9,
        _RESERVED_da = 0xda,
        _RESERVED_db = 0xdb,
        _RESERVED_dc = 0xdc,
        _RESERVED_dd = 0xdd,
        _RESERVED_de = 0xde,
        _RESERVED_df = 0xdf,
        _RESERVED_e0 = 0xe0,
        _RESERVED_e1 = 0xe1,
        _RESERVED_e2 = 0xe2,
        _RESERVED_e3 = 0xe3,
        _RESERVED_e4 = 0xe4,
        _RESERVED_e5 = 0xe5,
        _RESERVED_e6 = 0xe6,
        _RESERVED_e7 = 0xe7,
        _RESERVED_e8 = 0xe8,
        _RESERVED_e9 = 0xe9,
        _RESERVED_ea = 0xea,
        _RESERVED_eb = 0xeb,
        _RESERVED_ec = 0xec,
        #[doc = "Open"]
        OPEN = 0xed,
        _RESERVED_ee = 0xee,
        _RESERVED_ef = 0xef,
        _RESERVED_f0 = 0xf0,
        _RESERVED_f1 = 0xf1,
        _RESERVED_f2 = 0xf2,
        _RESERVED_f3 = 0xf3,
        _RESERVED_f4 = 0xf4,
        _RESERVED_f5 = 0xf5,
        _RESERVED_f6 = 0xf6,
        _RESERVED_f7 = 0xf7,
        _RESERVED_f8 = 0xf8,
        _RESERVED_f9 = 0xf9,
        _RESERVED_fa = 0xfa,
        _RESERVED_fb = 0xfb,
        _RESERVED_fc = 0xfc,
        _RESERVED_fd = 0xfd,
        _RESERVED_fe = 0xfe,
        _RESERVED_ff = 0xff,
    }
    impl ProductState {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> ProductState {
            unsafe { core::mem::transmute(val & 0xff) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for ProductState {
        #[inline(always)]
        fn from(val: u8) -> ProductState {
            ProductState::from_bits(val)
        }
    }
    impl From<ProductState> for u8 {
        #[inline(always)]
        fn from(val: ProductState) -> u8 {
            ProductState::to_bits(val)
        }
    }
}
