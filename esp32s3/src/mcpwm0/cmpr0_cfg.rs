#[doc = "Register `CMPR0_CFG` reader"]
pub type R = crate::R<CMPR0_CFG_SPEC>;
#[doc = "Register `CMPR0_CFG` writer"]
pub type W = crate::W<CMPR0_CFG_SPEC>;
#[doc = "Field `CMPR0_A_UPMETHOD` reader - Update method for PWM generator 0 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub type CMPR0_A_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `CMPR0_A_UPMETHOD` writer - Update method for PWM generator 0 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub type CMPR0_A_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMPR0_B_UPMETHOD` reader - Update method for PWM generator 0 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub type CMPR0_B_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `CMPR0_B_UPMETHOD` writer - Update method for PWM generator 0 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
pub type CMPR0_B_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CMPR0_A_SHDW_FULL` reader - Set and reset by hardware. If set, PWM generator 0 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
pub type CMPR0_A_SHDW_FULL_R = crate::BitReader;
#[doc = "Field `CMPR0_A_SHDW_FULL` writer - Set and reset by hardware. If set, PWM generator 0 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
pub type CMPR0_A_SHDW_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPR0_B_SHDW_FULL` reader - Set and reset by hardware. If set, PWM generator 0 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
pub type CMPR0_B_SHDW_FULL_R = crate::BitReader;
#[doc = "Field `CMPR0_B_SHDW_FULL` writer - Set and reset by hardware. If set, PWM generator 0 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
pub type CMPR0_B_SHDW_FULL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Update method for PWM generator 0 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    pub fn cmpr0_a_upmethod(&self) -> CMPR0_A_UPMETHOD_R {
        CMPR0_A_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Update method for PWM generator 0 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    pub fn cmpr0_b_upmethod(&self) -> CMPR0_B_UPMETHOD_R {
        CMPR0_B_UPMETHOD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Set and reset by hardware. If set, PWM generator 0 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    pub fn cmpr0_a_shdw_full(&self) -> CMPR0_A_SHDW_FULL_R {
        CMPR0_A_SHDW_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set and reset by hardware. If set, PWM generator 0 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    pub fn cmpr0_b_shdw_full(&self) -> CMPR0_B_SHDW_FULL_R {
        CMPR0_B_SHDW_FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMPR0_CFG")
            .field(
                "cmpr0_a_upmethod",
                &format_args!("{}", self.cmpr0_a_upmethod().bits()),
            )
            .field(
                "cmpr0_b_upmethod",
                &format_args!("{}", self.cmpr0_b_upmethod().bits()),
            )
            .field(
                "cmpr0_a_shdw_full",
                &format_args!("{}", self.cmpr0_a_shdw_full().bit()),
            )
            .field(
                "cmpr0_b_shdw_full",
                &format_args!("{}", self.cmpr0_b_shdw_full().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMPR0_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Update method for PWM generator 0 time stamp A's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_a_upmethod(&mut self) -> CMPR0_A_UPMETHOD_W<CMPR0_CFG_SPEC> {
        CMPR0_A_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Update method for PWM generator 0 time stamp B's active register. When all bits are set to 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1: TEP,when bit2 is set to 1: sync, when bit3 is set to 1: disable the update."]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_b_upmethod(&mut self) -> CMPR0_B_UPMETHOD_W<CMPR0_CFG_SPEC> {
        CMPR0_B_UPMETHOD_W::new(self, 4)
    }
    #[doc = "Bit 8 - Set and reset by hardware. If set, PWM generator 0 time stamp A's shadow reg is filled and waiting to be transferred to A's active reg. If cleared, A's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_a_shdw_full(&mut self) -> CMPR0_A_SHDW_FULL_W<CMPR0_CFG_SPEC> {
        CMPR0_A_SHDW_FULL_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set and reset by hardware. If set, PWM generator 0 time stamp B's shadow reg is filled and waiting to be transferred to B's active reg. If cleared, B's active reg has been updated with shadow register latest value"]
    #[inline(always)]
    #[must_use]
    pub fn cmpr0_b_shdw_full(&mut self) -> CMPR0_B_SHDW_FULL_W<CMPR0_CFG_SPEC> {
        CMPR0_B_SHDW_FULL_W::new(self, 9)
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
#[doc = "Transfer status and update method for time stamp registers A and B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmpr0_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmpr0_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMPR0_CFG_SPEC;
impl crate::RegisterSpec for CMPR0_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpr0_cfg::R`](R) reader structure"]
impl crate::Readable for CMPR0_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cmpr0_cfg::W`](W) writer structure"]
impl crate::Writable for CMPR0_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMPR0_CFG to value 0"]
impl crate::Resettable for CMPR0_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
