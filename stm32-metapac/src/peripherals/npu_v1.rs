#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "ATON AXI bus interface unit."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busif {
    ptr: *mut u8,
}
unsafe impl Send for Busif {}
unsafe impl Sync for Busif {}
impl Busif {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bus interface control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::BusifCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Bus interface error status register."]
    #[inline(always)]
    pub const fn err(self) -> crate::common::Reg<regs::BusifErr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
#[doc = "ATON internal clock controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkctrl {
    ptr: *mut u8,
}
unsafe impl Send for Clkctrl {}
unsafe impl Sync for Clkctrl {}
impl Clkctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock controller control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::ClkctrlCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Group-A infrastructure clock gates (lower 32)."]
    #[inline(always)]
    pub const fn agates0(self) -> crate::common::Reg<regs::Agates, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Group-A infrastructure clock gates (upper 32)."]
    #[inline(always)]
    pub const fn agates1(self) -> crate::common::Reg<regs::Agates, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Group-B accelerator unit clock gates."]
    #[inline(always)]
    pub const fn bgates(self) -> crate::common::Reg<regs::Bgates, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
}
#[doc = "ATON Epoch Controller execution engine."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Epochctrl {
    ptr: *mut u8,
}
unsafe impl Send for Epochctrl {}
unsafe impl Sync for Epochctrl {}
impl Epochctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Epoch controller control and status register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::EpochctrlCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Blob start address register (8-byte aligned)."]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::EpochctrlAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Epoch controller interrupt status/acknowledge register."]
    #[inline(always)]
    pub const fn irq(self) -> crate::common::Reg<regs::EpochctrlIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Current/last executed blob section label (debug)."]
    #[inline(always)]
    pub const fn label(self) -> crate::common::Reg<regs::EpochctrlLabel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x1cusize) as _) }
    }
    #[doc = "Opcode/byte counter (debug)."]
    #[inline(always)]
    pub const fn bc(self) -> crate::common::Reg<regs::EpochctrlBc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x20usize) as _) }
    }
}
#[doc = "ATON interrupt controller."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intctrl {
    ptr: *mut u8,
}
unsafe impl Send for Intctrl {}
unsafe impl Sync for Intctrl {}
impl Intctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt controller control register."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::IntctrlCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Latched interrupt status register."]
    #[inline(always)]
    pub const fn intreg(self) -> crate::common::Reg<regs::Intstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "Software interrupt set register."]
    #[inline(always)]
    pub const fn intset(self) -> crate::common::Reg<regs::Intstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "Interrupt clear register (write 1 to clear)."]
    #[inline(always)]
    pub const fn intclr(self) -> crate::common::Reg<regs::Intstatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "OR-mask registers for interrupt lines 0..3."]
    #[inline(always)]
    pub const fn intormsk(self, n: usize) -> crate::common::Reg<regs::Intstatus, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize + n * 4usize) as _) }
    }
    #[doc = "AND-mask registers for interrupt lines 0..3."]
    #[inline(always)]
    pub const fn intandmsk(self, n: usize) -> crate::common::Reg<regs::Intstatus, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x24usize + n * 4usize) as _) }
    }
}
#[doc = "ST Neural-ART accelerator (ATON) host interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Npu {
    ptr: *mut u8,
}
unsafe impl Send for Npu {}
unsafe impl Sync for Npu {}
impl Npu {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock controller."]
    #[inline(always)]
    pub const fn clkctrl(self) -> Clkctrl {
        unsafe { Clkctrl::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "Interrupt controller."]
    #[inline(always)]
    pub const fn intctrl(self) -> Intctrl {
        unsafe { Intctrl::from_ptr(self.ptr.wrapping_add(0x1000usize) as _) }
    }
    #[doc = "Bus interface units."]
    #[inline(always)]
    pub const fn busif(self, n: usize) -> Busif {
        assert!(n < 2usize);
        unsafe { Busif::from_ptr(self.ptr.wrapping_add(0x2000usize + n * 4096usize) as _) }
    }
    #[doc = "Streaming engines."]
    #[inline(always)]
    pub const fn streng(self, n: usize) -> Streng {
        assert!(n < 10usize);
        unsafe { Streng::from_ptr(self.ptr.wrapping_add(0x5000usize + n * 4096usize) as _) }
    }
    #[doc = "Epoch controller."]
    #[inline(always)]
    pub const fn epochctrl(self) -> Epochctrl {
        unsafe { Epochctrl::from_ptr(self.ptr.wrapping_add(0x0001_e000usize) as _) }
    }
}
#[doc = "ATON streaming engine host interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Streng {
    ptr: *mut u8,
}
unsafe impl Send for Streng {}
unsafe impl Sync for Streng {}
impl Streng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Streaming engine interrupt status/acknowledge register."]
    #[inline(always)]
    pub const fn irq(self) -> crate::common::Reg<regs::StrengIrq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x3cusize) as _) }
    }
}
pub mod regs {
    #[doc = "Group-A clock gating register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Agates(pub u32);
    impl Agates {
        #[doc = "Clock enable mask."]
        #[must_use]
        #[inline(always)]
        pub const fn gates(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Clock enable mask."]
        #[inline(always)]
        pub const fn set_gates(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Agates {
        #[inline(always)]
        fn default() -> Agates {
            Agates(0)
        }
    }
    impl core::fmt::Debug for Agates {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Agates").field("gates", &self.gates()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Agates {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Agates {{ gates: {=u32:?} }}", self.gates())
        }
    }
    #[doc = "Group-B unit clock gating register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bgates(pub u32);
    impl Bgates {
        #[doc = "Unit clock enable mask."]
        #[must_use]
        #[inline(always)]
        pub const fn gates(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Unit clock enable mask."]
        #[inline(always)]
        pub const fn set_gates(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for Bgates {
        #[inline(always)]
        fn default() -> Bgates {
            Bgates(0)
        }
    }
    impl core::fmt::Debug for Bgates {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Bgates").field("gates", &self.gates()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Bgates {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Bgates {{ gates: {=u32:?} }}", self.gates())
        }
    }
    #[doc = "Bus interface control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BusifCtrl(pub u32);
    impl BusifCtrl {
        #[doc = "Bus interface enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Bus interface enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
    }
    impl Default for BusifCtrl {
        #[inline(always)]
        fn default() -> BusifCtrl {
            BusifCtrl(0)
        }
    }
    impl core::fmt::Debug for BusifCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BusifCtrl").field("en", &self.en()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BusifCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BusifCtrl {{ en: {=bool:?} }}", self.en())
        }
    }
    #[doc = "Bus interface error status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BusifErr(pub u32);
    impl BusifErr {
        #[doc = "Bus error flags."]
        #[must_use]
        #[inline(always)]
        pub const fn err(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Bus error flags."]
        #[inline(always)]
        pub const fn set_err(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for BusifErr {
        #[inline(always)]
        fn default() -> BusifErr {
            BusifErr(0)
        }
    }
    impl core::fmt::Debug for BusifErr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("BusifErr").field("err", &self.err()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for BusifErr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "BusifErr {{ err: {=u32:?} }}", self.err())
        }
    }
    #[doc = "Clock controller control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClkctrlCtrl(pub u32);
    impl ClkctrlCtrl {
        #[doc = "Clock controller enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Clock controller enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clock controller synchronous clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clock controller synchronous clear."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
    }
    impl Default for ClkctrlCtrl {
        #[inline(always)]
        fn default() -> ClkctrlCtrl {
            ClkctrlCtrl(0)
        }
    }
    impl core::fmt::Debug for ClkctrlCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("ClkctrlCtrl")
                .field("en", &self.en())
                .field("clr", &self.clr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for ClkctrlCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "ClkctrlCtrl {{ en: {=bool:?}, clr: {=bool:?} }}",
                self.en(),
                self.clr()
            )
        }
    }
    #[doc = "Blob base address register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpochctrlAddr(pub u32);
    impl EpochctrlAddr {
        #[doc = "32-bit physical address of blob start."]
        #[must_use]
        #[inline(always)]
        pub const fn addr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "32-bit physical address of blob start."]
        #[inline(always)]
        pub const fn set_addr(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EpochctrlAddr {
        #[inline(always)]
        fn default() -> EpochctrlAddr {
            EpochctrlAddr(0)
        }
    }
    impl core::fmt::Debug for EpochctrlAddr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpochctrlAddr").field("addr", &self.addr()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EpochctrlAddr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EpochctrlAddr {{ addr: {=u32:?} }}", self.addr())
        }
    }
    #[doc = "Byte/opcode counter register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpochctrlBc(pub u32);
    impl EpochctrlBc {
        #[doc = "Executed instruction counter."]
        #[must_use]
        #[inline(always)]
        pub const fn count(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Executed instruction counter."]
        #[inline(always)]
        pub const fn set_count(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EpochctrlBc {
        #[inline(always)]
        fn default() -> EpochctrlBc {
            EpochctrlBc(0)
        }
    }
    impl core::fmt::Debug for EpochctrlBc {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpochctrlBc").field("count", &self.count()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EpochctrlBc {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EpochctrlBc {{ count: {=u32:?} }}", self.count())
        }
    }
    #[doc = "Epoch controller control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpochctrlCtrl(pub u32);
    impl EpochctrlCtrl {
        #[doc = "Start epoch controller execution."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Start epoch controller execution."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Clear controller state."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Clear controller state."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Single-step mode enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sm(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Single-step mode enable."]
        #[inline(always)]
        pub const fn set_sm(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Configuration clear."]
        #[must_use]
        #[inline(always)]
        pub const fn confclr(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration clear."]
        #[inline(always)]
        pub const fn set_confclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Controller execution status."]
        #[must_use]
        #[inline(always)]
        pub const fn running(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Controller execution status."]
        #[inline(always)]
        pub const fn set_running(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for EpochctrlCtrl {
        #[inline(always)]
        fn default() -> EpochctrlCtrl {
            EpochctrlCtrl(0)
        }
    }
    impl core::fmt::Debug for EpochctrlCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpochctrlCtrl")
                .field("en", &self.en())
                .field("clr", &self.clr())
                .field("sm", &self.sm())
                .field("confclr", &self.confclr())
                .field("running", &self.running())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EpochctrlCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "EpochctrlCtrl {{ en: {=bool:?}, clr: {=bool:?}, sm: {=bool:?}, confclr: {=bool:?}, running: {=bool:?} }}",
                self.en(),
                self.clr(),
                self.sm(),
                self.confclr(),
                self.running()
            )
        }
    }
    #[doc = "Epoch controller IRQ acknowledge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpochctrlIrq(pub u32);
    impl EpochctrlIrq {
        #[doc = "IRQ flag (write back read value to ack)."]
        #[must_use]
        #[inline(always)]
        pub const fn irq(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "IRQ flag (write back read value to ack)."]
        #[inline(always)]
        pub const fn set_irq(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EpochctrlIrq {
        #[inline(always)]
        fn default() -> EpochctrlIrq {
            EpochctrlIrq(0)
        }
    }
    impl core::fmt::Debug for EpochctrlIrq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpochctrlIrq").field("irq", &self.irq()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EpochctrlIrq {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EpochctrlIrq {{ irq: {=u32:?} }}", self.irq())
        }
    }
    #[doc = "Blob label debug register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EpochctrlLabel(pub u32);
    impl EpochctrlLabel {
        #[doc = "Current blob section label."]
        #[must_use]
        #[inline(always)]
        pub const fn label(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Current blob section label."]
        #[inline(always)]
        pub const fn set_label(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for EpochctrlLabel {
        #[inline(always)]
        fn default() -> EpochctrlLabel {
            EpochctrlLabel(0)
        }
    }
    impl core::fmt::Debug for EpochctrlLabel {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("EpochctrlLabel").field("label", &self.label()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for EpochctrlLabel {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "EpochctrlLabel {{ label: {=u32:?} }}", self.label())
        }
    }
    #[doc = "Interrupt controller control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct IntctrlCtrl(pub u32);
    impl IntctrlCtrl {
        #[doc = "Interrupt controller enable."]
        #[must_use]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Interrupt controller enable."]
        #[inline(always)]
        pub const fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Synchronous clear."]
        #[must_use]
        #[inline(always)]
        pub const fn clr(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronous clear."]
        #[inline(always)]
        pub const fn set_clr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Configuration clear."]
        #[must_use]
        #[inline(always)]
        pub const fn confclr(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Configuration clear."]
        #[inline(always)]
        pub const fn set_confclr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for IntctrlCtrl {
        #[inline(always)]
        fn default() -> IntctrlCtrl {
            IntctrlCtrl(0)
        }
    }
    impl core::fmt::Debug for IntctrlCtrl {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("IntctrlCtrl")
                .field("en", &self.en())
                .field("clr", &self.clr())
                .field("confclr", &self.confclr())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for IntctrlCtrl {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "IntctrlCtrl {{ en: {=bool:?}, clr: {=bool:?}, confclr: {=bool:?} }}",
                self.en(),
                self.clr(),
                self.confclr()
            )
        }
    }
    #[doc = "Interrupt status and mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Intstatus(pub u32);
    impl Intstatus {
        #[doc = "Streaming engine completion events (engines 0..9)."]
        #[must_use]
        #[inline(always)]
        pub const fn streng_evt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x03ff;
            val as u16
        }
        #[doc = "Streaming engine completion events (engines 0..9)."]
        #[inline(always)]
        pub const fn set_streng_evt(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
        }
        #[doc = "Streaming engine errors (engines 0..9)."]
        #[must_use]
        #[inline(always)]
        pub const fn streng_err(&self) -> u16 {
            let val = (self.0 >> 10usize) & 0x03ff;
            val as u16
        }
        #[doc = "Streaming engine errors (engines 0..9)."]
        #[inline(always)]
        pub const fn set_streng_err(&mut self, val: u16) {
            self.0 = (self.0 & !(0x03ff << 10usize)) | (((val as u32) & 0x03ff) << 10usize);
        }
        #[doc = "Bus interface error flags (BUSIF 0..1)."]
        #[must_use]
        #[inline(always)]
        pub const fn busif_err(&self) -> u8 {
            let val = (self.0 >> 25usize) & 0x03;
            val as u8
        }
        #[doc = "Bus interface error flags (BUSIF 0..1)."]
        #[inline(always)]
        pub const fn set_busif_err(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 25usize)) | (((val as u32) & 0x03) << 25usize);
        }
        #[doc = "Epoch controller end of epoch / IRQ event."]
        #[must_use]
        #[inline(always)]
        pub const fn ectrl_evt(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "Epoch controller end of epoch / IRQ event."]
        #[inline(always)]
        pub const fn set_ectrl_evt(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Epoch controller missing-acknowledge error."]
        #[must_use]
        #[inline(always)]
        pub const fn ectrl_noack(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Epoch controller missing-acknowledge error."]
        #[inline(always)]
        pub const fn set_ectrl_noack(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Epoch controller execution error."]
        #[must_use]
        #[inline(always)]
        pub const fn ectrl_err(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Epoch controller execution error."]
        #[inline(always)]
        pub const fn set_ectrl_err(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
    }
    impl Default for Intstatus {
        #[inline(always)]
        fn default() -> Intstatus {
            Intstatus(0)
        }
    }
    impl core::fmt::Debug for Intstatus {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Intstatus")
                .field("streng_evt", &self.streng_evt())
                .field("streng_err", &self.streng_err())
                .field("busif_err", &self.busif_err())
                .field("ectrl_evt", &self.ectrl_evt())
                .field("ectrl_noack", &self.ectrl_noack())
                .field("ectrl_err", &self.ectrl_err())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Intstatus {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Intstatus {{ streng_evt: {=u16:?}, streng_err: {=u16:?}, busif_err: {=u8:?}, ectrl_evt: {=bool:?}, ectrl_noack: {=bool:?}, ectrl_err: {=bool:?} }}",
                self.streng_evt(),
                self.streng_err(),
                self.busif_err(),
                self.ectrl_evt(),
                self.ectrl_noack(),
                self.ectrl_err()
            )
        }
    }
    #[doc = "Streaming engine IRQ status/acknowledge register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StrengIrq(pub u32);
    impl StrengIrq {
        #[doc = "Interrupt pending flag (write 1 to acknowledge)."]
        #[must_use]
        #[inline(always)]
        pub const fn irq(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0xffff_ffff;
            val as u32
        }
        #[doc = "Interrupt pending flag (write 1 to acknowledge)."]
        #[inline(always)]
        pub const fn set_irq(&mut self, val: u32) {
            self.0 = (self.0 & !(0xffff_ffff << 0usize)) | (((val as u32) & 0xffff_ffff) << 0usize);
        }
    }
    impl Default for StrengIrq {
        #[inline(always)]
        fn default() -> StrengIrq {
            StrengIrq(0)
        }
    }
    impl core::fmt::Debug for StrengIrq {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("StrengIrq").field("irq", &self.irq()).finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for StrengIrq {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "StrengIrq {{ irq: {=u32:?} }}", self.irq())
        }
    }
}
