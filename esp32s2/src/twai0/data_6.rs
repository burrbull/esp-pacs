#[doc = "Register `DATA_6` reader"]
pub type R = crate::R<DATA_6_SPEC>;
#[doc = "Register `DATA_6` writer"]
pub type W = crate::W<DATA_6_SPEC>;
#[doc = "Field `TX_BYTE_6` reader - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, it stores the 6th byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_6_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_6` writer - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, it stores the 6th byte information of the data to be transmitted under operating mode."]
pub type TX_BYTE_6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, it stores the 6th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    pub fn tx_byte_6(&self) -> TX_BYTE_6_R {
        TX_BYTE_6_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_6")
            .field("tx_byte_6", &self.tx_byte_6().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, it stores the 6th byte information of the data to be transmitted under operating mode."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_6(&mut self) -> TX_BYTE_6_W<DATA_6_SPEC> {
        TX_BYTE_6_W::new(self, 0)
    }
}
#[doc = "Data register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_6_SPEC;
impl crate::RegisterSpec for DATA_6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_6::R`](R) reader structure"]
impl crate::Readable for DATA_6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_6::W`](W) writer structure"]
impl crate::Writable for DATA_6_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_6 to value 0"]
impl crate::Resettable for DATA_6_SPEC {}
