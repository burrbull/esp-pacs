#[doc = "Register `PERI_BACKUP_INT_ENA` reader"]
pub type R = crate::R<PERI_BACKUP_INT_ENA_SPEC>;
#[doc = "Register `PERI_BACKUP_INT_ENA` writer"]
pub type W = crate::W<PERI_BACKUP_INT_ENA_SPEC>;
#[doc = "Field `DONE` reader - reg_peri_backup_done_int_ena"]
pub type DONE_R = crate::BitReader;
#[doc = "Field `DONE` writer - reg_peri_backup_done_int_ena"]
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR` reader - reg_peri_backup_err_int_ena"]
pub type ERR_R = crate::BitReader;
#[doc = "Field `ERR` writer - reg_peri_backup_err_int_ena"]
pub type ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_peri_backup_done_int_ena"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_peri_backup_err_int_ena"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_BACKUP_INT_ENA")
            .field("done", &self.done().bit())
            .field("err", &self.err().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_BACKUP_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_peri_backup_done_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<PERI_BACKUP_INT_ENA_SPEC> {
        DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_peri_backup_err_int_ena"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<PERI_BACKUP_INT_ENA_SPEC> {
        ERR_W::new(self, 1)
    }
}
#[doc = "APB_CTRL_PERI_BACKUP_INT_ENA_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`peri_backup_int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`peri_backup_int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_BACKUP_INT_ENA_SPEC;
impl crate::RegisterSpec for PERI_BACKUP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_backup_int_ena::R`](R) reader structure"]
impl crate::Readable for PERI_BACKUP_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_backup_int_ena::W`](W) writer structure"]
impl crate::Writable for PERI_BACKUP_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_BACKUP_INT_ENA to value 0"]
impl crate::Resettable for PERI_BACKUP_INT_ENA_SPEC {}
