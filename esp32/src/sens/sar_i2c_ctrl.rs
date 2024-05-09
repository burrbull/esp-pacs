#[doc = "Register `SAR_I2C_CTRL` reader"]
pub type R = crate::R<SAR_I2C_CTRL_SPEC>;
#[doc = "Register `SAR_I2C_CTRL` writer"]
pub type W = crate::W<SAR_I2C_CTRL_SPEC>;
#[doc = "Field `SAR_I2C_CTRL` reader - I2C control data only active when reg_sar_i2c_start_force = 1"]
pub type SAR_I2C_CTRL_R = crate::FieldReader<u32>;
#[doc = "Field `SAR_I2C_CTRL` writer - I2C control data only active when reg_sar_i2c_start_force = 1"]
pub type SAR_I2C_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `SAR_I2C_START` reader - start I2C only active when reg_sar_i2c_start_force = 1"]
pub type SAR_I2C_START_R = crate::BitReader;
#[doc = "Field `SAR_I2C_START` writer - start I2C only active when reg_sar_i2c_start_force = 1"]
pub type SAR_I2C_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAR_I2C_START_FORCE` reader - 1: I2C started by SW 0: I2C started by FSM"]
pub type SAR_I2C_START_FORCE_R = crate::BitReader;
#[doc = "Field `SAR_I2C_START_FORCE` writer - 1: I2C started by SW 0: I2C started by FSM"]
pub type SAR_I2C_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_ctrl(&self) -> SAR_I2C_CTRL_R {
        SAR_I2C_CTRL_R::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    pub fn sar_i2c_start(&self) -> SAR_I2C_START_R {
        SAR_I2C_START_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: I2C started by SW 0: I2C started by FSM"]
    #[inline(always)]
    pub fn sar_i2c_start_force(&self) -> SAR_I2C_START_FORCE_R {
        SAR_I2C_START_FORCE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_I2C_CTRL")
            .field("sar_i2c_ctrl", &self.sar_i2c_ctrl().bits())
            .field("sar_i2c_start", &self.sar_i2c_start().bit())
            .field("sar_i2c_start_force", &self.sar_i2c_start_force().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_I2C_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - I2C control data only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_ctrl(&mut self) -> SAR_I2C_CTRL_W<SAR_I2C_CTRL_SPEC> {
        SAR_I2C_CTRL_W::new(self, 0)
    }
    #[doc = "Bit 28 - start I2C only active when reg_sar_i2c_start_force = 1"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_start(&mut self) -> SAR_I2C_START_W<SAR_I2C_CTRL_SPEC> {
        SAR_I2C_START_W::new(self, 28)
    }
    #[doc = "Bit 29 - 1: I2C started by SW 0: I2C started by FSM"]
    #[inline(always)]
    #[must_use]
    pub fn sar_i2c_start_force(&mut self) -> SAR_I2C_START_FORCE_W<SAR_I2C_CTRL_SPEC> {
        SAR_I2C_START_FORCE_W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_i2c_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_i2c_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_I2C_CTRL_SPEC;
impl crate::RegisterSpec for SAR_I2C_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_i2c_ctrl::R`](R) reader structure"]
impl crate::Readable for SAR_I2C_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_i2c_ctrl::W`](W) writer structure"]
impl crate::Writable for SAR_I2C_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR_I2C_CTRL to value 0"]
impl crate::Resettable for SAR_I2C_CTRL_SPEC {}
