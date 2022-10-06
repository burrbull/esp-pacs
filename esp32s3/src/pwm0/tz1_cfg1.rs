#[doc = "Register `TZ1_CFG1` reader"]
pub struct R(crate::R<TZ1_CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZ1_CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZ1_CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZ1_CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZ1_CFG1` writer"]
pub struct W(crate::W<TZ1_CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZ1_CFG1_SPEC>;
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
impl From<crate::W<TZ1_CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZ1_CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TZ1_CLR_OST` reader - a rising edge will clear on going one-shot mode action"]
pub type TZ1_CLR_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_CLR_OST` writer - a rising edge will clear on going one-shot mode action"]
pub type TZ1_CLR_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZ1_CFG1_SPEC, bool, O>;
#[doc = "Field `TZ1_CBCPULSE` reader - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type TZ1_CBCPULSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TZ1_CBCPULSE` writer - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
pub type TZ1_CBCPULSE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TZ1_CFG1_SPEC, u8, u8, 2, O>;
#[doc = "Field `TZ1_FORCE_CBC` reader - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ1_FORCE_CBC_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_FORCE_CBC` writer - a toggle trigger a cycle-by-cycle mode action"]
pub type TZ1_FORCE_CBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZ1_CFG1_SPEC, bool, O>;
#[doc = "Field `TZ1_FORCE_OST` reader - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ1_FORCE_OST_R = crate::BitReader<bool>;
#[doc = "Field `TZ1_FORCE_OST` writer - a toggle (software negate its value) triggers a one-shot mode action"]
pub type TZ1_FORCE_OST_W<'a, const O: u8> = crate::BitWriter<'a, u32, TZ1_CFG1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn tz1_clr_ost(&self) -> TZ1_CLR_OST_R {
        TZ1_CLR_OST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    pub fn tz1_cbcpulse(&self) -> TZ1_CBCPULSE_R {
        TZ1_CBCPULSE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn tz1_force_cbc(&self) -> TZ1_FORCE_CBC_R {
        TZ1_FORCE_CBC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn tz1_force_ost(&self) -> TZ1_FORCE_OST_R {
        TZ1_FORCE_OST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - a rising edge will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn tz1_clr_ost(&mut self) -> TZ1_CLR_OST_W<0> {
        TZ1_CLR_OST_W::new(self)
    }
    #[doc = "Bits 1:2 - cycle-by-cycle mode action refresh moment selection. Bit0: TEZ, bit1:TEP"]
    #[inline(always)]
    pub fn tz1_cbcpulse(&mut self) -> TZ1_CBCPULSE_W<1> {
        TZ1_CBCPULSE_W::new(self)
    }
    #[doc = "Bit 3 - a toggle trigger a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn tz1_force_cbc(&mut self) -> TZ1_FORCE_CBC_W<3> {
        TZ1_FORCE_CBC_W::new(self)
    }
    #[doc = "Bit 4 - a toggle (software negate its value) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn tz1_force_ost(&mut self) -> TZ1_FORCE_OST_W<4> {
        TZ1_FORCE_OST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software triggers for fault handler actions\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tz1_cfg1](index.html) module"]
pub struct TZ1_CFG1_SPEC;
impl crate::RegisterSpec for TZ1_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tz1_cfg1::R](R) reader structure"]
impl crate::Readable for TZ1_CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tz1_cfg1::W](W) writer structure"]
impl crate::Writable for TZ1_CFG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZ1_CFG1 to value 0"]
impl crate::Resettable for TZ1_CFG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}