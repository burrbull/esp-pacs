#[doc = "Register `ROM_CTRL_1` reader"]
pub type R = crate::R<ROM_CTRL_1_SPEC>;
#[doc = "Register `ROM_CTRL_1` writer"]
pub type W = crate::W<ROM_CTRL_1_SPEC>;
#[doc = "Field `ROM_FORCE_PD` reader - This field is used to power down internal ROM."]
pub type ROM_FORCE_PD_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PD` writer - This field is used to power down internal ROM."]
pub type ROM_FORCE_PD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ROM_FORCE_PU` reader - This field is used to power up internal ROM."]
pub type ROM_FORCE_PU_R = crate::FieldReader;
#[doc = "Field `ROM_FORCE_PU` writer - This field is used to power up internal ROM."]
pub type ROM_FORCE_PU_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - This field is used to power down internal ROM."]
    #[inline(always)]
    pub fn rom_force_pd(&self) -> ROM_FORCE_PD_R {
        ROM_FORCE_PD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - This field is used to power up internal ROM."]
    #[inline(always)]
    pub fn rom_force_pu(&self) -> ROM_FORCE_PU_R {
        ROM_FORCE_PU_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ROM_CTRL_1")
            .field(
                "rom_force_pd",
                &format_args!("{}", self.rom_force_pd().bits()),
            )
            .field(
                "rom_force_pu",
                &format_args!("{}", self.rom_force_pu().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ROM_CTRL_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - This field is used to power down internal ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom_force_pd(&mut self) -> ROM_FORCE_PD_W<ROM_CTRL_1_SPEC> {
        ROM_FORCE_PD_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - This field is used to power up internal ROM."]
    #[inline(always)]
    #[must_use]
    pub fn rom_force_pu(&mut self) -> ROM_FORCE_PU_W<ROM_CTRL_1_SPEC> {
        ROM_FORCE_PU_W::new(self, 2)
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
#[doc = "System ROM configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rom_ctrl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rom_ctrl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ROM_CTRL_1_SPEC;
impl crate::RegisterSpec for ROM_CTRL_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rom_ctrl_1::R`](R) reader structure"]
impl crate::Readable for ROM_CTRL_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rom_ctrl_1::W`](W) writer structure"]
impl crate::Writable for ROM_CTRL_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ROM_CTRL_1 to value 0x0c"]
impl crate::Resettable for ROM_CTRL_1_SPEC {
    const RESET_VALUE: u32 = 0x0c;
}
