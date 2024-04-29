#[doc = "Register `STATE_W1` reader"]
pub type R = crate::R<STATE_W1_SPEC>;
#[doc = "Field `STATE4` reader - "]
pub type STATE4_R = crate::FieldReader;
#[doc = "Field `STATE5` reader - "]
pub type STATE5_R = crate::FieldReader;
#[doc = "Field `STATE6` reader - "]
pub type STATE6_R = crate::FieldReader;
#[doc = "Field `STATE7` reader - "]
pub type STATE7_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn state4(&self) -> STATE4_R {
        STATE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn state5(&self) -> STATE5_R {
        STATE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn state6(&self) -> STATE6_R {
        STATE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn state7(&self) -> STATE7_R {
        STATE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE_W1")
            .field("state4", &format_args!("{}", self.state4().bits()))
            .field("state5", &format_args!("{}", self.state5().bits()))
            .field("state6", &format_args!("{}", self.state6().bits()))
            .field("state7", &format_args!("{}", self.state7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE_W1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state_w1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_W1_SPEC;
impl crate::RegisterSpec for STATE_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state_w1::R`](R) reader structure"]
impl crate::Readable for STATE_W1_SPEC {}
#[doc = "`reset()` method sets STATE_W1 to value 0"]
impl crate::Resettable for STATE_W1_SPEC {
    const RESET_VALUE: u32 = 0;
}
