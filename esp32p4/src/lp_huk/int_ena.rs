///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `PREP_DONE` reader - The interrupt enable bit for the huk_prep_done_int interrupt
pub type PREP_DONE_R = crate::BitReader;
///Field `PREP_DONE` writer - The interrupt enable bit for the huk_prep_done_int interrupt
pub type PREP_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PROC_DONE` reader - The interrupt enable bit for the huk_proc_done_int interrupt
pub type PROC_DONE_R = crate::BitReader;
///Field `PROC_DONE` writer - The interrupt enable bit for the huk_proc_done_int interrupt
pub type PROC_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `POST_DONE` reader - The interrupt enable bit for the huk_post_done_int interrupt
pub type POST_DONE_R = crate::BitReader;
///Field `POST_DONE` writer - The interrupt enable bit for the huk_post_done_int interrupt
pub type POST_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The interrupt enable bit for the huk_prep_done_int interrupt
    #[inline(always)]
    pub fn prep_done(&self) -> PREP_DONE_R {
        PREP_DONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The interrupt enable bit for the huk_proc_done_int interrupt
    #[inline(always)]
    pub fn proc_done(&self) -> PROC_DONE_R {
        PROC_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The interrupt enable bit for the huk_post_done_int interrupt
    #[inline(always)]
    pub fn post_done(&self) -> POST_DONE_R {
        POST_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("prep_done", &self.prep_done())
            .field("proc_done", &self.proc_done())
            .field("post_done", &self.post_done())
            .finish()
    }
}
impl W {
    ///Bit 0 - The interrupt enable bit for the huk_prep_done_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn prep_done(&mut self) -> PREP_DONE_W<INT_ENA_SPEC> {
        PREP_DONE_W::new(self, 0)
    }
    ///Bit 1 - The interrupt enable bit for the huk_proc_done_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn proc_done(&mut self) -> PROC_DONE_W<INT_ENA_SPEC> {
        PROC_DONE_W::new(self, 1)
    }
    ///Bit 2 - The interrupt enable bit for the huk_post_done_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn post_done(&mut self) -> POST_DONE_W<INT_ENA_SPEC> {
        POST_DONE_W::new(self, 2)
    }
}
/**HUK Generator interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
