#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[doc = "SPDIF receiver interface."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spdifrx {
    ptr: *mut u8,
}
unsafe impl Send for Spdifrx {}
unsafe impl Sync for Spdifrx {}
impl Spdifrx {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "SPDIFRX control register."]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::Cr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0usize) as _) }
    }
    #[doc = "SPDIFRX interrupt mask register."]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x04usize) as _) }
    }
    #[doc = "SPDIFRX status register."]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x08usize) as _) }
    }
    #[doc = "SPDIFRX interrupt flag clear register."]
    #[inline(always)]
    pub const fn ifcr(self) -> crate::common::Reg<regs::Ifcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x0cusize) as _) }
    }
    #[doc = "SPDIFRX data input register."]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x10usize) as _) }
    }
    #[doc = "SPDIFRX channel status register."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x14usize) as _) }
    }
    #[doc = "SPDIFRX debug information register."]
    #[inline(always)]
    pub const fn dir(self) -> crate::common::Reg<regs::Dir, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.wrapping_add(0x18usize) as _) }
    }
}
pub mod regs {
    #[doc = "SPDIFRX control register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cr(pub u32);
    impl Cr {
        #[doc = "Peripheral block enableless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn spdifrxen(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x03;
            val as u8
        }
        #[doc = "Peripheral block enableless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_spdifrxen(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
        }
        #[doc = "Receiver DMA enable for data flowless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn rxdmaen(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Receiver DMA enable for data flowless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_rxdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Stereo modeless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn rxsteo(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Stereo modeless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_rxsteo(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "RX data formatless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn drfmt(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x03;
            val as u8
        }
        #[doc = "RX data formatless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_drfmt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
        }
        #[doc = "Mask parity error bitless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn pmsk(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Mask parity error bitless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_pmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Mask of validity bitless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn vmsk(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Mask of validity bitless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_vmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Mask of channel status and user bitsless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn cumsk(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Mask of channel status and user bitsless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_cumsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "Mask of preamble type bitsless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn ptmsk(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "Mask of preamble type bitsless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_ptmsk(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "Control buffer DMA enable for control flowless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn cbdmaen(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "Control buffer DMA enable for control flowless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_cbdmaen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "Channel selectionless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn chsel(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "Channel selectionless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_chsel(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "Maximum allowed re-tries during synchronization phaseless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn nbtr(&self) -> u8 {
            let val = (self.0 >> 12usize) & 0x03;
            val as u8
        }
        #[doc = "Maximum allowed re-tries during synchronization phaseless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_nbtr(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
        }
        #[doc = "Wait for activityless thansup>(1)less than/sup>."]
        #[must_use]
        #[inline(always)]
        pub const fn wfa(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "Wait for activityless thansup>(1)less than/sup>."]
        #[inline(always)]
        pub const fn set_wfa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "SPDIFRX input selection."]
        #[must_use]
        #[inline(always)]
        pub const fn insel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x07;
            val as u8
        }
        #[doc = "SPDIFRX input selection."]
        #[inline(always)]
        pub const fn set_insel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
        }
        #[doc = "Symbol clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cksen(&self) -> bool {
            let val = (self.0 >> 20usize) & 0x01;
            val != 0
        }
        #[doc = "Symbol clock enable."]
        #[inline(always)]
        pub const fn set_cksen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
        }
        #[doc = "Backup symbol clock enable."]
        #[must_use]
        #[inline(always)]
        pub const fn cksbkpen(&self) -> bool {
            let val = (self.0 >> 21usize) & 0x01;
            val != 0
        }
        #[doc = "Backup symbol clock enable."]
        #[inline(always)]
        pub const fn set_cksbkpen(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
        }
    }
    impl Default for Cr {
        #[inline(always)]
        fn default() -> Cr {
            Cr(0)
        }
    }
    impl core::fmt::Debug for Cr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Cr")
                .field("spdifrxen", &self.spdifrxen())
                .field("rxdmaen", &self.rxdmaen())
                .field("rxsteo", &self.rxsteo())
                .field("drfmt", &self.drfmt())
                .field("pmsk", &self.pmsk())
                .field("vmsk", &self.vmsk())
                .field("cumsk", &self.cumsk())
                .field("ptmsk", &self.ptmsk())
                .field("cbdmaen", &self.cbdmaen())
                .field("chsel", &self.chsel())
                .field("nbtr", &self.nbtr())
                .field("wfa", &self.wfa())
                .field("insel", &self.insel())
                .field("cksen", &self.cksen())
                .field("cksbkpen", &self.cksbkpen())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Cr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Cr {{ spdifrxen: {=u8:?}, rxdmaen: {=bool:?}, rxsteo: {=bool:?}, drfmt: {=u8:?}, pmsk: {=bool:?}, vmsk: {=bool:?}, cumsk: {=bool:?}, ptmsk: {=bool:?}, cbdmaen: {=bool:?}, chsel: {=bool:?}, nbtr: {=u8:?}, wfa: {=bool:?}, insel: {=u8:?}, cksen: {=bool:?}, cksbkpen: {=bool:?} }}",
                self.spdifrxen(),
                self.rxdmaen(),
                self.rxsteo(),
                self.drfmt(),
                self.pmsk(),
                self.vmsk(),
                self.cumsk(),
                self.ptmsk(),
                self.cbdmaen(),
                self.chsel(),
                self.nbtr(),
                self.wfa(),
                self.insel(),
                self.cksen(),
                self.cksbkpen()
            )
        }
    }
    #[doc = "SPDIFRX channel status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Csr(pub u32);
    impl Csr {
        #[doc = "user data information."]
        #[must_use]
        #[inline(always)]
        pub const fn usr(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "user data information."]
        #[inline(always)]
        pub const fn set_usr(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "channel A status information."]
        #[must_use]
        #[inline(always)]
        pub const fn cs(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "channel A status information."]
        #[inline(always)]
        pub const fn set_cs(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
        #[doc = "start of block."]
        #[must_use]
        #[inline(always)]
        pub const fn sob(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "start of block."]
        #[inline(always)]
        pub const fn set_sob(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
    }
    impl Default for Csr {
        #[inline(always)]
        fn default() -> Csr {
            Csr(0)
        }
    }
    impl core::fmt::Debug for Csr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Csr")
                .field("usr", &self.usr())
                .field("cs", &self.cs())
                .field("sob", &self.sob())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Csr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Csr {{ usr: {=u16:?}, cs: {=u8:?}, sob: {=bool:?} }}",
                self.usr(),
                self.cs(),
                self.sob()
            )
        }
    }
    #[doc = "SPDIFRX debug information register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dir(pub u32);
    impl Dir {
        #[doc = "threshold HIGH (THI = 2.5 x UI / Tless thansub>spdifrx_ker_ckless than/sub>)."]
        #[must_use]
        #[inline(always)]
        pub const fn thi(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "threshold HIGH (THI = 2.5 x UI / Tless thansub>spdifrx_ker_ckless than/sub>)."]
        #[inline(always)]
        pub const fn set_thi(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "threshold LOW (TLO = 1.5 x UI / Tless thansub>spdifrx_ker_ckless than/sub>)."]
        #[must_use]
        #[inline(always)]
        pub const fn tlo(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "threshold LOW (TLO = 1.5 x UI / Tless thansub>spdifrx_ker_ckless than/sub>)."]
        #[inline(always)]
        pub const fn set_tlo(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for Dir {
        #[inline(always)]
        fn default() -> Dir {
            Dir(0)
        }
    }
    impl core::fmt::Debug for Dir {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dir")
                .field("thi", &self.thi())
                .field("tlo", &self.tlo())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dir {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "Dir {{ thi: {=u16:?}, tlo: {=u16:?} }}", self.thi(), self.tlo())
        }
    }
    #[doc = "SPDIFRX data input register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dr(pub u32);
    impl Dr {
        #[doc = "data value."]
        #[must_use]
        #[inline(always)]
        pub const fn dr(&self) -> u32 {
            let val = (self.0 >> 0usize) & 0x00ff_ffff;
            val as u32
        }
        #[doc = "data value."]
        #[inline(always)]
        pub const fn set_dr(&mut self, val: u32) {
            self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
        }
        #[doc = "parity error bit."]
        #[must_use]
        #[inline(always)]
        pub const fn pe(&self) -> bool {
            let val = (self.0 >> 24usize) & 0x01;
            val != 0
        }
        #[doc = "parity error bit."]
        #[inline(always)]
        pub const fn set_pe(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
        }
        #[doc = "validity bit."]
        #[must_use]
        #[inline(always)]
        pub const fn v(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "validity bit."]
        #[inline(always)]
        pub const fn set_v(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "user bit."]
        #[must_use]
        #[inline(always)]
        pub const fn u(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "user bit."]
        #[inline(always)]
        pub const fn set_u(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "channel status bit."]
        #[must_use]
        #[inline(always)]
        pub const fn c(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "channel status bit."]
        #[inline(always)]
        pub const fn set_c(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "preamble type."]
        #[must_use]
        #[inline(always)]
        pub const fn pt(&self) -> u8 {
            let val = (self.0 >> 28usize) & 0x03;
            val as u8
        }
        #[doc = "preamble type."]
        #[inline(always)]
        pub const fn set_pt(&mut self, val: u8) {
            self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
        }
    }
    impl Default for Dr {
        #[inline(always)]
        fn default() -> Dr {
            Dr(0)
        }
    }
    impl core::fmt::Debug for Dr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Dr")
                .field("dr", &self.dr())
                .field("pe", &self.pe())
                .field("v", &self.v())
                .field("u", &self.u())
                .field("c", &self.c())
                .field("pt", &self.pt())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Dr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Dr {{ dr: {=u32:?}, pe: {=bool:?}, v: {=bool:?}, u: {=bool:?}, c: {=bool:?}, pt: {=u8:?} }}",
                self.dr(),
                self.pe(),
                self.v(),
                self.u(),
                self.c(),
                self.pt()
            )
        }
    }
    #[doc = "SPDIFRX interrupt flag clear register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ifcr(pub u32);
    impl Ifcr {
        #[doc = "clears the parity error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn perrcf(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "clears the parity error flag."]
        #[inline(always)]
        pub const fn set_perrcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "clears the overrun error flag."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrcf(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "clears the overrun error flag."]
        #[inline(always)]
        pub const fn set_ovrcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "clears the synchronization block detected flag."]
        #[must_use]
        #[inline(always)]
        pub const fn sbdcf(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "clears the synchronization block detected flag."]
        #[inline(always)]
        pub const fn set_sbdcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "clears the synchronization done flag."]
        #[must_use]
        #[inline(always)]
        pub const fn syncdcf(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "clears the synchronization done flag."]
        #[inline(always)]
        pub const fn set_syncdcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
    }
    impl Default for Ifcr {
        #[inline(always)]
        fn default() -> Ifcr {
            Ifcr(0)
        }
    }
    impl core::fmt::Debug for Ifcr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Ifcr")
                .field("perrcf", &self.perrcf())
                .field("ovrcf", &self.ovrcf())
                .field("sbdcf", &self.sbdcf())
                .field("syncdcf", &self.syncdcf())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Ifcr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Ifcr {{ perrcf: {=bool:?}, ovrcf: {=bool:?}, sbdcf: {=bool:?}, syncdcf: {=bool:?} }}",
                self.perrcf(),
                self.ovrcf(),
                self.sbdcf(),
                self.syncdcf()
            )
        }
    }
    #[doc = "SPDIFRX interrupt mask register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Imr(pub u32);
    impl Imr {
        #[doc = "RXNE interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn rxneie(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "RXNE interrupt enable."]
        #[inline(always)]
        pub const fn set_rxneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Control buffer ready interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn csrneie(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Control buffer ready interrupt enable."]
        #[inline(always)]
        pub const fn set_csrneie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Parity error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn perrie(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error interrupt enable."]
        #[inline(always)]
        pub const fn set_perrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ovrie(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error interrupt enable."]
        #[inline(always)]
        pub const fn set_ovrie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Synchronization block detected interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn sblkie(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization block detected interrupt enable."]
        #[inline(always)]
        pub const fn set_sblkie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Synchronization done."]
        #[must_use]
        #[inline(always)]
        pub const fn syncdie(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization done."]
        #[inline(always)]
        pub const fn set_syncdie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Serial interface error interrupt enable."]
        #[must_use]
        #[inline(always)]
        pub const fn ifeie(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Serial interface error interrupt enable."]
        #[inline(always)]
        pub const fn set_ifeie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
    }
    impl Default for Imr {
        #[inline(always)]
        fn default() -> Imr {
            Imr(0)
        }
    }
    impl core::fmt::Debug for Imr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Imr")
                .field("rxneie", &self.rxneie())
                .field("csrneie", &self.csrneie())
                .field("perrie", &self.perrie())
                .field("ovrie", &self.ovrie())
                .field("sblkie", &self.sblkie())
                .field("syncdie", &self.syncdie())
                .field("ifeie", &self.ifeie())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Imr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Imr {{ rxneie: {=bool:?}, csrneie: {=bool:?}, perrie: {=bool:?}, ovrie: {=bool:?}, sblkie: {=bool:?}, syncdie: {=bool:?}, ifeie: {=bool:?} }}",
                self.rxneie(),
                self.csrneie(),
                self.perrie(),
                self.ovrie(),
                self.sblkie(),
                self.syncdie(),
                self.ifeie()
            )
        }
    }
    #[doc = "SPDIFRX status register."]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sr(pub u32);
    impl Sr {
        #[doc = "Read data register not empty."]
        #[must_use]
        #[inline(always)]
        pub const fn rxne(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "Read data register not empty."]
        #[inline(always)]
        pub const fn set_rxne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "Control buffer register not empty."]
        #[must_use]
        #[inline(always)]
        pub const fn csrne(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "Control buffer register not empty."]
        #[inline(always)]
        pub const fn set_csrne(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "Parity error."]
        #[must_use]
        #[inline(always)]
        pub const fn perr(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "Parity error."]
        #[inline(always)]
        pub const fn set_perr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "Overrun error."]
        #[must_use]
        #[inline(always)]
        pub const fn ovr(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "Overrun error."]
        #[inline(always)]
        pub const fn set_ovr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "Synchronization block detected."]
        #[must_use]
        #[inline(always)]
        pub const fn sbd(&self) -> bool {
            let val = (self.0 >> 4usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization block detected."]
        #[inline(always)]
        pub const fn set_sbd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
        }
        #[doc = "Synchronization done."]
        #[must_use]
        #[inline(always)]
        pub const fn syncd(&self) -> bool {
            let val = (self.0 >> 5usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization done."]
        #[inline(always)]
        pub const fn set_syncd(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
        }
        #[doc = "Framing error."]
        #[must_use]
        #[inline(always)]
        pub const fn ferr(&self) -> bool {
            let val = (self.0 >> 6usize) & 0x01;
            val != 0
        }
        #[doc = "Framing error."]
        #[inline(always)]
        pub const fn set_ferr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
        }
        #[doc = "Synchronization error."]
        #[must_use]
        #[inline(always)]
        pub const fn serr(&self) -> bool {
            let val = (self.0 >> 7usize) & 0x01;
            val != 0
        }
        #[doc = "Synchronization error."]
        #[inline(always)]
        pub const fn set_serr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
        }
        #[doc = "Time-out error."]
        #[must_use]
        #[inline(always)]
        pub const fn terr(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "Time-out error."]
        #[inline(always)]
        pub const fn set_terr(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "duration of 5 symbols counted with spdifrx_ker_ck."]
        #[must_use]
        #[inline(always)]
        pub const fn width5(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x7fff;
            val as u16
        }
        #[doc = "duration of 5 symbols counted with spdifrx_ker_ck."]
        #[inline(always)]
        pub const fn set_width5(&mut self, val: u16) {
            self.0 = (self.0 & !(0x7fff << 16usize)) | (((val as u32) & 0x7fff) << 16usize);
        }
    }
    impl Default for Sr {
        #[inline(always)]
        fn default() -> Sr {
            Sr(0)
        }
    }
    impl core::fmt::Debug for Sr {
        fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
            f.debug_struct("Sr")
                .field("rxne", &self.rxne())
                .field("csrne", &self.csrne())
                .field("perr", &self.perr())
                .field("ovr", &self.ovr())
                .field("sbd", &self.sbd())
                .field("syncd", &self.syncd())
                .field("ferr", &self.ferr())
                .field("serr", &self.serr())
                .field("terr", &self.terr())
                .field("width5", &self.width5())
                .finish()
        }
    }
    #[cfg(feature = "defmt")]
    impl defmt::Format for Sr {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(
                f,
                "Sr {{ rxne: {=bool:?}, csrne: {=bool:?}, perr: {=bool:?}, ovr: {=bool:?}, sbd: {=bool:?}, syncd: {=bool:?}, ferr: {=bool:?}, serr: {=bool:?}, terr: {=bool:?}, width5: {=u16:?} }}",
                self.rxne(),
                self.csrne(),
                self.perr(),
                self.ovr(),
                self.sbd(),
                self.syncd(),
                self.ferr(),
                self.serr(),
                self.terr(),
                self.width5()
            )
        }
    }
}
