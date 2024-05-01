///Register `BROWN_OUT` reader
pub type R = crate::R<BROWN_OUT_SPEC>;
///Register `BROWN_OUT` writer
pub type W = crate::W<BROWN_OUT_SPEC>;
///Field `BROWN_OUT2_ENA` reader - Enables the brown_out2 to initiate a chip reset.
pub type BROWN_OUT2_ENA_R = crate::BitReader;
///Field `BROWN_OUT2_ENA` writer - Enables the brown_out2 to initiate a chip reset.
pub type BROWN_OUT2_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_WAIT` reader - Configures the waiting cycle before sending an interrupt.
pub type INT_WAIT_R = crate::FieldReader<u16>;
///Field `INT_WAIT` writer - Configures the waiting cycle before sending an interrupt.
pub type INT_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `CLOSE_FLASH_ENA` reader - Set this bit to enable PD the flash when a brown-out happens.
pub type CLOSE_FLASH_ENA_R = crate::BitReader;
///Field `CLOSE_FLASH_ENA` writer - Set this bit to enable PD the flash when a brown-out happens.
pub type CLOSE_FLASH_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD_RF_ENA` reader - Set this bit to enable PD the RF circuits when a brown-out happens.
pub type PD_RF_ENA_R = crate::BitReader;
///Field `PD_RF_ENA` writer - Set this bit to enable PD the RF circuits when a brown-out happens.
pub type PD_RF_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_WAIT` reader - Configures the waiting cycle before the reset after a brown-out.
pub type RST_WAIT_R = crate::FieldReader<u16>;
///Field `RST_WAIT` writer - Configures the waiting cycle before the reset after a brown-out.
pub type RST_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `RST_ENA` reader - Enables to reset brown-out.
pub type RST_ENA_R = crate::BitReader;
///Field `RST_ENA` writer - Enables to reset brown-out.
pub type RST_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RST_SEL` reader - Selects the reset type when a brown-out happens. 1: chip reset 0: system reset.
pub type RST_SEL_R = crate::BitReader;
///Field `RST_SEL` writer - Selects the reset type when a brown-out happens. 1: chip reset 0: system reset.
pub type RST_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNT_CLR` writer - Clears the brown-out counter.
pub type CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ENA` reader - Set this bit to enable brown-out detection.
pub type ENA_R = crate::BitReader;
///Field `ENA` writer - Set this bit to enable brown-out detection.
pub type ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DET` reader - Indicates the status of the brown-out signal.
pub type DET_R = crate::BitReader;
impl R {
    ///Bit 0 - Enables the brown_out2 to initiate a chip reset.
    #[inline(always)]
    pub fn brown_out2_ena(&self) -> BROWN_OUT2_ENA_R {
        BROWN_OUT2_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bits 4:13 - Configures the waiting cycle before sending an interrupt.
    #[inline(always)]
    pub fn int_wait(&self) -> INT_WAIT_R {
        INT_WAIT_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    ///Bit 14 - Set this bit to enable PD the flash when a brown-out happens.
    #[inline(always)]
    pub fn close_flash_ena(&self) -> CLOSE_FLASH_ENA_R {
        CLOSE_FLASH_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Set this bit to enable PD the RF circuits when a brown-out happens.
    #[inline(always)]
    pub fn pd_rf_ena(&self) -> PD_RF_ENA_R {
        PD_RF_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:25 - Configures the waiting cycle before the reset after a brown-out.
    #[inline(always)]
    pub fn rst_wait(&self) -> RST_WAIT_R {
        RST_WAIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    ///Bit 26 - Enables to reset brown-out.
    #[inline(always)]
    pub fn rst_ena(&self) -> RST_ENA_R {
        RST_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Selects the reset type when a brown-out happens. 1: chip reset 0: system reset.
    #[inline(always)]
    pub fn rst_sel(&self) -> RST_SEL_R {
        RST_SEL_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - Set this bit to enable brown-out detection.
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Indicates the status of the brown-out signal.
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BROWN_OUT")
            .field("brown_out2_ena", &self.brown_out2_ena())
            .field("int_wait", &self.int_wait())
            .field("close_flash_ena", &self.close_flash_ena())
            .field("pd_rf_ena", &self.pd_rf_ena())
            .field("rst_wait", &self.rst_wait())
            .field("rst_ena", &self.rst_ena())
            .field("rst_sel", &self.rst_sel())
            .field("ena", &self.ena())
            .field("det", &self.det())
            .finish()
    }
}
impl W {
    ///Bit 0 - Enables the brown_out2 to initiate a chip reset.
    #[inline(always)]
    #[must_use]
    pub fn brown_out2_ena(&mut self) -> BROWN_OUT2_ENA_W<BROWN_OUT_SPEC> {
        BROWN_OUT2_ENA_W::new(self, 0)
    }
    ///Bits 4:13 - Configures the waiting cycle before sending an interrupt.
    #[inline(always)]
    #[must_use]
    pub fn int_wait(&mut self) -> INT_WAIT_W<BROWN_OUT_SPEC> {
        INT_WAIT_W::new(self, 4)
    }
    ///Bit 14 - Set this bit to enable PD the flash when a brown-out happens.
    #[inline(always)]
    #[must_use]
    pub fn close_flash_ena(&mut self) -> CLOSE_FLASH_ENA_W<BROWN_OUT_SPEC> {
        CLOSE_FLASH_ENA_W::new(self, 14)
    }
    ///Bit 15 - Set this bit to enable PD the RF circuits when a brown-out happens.
    #[inline(always)]
    #[must_use]
    pub fn pd_rf_ena(&mut self) -> PD_RF_ENA_W<BROWN_OUT_SPEC> {
        PD_RF_ENA_W::new(self, 15)
    }
    ///Bits 16:25 - Configures the waiting cycle before the reset after a brown-out.
    #[inline(always)]
    #[must_use]
    pub fn rst_wait(&mut self) -> RST_WAIT_W<BROWN_OUT_SPEC> {
        RST_WAIT_W::new(self, 16)
    }
    ///Bit 26 - Enables to reset brown-out.
    #[inline(always)]
    #[must_use]
    pub fn rst_ena(&mut self) -> RST_ENA_W<BROWN_OUT_SPEC> {
        RST_ENA_W::new(self, 26)
    }
    ///Bit 27 - Selects the reset type when a brown-out happens. 1: chip reset 0: system reset.
    #[inline(always)]
    #[must_use]
    pub fn rst_sel(&mut self) -> RST_SEL_W<BROWN_OUT_SPEC> {
        RST_SEL_W::new(self, 27)
    }
    ///Bit 29 - Clears the brown-out counter.
    #[inline(always)]
    #[must_use]
    pub fn cnt_clr(&mut self) -> CNT_CLR_W<BROWN_OUT_SPEC> {
        CNT_CLR_W::new(self, 29)
    }
    ///Bit 30 - Set this bit to enable brown-out detection.
    #[inline(always)]
    #[must_use]
    pub fn ena(&mut self) -> ENA_W<BROWN_OUT_SPEC> {
        ENA_W::new(self, 30)
    }
}
/**Brownout configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`brown_out::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brown_out::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BROWN_OUT_SPEC;
impl crate::RegisterSpec for BROWN_OUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`brown_out::R`](R) reader structure
impl crate::Readable for BROWN_OUT_SPEC {}
///`write(|w| ..)` method takes [`brown_out::W`](W) writer structure
impl crate::Writable for BROWN_OUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BROWN_OUT to value 0x03ff_2ff1
impl crate::Resettable for BROWN_OUT_SPEC {
    const RESET_VALUE: u32 = 0x03ff_2ff1;
}
