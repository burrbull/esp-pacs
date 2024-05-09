#[doc = "Register `RX_CRC_DATA_EN_ADDR` reader"]
pub type R = crate::R<RX_CRC_DATA_EN_ADDR_SPEC>;
#[doc = "Register `RX_CRC_DATA_EN_ADDR` writer"]
pub type W = crate::W<RX_CRC_DATA_EN_ADDR_SPEC>;
#[doc = "Field `RX_CRC_DATA_EN_ADDR` reader - reserved"]
pub type RX_CRC_DATA_EN_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `RX_CRC_DATA_EN_ADDR` writer - reserved"]
pub type RX_CRC_DATA_EN_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn rx_crc_data_en_addr(&self) -> RX_CRC_DATA_EN_ADDR_R {
        RX_CRC_DATA_EN_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_CRC_DATA_EN_ADDR")
            .field("rx_crc_data_en_addr", &self.rx_crc_data_en_addr().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_CRC_DATA_EN_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn rx_crc_data_en_addr(&mut self) -> RX_CRC_DATA_EN_ADDR_W<RX_CRC_DATA_EN_ADDR_SPEC> {
        RX_CRC_DATA_EN_ADDR_W::new(self, 0)
    }
}
#[doc = "This register is used to config addr of crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_crc_data_en_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_crc_data_en_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_CRC_DATA_EN_ADDR_SPEC;
impl crate::RegisterSpec for RX_CRC_DATA_EN_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_data_en_addr::R`](R) reader structure"]
impl crate::Readable for RX_CRC_DATA_EN_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_crc_data_en_addr::W`](W) writer structure"]
impl crate::Writable for RX_CRC_DATA_EN_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_CRC_DATA_EN_ADDR to value 0"]
impl crate::Resettable for RX_CRC_DATA_EN_ADDR_SPEC {}
