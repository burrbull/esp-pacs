///Register `PARL_IO_RX_INTR_MAP` reader
pub type R = crate::R<PARL_IO_RX_INTR_MAP_SPEC>;
///Register `PARL_IO_RX_INTR_MAP` writer
pub type W = crate::W<PARL_IO_RX_INTR_MAP_SPEC>;
///Field `PARL_IO_RX_INTR_MAP` reader - CORE0_PARL_IO_RX_INTR mapping register
pub type PARL_IO_RX_INTR_MAP_R = crate::FieldReader;
///Field `PARL_IO_RX_INTR_MAP` writer - CORE0_PARL_IO_RX_INTR mapping register
pub type PARL_IO_RX_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - CORE0_PARL_IO_RX_INTR mapping register
    #[inline(always)]
    pub fn parl_io_rx_intr_map(&self) -> PARL_IO_RX_INTR_MAP_R {
        PARL_IO_RX_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PARL_IO_RX_INTR_MAP")
            .field("parl_io_rx_intr_map", &self.parl_io_rx_intr_map())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - CORE0_PARL_IO_RX_INTR mapping register
    #[inline(always)]
    #[must_use]
    pub fn parl_io_rx_intr_map(&mut self) -> PARL_IO_RX_INTR_MAP_W<PARL_IO_RX_INTR_MAP_SPEC> {
        PARL_IO_RX_INTR_MAP_W::new(self, 0)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`parl_io_rx_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_io_rx_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PARL_IO_RX_INTR_MAP_SPEC;
impl crate::RegisterSpec for PARL_IO_RX_INTR_MAP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`parl_io_rx_intr_map::R`](R) reader structure
impl crate::Readable for PARL_IO_RX_INTR_MAP_SPEC {}
///`write(|w| ..)` method takes [`parl_io_rx_intr_map::W`](W) writer structure
impl crate::Writable for PARL_IO_RX_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PARL_IO_RX_INTR_MAP to value 0
impl crate::Resettable for PARL_IO_RX_INTR_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
