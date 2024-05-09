#[doc = "Register `TX_CRC_DATA_EN_WR_DATA` reader"]
pub type R = crate::R<TX_CRC_DATA_EN_WR_DATA_SPEC>;
#[doc = "Register `TX_CRC_DATA_EN_WR_DATA` writer"]
pub type W = crate::W<TX_CRC_DATA_EN_WR_DATA_SPEC>;
#[doc = "Field `TX_CRC_DATA_EN_WR_DATA` reader - reserved"]
pub type TX_CRC_DATA_EN_WR_DATA_R = crate::FieldReader;
#[doc = "Field `TX_CRC_DATA_EN_WR_DATA` writer - reserved"]
pub type TX_CRC_DATA_EN_WR_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    pub fn tx_crc_data_en_wr_data(&self) -> TX_CRC_DATA_EN_WR_DATA_R {
        TX_CRC_DATA_EN_WR_DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_CRC_DATA_EN_WR_DATA")
            .field(
                "tx_crc_data_en_wr_data",
                &self.tx_crc_data_en_wr_data().bits(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TX_CRC_DATA_EN_WR_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    #[must_use]
    pub fn tx_crc_data_en_wr_data(
        &mut self,
    ) -> TX_CRC_DATA_EN_WR_DATA_W<TX_CRC_DATA_EN_WR_DATA_SPEC> {
        TX_CRC_DATA_EN_WR_DATA_W::new(self, 0)
    }
}
#[doc = "This register is used to config crc data_8bit en\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_crc_data_en_wr_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_crc_data_en_wr_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_CRC_DATA_EN_WR_DATA_SPEC;
impl crate::RegisterSpec for TX_CRC_DATA_EN_WR_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_crc_data_en_wr_data::R`](R) reader structure"]
impl crate::Readable for TX_CRC_DATA_EN_WR_DATA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_crc_data_en_wr_data::W`](W) writer structure"]
impl crate::Writable for TX_CRC_DATA_EN_WR_DATA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_CRC_DATA_EN_WR_DATA to value 0"]
impl crate::Resettable for TX_CRC_DATA_EN_WR_DATA_SPEC {}
