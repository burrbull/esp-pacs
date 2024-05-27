///Register `CIS_CONF_W3` reader
pub type R = crate::R<CIS_CONF_W3_SPEC>;
///Register `CIS_CONF_W3` writer
pub type W = crate::W<CIS_CONF_W3_SPEC>;
///Field `CIS_CONF_W3` reader - Configure cis addr 51~48
pub type CIS_CONF_W3_R = crate::FieldReader<u32>;
///Field `CIS_CONF_W3` writer - Configure cis addr 51~48
pub type CIS_CONF_W3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - Configure cis addr 51~48
    #[inline(always)]
    pub fn cis_conf_w3(&self) -> CIS_CONF_W3_R {
        CIS_CONF_W3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIS_CONF_W3")
            .field("cis_conf_w3", &self.cis_conf_w3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - Configure cis addr 51~48
    #[inline(always)]
    #[must_use]
    pub fn cis_conf_w3(&mut self) -> CIS_CONF_W3_W<CIS_CONF_W3_SPEC> {
        CIS_CONF_W3_W::new(self, 0)
    }
}
/**SDIO cis configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cis_conf_w3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cis_conf_w3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CIS_CONF_W3_SPEC;
impl crate::RegisterSpec for CIS_CONF_W3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cis_conf_w3::R`](R) reader structure
impl crate::Readable for CIS_CONF_W3_SPEC {}
///`write(|w| ..)` method takes [`cis_conf_w3::W`](W) writer structure
impl crate::Writable for CIS_CONF_W3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CIS_CONF_W3 to value 0xffff_ffff
impl crate::Resettable for CIS_CONF_W3_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
