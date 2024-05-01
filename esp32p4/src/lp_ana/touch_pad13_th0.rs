///Register `TOUCH_PAD13_TH0` reader
pub type R = crate::R<TOUCH_PAD13_TH0_SPEC>;
///Register `TOUCH_PAD13_TH0` writer
pub type W = crate::W<TOUCH_PAD13_TH0_SPEC>;
///Field `TOUCH_PAD13_TH0` reader - Reserved
pub type TOUCH_PAD13_TH0_R = crate::FieldReader<u16>;
///Field `TOUCH_PAD13_TH0` writer - Reserved
pub type TOUCH_PAD13_TH0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 16:31 - Reserved
    #[inline(always)]
    pub fn touch_pad13_th0(&self) -> TOUCH_PAD13_TH0_R {
        TOUCH_PAD13_TH0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_PAD13_TH0")
            .field("touch_pad13_th0", &self.touch_pad13_th0())
            .finish()
    }
}
impl W {
    ///Bits 16:31 - Reserved
    #[inline(always)]
    #[must_use]
    pub fn touch_pad13_th0(&mut self) -> TOUCH_PAD13_TH0_W<TOUCH_PAD13_TH0_SPEC> {
        TOUCH_PAD13_TH0_W::new(self, 16)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`touch_pad13_th0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_pad13_th0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TOUCH_PAD13_TH0_SPEC;
impl crate::RegisterSpec for TOUCH_PAD13_TH0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`touch_pad13_th0::R`](R) reader structure
impl crate::Readable for TOUCH_PAD13_TH0_SPEC {}
///`write(|w| ..)` method takes [`touch_pad13_th0::W`](W) writer structure
impl crate::Writable for TOUCH_PAD13_TH0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOUCH_PAD13_TH0 to value 0
impl crate::Resettable for TOUCH_PAD13_TH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
