#[doc = "Register `DATA_12` reader"]
pub type R = crate::R<DATA_12_SPEC>;
#[doc = "Register `DATA_12` writer"]
pub type W = crate::W<DATA_12_SPEC>;
#[doc = "Field `TX_BYTE_12` reader - In operation mode, it stores the 12th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_12_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_12` writer - In operation mode, it stores the 12th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_12_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In operation mode, it stores the 12th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_12(&self) -> TX_BYTE_12_R {
        TX_BYTE_12_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_12")
            .field("tx_byte_12", &self.tx_byte_12())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In operation mode, it stores the 12th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_12(&mut self) -> TX_BYTE_12_W<DATA_12_SPEC> {
        TX_BYTE_12_W::new(self, 0)
    }
}
#[doc = "Data register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`data_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_12_SPEC;
impl crate::RegisterSpec for DATA_12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_12::R`](R) reader structure"]
impl crate::Readable for DATA_12_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_12::W`](W) writer structure"]
impl crate::Writable for DATA_12_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_12 to value 0"]
impl crate::Resettable for DATA_12_SPEC {
    const RESET_VALUE: u32 = 0;
}
