///Register `LP_STORE13` reader
pub type R = crate::R<LP_STORE13_SPEC>;
///Register `LP_STORE13` writer
pub type W = crate::W<LP_STORE13_SPEC>;
///Field `LP_SCRATCH13` reader - need_des
pub type LP_SCRATCH13_R = crate::FieldReader<u32>;
///Field `LP_SCRATCH13` writer - need_des
pub type LP_SCRATCH13_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn lp_scratch13(&self) -> LP_SCRATCH13_R {
        LP_SCRATCH13_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_STORE13")
            .field("lp_scratch13", &self.lp_scratch13())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn lp_scratch13(&mut self) -> LP_SCRATCH13_W<LP_STORE13_SPEC> {
        LP_SCRATCH13_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_store13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_STORE13_SPEC;
impl crate::RegisterSpec for LP_STORE13_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_store13::R`](R) reader structure
impl crate::Readable for LP_STORE13_SPEC {}
///`write(|w| ..)` method takes [`lp_store13::W`](W) writer structure
impl crate::Writable for LP_STORE13_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_STORE13 to value 0
impl crate::Resettable for LP_STORE13_SPEC {
    const RESET_VALUE: u32 = 0;
}
