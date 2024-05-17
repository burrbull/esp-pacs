///Register `SYS` reader
pub type R = crate::R<SYS_SPEC>;
///Register `SYS` writer
pub type W = crate::W<SYS_SPEC>;
///Field `CLK_EN` reader - Reserved
pub type CLK_EN_R = crate::BitReader;
///Field `CLK_EN` writer - Reserved
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 31 - Reserved
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS").field("clk_en", &self.clk_en()).finish()
    }
}
impl W {
    ///Bit 31 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<SYS_SPEC> {
        CLK_EN_W::new(self, 31)
    }
}
/**Trace and Debug registers

You can [`read`](crate::generic::Reg::read) this register and get [`sys::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYS_SPEC;
impl crate::RegisterSpec for SYS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sys::R`](R) reader structure
impl crate::Readable for SYS_SPEC {}
///`write(|w| ..)` method takes [`sys::W`](W) writer structure
impl crate::Writable for SYS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYS to value 0
impl crate::Resettable for SYS_SPEC {
    const RESET_VALUE: u32 = 0;
}
