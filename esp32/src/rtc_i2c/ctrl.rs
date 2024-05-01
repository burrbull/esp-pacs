///Register `CTRL` reader
pub type R = crate::R<CTRL_SPEC>;
///Register `CTRL` writer
pub type W = crate::W<CTRL_SPEC>;
///Field `SDA_FORCE_OUT` reader - SDA is push-pull (1) or open-drain (0)
pub type SDA_FORCE_OUT_R = crate::BitReader;
///Field `SDA_FORCE_OUT` writer - SDA is push-pull (1) or open-drain (0)
pub type SDA_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SCL_FORCE_OUT` reader - SCL is push-pull (1) or open-drain (0)
pub type SCL_FORCE_OUT_R = crate::BitReader;
///Field `SCL_FORCE_OUT` writer - SCL is push-pull (1) or open-drain (0)
pub type SCL_FORCE_OUT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MS_MODE` reader - Master (1) or slave (0)
pub type MS_MODE_R = crate::BitReader;
///Field `MS_MODE` writer - Master (1) or slave (0)
pub type MS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRANS_START` reader - Force to generate start condition
pub type TRANS_START_R = crate::BitReader;
///Field `TRANS_START` writer - Force to generate start condition
pub type TRANS_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_LSB_FIRST` reader - Send LSB first
pub type TX_LSB_FIRST_R = crate::BitReader;
///Field `TX_LSB_FIRST` writer - Send LSB first
pub type TX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_LSB_FIRST` reader - Receive LSB first
pub type RX_LSB_FIRST_R = crate::BitReader;
///Field `RX_LSB_FIRST` writer - Receive LSB first
pub type RX_LSB_FIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - SDA is push-pull (1) or open-drain (0)
    #[inline(always)]
    pub fn sda_force_out(&self) -> SDA_FORCE_OUT_R {
        SDA_FORCE_OUT_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SCL is push-pull (1) or open-drain (0)
    #[inline(always)]
    pub fn scl_force_out(&self) -> SCL_FORCE_OUT_R {
        SCL_FORCE_OUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Master (1) or slave (0)
    #[inline(always)]
    pub fn ms_mode(&self) -> MS_MODE_R {
        MS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Force to generate start condition
    #[inline(always)]
    pub fn trans_start(&self) -> TRANS_START_R {
        TRANS_START_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Send LSB first
    #[inline(always)]
    pub fn tx_lsb_first(&self) -> TX_LSB_FIRST_R {
        TX_LSB_FIRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Receive LSB first
    #[inline(always)]
    pub fn rx_lsb_first(&self) -> RX_LSB_FIRST_R {
        RX_LSB_FIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL")
            .field("sda_force_out", &self.sda_force_out())
            .field("scl_force_out", &self.scl_force_out())
            .field("ms_mode", &self.ms_mode())
            .field("trans_start", &self.trans_start())
            .field("tx_lsb_first", &self.tx_lsb_first())
            .field("rx_lsb_first", &self.rx_lsb_first())
            .finish()
    }
}
impl W {
    ///Bit 0 - SDA is push-pull (1) or open-drain (0)
    #[inline(always)]
    #[must_use]
    pub fn sda_force_out(&mut self) -> SDA_FORCE_OUT_W<CTRL_SPEC> {
        SDA_FORCE_OUT_W::new(self, 0)
    }
    ///Bit 1 - SCL is push-pull (1) or open-drain (0)
    #[inline(always)]
    #[must_use]
    pub fn scl_force_out(&mut self) -> SCL_FORCE_OUT_W<CTRL_SPEC> {
        SCL_FORCE_OUT_W::new(self, 1)
    }
    ///Bit 4 - Master (1) or slave (0)
    #[inline(always)]
    #[must_use]
    pub fn ms_mode(&mut self) -> MS_MODE_W<CTRL_SPEC> {
        MS_MODE_W::new(self, 4)
    }
    ///Bit 5 - Force to generate start condition
    #[inline(always)]
    #[must_use]
    pub fn trans_start(&mut self) -> TRANS_START_W<CTRL_SPEC> {
        TRANS_START_W::new(self, 5)
    }
    ///Bit 6 - Send LSB first
    #[inline(always)]
    #[must_use]
    pub fn tx_lsb_first(&mut self) -> TX_LSB_FIRST_W<CTRL_SPEC> {
        TX_LSB_FIRST_W::new(self, 6)
    }
    ///Bit 7 - Receive LSB first
    #[inline(always)]
    #[must_use]
    pub fn rx_lsb_first(&mut self) -> RX_LSB_FIRST_W<CTRL_SPEC> {
        RX_LSB_FIRST_W::new(self, 7)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ctrl::R`](R) reader structure
impl crate::Readable for CTRL_SPEC {}
///`write(|w| ..)` method takes [`ctrl::W`](W) writer structure
impl crate::Writable for CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CTRL to value 0
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
