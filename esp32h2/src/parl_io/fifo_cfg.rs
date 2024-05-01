///Register `FIFO_CFG` reader
pub type R = crate::R<FIFO_CFG_SPEC>;
///Register `FIFO_CFG` writer
pub type W = crate::W<FIFO_CFG_SPEC>;
///Field `TX_FIFO_SRST` reader - Set this bit to reset async fifo in tx module.
pub type TX_FIFO_SRST_R = crate::BitReader;
///Field `TX_FIFO_SRST` writer - Set this bit to reset async fifo in tx module.
pub type TX_FIFO_SRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_FIFO_SRST` reader - Set this bit to reset async fifo in rx module.
pub type RX_FIFO_SRST_R = crate::BitReader;
///Field `RX_FIFO_SRST` writer - Set this bit to reset async fifo in rx module.
pub type RX_FIFO_SRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 30 - Set this bit to reset async fifo in tx module.
    #[inline(always)]
    pub fn tx_fifo_srst(&self) -> TX_FIFO_SRST_R {
        TX_FIFO_SRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Set this bit to reset async fifo in rx module.
    #[inline(always)]
    pub fn rx_fifo_srst(&self) -> RX_FIFO_SRST_R {
        RX_FIFO_SRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CFG")
            .field("tx_fifo_srst", &self.tx_fifo_srst())
            .field("rx_fifo_srst", &self.rx_fifo_srst())
            .finish()
    }
}
impl W {
    ///Bit 30 - Set this bit to reset async fifo in tx module.
    #[inline(always)]
    #[must_use]
    pub fn tx_fifo_srst(&mut self) -> TX_FIFO_SRST_W<FIFO_CFG_SPEC> {
        TX_FIFO_SRST_W::new(self, 30)
    }
    ///Bit 31 - Set this bit to reset async fifo in rx module.
    #[inline(always)]
    #[must_use]
    pub fn rx_fifo_srst(&mut self) -> RX_FIFO_SRST_W<FIFO_CFG_SPEC> {
        RX_FIFO_SRST_W::new(self, 31)
    }
}
/**Parallel IO FIFO configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`fifo_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FIFO_CFG_SPEC;
impl crate::RegisterSpec for FIFO_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fifo_cfg::R`](R) reader structure
impl crate::Readable for FIFO_CFG_SPEC {}
///`write(|w| ..)` method takes [`fifo_cfg::W`](W) writer structure
impl crate::Writable for FIFO_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FIFO_CFG to value 0
impl crate::Resettable for FIFO_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
