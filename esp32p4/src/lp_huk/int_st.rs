///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `PREP_DONE` reader - The masked interrupt status bit for the huk_prep_done_int interrupt
pub type PREP_DONE_R = crate::BitReader;
///Field `PROC_DONE` reader - The masked interrupt status bit for the huk_proc_done_int interrupt
pub type PROC_DONE_R = crate::BitReader;
///Field `POST_DONE` reader - The masked interrupt status bit for the huk_post_done_int interrupt
pub type POST_DONE_R = crate::BitReader;
impl R {
    ///Bit 0 - The masked interrupt status bit for the huk_prep_done_int interrupt
    #[inline(always)]
    pub fn prep_done(&self) -> PREP_DONE_R {
        PREP_DONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The masked interrupt status bit for the huk_proc_done_int interrupt
    #[inline(always)]
    pub fn proc_done(&self) -> PROC_DONE_R {
        PROC_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The masked interrupt status bit for the huk_post_done_int interrupt
    #[inline(always)]
    pub fn post_done(&self) -> POST_DONE_R {
        POST_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("prep_done", &self.prep_done())
            .field("proc_done", &self.proc_done())
            .field("post_done", &self.post_done())
            .finish()
    }
}
/**HUK Generator interrupt status register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_st::R`](R) reader structure
impl crate::Readable for INT_ST_SPEC {}
///`reset()` method sets INT_ST to value 0
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
