///Register `BLOCK_MODE` reader
pub type R = crate::R<BLOCK_MODE_SPEC>;
///Register `BLOCK_MODE` writer
pub type W = crate::W<BLOCK_MODE_SPEC>;
///Field `BLOCK_MODE` reader - Defines the operation type of the AES Accelerator operating under the DMA-AES working mode. For details, see Table 8. &amp; 3'h0(BLOCK_MODE_ECB): ECB # 3'h1(BLOCK_MODE_CBC): CBC # 3'h2(BLOCK_MODE_OFB): OFB # 3'h3(BLOCK_MODE_CTR): CTR # 3'h4(BLOCK_MODE_CFB8): CFB-8 # 3'h5(BLOCK_MODE_CFB128): CFB-128 # 3'h6(BLOCK_MODE_GCM): GCM &amp;
pub type BLOCK_MODE_R = crate::FieldReader;
///Field `BLOCK_MODE` writer - Defines the operation type of the AES Accelerator operating under the DMA-AES working mode. For details, see Table 8. &amp; 3'h0(BLOCK_MODE_ECB): ECB # 3'h1(BLOCK_MODE_CBC): CBC # 3'h2(BLOCK_MODE_OFB): OFB # 3'h3(BLOCK_MODE_CTR): CTR # 3'h4(BLOCK_MODE_CFB8): CFB-8 # 3'h5(BLOCK_MODE_CFB128): CFB-128 # 3'h6(BLOCK_MODE_GCM): GCM &amp;
pub type BLOCK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - Defines the operation type of the AES Accelerator operating under the DMA-AES working mode. For details, see Table 8. &amp; 3'h0(BLOCK_MODE_ECB): ECB # 3'h1(BLOCK_MODE_CBC): CBC # 3'h2(BLOCK_MODE_OFB): OFB # 3'h3(BLOCK_MODE_CTR): CTR # 3'h4(BLOCK_MODE_CFB8): CFB-8 # 3'h5(BLOCK_MODE_CFB128): CFB-128 # 3'h6(BLOCK_MODE_GCM): GCM &amp;
    #[inline(always)]
    pub fn block_mode(&self) -> BLOCK_MODE_R {
        BLOCK_MODE_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLOCK_MODE")
            .field("block_mode", &self.block_mode())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Defines the operation type of the AES Accelerator operating under the DMA-AES working mode. For details, see Table 8. &amp; 3'h0(BLOCK_MODE_ECB): ECB # 3'h1(BLOCK_MODE_CBC): CBC # 3'h2(BLOCK_MODE_OFB): OFB # 3'h3(BLOCK_MODE_CTR): CTR # 3'h4(BLOCK_MODE_CFB8): CFB-8 # 3'h5(BLOCK_MODE_CFB128): CFB-128 # 3'h6(BLOCK_MODE_GCM): GCM &amp;
    #[inline(always)]
    #[must_use]
    pub fn block_mode(&mut self) -> BLOCK_MODE_W<BLOCK_MODE_SPEC> {
        BLOCK_MODE_W::new(self, 0)
    }
}
/**Block operation type register

You can [`read`](crate::generic::Reg::read) this register and get [`block_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLOCK_MODE_SPEC;
impl crate::RegisterSpec for BLOCK_MODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`block_mode::R`](R) reader structure
impl crate::Readable for BLOCK_MODE_SPEC {}
///`write(|w| ..)` method takes [`block_mode::W`](W) writer structure
impl crate::Writable for BLOCK_MODE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BLOCK_MODE to value 0
impl crate::Resettable for BLOCK_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
