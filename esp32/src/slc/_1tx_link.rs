///Register `_1TX_LINK` reader
pub type R = crate::R<_1TX_LINK_SPEC>;
///Register `_1TX_LINK` writer
pub type W = crate::W<_1TX_LINK_SPEC>;
///Field `SLC1_TXLINK_ADDR` reader -
pub type SLC1_TXLINK_ADDR_R = crate::FieldReader<u32>;
///Field `SLC1_TXLINK_ADDR` writer -
pub type SLC1_TXLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
///Field `SLC1_TXLINK_STOP` reader -
pub type SLC1_TXLINK_STOP_R = crate::BitReader;
///Field `SLC1_TXLINK_STOP` writer -
pub type SLC1_TXLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC1_TXLINK_START` reader -
pub type SLC1_TXLINK_START_R = crate::BitReader;
///Field `SLC1_TXLINK_START` writer -
pub type SLC1_TXLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC1_TXLINK_RESTART` reader -
pub type SLC1_TXLINK_RESTART_R = crate::BitReader;
///Field `SLC1_TXLINK_RESTART` writer -
pub type SLC1_TXLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLC1_TXLINK_PARK` reader -
pub type SLC1_TXLINK_PARK_R = crate::BitReader;
impl R {
    ///Bits 0:19
    #[inline(always)]
    pub fn slc1_txlink_addr(&self) -> SLC1_TXLINK_ADDR_R {
        SLC1_TXLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    ///Bit 28
    #[inline(always)]
    pub fn slc1_txlink_stop(&self) -> SLC1_TXLINK_STOP_R {
        SLC1_TXLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29
    #[inline(always)]
    pub fn slc1_txlink_start(&self) -> SLC1_TXLINK_START_R {
        SLC1_TXLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30
    #[inline(always)]
    pub fn slc1_txlink_restart(&self) -> SLC1_TXLINK_RESTART_R {
        SLC1_TXLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn slc1_txlink_park(&self) -> SLC1_TXLINK_PARK_R {
        SLC1_TXLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_1TX_LINK")
            .field("slc1_txlink_addr", &self.slc1_txlink_addr())
            .field("slc1_txlink_stop", &self.slc1_txlink_stop())
            .field("slc1_txlink_start", &self.slc1_txlink_start())
            .field("slc1_txlink_restart", &self.slc1_txlink_restart())
            .field("slc1_txlink_park", &self.slc1_txlink_park())
            .finish()
    }
}
impl W {
    ///Bits 0:19
    #[inline(always)]
    #[must_use]
    pub fn slc1_txlink_addr(&mut self) -> SLC1_TXLINK_ADDR_W<_1TX_LINK_SPEC> {
        SLC1_TXLINK_ADDR_W::new(self, 0)
    }
    ///Bit 28
    #[inline(always)]
    #[must_use]
    pub fn slc1_txlink_stop(&mut self) -> SLC1_TXLINK_STOP_W<_1TX_LINK_SPEC> {
        SLC1_TXLINK_STOP_W::new(self, 28)
    }
    ///Bit 29
    #[inline(always)]
    #[must_use]
    pub fn slc1_txlink_start(&mut self) -> SLC1_TXLINK_START_W<_1TX_LINK_SPEC> {
        SLC1_TXLINK_START_W::new(self, 29)
    }
    ///Bit 30
    #[inline(always)]
    #[must_use]
    pub fn slc1_txlink_restart(&mut self) -> SLC1_TXLINK_RESTART_W<_1TX_LINK_SPEC> {
        SLC1_TXLINK_RESTART_W::new(self, 30)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_1tx_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_1tx_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _1TX_LINK_SPEC;
impl crate::RegisterSpec for _1TX_LINK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_1tx_link::R`](R) reader structure
impl crate::Readable for _1TX_LINK_SPEC {}
///`write(|w| ..)` method takes [`_1tx_link::W`](W) writer structure
impl crate::Writable for _1TX_LINK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets _1TX_LINK to value 0
impl crate::Resettable for _1TX_LINK_SPEC {
    const RESET_VALUE: u32 = 0;
}
