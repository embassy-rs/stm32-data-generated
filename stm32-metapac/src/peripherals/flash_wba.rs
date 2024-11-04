#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "Embedded memory"]
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
    #[doc = "access control register"]
    #[inline(always)]
    pub const fn acr(self) -> crate::common::Reg<regs::Acr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "key register"]
    #[inline(always)]
    pub const fn nskeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "secure key register"]
    #[inline(always)]
    pub const fn seckeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "option key register"]
    #[inline(always)]
    pub const fn optkeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "power-down key register"]
    #[inline(always)]
    pub const fn pdkeyr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "status register"]
    #[inline(always)]
    pub const fn nssr(self) -> crate::common::Reg<regs::Nssr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "secure status register"]
    #[inline(always)]
    pub const fn secsr(self) -> crate::common::Reg<regs::Secsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "control register"]
    #[inline(always)]
    pub const fn nscr1(self) -> crate::common::Reg<regs::Nscr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "secure control register"]
    #[inline(always)]
    pub const fn seccr1(self) -> crate::common::Reg<regs::Seccr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "ECC register"]
    #[inline(always)]
    pub const fn eccr(self) -> crate::common::Reg<regs::Eccr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "operation status register"]
    #[inline(always)]
    pub const fn opsr(self) -> crate::common::Reg<regs::Opsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "control 2 register"]
    #[inline(always)]
    pub const fn nscr2(self) -> crate::common::Reg<regs::Nscr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "secure control 2 register"]
    #[inline(always)]
    pub const fn seccr2(self) -> crate::common::Reg<regs::Seccr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "option register"]
    #[inline(always)]
    pub const fn optr(self) -> crate::common::Reg<regs::Optr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "boot address 0 register"]
    #[inline(always)]
    pub const fn nsbootadd0r(self) -> crate::common::Reg<regs::Nsbootadd0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "boot address 1 register"]
    #[inline(always)]
    pub const fn nsbootadd1r(self) -> crate::common::Reg<regs::Nsbootadd1r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "secure boot address 0 register"]
    #[inline(always)]
    pub const fn secbootadd0r(self) -> crate::common::Reg<regs::Secbootadd0r, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "secure watermark register 1"]
    #[inline(always)]
    pub const fn secwmr1(self) -> crate::common::Reg<regs::Secwmr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "secure watermark register 2"]
    #[inline(always)]
    pub const fn secwmr2(self) -> crate::common::Reg<regs::Secwmr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "WRP area A address register"]
    #[inline(always)]
    pub const fn wrpar(self) -> crate::common::Reg<regs::Wrpar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "WRP area B address register"]
    #[inline(always)]
    pub const fn wrpbr(self) -> crate::common::Reg<regs::Wrpbr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "OEM1 key register 1"]
    #[inline(always)]
    pub const fn oem1keyr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "OEM1 key register 2"]
    #[inline(always)]
    pub const fn oem1keyr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "OEM2 key register 1"]
    #[inline(always)]
    pub const fn oem2keyr1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "OEM2 key register 2"]
    #[inline(always)]
    pub const fn oem2keyr2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "secure block based register 1"]
    #[inline(always)]
    pub const fn secbbr(self, n: usize) -> crate::common::Reg<regs::Bbr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize + n * 4usize) as _) }
    }
    #[doc = "secure HDP control register"]
    #[inline(always)]
    pub const fn sechdpcr(self) -> crate::common::Reg<regs::Sechdpcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "privilege configuration register"]
    #[inline(always)]
    pub const fn prifcfgr(self) -> crate::common::Reg<regs::Prifcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
    #[doc = "privilege block based register 1"]
    #[inline(always)]
    pub const fn privbbr(self, n: usize) -> crate::common::Reg<regs::Bbr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize + n * 4usize) as _) }
    }
}
pub mod regs {
    #[doc = "access control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Acr(pub u32);
    impl Acr {
        #[doc = "Latency These bits represent the ratio between the AHB hclk1 clock period and the memory access time. Access to the bit can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. ... Note: Before entering Stop 1 mode software must set wait state latency to at least 1."]
        #[inline(always)]
        pub const fn latency(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x0f;
            val as u8
        }
        #[doc = "Latency These bits represent the ratio between the AHB hclk1 clock period and the memory access time. Access to the bit can be secured by RCC SYSCLKSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. ... Note: Before entering Stop 1 mode software must set wait state latency to at least 1."]
        #[inline(always)]
        pub fn set_latency(&mut self, val: u8) {
            self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
        }
        #[doc = "Prefetch enable This bit enables the prefetch buffer in the embedded memory. This bit can be protected against unprivileged access by NSPRIV."]
        #[inline(always)]
        pub const fn prften(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Prefetch enable This bit enables the prefetch buffer in the embedded memory. This bit can be protected against unprivileged access by NSPRIV."]
        #[inline(always)]
        pub fn set_prften(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Low-power read mode This bit puts the memory in low-power read mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. This bit can’t be written when a program or erase operation is busy (BSY = 1) or when the write buffer is not empty (WDW = 1). Changing this bit while a program or erase operation is busy (BSY = 1) is rejected."]
        #[inline(always)]
        pub const fn lpm(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Low-power read mode This bit puts the memory in low-power read mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. This bit can’t be written when a program or erase operation is busy (BSY = 1) or when the write buffer is not empty (WDW = 1). Changing this bit while a program or erase operation is busy (BSY = 1) is rejected."]
        #[inline(always)]
        pub fn set_lpm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "power-down mode request This bit requests to enter power-down mode. When enters power-down mode, this bit is cleared by hardware and the PDKEYR is locked. This bit is write-protected with PDKEYR. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub const fn pdreq(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "power-down mode request This bit requests to enter power-down mode. When enters power-down mode, this bit is cleared by hardware and the PDKEYR is locked. This bit is write-protected with PDKEYR. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV."]
        #[inline(always)]
        pub fn set_pdreq(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "memory power-down mode during Sleep mode This bit determines whether the memory is in power-down mode or Idle mode when the device is in Sleep mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. The must not be put in power-down while a program or an erase operation is ongoing."]
        #[inline(always)]
        pub const fn sleep_pd(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "memory power-down mode during Sleep mode This bit determines whether the memory is in power-down mode or Idle mode when the device is in Sleep mode. Access to the bit can be secured by PWR LPMSEC. When secure, a non-secure read/write access is RAZ/WI. It does not generate an illegal access interrupt. This bit can be protected against unprivileged access when secure with SPRIV or when non-secure with NSPRIV. The must not be put in power-down while a program or an erase operation is ongoing."]
        #[inline(always)]
        pub fn set_sleep_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for Acr {
        #[inline(always)]
        fn default() -> Acr {
            Acr(0)
        }
    }
    #[doc = "block based register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bbr(pub u32);
    impl Bbr {
        #[inline(always)]
        pub const fn block(&self, n: usize) -> bool {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[inline(always)]
        pub fn set_block(&mut self, n: usize, val: bool) {
            assert!(n < 32usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Bbr {
        #[inline(always)]
        fn default() -> Bbr {
            Bbr(0)
        }
    }
    #[doc = "ECC register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccr(pub u32);
    impl Eccr {
        #[doc = "ECC fail address This field indicates which address is concerned by the ECC error correction or by the double ECC error detection. The address is given relative to base address, from offset 0x0�0000 to 0xF�FFF0. Note that bit 19 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub const fn addr_ecc(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "ECC fail address This field indicates which address is concerned by the ECC error correction or by the double ECC error detection. The address is given relative to base address, from offset 0x0�0000 to 0xF�FFF0. Note that bit 19 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub fn set_addr_ecc(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "System memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system memory."]
        #[inline(always)]
        pub const fn sysf_ecc(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "System memory ECC fail This bit indicates that the ECC error correction or double ECC error detection is located in the system memory."]
        #[inline(always)]
        pub fn set_sysf_ecc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the ECCR register is set."]
        #[inline(always)]
        pub const fn eccie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "ECC correction interrupt enable This bit enables the interrupt generation when the ECCC bit in the ECCR register is set."]
        #[inline(always)]
        pub fn set_eccie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
    #[doc = "boot address 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootadd0r(pub u32);
    impl Nsbootadd0r {
        #[doc = "Non-secure boot base address 0 This address is only used when TZEN = 0. The non-secure boot memory address can be programmed to any address in the valid address range (see Table 28: Boot space versus RDP protection) with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state. Examples: NSBOOTADD0\\[24:0\\]
= 0x0100000: Boot from memory (0x0800 0000) NSBOOTADD0\\[24:0\\]
= 0x017F100: Boot from system memory bootloader (0x0BF8 8000) NSBOOTADD0\\[24:0\\]
= 0x0400200: Boot from SRAM2 on S-Bus (0x2001 0000)"]
        #[inline(always)]
        pub const fn nsbootadd0(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Non-secure boot base address 0 This address is only used when TZEN = 0. The non-secure boot memory address can be programmed to any address in the valid address range (see Table 28: Boot space versus RDP protection) with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state. Examples: NSBOOTADD0\\[24:0\\]
= 0x0100000: Boot from memory (0x0800 0000) NSBOOTADD0\\[24:0\\]
= 0x017F100: Boot from system memory bootloader (0x0BF8 8000) NSBOOTADD0\\[24:0\\]
= 0x0400200: Boot from SRAM2 on S-Bus (0x2001 0000)"]
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
    #[doc = "boot address 1 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nsbootadd1r(pub u32);
    impl Nsbootadd1r {
        #[doc = "Non-secure boot address 1 This address is only used when TZEN = 0. The non-secure boot memory address can be programmed to any address in the valid address range (see Table 28: Boot space versus RDP protection) with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F100: Boot from system memory bootloader (0x0BF8 8000) NSBOOTADD1\\[24:0\\]
= 0x0400200: Boot from SRAM2 (0x2001 0000)"]
        #[inline(always)]
        pub const fn nsbootadd1(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Non-secure boot address 1 This address is only used when TZEN = 0. The non-secure boot memory address can be programmed to any address in the valid address range (see Table 28: Boot space versus RDP protection) with a granularity of 128 bytes. These bits correspond to address \\[31:7\\]. The NSBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state. Examples: NSBOOTADD1\\[24:0\\]
= 0x0100000: Boot from memory (0x0800 0000) NSBOOTADD1\\[24:0\\]
= 0x017F100: Boot from system memory bootloader (0x0BF8 8000) NSBOOTADD1\\[24:0\\]
= 0x0400200: Boot from SRAM2 (0x2001 0000)"]
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
    #[doc = "control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr1(pub u32);
    impl Nscr1 {
        #[doc = "Non-secure programming"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-secure page erase"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Non-secure mass erase This bit triggers the non-secure mass erase (all user pages) when set."]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure mass erase This bit triggers the non-secure mass erase (all user pages) when set."]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Non-secure page number selection These bits select the page to erase. ... Note that bit 9 is reserved on STM32WBA5xEx devices."]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x7f;
            val as u8
        }
        #[doc = "Non-secure page number selection These bits select the page to erase. ... Note that bit 9 is reserved on STM32WBA5xEx devices."]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
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
        #[doc = "Non-secure operation start This bit triggers a non-secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR bit in NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in NSSR."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure operation start This bit triggers a non-secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR bit in NSSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in NSSR."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Options modification start This bit triggers an option bytes erase and program operation when set. This bit is write-protected with OPTLOCK.. This bit is set only by software, and is cleared when the BSY bit is cleared in NSSR."]
        #[inline(always)]
        pub const fn optstrt(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Options modification start This bit triggers an option bytes erase and program operation when set. This bit is write-protected with OPTLOCK.. This bit is set only by software, and is cleared when the BSY bit is cleared in NSSR."]
        #[inline(always)]
        pub fn set_optstrt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the NSSR is set to 1."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure end of operation interrupt enable This bit enables the interrupt generation when the EOP bit in the NSSR is set to 1."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the NSSR is set to 1."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in the NSSR is set to 1."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. This bit is write-protected with OPTLOCK. Note: The LSE oscillator must be disabled, LSEON = 0 and LSERDY = 0, before starting OBL_LAUNCH."]
        #[inline(always)]
        pub const fn obl_launch(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Force the option byte loading When set to 1, this bit forces the option byte reloading. This bit is cleared only when the option byte loading is complete. This bit is write-protected with OPTLOCK. Note: The LSE oscillator must be disabled, LSEON = 0 and LSERDY = 0, before starting OBL_LAUNCH."]
        #[inline(always)]
        pub fn set_obl_launch(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "Option lock This bit is set only. When set, the NSCR1.OPTSRT and OBL_LAUNCH bits concerning user options write access is locked. This bit is cleared by hardware after detecting the unlock sequence in OPTKEYR. The NSCR1.LOCK bit must be cleared before doing the OPTKEYR unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
        #[inline(always)]
        pub const fn optlock(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Option lock This bit is set only. When set, the NSCR1.OPTSRT and OBL_LAUNCH bits concerning user options write access is locked. This bit is cleared by hardware after detecting the unlock sequence in OPTKEYR. The NSCR1.LOCK bit must be cleared before doing the OPTKEYR unlock sequence. In case of an unsuccessful unlock operation, this bit remains set until the next reset."]
        #[inline(always)]
        pub fn set_optlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Non-secure lock This bit is set only. When set, the NSCR1 register write access is locked. This bit is cleared by hardware after detecting the unlock sequence in NSKEYR. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure lock This bit is set only. When set, the NSCR1 register write access is locked. This bit is cleared by hardware after detecting the unlock sequence in NSKEYR. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Nscr1 {
        #[inline(always)]
        fn default() -> Nscr1 {
            Nscr1(0)
        }
    }
    #[doc = "control 2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nscr2(pub u32);
    impl Nscr2 {
        #[doc = "Program suspend request"]
        #[inline(always)]
        pub const fn ps(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Program suspend request"]
        #[inline(always)]
        pub fn set_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Erase suspend request"]
        #[inline(always)]
        pub const fn es(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Erase suspend request"]
        #[inline(always)]
        pub fn set_es(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Nscr2 {
        #[inline(always)]
        fn default() -> Nscr2 {
            Nscr2(0)
        }
    }
    #[doc = "status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nssr(pub u32);
    impl Nssr {
        #[doc = "Non-secure end of operation This bit is set by hardware when one or more memory non-secure operation (program/erase) has been completed successfully. This bit is set only if the non-secure end of operation interrupts are enabled (EOPIE = 1 in NSCR1). This bit is cleared by writing�1."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure end of operation This bit is set by hardware when one or more memory non-secure operation (program/erase) has been completed successfully. This bit is set only if the non-secure end of operation interrupts are enabled (EOPIE = 1 in NSCR1). This bit is cleared by writing�1."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Non-secure operation error This bit is set by hardware when a memory non-secure operation (program/erase) completes unsuccessfully. This bit is set only if non-secure error interrupts are enabled (NSERRIE = 1). This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure operation error This bit is set by hardware when a memory non-secure operation (program/erase) completes unsuccessfully. This bit is set only if non-secure error interrupts are enabled (NSERRIE = 1). This bit is cleared by writing 1."]
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
        #[doc = "Non-secure write protection error This bit is set by hardware when a non-secure address to be erased/programmed belongs to a write-protected part (by WRP or HDP) of the memory. This bit is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure write protection error This bit is set by hardware when a non-secure address to be erased/programmed belongs to a write-protected part (by WRP or HDP) of the memory. This bit is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
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
        #[doc = "Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Option write error This bit is set by hardware when the options bytes are written with an invalid configuration or when modifying options in RDP level 2.. It is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn optwerr(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Option write error This bit is set by hardware when the options bytes are written with an invalid configuration or when modifying options in RDP level 2.. It is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_optwerr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "Non-secure busy This indicates that a memory secure or non-secure operation is in progress. This bit is set at the beginning of a operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure busy This indicates that a memory secure or non-secure operation is in progress. This bit is set at the beginning of a operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Non-secure wait data to write This bit indicates that the memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the memory."]
        #[inline(always)]
        pub const fn wdw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Non-secure wait data to write This bit indicates that the memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the memory."]
        #[inline(always)]
        pub fn set_wdw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "OEM1 key RDP lock This bit indicates that the OEM1 key read during the OBL is not virgin. When set, the OEM1 key RDP lock mechanism is active."]
        #[inline(always)]
        pub const fn oem1lock(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "OEM1 key RDP lock This bit indicates that the OEM1 key read during the OBL is not virgin. When set, the OEM1 key RDP lock mechanism is active."]
        #[inline(always)]
        pub fn set_oem1lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "OEM2 key RDP lock This bit indicates that the OEM2 key read during the OBL is not virgin. When set, the OEM2 key RDP lock mechanism is active."]
        #[inline(always)]
        pub const fn oem2lock(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "OEM2 key RDP lock This bit indicates that the OEM2 key read during the OBL is not virgin. When set, the OEM2 key RDP lock mechanism is active."]
        #[inline(always)]
        pub fn set_oem2lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "in power-down mode This bit indicates that the memory is in power-down state. It is reset when is in normal mode or being awaken."]
        #[inline(always)]
        pub const fn pd(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "in power-down mode This bit indicates that the memory is in power-down state. It is reset when is in normal mode or being awaken."]
        #[inline(always)]
        pub fn set_pd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
    }
    impl Default for Nssr {
        #[inline(always)]
        fn default() -> Nssr {
            Nssr(0)
        }
    }
    #[doc = "operation status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opsr(pub u32);
    impl Opsr {
        #[doc = "Interrupted operation address This field indicates which address in the memory was accessed when reset occurred. The address is given relative to the base address, from offset 0x0�0000 to 0xF�FFF0. Note that bit 19 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub const fn addr_op(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x000f_ffff;
            val as u32
        }
        #[doc = "Interrupted operation address This field indicates which address in the memory was accessed when reset occurred. The address is given relative to the base address, from offset 0x0�0000 to 0xF�FFF0. Note that bit 19 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub fn set_addr_op(&mut self, val: u32) {
            self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
        }
        #[doc = "Operation in system memory interrupted This bit indicates that the reset occurred during an operation in the system memory."]
        #[inline(always)]
        pub const fn sysf_op(&self) -> bool {
            let val = (self.0 >> 22usize) & 0x01;
            val != 0
        }
        #[doc = "Operation in system memory interrupted This bit indicates that the reset occurred during an operation in the system memory."]
        #[inline(always)]
        pub fn set_sysf_op(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
        }
        #[doc = "memory operation code This field indicates which memory operation has been interrupted by a system reset:"]
        #[inline(always)]
        pub const fn code_op(&self) -> super::vals::CodeOp {
            let val = (self.0 >> 29usize) & 0x07;
            super::vals::CodeOp::from_bits(val as u8)
        }
        #[doc = "memory operation code This field indicates which memory operation has been interrupted by a system reset:"]
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
    #[doc = "option register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Optr(pub u32);
    impl Optr {
        #[doc = "Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to Section�7.6.2: Readout protection (RDP) for more details."]
        #[inline(always)]
        pub const fn rdp(&self) -> super::vals::Rdp {
            let val = (self.0 >> 0usize) & 0xff;
            super::vals::Rdp::from_bits(val as u8)
        }
        #[doc = "Readout protection level Others: Level 1 (memories readout protection active) Note: Refer to Section�7.6.2: Readout protection (RDP) for more details."]
        #[inline(always)]
        pub fn set_rdp(&mut self, val: super::vals::Rdp) {
            self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
        }
        #[doc = "BOR reset level These bits contain the V<sub>DD</sub> supply level threshold that activates/releases the reset."]
        #[inline(always)]
        pub const fn bor_lev(&self) -> super::vals::BorLev {
            let val = (self.0 >> 8usize) & 0x07;
            super::vals::BorLev::from_bits(val as u8)
        }
        #[doc = "BOR reset level These bits contain the V<sub>DD</sub> supply level threshold that activates/releases the reset."]
        #[inline(always)]
        pub fn set_bor_lev(&mut self, val: super::vals::BorLev) {
            self.0 = (self.0 & !(0x07 << 8usize)) | (((val.to_bits() as u32) & 0x07) << 8usize);
        }
        #[doc = "Reset generation in Stop mode"]
        #[inline(always)]
        pub const fn nrst_stop(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generation in Stop mode"]
        #[inline(always)]
        pub fn set_nrst_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "Reset generation in Standby mode"]
        #[inline(always)]
        pub const fn nrst_stdby(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "Reset generation in Standby mode"]
        #[inline(always)]
        pub fn set_nrst_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "SRAM1 erase upon system reset"]
        #[inline(always)]
        pub const fn sram1_rst(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM1 erase upon system reset"]
        #[inline(always)]
        pub fn set_sram1_rst(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "Independent watchdog enable selection"]
        #[inline(always)]
        pub const fn iwdg_sw(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog enable selection"]
        #[inline(always)]
        pub fn set_iwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub const fn iwdg_stop(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Stop mode"]
        #[inline(always)]
        pub fn set_iwdg_stop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub const fn iwdg_stdby(&self) -> bool {
            let val = (self.0 >> 18usize) & 0x01;
            val != 0
        }
        #[doc = "Independent watchdog counter freeze in Standby mode"]
        #[inline(always)]
        pub fn set_iwdg_stdby(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub const fn wwdg_sw(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "Window watchdog selection"]
        #[inline(always)]
        pub fn set_wwdg_sw(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "SRAM2 parity check enable"]
        #[inline(always)]
        pub const fn sram2_pe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "SRAM2 parity check enable"]
        #[inline(always)]
        pub fn set_sram2_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
        pub const fn nswboot0(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Software BOOT0"]
        #[inline(always)]
        pub fn set_nswboot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "NBOOT0 option bit"]
        #[inline(always)]
        pub const fn nboot0(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "NBOOT0 option bit"]
        #[inline(always)]
        pub fn set_nboot0(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
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
    #[doc = "privilege configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Prifcfgr(pub u32);
    impl Prifcfgr {
        #[doc = "Privileged protection for secure registers This bit is secure write protected. It can only be written by a secure privileged access when TrustZone is enabled (TZEN�=�1)."]
        #[inline(always)]
        pub const fn spriv(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged protection for secure registers This bit is secure write protected. It can only be written by a secure privileged access when TrustZone is enabled (TZEN�=�1)."]
        #[inline(always)]
        pub fn set_spriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Privileged protection for non-secure registers"]
        #[inline(always)]
        pub const fn nspriv(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Privileged protection for non-secure registers"]
        #[inline(always)]
        pub fn set_nspriv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Prifcfgr {
        #[inline(always)]
        fn default() -> Prifcfgr {
            Prifcfgr(0)
        }
    }
    #[doc = "secure boot address 0 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secbootadd0r(pub u32);
    impl Secbootadd0r {
        #[doc = "Boot lock This lock is only used when TZEN = 0. When set, the boot is always forced to base address value programmed in SECBOOTADD0\\[24:0\\]
option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP regression level 1 to level 0."]
        #[inline(always)]
        pub const fn boot_lock(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Boot lock This lock is only used when TZEN = 0. When set, the boot is always forced to base address value programmed in SECBOOTADD0\\[24:0\\]
option bytes whatever the boot selection option. When set, this bit can only be cleared by an RDP regression level 1 to level 0."]
        #[inline(always)]
        pub fn set_boot_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure boot base address 0 This address is only used when TZEN = 1. The secure boot memory address can be programmed to any address in the valid address range (see Table�28: Boot space versus RDP protection) with a granularity of 128 bytes. This bits correspond to address \\[31:7\\]
The SECBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state. Examples: SECBOOTADD0\\[24:0\\]
= 0x018 0000: Boot from secure user memory (0x0C00 0000) SECBOOTADD0\\[24:0\\]
= 0x01F F000: Boot from RSS system memory (0x0FF8 0000) SECBOOTADD0\\[24:0\\]
= 0x060 0000: Boot from secure SRAM1 on S-Bus (0x3000 0000)"]
        #[inline(always)]
        pub const fn secbootadd0(&self) -> u32 {
            let val = (self.0 >> 7usize) & 0x01ff_ffff;
            val as u32
        }
        #[doc = "Secure boot base address 0 This address is only used when TZEN = 1. The secure boot memory address can be programmed to any address in the valid address range (see Table�28: Boot space versus RDP protection) with a granularity of 128 bytes. This bits correspond to address \\[31:7\\]
The SECBOOTADD0 option bytes are selected following the BOOT0 pin or NSWBOOT0 state. Examples: SECBOOTADD0\\[24:0\\]
= 0x018 0000: Boot from secure user memory (0x0C00 0000) SECBOOTADD0\\[24:0\\]
= 0x01F F000: Boot from RSS system memory (0x0FF8 0000) SECBOOTADD0\\[24:0\\]
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
    #[doc = "secure control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccr1(pub u32);
    impl Seccr1 {
        #[doc = "Secure programming"]
        #[inline(always)]
        pub const fn pg(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming"]
        #[inline(always)]
        pub fn set_pg(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure page erase"]
        #[inline(always)]
        pub const fn per(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure page erase"]
        #[inline(always)]
        pub fn set_per(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Secure mass erase This bit triggers the secure mass erase (all user pages) when set."]
        #[inline(always)]
        pub const fn mer(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Secure mass erase This bit triggers the secure mass erase (all user pages) when set."]
        #[inline(always)]
        pub fn set_mer(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Secure page number selection These bits select the page to erase: ... Note that bit 9 is reserved on STM32WBA5xEx devices."]
        #[inline(always)]
        pub const fn pnb(&self) -> u8 {
            let val = (self.0 >> 3usize) & 0x7f;
            val as u8
        }
        #[doc = "Secure page number selection These bits select the page to erase: ... Note that bit 9 is reserved on STM32WBA5xEx devices."]
        #[inline(always)]
        pub fn set_pnb(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 3usize)) | (((val as u32) & 0x7f) << 3usize);
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
        #[doc = "Secure start This bit triggers a secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR in the SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in SECSR."]
        #[inline(always)]
        pub const fn strt(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure start This bit triggers a secure erase operation when set. If MER and PER bits are reset and the STRT bit is set, the PGSERR in the SECSR is set (this condition is forbidden). This bit is set only by software and is cleared when the BSY bit is cleared in SECSR."]
        #[inline(always)]
        pub fn set_strt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in SECSR is set to 1."]
        #[inline(always)]
        pub const fn eopie(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "Secure End of operation interrupt enable This bit enables the interrupt generation when the EOP bit in SECSR is set to 1."]
        #[inline(always)]
        pub fn set_eopie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "Secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in SECSR is set to 1."]
        #[inline(always)]
        pub const fn errie(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Secure error interrupt enable This bit enables the interrupt generation when the OPERR bit in SECSR is set to 1."]
        #[inline(always)]
        pub fn set_errie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "memory security state invert This bit inverts the memory security state."]
        #[inline(always)]
        pub const fn inv(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "memory security state invert This bit inverts the memory security state."]
        #[inline(always)]
        pub fn set_inv(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Secure lock This bit is set only. When set, the SECCR1 register is locked. It is cleared by hardware after detecting the unlock sequence in SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub const fn lock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Secure lock This bit is set only. When set, the SECCR1 register is locked. It is cleared by hardware after detecting the unlock sequence in SECKEYR register. In case of an unsuccessful unlock operation, this bit remains set until the next system reset."]
        #[inline(always)]
        pub fn set_lock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Seccr1 {
        #[inline(always)]
        fn default() -> Seccr1 {
            Seccr1(0)
        }
    }
    #[doc = "secure control 2 register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccr2(pub u32);
    impl Seccr2 {
        #[doc = "Program suspend request"]
        #[inline(always)]
        pub const fn ps(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Program suspend request"]
        #[inline(always)]
        pub fn set_ps(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Erase suspend request"]
        #[inline(always)]
        pub const fn es(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Erase suspend request"]
        #[inline(always)]
        pub fn set_es(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for Seccr2 {
        #[inline(always)]
        fn default() -> Seccr2 {
            Seccr2(0)
        }
    }
    #[doc = "secure HDP control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sechdpcr(pub u32);
    impl Sechdpcr {
        #[doc = "Secure HDP area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub const fn hdp_accdis(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure HDP area access disable When set, this bit is only cleared by a system reset."]
        #[inline(always)]
        pub fn set_hdp_accdis(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for Sechdpcr {
        #[inline(always)]
        fn default() -> Sechdpcr {
            Sechdpcr(0)
        }
    }
    #[doc = "secure status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secsr(pub u32);
    impl Secsr {
        #[doc = "Secure end of operation This bit is set by hardware when one or more memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in SECCR1). This bit is cleared by writing�1."]
        #[inline(always)]
        pub const fn eop(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Secure end of operation This bit is set by hardware when one or more memory secure operation (program/erase) has been completed successfully. This bit is set only if the secure end of operation interrupts are enabled (EOPIE = 1 in SECCR1). This bit is cleared by writing�1."]
        #[inline(always)]
        pub fn set_eop(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Secure operation error This bit is set by hardware when a memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1."]
        #[inline(always)]
        pub const fn operr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Secure operation error This bit is set by hardware when a memory secure operation (program/erase) completes unsuccessfully. This bit is set only if secure error interrupts are enabled (SECERRIE = 1). This bit is cleared by writing 1."]
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
        #[doc = "Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP or HDP) of the memory. This bit is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn wrperr(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Secure write protection error This bit is set by hardware when an secure address to be erased/programmed belongs to a write-protected part (by WRP or HDP) of the memory. This bit is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
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
        #[doc = "Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
        #[inline(always)]
        pub const fn pgserr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Secure programming sequence error This bit is set by hardware when programming sequence is not correct. It is cleared by writing 1. Refer to Section�7.3.10: memory errors flags for full conditions of error flag setting."]
        #[inline(always)]
        pub fn set_pgserr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Secure busy This bit indicates that a memory secure or non-secure operation is in progress. This is set on the beginning of a operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub const fn bsy(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Secure busy This bit indicates that a memory secure or non-secure operation is in progress. This is set on the beginning of a operation and reset when the operation finishes or when an error occurs."]
        #[inline(always)]
        pub fn set_bsy(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Secure wait data to write This bit indicates that the memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the memory."]
        #[inline(always)]
        pub const fn wdw(&self) -> bool {
            let val = (self.0 >> 17usize) & 0x01;
            val != 0
        }
        #[doc = "Secure wait data to write This bit indicates that the memory write buffer has been written by a secure or non-secure operation. It is set when the first data is stored in the buffer and cleared when the write is performed in the memory."]
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
    #[doc = "secure watermark register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwmr1(pub u32);
    impl Secwmr1 {
        #[doc = "Start page of secure area This field contains the first page of the secure area."]
        #[inline(always)]
        pub const fn secwm_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "Start page of secure area This field contains the first page of the secure area."]
        #[inline(always)]
        pub fn set_secwm_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "End page of secure area This field contains the last page of the secure area."]
        #[inline(always)]
        pub const fn secwm_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of secure area This field contains the last page of the secure area."]
        #[inline(always)]
        pub fn set_secwm_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
    }
    impl Default for Secwmr1 {
        #[inline(always)]
        fn default() -> Secwmr1 {
            Secwmr1(0)
        }
    }
    #[doc = "secure watermark register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Secwmr2(pub u32);
    impl Secwmr2 {
        #[doc = "End page of secure hide protection area This field contains the last page of the secure HDP area."]
        #[inline(always)]
        pub const fn hdp_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "End page of secure hide protection area This field contains the last page of the secure HDP area."]
        #[inline(always)]
        pub fn set_hdp_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "Secure Hide protection area enable"]
        #[inline(always)]
        pub const fn hdpen(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Secure Hide protection area enable"]
        #[inline(always)]
        pub fn set_hdpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Secwmr2 {
        #[inline(always)]
        fn default() -> Secwmr2 {
            Secwmr2(0)
        }
    }
    #[doc = "WRP area A address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpar(pub u32);
    impl Wrpar {
        #[doc = "WPR area A start page This field contains the first page of the WPR area A. Note that bit 6 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub const fn wrpa_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "WPR area A start page This field contains the first page of the WPR area A. Note that bit 6 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub fn set_wrpa_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "WPR area A end page This field contains the last page of the WPR area A. Note that bit 22 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub const fn wrpa_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "WPR area A end page This field contains the last page of the WPR area A. Note that bit 22 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub fn set_wrpa_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "WPR area A unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "WPR area A unlock"]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrpar {
        #[inline(always)]
        fn default() -> Wrpar {
            Wrpar(0)
        }
    }
    #[doc = "WRP area B address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Wrpbr(pub u32);
    impl Wrpbr {
        #[doc = "WRP area B start page This field contains the first page of the WRP area B. Note that bit 6 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub const fn wrpb_pstrt(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP area B start page This field contains the first page of the WRP area B. Note that bit 6 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub fn set_wrpb_pstrt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "WRP area B end page This field contains the last page of the WRP area B. Note that bit 22 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub const fn wrpb_pend(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x7f;
            val as u8
        }
        #[doc = "WRP area B end page This field contains the last page of the WRP area B. Note that bit 22 is reserved on STM32WBAxEx devices."]
        #[inline(always)]
        pub fn set_wrpb_pend(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 16usize)) | (((val as u32) & 0x7f) << 16usize);
        }
        #[doc = "WPR area B unlock"]
        #[inline(always)]
        pub const fn unlock(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "WPR area B unlock"]
        #[inline(always)]
        pub fn set_unlock(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for Wrpbr {
        #[inline(always)]
        fn default() -> Wrpbr {
            Wrpbr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum BorLev {
        #[doc = "BOR level 0 (reset level threshold around 1.7�V)"]
        LEVEL0 = 0x0,
        #[doc = "BOR level 1 (reset level threshold around 2.0�V)"]
        LEVEL1 = 0x01,
        #[doc = "BOR level 2 (reset level threshold around 2.2�V)"]
        LEVEL2 = 0x02,
        #[doc = "BOR level 3 (reset level threshold around 2.5�V)"]
        LEVEL3 = 0x03,
        #[doc = "BOR level 4 (reset level threshold around 2.8�V)"]
        LEVEL4 = 0x04,
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
        #[doc = "No operation interrupted by previous reset"]
        B_0X0 = 0x0,
        #[doc = "Single write operation interrupted"]
        B_0X1 = 0x01,
        #[doc = "Burst write operation interrupted"]
        B_0X2 = 0x02,
        #[doc = "Page erase operation interrupted"]
        B_0X3 = 0x03,
        #[doc = "Reserved"]
        B_0X4 = 0x04,
        #[doc = "Mass erase operation interrupted"]
        B_0X5 = 0x05,
        #[doc = "Option change operation interrupted"]
        B_0X6 = 0x06,
        #[doc = "Reserved"]
        B_0X7 = 0x07,
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
}
