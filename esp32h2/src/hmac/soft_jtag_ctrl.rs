#[doc = "Register `SOFT_JTAG_CTRL` writer"]
pub struct W(crate::W<SOFT_JTAG_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOFT_JTAG_CTRL_SPEC>;
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
impl From<crate::W<SOFT_JTAG_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOFT_JTAG_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOFT_JTAG_CTRL` writer - Turn on JTAG verification."]
pub type SOFT_JTAG_CTRL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SOFT_JTAG_CTRL_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Turn on JTAG verification."]
    #[inline(always)]
    #[must_use]
    pub fn soft_jtag_ctrl(&mut self) -> SOFT_JTAG_CTRL_W<0> {
        SOFT_JTAG_CTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Jtag register 0.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [soft_jtag_ctrl](index.html) module"]
pub struct SOFT_JTAG_CTRL_SPEC;
impl crate::RegisterSpec for SOFT_JTAG_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [soft_jtag_ctrl::W](W) writer structure"]
impl crate::Writable for SOFT_JTAG_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOFT_JTAG_CTRL to value 0"]
impl crate::Resettable for SOFT_JTAG_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}