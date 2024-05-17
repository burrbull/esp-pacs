///Register `PWC` reader
pub type R = crate::R<PWC_SPEC>;
///Register `PWC` writer
pub type W = crate::W<PWC_SPEC>;
///Field `PAD_FORCE_HOLD` reader - rtc pad force hold
pub type PAD_FORCE_HOLD_R = crate::BitReader;
///Field `PAD_FORCE_HOLD` writer - rtc pad force hold
pub type PAD_FORCE_HOLD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 21 - rtc pad force hold
    #[inline(always)]
    pub fn pad_force_hold(&self) -> PAD_FORCE_HOLD_R {
        PAD_FORCE_HOLD_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWC").field("pad_force_hold", &self.pad_force_hold()).finish()
    }
}
impl W {
    ///Bit 21 - rtc pad force hold
    #[inline(always)]
    #[must_use]
    pub fn pad_force_hold(&mut self) -> PAD_FORCE_HOLD_W<PWC_SPEC> {
        PAD_FORCE_HOLD_W::new(self, 21)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`pwc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PWC_SPEC;
impl crate::RegisterSpec for PWC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pwc::R`](R) reader structure
impl crate::Readable for PWC_SPEC {}
///`write(|w| ..)` method takes [`pwc::W`](W) writer structure
impl crate::Writable for PWC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWC to value 0
impl crate::Resettable for PWC_SPEC {
    const RESET_VALUE: u32 = 0;
}
