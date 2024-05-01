///Register `AXI_ID0` reader
pub type R = crate::R<AXI_ID0_SPEC>;
///Register `AXI_ID0` writer
pub type W = crate::W<AXI_ID0_SPEC>;
///Field `CH1_AXI_READ_ID_SUFFIX` reader - NA
pub type CH1_AXI_READ_ID_SUFFIX_R = crate::BitReader;
///Field `CH1_AXI_READ_ID_SUFFIX` writer - NA
pub type CH1_AXI_READ_ID_SUFFIX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_AXI_WRITE_ID_SUFFIX` reader - NA
pub type CH1_AXI_WRITE_ID_SUFFIX_R = crate::BitReader;
///Field `CH1_AXI_WRITE_ID_SUFFIX` writer - NA
pub type CH1_AXI_WRITE_ID_SUFFIX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn ch1_axi_read_id_suffix(&self) -> CH1_AXI_READ_ID_SUFFIX_R {
        CH1_AXI_READ_ID_SUFFIX_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - NA
    #[inline(always)]
    pub fn ch1_axi_write_id_suffix(&self) -> CH1_AXI_WRITE_ID_SUFFIX_R {
        CH1_AXI_WRITE_ID_SUFFIX_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_ID0")
            .field("ch1_axi_read_id_suffix", &self.ch1_axi_read_id_suffix())
            .field("ch1_axi_write_id_suffix", &self.ch1_axi_write_id_suffix())
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_axi_read_id_suffix(&mut self) -> CH1_AXI_READ_ID_SUFFIX_W<AXI_ID0_SPEC> {
        CH1_AXI_READ_ID_SUFFIX_W::new(self, 0)
    }
    ///Bit 16 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_axi_write_id_suffix(&mut self) -> CH1_AXI_WRITE_ID_SUFFIX_W<AXI_ID0_SPEC> {
        CH1_AXI_WRITE_ID_SUFFIX_W::new(self, 16)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`axi_id0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_id0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AXI_ID0_SPEC;
impl crate::RegisterSpec for AXI_ID0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`axi_id0::R`](R) reader structure
impl crate::Readable for AXI_ID0_SPEC {}
///`write(|w| ..)` method takes [`axi_id0::W`](W) writer structure
impl crate::Writable for AXI_ID0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AXI_ID0 to value 0
impl crate::Resettable for AXI_ID0_SPEC {
    const RESET_VALUE: u32 = 0;
}
