///Register `ECO_CONF` reader
pub type R = crate::R<ECO_CONF_SPEC>;
///Register `ECO_CONF` writer
pub type W = crate::W<ECO_CONF_SPEC>;
///Field `RDN_ENA` reader - enable rdn counter bit
pub type RDN_ENA_R = crate::BitReader;
///Field `RDN_ENA` writer - enable rdn counter bit
pub type RDN_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RDN_RESULT` reader - rdn result
pub type RDN_RESULT_R = crate::BitReader;
impl R {
    ///Bit 0 - enable rdn counter bit
    #[inline(always)]
    pub fn rdn_ena(&self) -> RDN_ENA_R {
        RDN_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - rdn result
    #[inline(always)]
    pub fn rdn_result(&self) -> RDN_RESULT_R {
        RDN_RESULT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_CONF")
            .field("rdn_ena", &self.rdn_ena())
            .field("rdn_result", &self.rdn_result())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable rdn counter bit
    #[inline(always)]
    #[must_use]
    pub fn rdn_ena(&mut self) -> RDN_ENA_W<ECO_CONF_SPEC> {
        RDN_ENA_W::new(self, 0)
    }
}
/**I2S ECO register

You can [`read`](crate::generic::Reg::read) this register and get [`eco_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ECO_CONF_SPEC;
impl crate::RegisterSpec for ECO_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`eco_conf::R`](R) reader structure
impl crate::Readable for ECO_CONF_SPEC {}
///`write(|w| ..)` method takes [`eco_conf::W`](W) writer structure
impl crate::Writable for ECO_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ECO_CONF to value 0
impl crate::Resettable for ECO_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
