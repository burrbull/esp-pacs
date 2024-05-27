///Register `AXI_QOS0` reader
pub type R = crate::R<AXI_QOS0_SPEC>;
///Register `AXI_QOS0` writer
pub type W = crate::W<AXI_QOS0_SPEC>;
///Field `CH1_AXI_AWQOS` reader - NA
pub type CH1_AXI_AWQOS_R = crate::FieldReader;
///Field `CH1_AXI_AWQOS` writer - NA
pub type CH1_AXI_AWQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CH1_AXI_ARQOS` reader - NA
pub type CH1_AXI_ARQOS_R = crate::FieldReader;
///Field `CH1_AXI_ARQOS` writer - NA
pub type CH1_AXI_ARQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - NA
    #[inline(always)]
    pub fn ch1_axi_awqos(&self) -> CH1_AXI_AWQOS_R {
        CH1_AXI_AWQOS_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - NA
    #[inline(always)]
    pub fn ch1_axi_arqos(&self) -> CH1_AXI_ARQOS_R {
        CH1_AXI_ARQOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_QOS0")
            .field("ch1_axi_awqos", &self.ch1_axi_awqos())
            .field("ch1_axi_arqos", &self.ch1_axi_arqos())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_axi_awqos(&mut self) -> CH1_AXI_AWQOS_W<AXI_QOS0_SPEC> {
        CH1_AXI_AWQOS_W::new(self, 0)
    }
    ///Bits 4:7 - NA
    #[inline(always)]
    #[must_use]
    pub fn ch1_axi_arqos(&mut self) -> CH1_AXI_ARQOS_W<AXI_QOS0_SPEC> {
        CH1_AXI_ARQOS_W::new(self, 4)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`axi_qos0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_qos0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AXI_QOS0_SPEC;
impl crate::RegisterSpec for AXI_QOS0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`axi_qos0::R`](R) reader structure
impl crate::Readable for AXI_QOS0_SPEC {}
///`write(|w| ..)` method takes [`axi_qos0::W`](W) writer structure
impl crate::Writable for AXI_QOS0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AXI_QOS0 to value 0
impl crate::Resettable for AXI_QOS0_SPEC {
    const RESET_VALUE: u32 = 0;
}
