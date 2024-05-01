///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `RX_DONE` reader - The interrupt enable bit for the i2s_rx_done_int interrupt
pub type RX_DONE_R = crate::BitReader;
///Field `RX_DONE` writer - The interrupt enable bit for the i2s_rx_done_int interrupt
pub type RX_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_DONE` reader - The interrupt enable bit for the i2s_tx_done_int interrupt
pub type TX_DONE_R = crate::BitReader;
///Field `TX_DONE` writer - The interrupt enable bit for the i2s_tx_done_int interrupt
pub type TX_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_HUNG` reader - The interrupt enable bit for the i2s_rx_hung_int interrupt
pub type RX_HUNG_R = crate::BitReader;
///Field `RX_HUNG` writer - The interrupt enable bit for the i2s_rx_hung_int interrupt
pub type RX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_HUNG` reader - The interrupt enable bit for the i2s_tx_hung_int interrupt
pub type TX_HUNG_R = crate::BitReader;
///Field `TX_HUNG` writer - The interrupt enable bit for the i2s_tx_hung_int interrupt
pub type TX_HUNG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The interrupt enable bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The interrupt enable bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    pub fn rx_hung(&self) -> RX_HUNG_R {
        RX_HUNG_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The interrupt enable bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    pub fn tx_hung(&self) -> TX_HUNG_R {
        TX_HUNG_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("rx_done", &self.rx_done())
            .field("tx_done", &self.tx_done())
            .field("rx_hung", &self.rx_hung())
            .field("tx_hung", &self.tx_hung())
            .finish()
    }
}
impl W {
    ///Bit 0 - The interrupt enable bit for the i2s_rx_done_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn rx_done(&mut self) -> RX_DONE_W<INT_ENA_SPEC> {
        RX_DONE_W::new(self, 0)
    }
    ///Bit 1 - The interrupt enable bit for the i2s_tx_done_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn tx_done(&mut self) -> TX_DONE_W<INT_ENA_SPEC> {
        TX_DONE_W::new(self, 1)
    }
    ///Bit 2 - The interrupt enable bit for the i2s_rx_hung_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn rx_hung(&mut self) -> RX_HUNG_W<INT_ENA_SPEC> {
        RX_HUNG_W::new(self, 2)
    }
    ///Bit 3 - The interrupt enable bit for the i2s_tx_hung_int interrupt
    #[inline(always)]
    #[must_use]
    pub fn tx_hung(&mut self) -> TX_HUNG_W<INT_ENA_SPEC> {
        TX_HUNG_W::new(self, 3)
    }
}
/**I2S interrupt enable register.

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
