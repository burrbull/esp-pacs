///Register `RXEOF_NUM` reader
pub type R = crate::R<RXEOF_NUM_SPEC>;
///Register `RXEOF_NUM` writer
pub type W = crate::W<RXEOF_NUM_SPEC>;
///Field `RX_EOF_NUM` reader - The length of data to be received. It will trigger I2S_IN_SUC_EOF_INT.
pub type RX_EOF_NUM_R = crate::FieldReader<u32>;
///Field `RX_EOF_NUM` writer - The length of data to be received. It will trigger I2S_IN_SUC_EOF_INT.
pub type RX_EOF_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The length of data to be received. It will trigger I2S_IN_SUC_EOF_INT.
    #[inline(always)]
    pub fn rx_eof_num(&self) -> RX_EOF_NUM_R {
        RX_EOF_NUM_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXEOF_NUM")
            .field("rx_eof_num", &self.rx_eof_num())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The length of data to be received. It will trigger I2S_IN_SUC_EOF_INT.
    #[inline(always)]
    #[must_use]
    pub fn rx_eof_num(&mut self) -> RX_EOF_NUM_W<RXEOF_NUM_SPEC> {
        RX_EOF_NUM_W::new(self, 0)
    }
}
/**I2S DMA RX EOF data length

You can [`read`](crate::generic::Reg::read) this register and get [`rxeof_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxeof_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RXEOF_NUM_SPEC;
impl crate::RegisterSpec for RXEOF_NUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rxeof_num::R`](R) reader structure
impl crate::Readable for RXEOF_NUM_SPEC {}
///`write(|w| ..)` method takes [`rxeof_num::W`](W) writer structure
impl crate::Writable for RXEOF_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RXEOF_NUM to value 0x40
impl crate::Resettable for RXEOF_NUM_SPEC {
    const RESET_VALUE: u32 = 0x40;
}
