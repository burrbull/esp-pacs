#[doc = "Register `STATE_W0` reader"]
pub type R = crate::R<STATE_W0_SPEC>;
#[doc = "Field `STATE0` reader - *******Description***********"]
pub type STATE0_R = crate::FieldReader;
#[doc = "Field `STATE1` reader - *******Description***********"]
pub type STATE1_R = crate::FieldReader;
#[doc = "Field `STATE2` reader - *******Description***********"]
pub type STATE2_R = crate::FieldReader;
#[doc = "Field `STATE3` reader - *******Description***********"]
pub type STATE3_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - *******Description***********"]
    #[inline(always)]
    pub fn state0(&self) -> STATE0_R {
        STATE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - *******Description***********"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - *******Description***********"]
    #[inline(always)]
    pub fn state2(&self) -> STATE2_R {
        STATE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - *******Description***********"]
    #[inline(always)]
    pub fn state3(&self) -> STATE3_R {
        STATE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE_W0")
            .field("state0", &format_args!("{}", self.state0().bits()))
            .field("state1", &format_args!("{}", self.state1().bits()))
            .field("state2", &format_args!("{}", self.state2().bits()))
            .field("state3", &format_args!("{}", self.state3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE_W0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state_w0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE_W0_SPEC;
impl crate::RegisterSpec for STATE_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state_w0::R`](R) reader structure"]
impl crate::Readable for STATE_W0_SPEC {}
#[doc = "`reset()` method sets STATE_W0 to value 0"]
impl crate::Resettable for STATE_W0_SPEC {
    const RESET_VALUE: u32 = 0;
}
