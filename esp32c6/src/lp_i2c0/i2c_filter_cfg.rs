#[doc = "Register `I2C_FILTER_CFG` reader"]
pub type R = crate::R<I2C_FILTER_CFG_SPEC>;
#[doc = "Register `I2C_FILTER_CFG` writer"]
pub type W = crate::W<I2C_FILTER_CFG_SPEC>;
#[doc = "Field `I2C_SCL_FILTER_THRES` reader - When a pulse on the SCL input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub type I2C_SCL_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `I2C_SCL_FILTER_THRES` writer - When a pulse on the SCL input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub type I2C_SCL_FILTER_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2C_SDA_FILTER_THRES` reader - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub type I2C_SDA_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `I2C_SDA_FILTER_THRES` writer - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
pub type I2C_SDA_FILTER_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `I2C_SCL_FILTER_EN` reader - This is the filter enable bit for SCL."]
pub type I2C_SCL_FILTER_EN_R = crate::BitReader;
#[doc = "Field `I2C_SCL_FILTER_EN` writer - This is the filter enable bit for SCL."]
pub type I2C_SCL_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C_SDA_FILTER_EN` reader - This is the filter enable bit for SDA."]
pub type I2C_SDA_FILTER_EN_R = crate::BitReader;
#[doc = "Field `I2C_SDA_FILTER_EN` writer - This is the filter enable bit for SDA."]
pub type I2C_SDA_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - When a pulse on the SCL input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    pub fn i2c_scl_filter_thres(&self) -> I2C_SCL_FILTER_THRES_R {
        I2C_SCL_FILTER_THRES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    pub fn i2c_sda_filter_thres(&self) -> I2C_SDA_FILTER_THRES_R {
        I2C_SDA_FILTER_THRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - This is the filter enable bit for SCL."]
    #[inline(always)]
    pub fn i2c_scl_filter_en(&self) -> I2C_SCL_FILTER_EN_R {
        I2C_SCL_FILTER_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This is the filter enable bit for SDA."]
    #[inline(always)]
    pub fn i2c_sda_filter_en(&self) -> I2C_SDA_FILTER_EN_R {
        I2C_SDA_FILTER_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_FILTER_CFG")
            .field(
                "i2c_scl_filter_thres",
                &format_args!("{}", self.i2c_scl_filter_thres().bits()),
            )
            .field(
                "i2c_sda_filter_thres",
                &format_args!("{}", self.i2c_sda_filter_thres().bits()),
            )
            .field(
                "i2c_scl_filter_en",
                &format_args!("{}", self.i2c_scl_filter_en().bit()),
            )
            .field(
                "i2c_sda_filter_en",
                &format_args!("{}", self.i2c_sda_filter_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2C_FILTER_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - When a pulse on the SCL input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_filter_thres(&mut self) -> I2C_SCL_FILTER_THRES_W<I2C_FILTER_CFG_SPEC> {
        I2C_SCL_FILTER_THRES_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - When a pulse on the SDA input has smaller width than this register value in I2C module clock cycles, the I2C controller will ignore that pulse."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sda_filter_thres(&mut self) -> I2C_SDA_FILTER_THRES_W<I2C_FILTER_CFG_SPEC> {
        I2C_SDA_FILTER_THRES_W::new(self, 4)
    }
    #[doc = "Bit 8 - This is the filter enable bit for SCL."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_scl_filter_en(&mut self) -> I2C_SCL_FILTER_EN_W<I2C_FILTER_CFG_SPEC> {
        I2C_SCL_FILTER_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - This is the filter enable bit for SDA."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_sda_filter_en(&mut self) -> I2C_SDA_FILTER_EN_W<I2C_FILTER_CFG_SPEC> {
        I2C_SDA_FILTER_EN_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "SCL and SDA filter configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_filter_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_filter_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_FILTER_CFG_SPEC;
impl crate::RegisterSpec for I2C_FILTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_filter_cfg::R`](R) reader structure"]
impl crate::Readable for I2C_FILTER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_filter_cfg::W`](W) writer structure"]
impl crate::Writable for I2C_FILTER_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2C_FILTER_CFG to value 0x0300"]
impl crate::Resettable for I2C_FILTER_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0300;
}
