///Register `AHB_LITE_MASK` reader
pub type R = crate::R<AHB_LITE_MASK_SPEC>;
///Register `AHB_LITE_MASK` writer
pub type W = crate::W<AHB_LITE_MASK_SPEC>;
///Field `PRO` reader -
pub type PRO_R = crate::BitReader;
///Field `PRO` writer -
pub type PRO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APP` reader -
pub type APP_R = crate::BitReader;
///Field `APP` writer -
pub type APP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO` reader -
pub type SDIO_R = crate::BitReader;
///Field `SDIO` writer -
pub type SDIO_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRODPORT` reader -
pub type PRODPORT_R = crate::BitReader;
///Field `PRODPORT` writer -
pub type PRODPORT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `APPDPORT` reader -
pub type APPDPORT_R = crate::BitReader;
///Field `APPDPORT` writer -
pub type APPDPORT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `AHB_LITE_SDHOST_PID` reader -
pub type AHB_LITE_SDHOST_PID_R = crate::FieldReader;
///Field `AHB_LITE_SDHOST_PID` writer -
pub type AHB_LITE_SDHOST_PID_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn pro(&self) -> PRO_R {
        PRO_R::new((self.bits & 1) != 0)
    }
    ///Bit 4
    #[inline(always)]
    pub fn app(&self) -> APP_R {
        APP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8
    #[inline(always)]
    pub fn sdio(&self) -> SDIO_R {
        SDIO_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9
    #[inline(always)]
    pub fn prodport(&self) -> PRODPORT_R {
        PRODPORT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10
    #[inline(always)]
    pub fn appdport(&self) -> APPDPORT_R {
        APPDPORT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:13
    #[inline(always)]
    pub fn ahb_lite_sdhost_pid(&self) -> AHB_LITE_SDHOST_PID_R {
        AHB_LITE_SDHOST_PID_R::new(((self.bits >> 11) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_LITE_MASK")
            .field("pro", &self.pro())
            .field("app", &self.app())
            .field("sdio", &self.sdio())
            .field("prodport", &self.prodport())
            .field("appdport", &self.appdport())
            .field("ahb_lite_sdhost_pid", &self.ahb_lite_sdhost_pid())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn pro(&mut self) -> PRO_W<AHB_LITE_MASK_SPEC> {
        PRO_W::new(self, 0)
    }
    ///Bit 4
    #[inline(always)]
    #[must_use]
    pub fn app(&mut self) -> APP_W<AHB_LITE_MASK_SPEC> {
        APP_W::new(self, 4)
    }
    ///Bit 8
    #[inline(always)]
    #[must_use]
    pub fn sdio(&mut self) -> SDIO_W<AHB_LITE_MASK_SPEC> {
        SDIO_W::new(self, 8)
    }
    ///Bit 9
    #[inline(always)]
    #[must_use]
    pub fn prodport(&mut self) -> PRODPORT_W<AHB_LITE_MASK_SPEC> {
        PRODPORT_W::new(self, 9)
    }
    ///Bit 10
    #[inline(always)]
    #[must_use]
    pub fn appdport(&mut self) -> APPDPORT_W<AHB_LITE_MASK_SPEC> {
        APPDPORT_W::new(self, 10)
    }
    ///Bits 11:13
    #[inline(always)]
    #[must_use]
    pub fn ahb_lite_sdhost_pid(&mut self) -> AHB_LITE_SDHOST_PID_W<AHB_LITE_MASK_SPEC> {
        AHB_LITE_SDHOST_PID_W::new(self, 11)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ahb_lite_mask::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_lite_mask::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AHB_LITE_MASK_SPEC;
impl crate::RegisterSpec for AHB_LITE_MASK_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ahb_lite_mask::R`](R) reader structure
impl crate::Readable for AHB_LITE_MASK_SPEC {}
///`write(|w| ..)` method takes [`ahb_lite_mask::W`](W) writer structure
impl crate::Writable for AHB_LITE_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB_LITE_MASK to value 0
impl crate::Resettable for AHB_LITE_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
