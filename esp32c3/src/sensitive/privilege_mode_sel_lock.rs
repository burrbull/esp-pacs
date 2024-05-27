///Register `PRIVILEGE_MODE_SEL_LOCK` reader
pub type R = crate::R<PRIVILEGE_MODE_SEL_LOCK_SPEC>;
///Register `PRIVILEGE_MODE_SEL_LOCK` writer
pub type W = crate::W<PRIVILEGE_MODE_SEL_LOCK_SPEC>;
///Field `PRIVILEGE_MODE_SEL_LOCK` reader - privilege_mode_sel_lock
pub type PRIVILEGE_MODE_SEL_LOCK_R = crate::BitReader;
///Field `PRIVILEGE_MODE_SEL_LOCK` writer - privilege_mode_sel_lock
pub type PRIVILEGE_MODE_SEL_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - privilege_mode_sel_lock
    #[inline(always)]
    pub fn privilege_mode_sel_lock(&self) -> PRIVILEGE_MODE_SEL_LOCK_R {
        PRIVILEGE_MODE_SEL_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVILEGE_MODE_SEL_LOCK")
            .field("privilege_mode_sel_lock", &self.privilege_mode_sel_lock())
            .finish()
    }
}
impl W {
    ///Bit 0 - privilege_mode_sel_lock
    #[inline(always)]
    #[must_use]
    pub fn privilege_mode_sel_lock(
        &mut self,
    ) -> PRIVILEGE_MODE_SEL_LOCK_W<PRIVILEGE_MODE_SEL_LOCK_SPEC> {
        PRIVILEGE_MODE_SEL_LOCK_W::new(self, 0)
    }
}
/**SENSITIVE_PRIVILEGE_MODE_SEL_LOCK_REG

You can [`read`](crate::generic::Reg::read) this register and get [`privilege_mode_sel_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`privilege_mode_sel_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRIVILEGE_MODE_SEL_LOCK_SPEC;
impl crate::RegisterSpec for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`privilege_mode_sel_lock::R`](R) reader structure
impl crate::Readable for PRIVILEGE_MODE_SEL_LOCK_SPEC {}
///`write(|w| ..)` method takes [`privilege_mode_sel_lock::W`](W) writer structure
impl crate::Writable for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRIVILEGE_MODE_SEL_LOCK to value 0
impl crate::Resettable for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
