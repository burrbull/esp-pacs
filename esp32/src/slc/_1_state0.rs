///Register `_1_STATE0` reader
pub type R = crate::R<_1_STATE0_SPEC>;
///Field `SLC1_STATE0` reader -
pub type SLC1_STATE0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn slc1_state0(&self) -> SLC1_STATE0_R {
        SLC1_STATE0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1_STATE0").field("slc1_state0", &self.slc1_state0()).finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_1_state0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _1_STATE0_SPEC;
impl crate::RegisterSpec for _1_STATE0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_1_state0::R`](R) reader structure
impl crate::Readable for _1_STATE0_SPEC {}
///`reset()` method sets _1_STATE0 to value 0
impl crate::Resettable for _1_STATE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
