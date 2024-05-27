///Register `FH_STATUS` reader
pub type R = crate::R<FH_STATUS_SPEC>;
///Field `CBC_ON` reader - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going
pub type CBC_ON_R = crate::BitReader;
///Field `OST_ON` reader - Set and reset by hardware. If set, an one-shot mode action is on going
pub type OST_ON_R = crate::BitReader;
impl R {
    ///Bit 0 - Set and reset by hardware. If set, a cycle-by-cycle mode action is on going
    #[inline(always)]
    pub fn cbc_on(&self) -> CBC_ON_R {
        CBC_ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set and reset by hardware. If set, an one-shot mode action is on going
    #[inline(always)]
    pub fn ost_on(&self) -> OST_ON_R {
        OST_ON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FH_STATUS")
            .field("cbc_on", &self.cbc_on())
            .field("ost_on", &self.ost_on())
            .finish()
    }
}
/**Status of fault events.

You can [`read`](crate::generic::Reg::read) this register and get [`fh_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FH_STATUS_SPEC;
impl crate::RegisterSpec for FH_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fh_status::R`](R) reader structure
impl crate::Readable for FH_STATUS_SPEC {}
///`reset()` method sets FH_STATUS to value 0
impl crate::Resettable for FH_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
