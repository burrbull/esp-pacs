#[doc = "Register `LP_I2S_TXCLK_DIV_XYZ` reader"]
pub type R = crate::R<LP_I2S_TXCLK_DIV_XYZ_SPEC>;
#[doc = "Register `LP_I2S_TXCLK_DIV_XYZ` writer"]
pub type W = crate::W<LP_I2S_TXCLK_DIV_XYZ_SPEC>;
#[doc = "Field `YN1` reader - need_des"]
pub type YN1_R = crate::BitReader;
#[doc = "Field `YN1` writer - need_des"]
pub type YN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Z` reader - need_des"]
pub type Z_R = crate::FieldReader<u16>;
#[doc = "Field `Z` writer - need_des"]
pub type Z_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `Y` reader - need_des"]
pub type Y_R = crate::FieldReader<u16>;
#[doc = "Field `Y` writer - need_des"]
pub type Y_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `X` reader - need_des"]
pub type X_R = crate::FieldReader<u16>;
#[doc = "Field `X` writer - need_des"]
pub type X_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn yn1(&self) -> YN1_R {
        YN1_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    pub fn z(&self) -> Z_R {
        Z_R::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    #[doc = "Bits 14:22 - need_des"]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new(((self.bits >> 14) & 0x01ff) as u16)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_I2S_TXCLK_DIV_XYZ")
            .field("yn1", &format_args!("{}", self.yn1().bit()))
            .field("z", &format_args!("{}", self.z().bits()))
            .field("y", &format_args!("{}", self.y().bits()))
            .field("x", &format_args!("{}", self.x().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn yn1(&mut self) -> YN1_W<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
        YN1_W::new(self, 4)
    }
    #[doc = "Bits 5:13 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn z(&mut self) -> Z_W<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
        Z_W::new(self, 5)
    }
    #[doc = "Bits 14:22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> Y_W<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
        Y_W::new(self, 14)
    }
    #[doc = "Bits 23:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<LP_I2S_TXCLK_DIV_XYZ_SPEC> {
        X_W::new(self, 23)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_i2s_txclk_div_xyz::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_i2s_txclk_div_xyz::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_I2S_TXCLK_DIV_XYZ_SPEC;
impl crate::RegisterSpec for LP_I2S_TXCLK_DIV_XYZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_i2s_txclk_div_xyz::R`](R) reader structure"]
impl crate::Readable for LP_I2S_TXCLK_DIV_XYZ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_i2s_txclk_div_xyz::W`](W) writer structure"]
impl crate::Writable for LP_I2S_TXCLK_DIV_XYZ_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_I2S_TXCLK_DIV_XYZ to value 0x4000"]
impl crate::Resettable for LP_I2S_TXCLK_DIV_XYZ_SPEC {
    const RESET_VALUE: u32 = 0x4000;
}
