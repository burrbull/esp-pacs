#[doc = "Register `CTRL_DATE` reader"]
pub type R = crate::R<CTRL_DATE_SPEC>;
#[doc = "Register `CTRL_DATE` writer"]
pub type W = crate::W<CTRL_DATE_SPEC>;
#[doc = "Field `CTRL_DATE` reader - need_des"]
pub type CTRL_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `CTRL_DATE` writer - need_des"]
pub type CTRL_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `CLK_EN` reader - need_des"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - need_des"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    pub fn ctrl_date(&self) -> CTRL_DATE_R {
        CTRL_DATE_R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_DATE")
            .field("ctrl_date", &format_args!("{}", self.ctrl_date().bits()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CTRL_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ctrl_date(&mut self) -> CTRL_DATE_W<CTRL_DATE_SPEC> {
        CTRL_DATE_W::new(self, 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CTRL_DATE_SPEC> {
        CLK_EN_W::new(self, 31)
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
#[doc = "Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_DATE_SPEC;
impl crate::RegisterSpec for CTRL_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_date::R`](R) reader structure"]
impl crate::Readable for CTRL_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_date::W`](W) writer structure"]
impl crate::Writable for CTRL_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_DATE to value 0x0221_2260"]
impl crate::Resettable for CTRL_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0221_2260;
}
