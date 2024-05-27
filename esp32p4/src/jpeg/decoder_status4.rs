///Register `DECODER_STATUS4` reader
pub type R = crate::R<DECODER_STATUS4_SPEC>;
///Field `BLOCK_EOF_CNT` reader - Reserved
pub type BLOCK_EOF_CNT_R = crate::FieldReader<u32>;
///Field `DEZIGZAG_READY` reader - Reserved
pub type DEZIGZAG_READY_R = crate::BitReader;
///Field `DE_FRAME_EOF_CHECK` reader - Reserved
pub type DE_FRAME_EOF_CHECK_R = crate::BitReader;
///Field `DE_DMA2D_IN_PUSH` reader - Reserved
pub type DE_DMA2D_IN_PUSH_R = crate::BitReader;
impl R {
    ///Bits 0:25 - Reserved
    #[inline(always)]
    pub fn block_eof_cnt(&self) -> BLOCK_EOF_CNT_R {
        BLOCK_EOF_CNT_R::new(self.bits & 0x03ff_ffff)
    }
    ///Bit 26 - Reserved
    #[inline(always)]
    pub fn dezigzag_ready(&self) -> DEZIGZAG_READY_R {
        DEZIGZAG_READY_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Reserved
    #[inline(always)]
    pub fn de_frame_eof_check(&self) -> DE_FRAME_EOF_CHECK_R {
        DE_FRAME_EOF_CHECK_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Reserved
    #[inline(always)]
    pub fn de_dma2d_in_push(&self) -> DE_DMA2D_IN_PUSH_R {
        DE_DMA2D_IN_PUSH_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DECODER_STATUS4")
            .field("block_eof_cnt", &self.block_eof_cnt())
            .field("dezigzag_ready", &self.dezigzag_ready())
            .field("de_frame_eof_check", &self.de_frame_eof_check())
            .field("de_dma2d_in_push", &self.de_dma2d_in_push())
            .finish()
    }
}
/**Trace and Debug registers

You can [`read`](crate::generic::Reg::read) this register and get [`decoder_status4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DECODER_STATUS4_SPEC;
impl crate::RegisterSpec for DECODER_STATUS4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`decoder_status4::R`](R) reader structure
impl crate::Readable for DECODER_STATUS4_SPEC {}
///`reset()` method sets DECODER_STATUS4 to value 0
impl crate::Resettable for DECODER_STATUS4_SPEC {
    const RESET_VALUE: u32 = 0;
}
