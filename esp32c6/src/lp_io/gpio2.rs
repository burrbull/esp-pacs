#[doc = "Register `GPIO2` reader"]
pub struct R(crate::R<GPIO2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPIO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPIO2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPIO2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPIO2` writer"]
pub struct W(crate::W<GPIO2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPIO2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPIO2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPIO2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_GPIO2_MCU_OE` reader - need des"]
pub type LP_GPIO2_MCU_OE_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO2_MCU_OE` writer - need des"]
pub type LP_GPIO2_MCU_OE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO2_SPEC, bool, O>;
#[doc = "Field `LP_GPIO2_SLP_SEL` reader - need des"]
pub type LP_GPIO2_SLP_SEL_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO2_SLP_SEL` writer - need des"]
pub type LP_GPIO2_SLP_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO2_SPEC, bool, O>;
#[doc = "Field `LP_GPIO2_MCU_WPD` reader - need des"]
pub type LP_GPIO2_MCU_WPD_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO2_MCU_WPD` writer - need des"]
pub type LP_GPIO2_MCU_WPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO2_SPEC, bool, O>;
#[doc = "Field `LP_GPIO2_MCU_WPU` reader - need des"]
pub type LP_GPIO2_MCU_WPU_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO2_MCU_WPU` writer - need des"]
pub type LP_GPIO2_MCU_WPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO2_SPEC, bool, O>;
#[doc = "Field `LP_GPIO2_MCU_IE` reader - need des"]
pub type LP_GPIO2_MCU_IE_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO2_MCU_IE` writer - need des"]
pub type LP_GPIO2_MCU_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO2_SPEC, bool, O>;
#[doc = "Field `LP_GPIO2_MCU_DRV` reader - need des"]
pub type LP_GPIO2_MCU_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_GPIO2_MCU_DRV` writer - need des"]
pub type LP_GPIO2_MCU_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO2_SPEC, u8, u8, 2, O>;
#[doc = "Field `LP_GPIO2_FUN_WPD` reader - need des"]
pub type LP_GPIO2_FUN_WPD_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO2_FUN_WPD` writer - need des"]
pub type LP_GPIO2_FUN_WPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO2_SPEC, bool, O>;
#[doc = "Field `LP_GPIO2_FUN_WPU` reader - need des"]
pub type LP_GPIO2_FUN_WPU_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO2_FUN_WPU` writer - need des"]
pub type LP_GPIO2_FUN_WPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO2_SPEC, bool, O>;
#[doc = "Field `LP_GPIO2_FUN_IE` reader - need des"]
pub type LP_GPIO2_FUN_IE_R = crate::BitReader<bool>;
#[doc = "Field `LP_GPIO2_FUN_IE` writer - need des"]
pub type LP_GPIO2_FUN_IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPIO2_SPEC, bool, O>;
#[doc = "Field `LP_GPIO2_FUN_DRV` reader - need des"]
pub type LP_GPIO2_FUN_DRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_GPIO2_FUN_DRV` writer - need des"]
pub type LP_GPIO2_FUN_DRV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO2_SPEC, u8, u8, 2, O>;
#[doc = "Field `LP_GPIO2_MCU_SEL` reader - need des"]
pub type LP_GPIO2_MCU_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_GPIO2_MCU_SEL` writer - need des"]
pub type LP_GPIO2_MCU_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPIO2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_mcu_oe(&self) -> LP_GPIO2_MCU_OE_R {
        LP_GPIO2_MCU_OE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_slp_sel(&self) -> LP_GPIO2_SLP_SEL_R {
        LP_GPIO2_SLP_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_mcu_wpd(&self) -> LP_GPIO2_MCU_WPD_R {
        LP_GPIO2_MCU_WPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_mcu_wpu(&self) -> LP_GPIO2_MCU_WPU_R {
        LP_GPIO2_MCU_WPU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_mcu_ie(&self) -> LP_GPIO2_MCU_IE_R {
        LP_GPIO2_MCU_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_mcu_drv(&self) -> LP_GPIO2_MCU_DRV_R {
        LP_GPIO2_MCU_DRV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_fun_wpd(&self) -> LP_GPIO2_FUN_WPD_R {
        LP_GPIO2_FUN_WPD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_fun_wpu(&self) -> LP_GPIO2_FUN_WPU_R {
        LP_GPIO2_FUN_WPU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_fun_ie(&self) -> LP_GPIO2_FUN_IE_R {
        LP_GPIO2_FUN_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_fun_drv(&self) -> LP_GPIO2_FUN_DRV_R {
        LP_GPIO2_FUN_DRV_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - need des"]
    #[inline(always)]
    pub fn lp_gpio2_mcu_sel(&self) -> LP_GPIO2_MCU_SEL_R {
        LP_GPIO2_MCU_SEL_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_mcu_oe(&mut self) -> LP_GPIO2_MCU_OE_W<0> {
        LP_GPIO2_MCU_OE_W::new(self)
    }
    #[doc = "Bit 1 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_slp_sel(&mut self) -> LP_GPIO2_SLP_SEL_W<1> {
        LP_GPIO2_SLP_SEL_W::new(self)
    }
    #[doc = "Bit 2 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_mcu_wpd(&mut self) -> LP_GPIO2_MCU_WPD_W<2> {
        LP_GPIO2_MCU_WPD_W::new(self)
    }
    #[doc = "Bit 3 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_mcu_wpu(&mut self) -> LP_GPIO2_MCU_WPU_W<3> {
        LP_GPIO2_MCU_WPU_W::new(self)
    }
    #[doc = "Bit 4 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_mcu_ie(&mut self) -> LP_GPIO2_MCU_IE_W<4> {
        LP_GPIO2_MCU_IE_W::new(self)
    }
    #[doc = "Bits 5:6 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_mcu_drv(&mut self) -> LP_GPIO2_MCU_DRV_W<5> {
        LP_GPIO2_MCU_DRV_W::new(self)
    }
    #[doc = "Bit 7 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_fun_wpd(&mut self) -> LP_GPIO2_FUN_WPD_W<7> {
        LP_GPIO2_FUN_WPD_W::new(self)
    }
    #[doc = "Bit 8 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_fun_wpu(&mut self) -> LP_GPIO2_FUN_WPU_W<8> {
        LP_GPIO2_FUN_WPU_W::new(self)
    }
    #[doc = "Bit 9 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_fun_ie(&mut self) -> LP_GPIO2_FUN_IE_W<9> {
        LP_GPIO2_FUN_IE_W::new(self)
    }
    #[doc = "Bits 10:11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_fun_drv(&mut self) -> LP_GPIO2_FUN_DRV_W<10> {
        LP_GPIO2_FUN_DRV_W::new(self)
    }
    #[doc = "Bits 12:14 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_gpio2_mcu_sel(&mut self) -> LP_GPIO2_MCU_SEL_W<12> {
        LP_GPIO2_MCU_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpio2](index.html) module"]
pub struct GPIO2_SPEC;
impl crate::RegisterSpec for GPIO2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpio2::R](R) reader structure"]
impl crate::Readable for GPIO2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpio2::W](W) writer structure"]
impl crate::Writable for GPIO2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPIO2 to value 0"]
impl crate::Resettable for GPIO2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}