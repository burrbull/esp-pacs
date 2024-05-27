///Register `DATA` reader
pub type R = crate::R<DATA_SPEC>;
///Register `DATA` writer
pub type W = crate::W<DATA_SPEC>;
///Field `RDATA` reader - Data received
pub type RDATA_R = crate::FieldReader;
///Field `SLAVE_TX_DATA` reader - The data sent by slave
pub type SLAVE_TX_DATA_R = crate::FieldReader;
///Field `SLAVE_TX_DATA` writer - The data sent by slave
pub type SLAVE_TX_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DONE` reader - RTC I2C transmission is done.
pub type DONE_R = crate::BitReader;
impl R {
    ///Bits 0:7 - Data received
    #[inline(always)]
    pub fn rdata(&self) -> RDATA_R {
        RDATA_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - The data sent by slave
    #[inline(always)]
    pub fn slave_tx_data(&self) -> SLAVE_TX_DATA_R {
        SLAVE_TX_DATA_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bit 31 - RTC I2C transmission is done.
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA")
            .field("rdata", &self.rdata())
            .field("slave_tx_data", &self.slave_tx_data())
            .field("done", &self.done())
            .finish()
    }
}
impl W {
    ///Bits 8:15 - The data sent by slave
    #[inline(always)]
    #[must_use]
    pub fn slave_tx_data(&mut self) -> SLAVE_TX_DATA_W<DATA_SPEC> {
        SLAVE_TX_DATA_W::new(self, 8)
    }
}
/**RTC I2C read data

You can [`read`](crate::generic::Reg::read) this register and get [`data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_SPEC;
impl crate::RegisterSpec for DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data::R`](R) reader structure
impl crate::Readable for DATA_SPEC {}
///`write(|w| ..)` method takes [`data::W`](W) writer structure
impl crate::Writable for DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA to value 0
impl crate::Resettable for DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
