///Register `TOUCH_SCAN_CTRL` reader
pub type R = crate::R<TOUCH_SCAN_CTRL_SPEC>;
///Register `TOUCH_SCAN_CTRL` writer
pub type W = crate::W<TOUCH_SCAN_CTRL_SPEC>;
///Field `TOUCH_DENOISE_RES` reader - Denoise resolution. 0: 12-bit; 1: 10-bit; 2: 8-bit; 3: 4-bit.
pub type TOUCH_DENOISE_RES_R = crate::FieldReader;
///Field `TOUCH_DENOISE_RES` writer - Denoise resolution. 0: 12-bit; 1: 10-bit; 2: 8-bit; 3: 4-bit.
pub type TOUCH_DENOISE_RES_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TOUCH_DENOISE_EN` reader - Touch pad 0 will be used to denoise.
pub type TOUCH_DENOISE_EN_R = crate::BitReader;
///Field `TOUCH_DENOISE_EN` writer - Touch pad 0 will be used to denoise.
pub type TOUCH_DENOISE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_INACTIVE_CONNECTION` reader - Inactive touch pads connect to 0: HighZ, 1: GND.
pub type TOUCH_INACTIVE_CONNECTION_R = crate::BitReader;
///Field `TOUCH_INACTIVE_CONNECTION` writer - Inactive touch pads connect to 0: HighZ, 1: GND.
pub type TOUCH_INACTIVE_CONNECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_SHIELD_PAD_EN` reader - Touch pad 14 will be used as shield_pad.
pub type TOUCH_SHIELD_PAD_EN_R = crate::BitReader;
///Field `TOUCH_SHIELD_PAD_EN` writer - Touch pad 14 will be used as shield_pad.
pub type TOUCH_SHIELD_PAD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TOUCH_SCAN_PAD_MAP` reader - Pad enable map for touch scan mode.
pub type TOUCH_SCAN_PAD_MAP_R = crate::FieldReader<u16>;
///Field `TOUCH_SCAN_PAD_MAP` writer - Pad enable map for touch scan mode.
pub type TOUCH_SCAN_PAD_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
///Field `TOUCH_BUFDRV` reader - Touch 14 buffer driver strength.
pub type TOUCH_BUFDRV_R = crate::FieldReader;
///Field `TOUCH_BUFDRV` writer - Touch 14 buffer driver strength.
pub type TOUCH_BUFDRV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `TOUCH_OUT_RING` reader - Select out one pad as guard_ring.
pub type TOUCH_OUT_RING_R = crate::FieldReader;
///Field `TOUCH_OUT_RING` writer - Select out one pad as guard_ring.
pub type TOUCH_OUT_RING_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Denoise resolution. 0: 12-bit; 1: 10-bit; 2: 8-bit; 3: 4-bit.
    #[inline(always)]
    pub fn touch_denoise_res(&self) -> TOUCH_DENOISE_RES_R {
        TOUCH_DENOISE_RES_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - Touch pad 0 will be used to denoise.
    #[inline(always)]
    pub fn touch_denoise_en(&self) -> TOUCH_DENOISE_EN_R {
        TOUCH_DENOISE_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 8 - Inactive touch pads connect to 0: HighZ, 1: GND.
    #[inline(always)]
    pub fn touch_inactive_connection(&self) -> TOUCH_INACTIVE_CONNECTION_R {
        TOUCH_INACTIVE_CONNECTION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Touch pad 14 will be used as shield_pad.
    #[inline(always)]
    pub fn touch_shield_pad_en(&self) -> TOUCH_SHIELD_PAD_EN_R {
        TOUCH_SHIELD_PAD_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:24 - Pad enable map for touch scan mode.
    #[inline(always)]
    pub fn touch_scan_pad_map(&self) -> TOUCH_SCAN_PAD_MAP_R {
        TOUCH_SCAN_PAD_MAP_R::new(((self.bits >> 10) & 0x7fff) as u16)
    }
    ///Bits 25:27 - Touch 14 buffer driver strength.
    #[inline(always)]
    pub fn touch_bufdrv(&self) -> TOUCH_BUFDRV_R {
        TOUCH_BUFDRV_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bits 28:31 - Select out one pad as guard_ring.
    #[inline(always)]
    pub fn touch_out_ring(&self) -> TOUCH_OUT_RING_R {
        TOUCH_OUT_RING_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_SCAN_CTRL")
            .field("touch_denoise_res", &self.touch_denoise_res())
            .field("touch_denoise_en", &self.touch_denoise_en())
            .field(
                "touch_inactive_connection",
                &self.touch_inactive_connection(),
            )
            .field("touch_shield_pad_en", &self.touch_shield_pad_en())
            .field("touch_scan_pad_map", &self.touch_scan_pad_map())
            .field("touch_bufdrv", &self.touch_bufdrv())
            .field("touch_out_ring", &self.touch_out_ring())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Denoise resolution. 0: 12-bit; 1: 10-bit; 2: 8-bit; 3: 4-bit.
    #[inline(always)]
    #[must_use]
    pub fn touch_denoise_res(&mut self) -> TOUCH_DENOISE_RES_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_DENOISE_RES_W::new(self, 0)
    }
    ///Bit 2 - Touch pad 0 will be used to denoise.
    #[inline(always)]
    #[must_use]
    pub fn touch_denoise_en(&mut self) -> TOUCH_DENOISE_EN_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_DENOISE_EN_W::new(self, 2)
    }
    ///Bit 8 - Inactive touch pads connect to 0: HighZ, 1: GND.
    #[inline(always)]
    #[must_use]
    pub fn touch_inactive_connection(
        &mut self,
    ) -> TOUCH_INACTIVE_CONNECTION_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_INACTIVE_CONNECTION_W::new(self, 8)
    }
    ///Bit 9 - Touch pad 14 will be used as shield_pad.
    #[inline(always)]
    #[must_use]
    pub fn touch_shield_pad_en(&mut self) -> TOUCH_SHIELD_PAD_EN_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_SHIELD_PAD_EN_W::new(self, 9)
    }
    ///Bits 10:24 - Pad enable map for touch scan mode.
    #[inline(always)]
    #[must_use]
    pub fn touch_scan_pad_map(&mut self) -> TOUCH_SCAN_PAD_MAP_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_SCAN_PAD_MAP_W::new(self, 10)
    }
    ///Bits 25:27 - Touch 14 buffer driver strength.
    #[inline(always)]
    #[must_use]
    pub fn touch_bufdrv(&mut self) -> TOUCH_BUFDRV_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_BUFDRV_W::new(self, 25)
    }
    ///Bits 28:31 - Select out one pad as guard_ring.
    #[inline(always)]
    #[must_use]
    pub fn touch_out_ring(&mut self) -> TOUCH_OUT_RING_W<TOUCH_SCAN_CTRL_SPEC> {
        TOUCH_OUT_RING_W::new(self, 28)
    }
}
/**Configure touch scan settings

You can [`read`](crate::generic::Reg::read) this register and get [`touch_scan_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_scan_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TOUCH_SCAN_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_SCAN_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`touch_scan_ctrl::R`](R) reader structure
impl crate::Readable for TOUCH_SCAN_CTRL_SPEC {}
///`write(|w| ..)` method takes [`touch_scan_ctrl::W`](W) writer structure
impl crate::Writable for TOUCH_SCAN_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOUCH_SCAN_CTRL to value 0xf000_0102
impl crate::Resettable for TOUCH_SCAN_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xf000_0102;
}
