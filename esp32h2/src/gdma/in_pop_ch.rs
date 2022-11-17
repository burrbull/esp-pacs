#[doc = "Register `IN_POP_CH%s` reader"]
pub struct R(crate::R<IN_POP_CH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IN_POP_CH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IN_POP_CH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IN_POP_CH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IN_POP_CH%s` writer"]
pub struct W(crate::W<IN_POP_CH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IN_POP_CH_SPEC>;
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
impl From<crate::W<IN_POP_CH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IN_POP_CH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INFIFO_RDATA_CH` reader - This register stores the data popping from DMA FIFO."]
pub type INFIFO_RDATA_CH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INFIFO_POP_CH` writer - Set this bit to pop data from DMA FIFO."]
pub type INFIFO_POP_CH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IN_POP_CH_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:11 - This register stores the data popping from DMA FIFO."]
    #[inline(always)]
    pub fn infifo_rdata_ch(&self) -> INFIFO_RDATA_CH_R {
        INFIFO_RDATA_CH_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 12 - Set this bit to pop data from DMA FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_pop_ch(&mut self) -> INFIFO_POP_CH_W<12> {
        INFIFO_POP_CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pop control register of Rx channel 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [in_pop_ch](index.html) module"]
pub struct IN_POP_CH_SPEC;
impl crate::RegisterSpec for IN_POP_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [in_pop_ch::R](R) reader structure"]
impl crate::Readable for IN_POP_CH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [in_pop_ch::W](W) writer structure"]
impl crate::Writable for IN_POP_CH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_POP_CH%s to value 0x0800"]
impl crate::Resettable for IN_POP_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}