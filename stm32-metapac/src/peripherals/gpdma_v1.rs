#![allow(clippy::missing_safety_doc)]
#![allow(clippy::identity_op)]
#![allow(clippy::unnecessary_cast)]
#![allow(clippy::erasing_op)]

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPDMA channel 15 linked-list base address register"]
    #[inline(always)]
    pub const fn lbar(self) -> crate::common::Reg<regs::ChLbar, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPDMA channel 15 flag clear register"]
    #[inline(always)]
    pub const fn fcr(self) -> crate::common::Reg<regs::ChFcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "GPDMA channel 15 status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::ChSr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "GPDMA channel 15 control register"]
    #[inline(always)]
    pub const fn cr(self) -> crate::common::Reg<regs::ChCr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "GPDMA channel 15 transfer register 1"]
    #[inline(always)]
    pub const fn tr1(self) -> crate::common::Reg<regs::ChTr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "GPDMA channel 15 transfer register 2"]
    #[inline(always)]
    pub const fn tr2(self) -> crate::common::Reg<regs::ChTr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "GPDMA channel 15 alternate block register 1"]
    #[inline(always)]
    pub const fn br1(self) -> crate::common::Reg<regs::ChBr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "GPDMA channel 15 source address register"]
    #[inline(always)]
    pub const fn sar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "GPDMA channel 15 destination address register"]
    #[inline(always)]
    pub const fn dar(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "GPDMA channel 15 transfer register 3"]
    #[inline(always)]
    pub const fn tr3(self) -> crate::common::Reg<regs::ChTr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "GPDMA channel 15 block register 2"]
    #[inline(always)]
    pub const fn br2(self) -> crate::common::Reg<regs::ChBr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "GPDMA channel 15 alternate linked-list address register"]
    #[inline(always)]
    pub const fn llr(self) -> crate::common::Reg<regs::ChLlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
}
#[doc = "GPDMA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpdma {
    ptr: *mut u8,
}
unsafe impl Send for Gpdma {}
unsafe impl Sync for Gpdma {}
impl Gpdma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPDMA secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(self) -> crate::common::Reg<regs::Seccfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPDMA privileged configuration register"]
    #[inline(always)]
    pub const fn privcfgr(self) -> crate::common::Reg<regs::Privcfgr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "GPDMA configuration lock register"]
    #[inline(always)]
    pub const fn rcfglockr(self) -> crate::common::Reg<regs::Rcfglockr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPDMA non-secure masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(self) -> crate::common::Reg<regs::Misr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "GPDMA secure masked interrupt status register"]
    #[inline(always)]
    pub const fn smisr(self) -> crate::common::Reg<regs::Misr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Channel {
        assert!(n < 16usize);
        unsafe { Channel::from_ptr(self.ptr.add(0x50usize + n * 128usize) as _) }
    }
}
pub mod regs {
    #[doc = "GPDMA channel 15 alternate block register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChBr1(pub u32);
    impl ChBr1 {
        #[doc = "block number of data bytes to transfer from the source. Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if CH\\[x\\].LLR.UB1 = 1, this field is updated by the LLI in the memory. - if CH\\[x\\].LLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all CH\\[x\\].LLR.Uxx = 0 and if CH\\[x\\].LLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if CH\\[x\\].LLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus CH\\[x\\].TR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (CH\\[x\\].TR1.PAM\\[1\\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus CH\\[x\\].TR1.DDW\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub const fn bndt(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "block number of data bytes to transfer from the source. Block size transferred from the source. When the channel is enabled, this field becomes read-only and is decremented, indicating the remaining number of data items in the current source block to be transferred. BNDT\\[15:0\\]
is programmed in number of bytes, maximum source block size is 64 Kbytes -1. Once the last data transfer is completed (BNDT\\[15:0\\]
= 0): - if CH\\[x\\].LLR.UB1 = 1, this field is updated by the LLI in the memory. - if CH\\[x\\].LLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. - if all CH\\[x\\].LLR.Uxx = 0 and if CH\\[x\\].LLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). - if CH\\[x\\].LLR = 0, this field is kept as zero following the last LLI data transfer. Note: A non-null source block size must be a multiple of the source data width (BNDT\\[2:0\\]
versus CH\\[x\\].TR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued. When configured in packing mode (CH\\[x\\].TR1.PAM\\[1\\]=1 and destination data width different from source data width), a non-null source block size must be a multiple of the destination data width (BNDT\\[2:0\\]
versus CH\\[x\\].TR1.DDW\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub fn set_bndt(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Block repeat counter. This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\\[10:0\\]
= BNDT\\[15:0\\]
= 0): If CH\\[x\\].LLR.UB1 = 1, all CH\\[x\\].BR1 fields are updated by the next LLI in the memory. If CH\\[x\\].LLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all CH\\[x\\].LLR.Uxx = 0 and if CH\\[x\\].LLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if CH\\[x\\].LLR = 0, this field is kept as zero following the last LLI and data transfer."]
        #[inline(always)]
        pub const fn brc(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x07ff;
            val as u16
        }
        #[doc = "Block repeat counter. This field contains the number of repetitions of the current block (0 to 2047). When the channel is enabled, this field becomes read-only. After decrements, this field indicates the remaining number of blocks, excluding the current one. This counter is hardware decremented for each completed block transfer. Once the last block transfer is completed (BRC\\[10:0\\]
= BNDT\\[15:0\\]
= 0): If CH\\[x\\].LLR.UB1 = 1, all CH\\[x\\].BR1 fields are updated by the next LLI in the memory. If CH\\[x\\].LLR.UB1 = 0 and if there is at least one not null Uxx update bit, this field is internally restored to the programmed value. if all CH\\[x\\].LLR.Uxx = 0 and if CH\\[x\\].LLR.LA\\[15:0\\]
≠ 0, this field is internally restored to the programmed value (infinite/continuous last LLI). if CH\\[x\\].LLR = 0, this field is kept as zero following the last LLI and data transfer."]
        #[inline(always)]
        pub fn set_brc(&mut self, val: u16) {
            self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
        }
        #[doc = "source address decrement"]
        #[inline(always)]
        pub const fn sdec(&self) -> super::vals::Dec {
            let val = (self.0 >> 28usize) & 0x01;
            super::vals::Dec::from_bits(val as u8)
        }
        #[doc = "source address decrement"]
        #[inline(always)]
        pub fn set_sdec(&mut self, val: super::vals::Dec) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
        }
        #[doc = "destination address decrement"]
        #[inline(always)]
        pub const fn ddec(&self) -> super::vals::Dec {
            let val = (self.0 >> 29usize) & 0x01;
            super::vals::Dec::from_bits(val as u8)
        }
        #[doc = "destination address decrement"]
        #[inline(always)]
        pub fn set_ddec(&mut self, val: super::vals::Dec) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
        }
        #[doc = "Block repeat source address decrement. Note: On top of this increment/decrement (depending on BRSDEC), CH\\[x\\].SAR is in the same time also updated by the increment/decrement (depending on SDEC) of the CH\\[x\\].TR3.SAO value, as it is done after any programmed burst transfer."]
        #[inline(always)]
        pub const fn brsdec(&self) -> super::vals::Dec {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Dec::from_bits(val as u8)
        }
        #[doc = "Block repeat source address decrement. Note: On top of this increment/decrement (depending on BRSDEC), CH\\[x\\].SAR is in the same time also updated by the increment/decrement (depending on SDEC) of the CH\\[x\\].TR3.SAO value, as it is done after any programmed burst transfer."]
        #[inline(always)]
        pub fn set_brsdec(&mut self, val: super::vals::Dec) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "Block repeat destination address decrement. Note: On top of this increment/decrement (depending on BRDDEC), CH\\[x\\].DAR is in the same time also updated by the increment/decrement (depending on DDEC) of the CH\\[x\\].TR3.DAO value, as it is usually done at the end of each programmed burst transfer."]
        #[inline(always)]
        pub const fn brddec(&self) -> super::vals::Dec {
            let val = (self.0 >> 31usize) & 0x01;
            super::vals::Dec::from_bits(val as u8)
        }
        #[doc = "Block repeat destination address decrement. Note: On top of this increment/decrement (depending on BRDDEC), CH\\[x\\].DAR is in the same time also updated by the increment/decrement (depending on DDEC) of the CH\\[x\\].TR3.DAO value, as it is usually done at the end of each programmed burst transfer."]
        #[inline(always)]
        pub fn set_brddec(&mut self, val: super::vals::Dec) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ChBr1 {
        #[inline(always)]
        fn default() -> ChBr1 {
            ChBr1(0)
        }
    }
    #[doc = "GPDMA channel 12 block register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChBr2(pub u32);
    impl ChBr2 {
        #[doc = "Block repeated source address offset. For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on CH\\[x\\].BR1.BRSDEC) the current source address (CH\\[x\\].SAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\]
versus CH\\[x\\].TR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub const fn brsao(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0xffff;
            val as u16
        }
        #[doc = "Block repeated source address offset. For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on CH\\[x\\].BR1.BRSDEC) the current source address (CH\\[x\\].SAR) at the end of a block transfer. Note: A block repeated source address offset must be aligned with the programmed data width of a source burst (BRSAO\\[2:0\\]
versus CH\\[x\\].TR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub fn set_brsao(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
        }
        #[doc = "Block repeated destination address offset. For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on CH\\[x\\].BR1.BRDDEC) the current destination address (CH\\[x\\].DAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\]
versus CH\\[x\\].TR1.DDW\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub const fn brdao(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "Block repeated destination address offset. For a channel with 2D addressing capability, this field is used to update (by addition or subtraction depending on CH\\[x\\].BR1.BRDDEC) the current destination address (CH\\[x\\].DAR) at the end of a block transfer. Note: A block repeated destination address offset must be aligned with the programmed data width of a destination burst (BRDAO\\[2:0\\]
versus CH\\[x\\].TR1.DDW\\[1:0\\]). Else a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub fn set_brdao(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for ChBr2 {
        #[inline(always)]
        fn default() -> ChBr2 {
            ChBr2(0)
        }
    }
    #[doc = "GPDMA channel 11 control register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChCr(pub u32);
    impl ChCr {
        #[doc = "enable. Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
        #[inline(always)]
        pub const fn en(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "enable. Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 0. Else: this bit is de-asserted by hardware when there is a transfer error (master bus error or user setting error) or when there is a channel transfer complete (channel ready to be configured, e.g. if LSM=1 at the end of a single execution of the LLI). Else, this bit can be asserted by software. Writing 0 into this EN bit is ignored."]
        #[inline(always)]
        pub fn set_en(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "reset. This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (CH\\[x\\].SR.SUSPF = 1 and CH\\[x\\].SR.IDLEF = CH\\[x\\].CR.EN = 1). - channel in disabled state (CH\\[x\\].SR.IDLEF = 1 and CH\\[x\\].CR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (CH\\[x\\].BR1, CH\\[x\\].SAR and CH\\[x\\].DAR) before enabling again the channel (see the programming sequence in )."]
        #[inline(always)]
        pub const fn reset(&self) -> bool {
            let val = (self.0 >> 1usize) & 0x01;
            val != 0
        }
        #[doc = "reset. This bit is write only. Writing 0 has no impact. Writing 1 implies the reset of the following: the FIFO, the channel internal state, SUSP and EN bits (whatever is written receptively in bit 2 and bit 0). The reset is effective when the channel is in steady state, meaning one of the following: - active channel in suspended state (CH\\[x\\].SR.SUSPF = 1 and CH\\[x\\].SR.IDLEF = CH\\[x\\].CR.EN = 1). - channel in disabled state (CH\\[x\\].SR.IDLEF = 1 and CH\\[x\\].CR.EN = 0). After writing a RESET, to continue using this channel, the user must explicitly reconfigure the channel including the hardware-modified configuration registers (CH\\[x\\].BR1, CH\\[x\\].SAR and CH\\[x\\].DAR) before enabling again the channel (see the programming sequence in )."]
        #[inline(always)]
        pub fn set_reset(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
        }
        #[doc = "suspend. Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
        #[inline(always)]
        pub const fn susp(&self) -> bool {
            let val = (self.0 >> 2usize) & 0x01;
            val != 0
        }
        #[doc = "suspend. Writing 1 into the field RESET (bit 1) causes the hardware to de-assert this bit, whatever is written into this bit 2. Else: Software must write 1 in order to suspend an active channel i.e. a channel with an on-going GPDMA transfer over its master ports. The software must write 0 in order to resume a suspended channel, following the programming sequence detailed in ."]
        #[inline(always)]
        pub fn set_susp(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
        }
        #[doc = "transfer complete interrupt enable"]
        #[inline(always)]
        pub const fn tcie(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "transfer complete interrupt enable"]
        #[inline(always)]
        pub fn set_tcie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "half transfer complete interrupt enable"]
        #[inline(always)]
        pub const fn htie(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "half transfer complete interrupt enable"]
        #[inline(always)]
        pub fn set_htie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "data transfer error interrupt enable"]
        #[inline(always)]
        pub const fn dteie(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "data transfer error interrupt enable"]
        #[inline(always)]
        pub fn set_dteie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "update link transfer error interrupt enable"]
        #[inline(always)]
        pub const fn uleie(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "update link transfer error interrupt enable"]
        #[inline(always)]
        pub fn set_uleie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "user setting error interrupt enable"]
        #[inline(always)]
        pub const fn useie(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "user setting error interrupt enable"]
        #[inline(always)]
        pub fn set_useie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "completed suspension interrupt enable"]
        #[inline(always)]
        pub const fn suspie(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "completed suspension interrupt enable"]
        #[inline(always)]
        pub fn set_suspie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "trigger overrun interrupt enable"]
        #[inline(always)]
        pub const fn toie(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "trigger overrun interrupt enable"]
        #[inline(always)]
        pub fn set_toie(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "Link step mode. First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until CH\\[x\\].BR1.BNDT\\[15:0\\]
= 0 and CH\\[x\\].BR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by CH\\[x\\].LLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
        #[inline(always)]
        pub const fn lsm(&self) -> super::vals::Lsm {
            let val = (self.0 >> 16usize) & 0x01;
            super::vals::Lsm::from_bits(val as u8)
        }
        #[doc = "Link step mode. First the (possible 1D/repeated) block transfer is executed as defined by the current internal register file until CH\\[x\\].BR1.BNDT\\[15:0\\]
= 0 and CH\\[x\\].BR1.BRC\\[10:0\\]
= 0 if present. Secondly the next linked-list data structure is conditionally uploaded from memory as defined by CH\\[x\\].LLR. Then channel execution is completed. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
        #[inline(always)]
        pub fn set_lsm(&mut self, val: super::vals::Lsm) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
        }
        #[doc = "linked-list allocated port. This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
        #[inline(always)]
        pub const fn lap(&self) -> super::vals::Ap {
            let val = (self.0 >> 17usize) & 0x01;
            super::vals::Ap::from_bits(val as u8)
        }
        #[doc = "linked-list allocated port. This bit is used to allocate the master port for the update of the GPDMA linked-list registers from the memory. Note: This bit must be written when EN=0. This bit is read-only when EN=1."]
        #[inline(always)]
        pub fn set_lap(&mut self, val: super::vals::Ap) {
            self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
        }
        #[doc = "priority level of the channel x GPDMA transfer versus others. Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
        #[inline(always)]
        pub const fn prio(&self) -> super::vals::Prio {
            let val = (self.0 >> 22usize) & 0x03;
            super::vals::Prio::from_bits(val as u8)
        }
        #[doc = "priority level of the channel x GPDMA transfer versus others. Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
        #[inline(always)]
        pub fn set_prio(&mut self, val: super::vals::Prio) {
            self.0 = (self.0 & !(0x03 << 22usize)) | (((val.to_bits() as u32) & 0x03) << 22usize);
        }
    }
    impl Default for ChCr {
        #[inline(always)]
        fn default() -> ChCr {
            ChCr(0)
        }
    }
    #[doc = "GPDMA channel 7 flag clear register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChFcr(pub u32);
    impl ChFcr {
        #[doc = "transfer complete flag clear"]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "transfer complete flag clear"]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "half transfer flag clear"]
        #[inline(always)]
        pub const fn htf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "half transfer flag clear"]
        #[inline(always)]
        pub fn set_htf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "data transfer error flag clear"]
        #[inline(always)]
        pub const fn dtef(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "data transfer error flag clear"]
        #[inline(always)]
        pub fn set_dtef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "update link transfer error flag clear"]
        #[inline(always)]
        pub const fn ulef(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "update link transfer error flag clear"]
        #[inline(always)]
        pub fn set_ulef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "user setting error flag clear"]
        #[inline(always)]
        pub const fn usef(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "user setting error flag clear"]
        #[inline(always)]
        pub fn set_usef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "completed suspension flag clear"]
        #[inline(always)]
        pub const fn suspf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "completed suspension flag clear"]
        #[inline(always)]
        pub fn set_suspf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "trigger overrun flag clear"]
        #[inline(always)]
        pub const fn tof(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "trigger overrun flag clear"]
        #[inline(always)]
        pub fn set_tof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
    }
    impl Default for ChFcr {
        #[inline(always)]
        fn default() -> ChFcr {
            ChFcr(0)
        }
    }
    #[doc = "GPDMA channel 14 linked-list base address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChLbar(pub u32);
    impl ChLbar {
        #[doc = "linked-list base address of GPDMA channel x"]
        #[inline(always)]
        pub const fn lba(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0xffff;
            val as u16
        }
        #[doc = "linked-list base address of GPDMA channel x"]
        #[inline(always)]
        pub fn set_lba(&mut self, val: u16) {
            self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
        }
    }
    impl Default for ChLbar {
        #[inline(always)]
        fn default() -> ChLbar {
            ChLbar(0)
        }
    }
    #[doc = "GPDMA channel 15 alternate linked-list address register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChLlr(pub u32);
    impl ChLlr {
        #[doc = "pointer (16-bit low-significant address) to the next linked-list data structure. If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (CH\\[x\\].CTR1, CH\\[x\\].TR2, CH\\[x\\].BR1, CH\\[x\\].SAR, CH\\[x\\].DAR and CH\\[x\\].LLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
        #[inline(always)]
        pub const fn la(&self) -> u16 {
            let val = (self.0 >> 2usize) & 0x3fff;
            val as u16
        }
        #[doc = "pointer (16-bit low-significant address) to the next linked-list data structure. If UT1 = UT2 = UB1 = USA = UDA = ULL = 0 and if LA\\[15:20\\]
= 0, the current LLI is the last one. The channel transfer is completed without any update of the linked-list GPDMA register file. Else, this field is the pointer to the memory address offset from which the next linked-list data structure is automatically fetched from, once the data transfer is completed, in order to conditionally update the linked-list GPDMA internal register file (CH\\[x\\].CTR1, CH\\[x\\].TR2, CH\\[x\\].BR1, CH\\[x\\].SAR, CH\\[x\\].DAR and CH\\[x\\].LLR). Note: The user must program the pointer to be 32-bit aligned. The two low-significant bits are write ignored."]
        #[inline(always)]
        pub fn set_la(&mut self, val: u16) {
            self.0 = (self.0 & !(0x3fff << 2usize)) | (((val as u32) & 0x3fff) << 2usize);
        }
        #[doc = "Update CH\\[x\\].LLR register from memory. This bit is used to control the update of CH\\[x\\].LLR from the memory during the link transfer."]
        #[inline(always)]
        pub const fn ull(&self) -> bool {
            let val = (self.0 >> 16usize) & 0x01;
            val != 0
        }
        #[doc = "Update CH\\[x\\].LLR register from memory. This bit is used to control the update of CH\\[x\\].LLR from the memory during the link transfer."]
        #[inline(always)]
        pub fn set_ull(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
        }
        #[doc = "Update CH\\[x\\].BR2 from memory. This bit controls the update of CH\\[x\\].BR2 from the memory during the link transfer."]
        #[inline(always)]
        pub const fn ub2(&self) -> bool {
            let val = (self.0 >> 25usize) & 0x01;
            val != 0
        }
        #[doc = "Update CH\\[x\\].BR2 from memory. This bit controls the update of CH\\[x\\].BR2 from the memory during the link transfer."]
        #[inline(always)]
        pub fn set_ub2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
        }
        #[doc = "Update CH\\[x\\].TR3 from memory. This bit controls the update of CH\\[x\\].TR3 from the memory during the link transfer."]
        #[inline(always)]
        pub const fn ut3(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "Update CH\\[x\\].TR3 from memory. This bit controls the update of CH\\[x\\].TR3 from the memory during the link transfer."]
        #[inline(always)]
        pub fn set_ut3(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "Update CH\\[x\\].DAR register from memory. This bit is used to control the update of CH\\[x\\].DAR from the memory during the link transfer."]
        #[inline(always)]
        pub const fn uda(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "Update CH\\[x\\].DAR register from memory. This bit is used to control the update of CH\\[x\\].DAR from the memory during the link transfer."]
        #[inline(always)]
        pub fn set_uda(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "update CH\\[x\\].SAR from memory. This bit controls the update of CH\\[x\\].SAR from the memory during the link transfer."]
        #[inline(always)]
        pub const fn usa(&self) -> bool {
            let val = (self.0 >> 28usize) & 0x01;
            val != 0
        }
        #[doc = "update CH\\[x\\].SAR from memory. This bit controls the update of CH\\[x\\].SAR from the memory during the link transfer."]
        #[inline(always)]
        pub fn set_usa(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
        }
        #[doc = "Update CH\\[x\\].BR1 from memory. This bit controls the update of CH\\[x\\].BR1 from the memory during the link transfer. If UB1 = 0 and if CH\\[x\\].LLR ≠ 0, the linked-list is not completed. CH\\[x\\].BR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
        #[inline(always)]
        pub const fn ub1(&self) -> bool {
            let val = (self.0 >> 29usize) & 0x01;
            val != 0
        }
        #[doc = "Update CH\\[x\\].BR1 from memory. This bit controls the update of CH\\[x\\].BR1 from the memory during the link transfer. If UB1 = 0 and if CH\\[x\\].LLR ≠ 0, the linked-list is not completed. CH\\[x\\].BR1.BNDT\\[15:0\\]
is then restored to the programmed value after data transfer is completed and before the link transfer."]
        #[inline(always)]
        pub fn set_ub1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
        }
        #[doc = "Update CH\\[x\\].TR2 from memory. This bit controls the update of CH\\[x\\].TR2 from the memory during the link transfer."]
        #[inline(always)]
        pub const fn ut2(&self) -> bool {
            let val = (self.0 >> 30usize) & 0x01;
            val != 0
        }
        #[doc = "Update CH\\[x\\].TR2 from memory. This bit controls the update of CH\\[x\\].TR2 from the memory during the link transfer."]
        #[inline(always)]
        pub fn set_ut2(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
        }
        #[doc = "Update CH\\[x\\].TR1 from memory. This bit controls the update of CH\\[x\\].TR1 from the memory during the link transfer."]
        #[inline(always)]
        pub const fn ut1(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "Update CH\\[x\\].TR1 from memory. This bit controls the update of CH\\[x\\].TR1 from the memory during the link transfer."]
        #[inline(always)]
        pub fn set_ut1(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ChLlr {
        #[inline(always)]
        fn default() -> ChLlr {
            ChLlr(0)
        }
    }
    #[doc = "GPDMA channel 15 status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChSr(pub u32);
    impl ChSr {
        #[doc = "idle flag. This idle flag is de-asserted by hardware when the channel is enabled (CH\\[x\\].CR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state)."]
        #[inline(always)]
        pub const fn idlef(&self) -> bool {
            let val = (self.0 >> 0usize) & 0x01;
            val != 0
        }
        #[doc = "idle flag. This idle flag is de-asserted by hardware when the channel is enabled (CH\\[x\\].CR.EN = 1) with a valid channel configuration (no USEF to be immediately reported). This idle flag is asserted after hard reset or by hardware when the channel is back in idle state (in suspended or disabled state)."]
        #[inline(always)]
        pub fn set_idlef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
        }
        #[doc = "transfer complete flag. A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (CH\\[x\\].TR2.TCEM\\[1:0\\])."]
        #[inline(always)]
        pub const fn tcf(&self) -> bool {
            let val = (self.0 >> 8usize) & 0x01;
            val != 0
        }
        #[doc = "transfer complete flag. A transfer complete event is either a block transfer complete, a 2D/repeated block transfer complete, a LLI transfer complete including the upload of the next LLI if any, or the full linked-list completion, depending on the transfer complete event mode (CH\\[x\\].TR2.TCEM\\[1:0\\])."]
        #[inline(always)]
        pub fn set_tcf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
        }
        #[doc = "half transfer flag. An half transfer event is either an half block transfer or an half 2D/repeated block transfer, depending on the transfer complete event mode (CH\\[x\\].TR2.TCEM\\[1:0\\]). An half block transfer occurs when half of the bytes of the source block size (rounded up integer of CH\\[x\\].BR1.BNDT\\[15:0\\]/2) has been transferred to the destination. An half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (CH\\[x\\].BR1.BRC\\[10:0\\]+1)/2)) has been transferred to the destination."]
        #[inline(always)]
        pub const fn htf(&self) -> bool {
            let val = (self.0 >> 9usize) & 0x01;
            val != 0
        }
        #[doc = "half transfer flag. An half transfer event is either an half block transfer or an half 2D/repeated block transfer, depending on the transfer complete event mode (CH\\[x\\].TR2.TCEM\\[1:0\\]). An half block transfer occurs when half of the bytes of the source block size (rounded up integer of CH\\[x\\].BR1.BNDT\\[15:0\\]/2) has been transferred to the destination. An half 2D/repeated block transfer occurs when half of the repeated blocks (rounded up integer of (CH\\[x\\].BR1.BRC\\[10:0\\]+1)/2)) has been transferred to the destination."]
        #[inline(always)]
        pub fn set_htf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
        }
        #[doc = "data transfer error flag"]
        #[inline(always)]
        pub const fn dtef(&self) -> bool {
            let val = (self.0 >> 10usize) & 0x01;
            val != 0
        }
        #[doc = "data transfer error flag"]
        #[inline(always)]
        pub fn set_dtef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
        }
        #[doc = "update link transfer error flag"]
        #[inline(always)]
        pub const fn ulef(&self) -> bool {
            let val = (self.0 >> 11usize) & 0x01;
            val != 0
        }
        #[doc = "update link transfer error flag"]
        #[inline(always)]
        pub fn set_ulef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
        }
        #[doc = "user setting error flag"]
        #[inline(always)]
        pub const fn usef(&self) -> bool {
            let val = (self.0 >> 12usize) & 0x01;
            val != 0
        }
        #[doc = "user setting error flag"]
        #[inline(always)]
        pub fn set_usef(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
        }
        #[doc = "completed suspension flag"]
        #[inline(always)]
        pub const fn suspf(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "completed suspension flag"]
        #[inline(always)]
        pub fn set_suspf(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "trigger overrun flag"]
        #[inline(always)]
        pub const fn tof(&self) -> bool {
            let val = (self.0 >> 14usize) & 0x01;
            val != 0
        }
        #[doc = "trigger overrun flag"]
        #[inline(always)]
        pub fn set_tof(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
        }
        #[doc = "monitored FIFO level. Number of available write beats in the FIFO, in units of the programmed destination data width (see CH\\[x\\].TR1.DDW\\[1:0\\], in units of bytes, half-words, or words). Note: After having suspended an active transfer, the user may need to read FIFOL\\[7:0\\], additionally to CH\\[x\\].BR1.BDNT\\[15:0\\]
and CH\\[x\\].BR1.BRC\\[10:0\\], to know how many data have been transferred to the destination. Before reading, the user may wait for the transfer to be suspended (CH\\[x\\].SR.SUSPF = 1)."]
        #[inline(always)]
        pub const fn fifol(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0xff;
            val as u8
        }
        #[doc = "monitored FIFO level. Number of available write beats in the FIFO, in units of the programmed destination data width (see CH\\[x\\].TR1.DDW\\[1:0\\], in units of bytes, half-words, or words). Note: After having suspended an active transfer, the user may need to read FIFOL\\[7:0\\], additionally to CH\\[x\\].BR1.BDNT\\[15:0\\]
and CH\\[x\\].BR1.BRC\\[10:0\\], to know how many data have been transferred to the destination. Before reading, the user may wait for the transfer to be suspended (CH\\[x\\].SR.SUSPF = 1)."]
        #[inline(always)]
        pub fn set_fifol(&mut self, val: u8) {
            self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
        }
    }
    impl Default for ChSr {
        #[inline(always)]
        fn default() -> ChSr {
            ChSr(0)
        }
    }
    #[doc = "GPDMA channel 8 transfer register 1"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChTr1(pub u32);
    impl ChTr1 {
        #[doc = "binary logarithm of the source data width of a burst in bytes. Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (CH\\[x\\].BR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address CH\\[x\\].SAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
        #[inline(always)]
        pub const fn sdw(&self) -> super::vals::Dw {
            let val = (self.0 >> 0usize) & 0x03;
            super::vals::Dw::from_bits(val as u8)
        }
        #[doc = "binary logarithm of the source data width of a burst in bytes. Note: Setting a 8-byte data width causes a user setting error to be reported and no transfer is issued. A source block size must be a multiple of the source data width (CH\\[x\\].BR1.BNDT\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and no transfer is issued. A source single transfer must have an aligned address with its data width (start address CH\\[x\\].SAR\\[2:0\\]
versus SDW_LOG2\\[1:0\\]). Otherwise, a user setting error is reported and none transfer is issued."]
        #[inline(always)]
        pub fn set_sdw(&mut self, val: super::vals::Dw) {
            self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
        }
        #[doc = "source incrementing burst. The source address, pointed by CH\\[x\\].SAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
        #[inline(always)]
        pub const fn sinc(&self) -> bool {
            let val = (self.0 >> 3usize) & 0x01;
            val != 0
        }
        #[doc = "source incrementing burst. The source address, pointed by CH\\[x\\].SAR, is kept constant after a burst beat/single transfer or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
        #[inline(always)]
        pub fn set_sinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
        }
        #[doc = "source burst length minus 1, between 0 and 63. The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed."]
        #[inline(always)]
        pub const fn sbl_1(&self) -> u8 {
            let val = (self.0 >> 4usize) & 0x3f;
            val as u8
        }
        #[doc = "source burst length minus 1, between 0 and 63. The burst length unit is one data named beat within a burst. If SBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width SDW_LOG2\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed."]
        #[inline(always)]
        pub fn set_sbl_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
        }
        #[doc = "padding/alignment mode. If DDW\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else: - Case 1: If destination data width > source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer. - Case 2: If destination data width < source data width. 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination. Note:"]
        #[inline(always)]
        pub const fn pam(&self) -> super::vals::Pam {
            let val = (self.0 >> 11usize) & 0x03;
            super::vals::Pam::from_bits(val as u8)
        }
        #[doc = "padding/alignment mode. If DDW\\[1:0\\]
= SDW_LOG2\\[1:0\\]: if the data width of a burst destination transfer is equal to the data width of a burst source transfer, these bits are ignored. Else: - Case 1: If destination data width > source data width. 1x: successive source data are FIFO queued and packed at the destination data width, in a left (LSB) to right (MSB) order (named little endian), before a destination transfer. - Case 2: If destination data width < source data width. 1x: source data is FIFO queued and unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination. Note:"]
        #[inline(always)]
        pub fn set_pam(&mut self, val: super::vals::Pam) {
            self.0 = (self.0 & !(0x03 << 11usize)) | (((val.to_bits() as u32) & 0x03) << 11usize);
        }
        #[doc = "source byte exchange within the unaligned half-word of each source word. If set, the two consecutive bytes within the unaligned half-word of each source word are exchanged. If the source data width is shorter than a word, this bit is ignored."]
        #[inline(always)]
        pub const fn sbx(&self) -> bool {
            let val = (self.0 >> 13usize) & 0x01;
            val != 0
        }
        #[doc = "source byte exchange within the unaligned half-word of each source word. If set, the two consecutive bytes within the unaligned half-word of each source word are exchanged. If the source data width is shorter than a word, this bit is ignored."]
        #[inline(always)]
        pub fn set_sbx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
        }
        #[doc = "source allocated port. This bit is used to allocate the master port for the source transfer. Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
        #[inline(always)]
        pub const fn sap(&self) -> super::vals::Ap {
            let val = (self.0 >> 14usize) & 0x01;
            super::vals::Ap::from_bits(val as u8)
        }
        #[doc = "source allocated port. This bit is used to allocate the master port for the source transfer. Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
        #[inline(always)]
        pub fn set_sap(&mut self, val: super::vals::Ap) {
            self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
        }
        #[doc = "security attribute of the GPDMA transfer from the source. If SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when SECCFGR.SECx =1 . A secure write is ignored when SECCFGR.SECx = 0. When SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure."]
        #[inline(always)]
        pub const fn ssec(&self) -> bool {
            let val = (self.0 >> 15usize) & 0x01;
            val != 0
        }
        #[doc = "security attribute of the GPDMA transfer from the source. If SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when SECCFGR.SECx =1 . A secure write is ignored when SECCFGR.SECx = 0. When SECCFGR.SECx is de-asserted, this SSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer from the source is non-secure."]
        #[inline(always)]
        pub fn set_ssec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
        }
        #[doc = "binary logarithm of the destination data width of a burst, in bytes. Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address CH\\[x\\].DAR\\[2:0\\]
and address offset CH\\[x\\].TR3.DAO\\[2:0\\], versus DDW\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub const fn ddw(&self) -> super::vals::Dw {
            let val = (self.0 >> 16usize) & 0x03;
            super::vals::Dw::from_bits(val as u8)
        }
        #[doc = "binary logarithm of the destination data width of a burst, in bytes. Note: Setting a 8-byte data width causes a user setting error to be reported and none transfer is issued. A destination burst transfer must have an aligned address with its data width (start address CH\\[x\\].DAR\\[2:0\\]
and address offset CH\\[x\\].TR3.DAO\\[2:0\\], versus DDW\\[1:0\\]). Otherwise a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub fn set_ddw(&mut self, val: super::vals::Dw) {
            self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
        }
        #[doc = "destination incrementing burst. The destination address, pointed by CH\\[x\\].DAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
        #[inline(always)]
        pub const fn dinc(&self) -> bool {
            let val = (self.0 >> 19usize) & 0x01;
            val != 0
        }
        #[doc = "destination incrementing burst. The destination address, pointed by CH\\[x\\].DAR, is kept constant after a burst beat/single transfer, or is incremented by the offset value corresponding to a contiguous data after a burst beat/single transfer."]
        #[inline(always)]
        pub fn set_dinc(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
        }
        #[doc = "destination burst length minus 1, between 0 and 63. The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed."]
        #[inline(always)]
        pub const fn dbl_1(&self) -> u8 {
            let val = (self.0 >> 20usize) & 0x3f;
            val as u8
        }
        #[doc = "destination burst length minus 1, between 0 and 63. The burst length unit is one data named beat within a burst. If DBL_1\\[5:0\\]
=0 , the burst can be named as single. Each data/beat has a width defined by the destination data width DDW\\[1:0\\]. Note: If a burst transfer crossed a 1-Kbyte address boundary on a AHB transfer, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the AHB protocol. If a burst transfer is of length greater than the FIFO size of the channel x, the GPDMA modifies and shortens the programmed burst into singles or bursts of lower length, to be compliant with the FIFO size. Transfer performance is lower, with GPDMA re-arbitration between effective and lower bursts/singles, but the data integrity is guaranteed."]
        #[inline(always)]
        pub fn set_dbl_1(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
        }
        #[doc = "destination byte exchange. IF set, the two consecutive (post PAM) bytes are exchanged in each destination half-word. If the destination data size is a byte, this bit is ignored."]
        #[inline(always)]
        pub const fn dbx(&self) -> bool {
            let val = (self.0 >> 26usize) & 0x01;
            val != 0
        }
        #[doc = "destination byte exchange. IF set, the two consecutive (post PAM) bytes are exchanged in each destination half-word. If the destination data size is a byte, this bit is ignored."]
        #[inline(always)]
        pub fn set_dbx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
        }
        #[doc = "destination half-word exchange. If set, e two consecutive (post PAM) half-words are exchanged in each destination word. If the destination data size is shorter than a word, this bit is ignored."]
        #[inline(always)]
        pub const fn dhx(&self) -> bool {
            let val = (self.0 >> 27usize) & 0x01;
            val != 0
        }
        #[doc = "destination half-word exchange. If set, e two consecutive (post PAM) half-words are exchanged in each destination word. If the destination data size is shorter than a word, this bit is ignored."]
        #[inline(always)]
        pub fn set_dhx(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
        }
        #[doc = "destination allocated port. This bit is used to allocate the master port for the destination transfer. Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
        #[inline(always)]
        pub const fn dap(&self) -> super::vals::Ap {
            let val = (self.0 >> 30usize) & 0x01;
            super::vals::Ap::from_bits(val as u8)
        }
        #[doc = "destination allocated port. This bit is used to allocate the master port for the destination transfer. Note: This bit must be written when EN = 0. This bit is read-only when EN = 1."]
        #[inline(always)]
        pub fn set_dap(&mut self, val: super::vals::Ap) {
            self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
        }
        #[doc = "security attribute of the GPDMA transfer to the destination. If SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when SECCFGR.SECx = 1. A secure write is ignored when SECCFGR.SECx = 0. When SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure."]
        #[inline(always)]
        pub const fn dsec(&self) -> bool {
            let val = (self.0 >> 31usize) & 0x01;
            val != 0
        }
        #[doc = "security attribute of the GPDMA transfer to the destination. If SECCFGR.SECx = 1 and the access is secure: This is a secure register bit. This bit can only be read by a secure software. This bit must be written by a secure software when SECCFGR.SECx = 1. A secure write is ignored when SECCFGR.SECx = 0. When SECCFGR.SECx is de-asserted, this DSEC bit is also de-asserted by hardware (on a secure reconfiguration of the channel as non-secure), and the GPDMA transfer to the destination is non-secure."]
        #[inline(always)]
        pub fn set_dsec(&mut self, val: bool) {
            self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
        }
    }
    impl Default for ChTr1 {
        #[inline(always)]
        fn default() -> ChTr1 {
            ChTr1(0)
        }
    }
    #[doc = "GPDMA channel 10 transfer register 2"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChTr2(pub u32);
    impl ChTr2 {
        #[doc = "GPDMA hardware request selection. These bits are ignored if channel x is activated (CH\\[x\\].CR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\\[6:0\\]
value) to different active GPDMA channels (CH\\[x\\].CR.EN = 1 and CH\\[x\\].TR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
        #[inline(always)]
        pub const fn reqsel(&self) -> u8 {
            let val = (self.0 >> 0usize) & 0x7f;
            val as u8
        }
        #[doc = "GPDMA hardware request selection. These bits are ignored if channel x is activated (CH\\[x\\].CR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else, the selected hardware request is internally taken into account as per . The user must not assign a same input hardware request (same REQSEL\\[6:0\\]
value) to different active GPDMA channels (CH\\[x\\].CR.EN = 1 and CH\\[x\\].TR2.SWREQ = 0 for these channels). GPDMA is not intended to hardware support the case of simultaneous enabled channels incorrectly configured with a same hardware peripheral request signal, and there is no user setting error reporting."]
        #[inline(always)]
        pub fn set_reqsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
        }
        #[doc = "software request. This bit is internally taken into account when CH\\[x\\].CR.EN is asserted."]
        #[inline(always)]
        pub const fn swreq(&self) -> super::vals::Swreq {
            let val = (self.0 >> 9usize) & 0x01;
            super::vals::Swreq::from_bits(val as u8)
        }
        #[doc = "software request. This bit is internally taken into account when CH\\[x\\].CR.EN is asserted."]
        #[inline(always)]
        pub fn set_swreq(&mut self, val: super::vals::Swreq) {
            self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
        }
        #[doc = "destination hardware request. This bit is ignored if channel x is activated (CH\\[x\\].CR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note:"]
        #[inline(always)]
        pub const fn dreq(&self) -> super::vals::Dreq {
            let val = (self.0 >> 10usize) & 0x01;
            super::vals::Dreq::from_bits(val as u8)
        }
        #[doc = "destination hardware request. This bit is ignored if channel x is activated (CH\\[x\\].CR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer). Else: Note:"]
        #[inline(always)]
        pub fn set_dreq(&mut self, val: super::vals::Dreq) {
            self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
        }
        #[doc = "Block hardware request. If the channel x is activated (CH\\[x\\].CR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
        #[inline(always)]
        pub const fn breq(&self) -> super::vals::Breq {
            let val = (self.0 >> 11usize) & 0x01;
            super::vals::Breq::from_bits(val as u8)
        }
        #[doc = "Block hardware request. If the channel x is activated (CH\\[x\\].CR.EN asserted) with SWREQ = 1 (software request for a memory-to-memory transfer), this bit is ignored. Else:"]
        #[inline(always)]
        pub fn set_breq(&mut self, val: super::vals::Breq) {
            self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
        }
        #[doc = "trigger mode. These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (CH\\[x\\].CR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the CH\\[x\\].TR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized.memorized. A trigger overrun flag is reported (CH\\[x\\].SR.TOF =1 ), and an interrupt is generated if enabled (CH\\[x\\].CR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
        #[inline(always)]
        pub const fn trigm(&self) -> super::vals::Trigm {
            let val = (self.0 >> 14usize) & 0x03;
            super::vals::Trigm::from_bits(val as u8)
        }
        #[doc = "trigger mode. These bits define the transfer granularity for its conditioning by the trigger. If the channel x is enabled (CH\\[x\\].CR.EN asserted) with TRIGPOL\\[1:0\\]
= 00 or 11, these TRIGM\\[1:0\\]
bits are ignored. Else, a GPDMA transfer is conditioned by at least one trigger hit: first burst read of a 2D/repeated block transfer is conditioned by one hit trigger. – If the peripheral is programmed as a source (DREQ = 0) of the LLI data transfer, each programmed burst read is conditioned. – If the peripheral is programmed as a destination (DREQ = 1) of the LLI data transfer, each programmed burst write is conditioned. The first memory burst read of a (possibly 2D/repeated) block, also named as the first ready FIFO-based source burst, is gated by the occurrence of both the hardware request and the first trigger hit. The GPDMA monitoring of a trigger for channel x is started when the channel is enabled/loaded with a new active trigger configuration: rising or falling edge on a selected trigger (TRIGPOL\\[1:0\\]
= 01 or respectively TRIGPOL\\[1:0\\]
= 10). The monitoring of this trigger is kept active during the triggered and uncompleted (data or link) transfer; and if a new trigger is detected then, this hit is internally memorized to grant the next transfer, as long as the defined rising or falling edge is not modified, and the TRIGSEL\\[5:0\\]
is not modified, and the channel is enabled. Transferring a next LLIn+1 that updates the CH\\[x\\].TR2 with a new value for any of TRIGSEL\\[5:0\\]
or TRIGPOL\\[1:0\\], resets the monitoring, trashing the memorized hit of the formerly defined LLIn trigger. After a first new trigger hitn+1 is memorized, if another second trigger hitn+2 is detected and if the hitn triggered transfer is still not completed, hitn+2 is lost and not memorized.memorized. A trigger overrun flag is reported (CH\\[x\\].SR.TOF =1 ), and an interrupt is generated if enabled (CH\\[x\\].CR.TOIE = 1). The channel is not automatically disabled by hardware due to a trigger overrun. Note: When the source block size is not a multiple of the source burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, if TRIGM\\[1:0\\]
= 11 and (SWREQ =1 or (SWREQ = 0 and DREQ =0 )), the shortened burst transfer (by singles or/and by bursts of lower length) is conditioned once by the trigger. When the programmed destination burst is internally shortened by singles or/and by bursts of lower length (versus FIFO size, versus block size, 1-Kbyte boundary address crossing): if the trigger is conditioning the programmed destination burst (if TRIGM\\[1:0\\]
= 11 and SWREQ = 0 and DREQ = 1), this shortened destination burst transfer is conditioned once by the trigger."]
        #[inline(always)]
        pub fn set_trigm(&mut self, val: super::vals::Trigm) {
            self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
        }
        #[doc = "trigger event input selection. These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
        #[inline(always)]
        pub const fn trigsel(&self) -> u8 {
            let val = (self.0 >> 16usize) & 0x3f;
            val as u8
        }
        #[doc = "trigger event input selection. These bits select the trigger event input of the GPDMA transfer (as per ), with an active trigger event if TRIGPOL\\[1:0\\]
≠ 00."]
        #[inline(always)]
        pub fn set_trigsel(&mut self, val: u8) {
            self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
        }
        #[doc = "trigger event polarity. These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
        #[inline(always)]
        pub const fn trigpol(&self) -> super::vals::Trigpol {
            let val = (self.0 >> 24usize) & 0x03;
            super::vals::Trigpol::from_bits(val as u8)
        }
        #[doc = "trigger event polarity. These bits define the polarity of the selected trigger event input defined by TRIGSEL\\[5:0\\]."]
        #[inline(always)]
        pub fn set_trigpol(&mut self, val: super::vals::Trigpol) {
            self.0 = (self.0 & !(0x03 << 24usize)) | (((val.to_bits() as u32) & 0x03) << 24usize);
        }
        #[doc = "transfer complete event mode. These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with CH\\[x\\].BR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with CH\\[x\\].BR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with CH\\[x\\].BR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1."]
        #[inline(always)]
        pub const fn tcem(&self) -> super::vals::Tcem {
            let val = (self.0 >> 30usize) & 0x03;
            super::vals::Tcem::from_bits(val as u8)
        }
        #[doc = "transfer complete event mode. These bits define the transfer granularity for the transfer complete and half transfer complete events generation. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with CH\\[x\\].BR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (directly programmed by the internal register file with CH\\[x\\].BR1.BNDT\\[15:0\\]
= 0), then neither the complete transfer event nor the half transfer event is generated. Note: If the initial LLI0 data transfer is null/void (i.e. directly programmed by the internal register file with CH\\[x\\].BR1.BNDT\\[15:0\\]
=0 ), then the half transfer event is not generated, and the transfer complete event is generated when is completed the loading of the LLI1."]
        #[inline(always)]
        pub fn set_tcem(&mut self, val: super::vals::Tcem) {
            self.0 = (self.0 & !(0x03 << 30usize)) | (((val.to_bits() as u32) & 0x03) << 30usize);
        }
    }
    impl Default for ChTr2 {
        #[inline(always)]
        fn default() -> ChTr2 {
            ChTr2(0)
        }
    }
    #[doc = "GPDMA channel 14 transfer register 3"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChTr3(pub u32);
    impl ChTr3 {
        #[doc = "source address offset increment. The source address, pointed by CH\\[x\\].SAR, is incremented or decremented (depending on CH\\[x\\].BR1.SDEC) by this offset SAO\\[12:0\\]
for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (CH\\[x\\].TR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\\[2:0\\]
versus CH\\[x\\].TR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued. When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional CH\\[x\\].TR3.SAO\\[12:0\\]
is not applied."]
        #[inline(always)]
        pub const fn sao(&self) -> u16 {
            let val = (self.0 >> 0usize) & 0x1fff;
            val as u16
        }
        #[doc = "source address offset increment. The source address, pointed by CH\\[x\\].SAR, is incremented or decremented (depending on CH\\[x\\].BR1.SDEC) by this offset SAO\\[12:0\\]
for each programmed source burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (CH\\[x\\].TR1.SINC = 1). Note: A source address offset must be aligned with the programmed data width of a source burst (SAO\\[2:0\\]
versus CH\\[x\\].TR1.SDW_LOG2\\[1:0\\]). Else a user setting error is reported and none transfer is issued. When the source block size is not a multiple of the destination burst size and is a multiple of the source data width, then the last programmed source burst is not completed and is internally shorten to match the block size. In this case, the additional CH\\[x\\].TR3.SAO\\[12:0\\]
is not applied."]
        #[inline(always)]
        pub fn set_sao(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
        }
        #[doc = "destination address offset increment. The destination address, pointed by CH\\[x\\].DAR, is incremented or decremented (depending on CH\\[x\\].BR1.DDEC) by this offset DAO\\[12:0\\]
for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (CH\\[x\\].TR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\\[2:0\\]
versus CH\\[x\\].TR1.DDW\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub const fn dao(&self) -> u16 {
            let val = (self.0 >> 16usize) & 0x1fff;
            val as u16
        }
        #[doc = "destination address offset increment. The destination address, pointed by CH\\[x\\].DAR, is incremented or decremented (depending on CH\\[x\\].BR1.DDEC) by this offset DAO\\[12:0\\]
for each programmed destination burst. This offset is not including and is added to the programmed burst size when the completed burst is addressed in incremented mode (CH\\[x\\].TR1.DINC = 1). Note: A destination address offset must be aligned with the programmed data width of a destination burst (DAO\\[2:0\\]
versus CH\\[x\\].TR1.DDW\\[1:0\\]). Else, a user setting error is reported and no transfer is issued."]
        #[inline(always)]
        pub fn set_dao(&mut self, val: u16) {
            self.0 = (self.0 & !(0x1fff << 16usize)) | (((val as u32) & 0x1fff) << 16usize);
        }
    }
    impl Default for ChTr3 {
        #[inline(always)]
        fn default() -> ChTr3 {
            ChTr3(0)
        }
    }
    #[doc = "GPDMA secure masked interrupt status register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Misr(pub u32);
    impl Misr {
        #[doc = "MIS0"]
        #[inline(always)]
        pub const fn mis(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "MIS0"]
        #[inline(always)]
        pub fn set_mis(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Misr {
        #[inline(always)]
        fn default() -> Misr {
            Misr(0)
        }
    }
    #[doc = "GPDMA privileged configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Privcfgr(pub u32);
    impl Privcfgr {
        #[doc = "PRIV0"]
        #[inline(always)]
        pub const fn priv_(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "PRIV0"]
        #[inline(always)]
        pub fn set_priv_(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Privcfgr {
        #[inline(always)]
        fn default() -> Privcfgr {
            Privcfgr(0)
        }
    }
    #[doc = "GPDMA configuration lock register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rcfglockr(pub u32);
    impl Rcfglockr {
        #[doc = "LOCK0"]
        #[inline(always)]
        pub const fn lock(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "LOCK0"]
        #[inline(always)]
        pub fn set_lock(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Rcfglockr {
        #[inline(always)]
        fn default() -> Rcfglockr {
            Rcfglockr(0)
        }
    }
    #[doc = "GPDMA secure configuration register"]
    #[repr(transparent)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Seccfgr(pub u32);
    impl Seccfgr {
        #[doc = "SEC0"]
        #[inline(always)]
        pub const fn sec(&self, n: usize) -> bool {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            let val = (self.0 >> offs) & 0x01;
            val != 0
        }
        #[doc = "SEC0"]
        #[inline(always)]
        pub fn set_sec(&mut self, n: usize, val: bool) {
            assert!(n < 16usize);
            let offs = 0usize + n * 1usize;
            self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
        }
    }
    impl Default for Seccfgr {
        #[inline(always)]
        fn default() -> Seccfgr {
            Seccfgr(0)
        }
    }
}
pub mod vals {
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Ap {
        #[doc = "port 0 (AHB) allocated"]
        PORT0 = 0x0,
        #[doc = "port 1 (AHB) allocated"]
        PORT1 = 0x01,
    }
    impl Ap {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Ap {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Ap {
        #[inline(always)]
        fn from(val: u8) -> Ap {
            Ap::from_bits(val)
        }
    }
    impl From<Ap> for u8 {
        #[inline(always)]
        fn from(val: Ap) -> u8 {
            Ap::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Breq {
        #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a burst level."]
        BURST = 0x0,
        #[doc = "the selected hardware request is driven by a peripheral with a hardware request/acknowledge protocol at a block level (see )."]
        BLOCK = 0x01,
    }
    impl Breq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Breq {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Breq {
        #[inline(always)]
        fn from(val: u8) -> Breq {
            Breq::from_bits(val)
        }
    }
    impl From<Breq> for u8 {
        #[inline(always)]
        fn from(val: Breq) -> u8 {
            Breq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dec {
        #[doc = "The address is incremented by the programmed offset."]
        ADD = 0x0,
        #[doc = "The address is decremented by the programmed offset."]
        SUBTRACT = 0x01,
    }
    impl Dec {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dec {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dec {
        #[inline(always)]
        fn from(val: u8) -> Dec {
            Dec::from_bits(val)
        }
    }
    impl From<Dec> for u8 {
        #[inline(always)]
        fn from(val: Dec) -> u8 {
            Dec::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dreq {
        #[doc = "selected hardware request driven by a source peripheral (request signal taken into account by the GPDMA transfer scheduler over the source/read port)"]
        SOURCEPERIPHERAL = 0x0,
        #[doc = "selected hardware request driven by a destination peripheral (request signal taken into account by the GPDMA transfer scheduler over the destination/write port)"]
        DESTINATIONPERIPHERAL = 0x01,
    }
    impl Dreq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dreq {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dreq {
        #[inline(always)]
        fn from(val: u8) -> Dreq {
            Dreq::from_bits(val)
        }
    }
    impl From<Dreq> for u8 {
        #[inline(always)]
        fn from(val: Dreq) -> u8 {
            Dreq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Dw {
        #[doc = "byte"]
        BYTE = 0x0,
        #[doc = "half-word (2 bytes)"]
        HALFWORD = 0x01,
        #[doc = "word (4 bytes)"]
        WORD = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Dw {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Dw {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Dw {
        #[inline(always)]
        fn from(val: u8) -> Dw {
            Dw::from_bits(val)
        }
    }
    impl From<Dw> for u8 {
        #[inline(always)]
        fn from(val: Dw) -> u8 {
            Dw::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Lsm {
        #[doc = "channel executed for the full linked-list and completed at the end of the last LLI (CH\\[x\\].LLR = 0). The 16 low-significant bits of the link address are null (LA\\[15:0\\]
= 0) and all the update bits are null (UT1 =UB1 = UT2 = USA = UDA = ULL = 0 and UT3 = UB2 = 0 if present). Then CH\\[x\\].BR1.BNDT\\[15:0\\]
= 0 and CH\\[x\\].BR1.BRC\\[10:0\\]
= 0 if present."]
        RUNTOCOMPLETION = 0x0,
        #[doc = "channel executed once for the current LLI"]
        LINKSTEP = 0x01,
    }
    impl Lsm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Lsm {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Lsm {
        #[inline(always)]
        fn from(val: u8) -> Lsm {
            Lsm::from_bits(val)
        }
    }
    impl From<Lsm> for u8 {
        #[inline(always)]
        fn from(val: Lsm) -> u8 {
            Lsm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Pam {
        #[doc = "If destination is wider: source data is transferred as right aligned, padded with 0s up to the destination data width If source is wider: source data is transferred as right aligned, left-truncated down to the destination data width"]
        ZEROEXTENDORLEFTTRUNCATE = 0x0,
        #[doc = "If destination is wider: source data is transferred as right aligned, sign extended up to the destination data width If source is wider: source data is transferred as left-aligned, right-truncated down to the destination data width"]
        SIGNEXTENDORRIGHTTRUNCATE = 0x01,
        #[doc = "source data is FIFO queued and packed/unpacked at the destination data width, to be transferred in a left (LSB) to right (MSB) order (named little endian) to the destination"]
        PACK = 0x02,
        _RESERVED_3 = 0x03,
    }
    impl Pam {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Pam {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Pam {
        #[inline(always)]
        fn from(val: u8) -> Pam {
            Pam::from_bits(val)
        }
    }
    impl From<Pam> for u8 {
        #[inline(always)]
        fn from(val: Pam) -> u8 {
            Pam::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Prio {
        #[doc = "low priority, low weight"]
        LOWWITHLOWHWEIGHT = 0x0,
        #[doc = "low priority, mid weight"]
        LOWWITHMIDWEIGHT = 0x01,
        #[doc = "low priority, high weight"]
        LOWWITHHIGHWEIGHT = 0x02,
        #[doc = "high priority"]
        HIGH = 0x03,
    }
    impl Prio {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Prio {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Prio {
        #[inline(always)]
        fn from(val: u8) -> Prio {
            Prio::from_bits(val)
        }
    }
    impl From<Prio> for u8 {
        #[inline(always)]
        fn from(val: Prio) -> u8 {
            Prio::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Swreq {
        #[doc = "no software request. The selected hardware request REQSEL\\[6:0\\]
is taken into account."]
        HARDWARE = 0x0,
        #[doc = "software request for a memory-to-memory transfer. The default selected hardware request as per REQSEL\\[6:0\\]
is ignored."]
        SOFTWARE = 0x01,
    }
    impl Swreq {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Swreq {
            unsafe { core::mem::transmute(val & 0x01) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Swreq {
        #[inline(always)]
        fn from(val: u8) -> Swreq {
            Swreq::from_bits(val)
        }
    }
    impl From<Swreq> for u8 {
        #[inline(always)]
        fn from(val: Swreq) -> u8 {
            Swreq::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Tcem {
        #[doc = "at block level (when CH\\[x\\].BR1.BNDT\\[15:0\\]
= 0): the complete (and the half) transfer event is generated at the (respectively half of the) end of a block."]
        EACHBLOCK = 0x0,
        #[doc = "channel x = 0 to 11, same as 00; channel x=12 to 15, at 2D/repeated block level (when CH\\[x\\].BR1.BRC\\[10:0\\]
= 0 and CH\\[x\\].BR1.BNDT\\[15:0\\]
= 0), the complete (and the half) transfer event is generated at the end (respectively half of the end) of the 2D/repeated block."]
        EACH2DBLOCK = 0x01,
        #[doc = "at LLI level: the complete transfer event is generated at the end of the LLI transfer, including the update of the LLI if any. The half transfer event is generated at the half of the LLI data transfer (the LLI data transfer being a block transfer or a 2D/repeated block transfer for channel x = 12 to 15), if any data transfer."]
        EACHLINKEDLISTITEM = 0x02,
        #[doc = "at channel level: the complete transfer event is generated at the end of the last LLI transfer. The half transfer event is generated at the half of the data transfer of the last LLI. The last LLI updates the link address CH\\[x\\].LLR.LA\\[15:2\\]
to zero and clears all the CH\\[x\\].LLR update bits (UT1, UT2, UB1, USA, UDA and ULL, plus UT3 and UB2 if present). If the channel transfer is continuous/infinite, no event is generated."]
        LASTLINKEDLISTITEM = 0x03,
    }
    impl Tcem {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Tcem {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Tcem {
        #[inline(always)]
        fn from(val: u8) -> Tcem {
            Tcem::from_bits(val)
        }
    }
    impl From<Tcem> for u8 {
        #[inline(always)]
        fn from(val: Tcem) -> u8 {
            Tcem::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Trigm {
        #[doc = "at block level: the first burst read of each block transfer is conditioned by one hit trigger (channel x = 12 to 15, for each block if a 2D/repeated block is configured with CH\\[x\\].BR1.BRC\\[10:0\\]
≠ 0)."]
        BLOCK = 0x0,
        #[doc = "channel x = 0 to 11, same as 00; channel x=12 to 15, at 2D/repeated block level, the"]
        _2DBLOCK = 0x01,
        #[doc = "at link level: a LLI link transfer is conditioned by one hit trigger. The LLI data transfer (if any) is not conditioned."]
        LINKEDLISTITEM = 0x02,
        #[doc = "at programmed burst level: If SWREQ = 1, each programmed burst read is conditioned by one hit trigger. If SWREQ = 0, each programmed burst that is requested by the selected peripheral, is conditioned by one hit trigger."]
        BURST = 0x03,
    }
    impl Trigm {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trigm {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trigm {
        #[inline(always)]
        fn from(val: u8) -> Trigm {
            Trigm::from_bits(val)
        }
    }
    impl From<Trigm> for u8 {
        #[inline(always)]
        fn from(val: Trigm) -> u8 {
            Trigm::to_bits(val)
        }
    }
    #[repr(u8)]
    #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
    pub enum Trigpol {
        #[doc = "no trigger (masked trigger event)"]
        NONE = 0x0,
        #[doc = "trigger on the rising edge"]
        RISINGEDGE = 0x01,
        #[doc = "trigger on the falling edge"]
        FALLINGEDGE = 0x02,
        #[doc = "same as 00"]
        NONEALT = 0x03,
    }
    impl Trigpol {
        #[inline(always)]
        pub const fn from_bits(val: u8) -> Trigpol {
            unsafe { core::mem::transmute(val & 0x03) }
        }
        #[inline(always)]
        pub const fn to_bits(self) -> u8 {
            unsafe { core::mem::transmute(self) }
        }
    }
    impl From<u8> for Trigpol {
        #[inline(always)]
        fn from(val: u8) -> Trigpol {
            Trigpol::from_bits(val)
        }
    }
    impl From<Trigpol> for u8 {
        #[inline(always)]
        fn from(val: Trigpol) -> u8 {
            Trigpol::to_bits(val)
        }
    }
}
