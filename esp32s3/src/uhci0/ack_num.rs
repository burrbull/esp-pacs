#[doc = "Register `ACK_NUM` reader"]
pub struct R(crate::R<ACK_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACK_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACK_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACK_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACK_NUM` writer"]
pub struct W(crate::W<ACK_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACK_NUM_SPEC>;
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
impl From<crate::W<ACK_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACK_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACK_NUM` reader - This ACK number used in software flow control."]
pub type ACK_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ACK_NUM` writer - This ACK number used in software flow control."]
pub type ACK_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACK_NUM_SPEC, u8, u8, 3, O>;
#[doc = "Field `LOAD` writer - Set this bit to 1, the value configured by UHCI_ACK_NUM would be loaded."]
pub type LOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACK_NUM_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - This ACK number used in software flow control."]
    #[inline(always)]
    pub fn ack_num(&self) -> ACK_NUM_R {
        ACK_NUM_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This ACK number used in software flow control."]
    #[inline(always)]
    pub fn ack_num(&mut self) -> ACK_NUM_W<0> {
        ACK_NUM_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to 1, the value configured by UHCI_ACK_NUM would be loaded."]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W<3> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UHCI ACK number configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ack_num](index.html) module"]
pub struct ACK_NUM_SPEC;
impl crate::RegisterSpec for ACK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ack_num::R](R) reader structure"]
impl crate::Readable for ACK_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ack_num::W](W) writer structure"]
impl crate::Writable for ACK_NUM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ACK_NUM to value 0x08"]
impl crate::Resettable for ACK_NUM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}