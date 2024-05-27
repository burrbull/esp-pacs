///Register `ECO_CELL_CTRL_APB` reader
pub type R = crate::R<ECO_CELL_CTRL_APB_SPEC>;
///Register `ECO_CELL_CTRL_APB` writer
pub type W = crate::W<ECO_CELL_CTRL_APB_SPEC>;
///Field `RDN_RESULT_APB` reader - Reserved.
pub type RDN_RESULT_APB_R = crate::BitReader;
///Field `RDN_ENA_APB` reader - Reserved.
pub type RDN_ENA_APB_R = crate::BitReader;
///Field `RDN_ENA_APB` writer - Reserved.
pub type RDN_ENA_APB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Reserved.
    #[inline(always)]
    pub fn rdn_result_apb(&self) -> RDN_RESULT_APB_R {
        RDN_RESULT_APB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reserved.
    #[inline(always)]
    pub fn rdn_ena_apb(&self) -> RDN_ENA_APB_R {
        RDN_ENA_APB_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_CELL_CTRL_APB")
            .field("rdn_result_apb", &self.rdn_result_apb())
            .field("rdn_ena_apb", &self.rdn_ena_apb())
            .finish()
    }
}
impl W {
    ///Bit 1 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn rdn_ena_apb(&mut self) -> RDN_ENA_APB_W<ECO_CELL_CTRL_APB_SPEC> {
        RDN_ENA_APB_W::new(self, 1)
    }
}
/**Reserved.

You can [`read`](crate::generic::Reg::read) this register and get [`eco_cell_ctrl_apb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_cell_ctrl_apb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECO_CELL_CTRL_APB_SPEC;
impl crate::RegisterSpec for ECO_CELL_CTRL_APB_SPEC {
    type Ux = u32;
}
///`read()` method returns [`eco_cell_ctrl_apb::R`](R) reader structure
impl crate::Readable for ECO_CELL_CTRL_APB_SPEC {}
///`write(|w| ..)` method takes [`eco_cell_ctrl_apb::W`](W) writer structure
impl crate::Writable for ECO_CELL_CTRL_APB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ECO_CELL_CTRL_APB to value 0
impl crate::Resettable for ECO_CELL_CTRL_APB_SPEC {
    const RESET_VALUE: u32 = 0;
}
