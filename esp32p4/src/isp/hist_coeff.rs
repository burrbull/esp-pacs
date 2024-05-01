///Register `HIST_COEFF` reader
pub type R = crate::R<HIST_COEFF_SPEC>;
///Register `HIST_COEFF` writer
pub type W = crate::W<HIST_COEFF_SPEC>;
///Field `B` reader - this field configures coefficient of B when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
pub type B_R = crate::FieldReader;
///Field `B` writer - this field configures coefficient of B when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `G` reader - this field configures coefficient of G when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
pub type G_R = crate::FieldReader;
///Field `G` writer - this field configures coefficient of G when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
pub type G_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `R` reader - this field configures coefficient of R when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
pub type R_R = crate::FieldReader;
///Field `R` writer - this field configures coefficient of R when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
pub type R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - this field configures coefficient of B when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - this field configures coefficient of G when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - this field configures coefficient of R when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_COEFF")
            .field("b", &self.b())
            .field("g", &self.g())
            .field("r", &self.r())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - this field configures coefficient of B when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
    #[inline(always)]
    #[must_use]
    pub fn b(&mut self) -> B_W<HIST_COEFF_SPEC> {
        B_W::new(self, 0)
    }
    ///Bits 8:15 - this field configures coefficient of G when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
    #[inline(always)]
    #[must_use]
    pub fn g(&mut self) -> G_W<HIST_COEFF_SPEC> {
        G_W::new(self, 8)
    }
    ///Bits 16:23 - this field configures coefficient of R when set hist_mode to RGB, sum of coeff_r and coeff_g and coeff_b should be 256
    #[inline(always)]
    #[must_use]
    pub fn r(&mut self) -> R_W<HIST_COEFF_SPEC> {
        R_W::new(self, 16)
    }
}
/**histogram rgb to gray coefficients register

You can [`read`](crate::generic::Reg::read) this register and get [`hist_coeff::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hist_coeff::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HIST_COEFF_SPEC;
impl crate::RegisterSpec for HIST_COEFF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hist_coeff::R`](R) reader structure
impl crate::Readable for HIST_COEFF_SPEC {}
///`write(|w| ..)` method takes [`hist_coeff::W`](W) writer structure
impl crate::Writable for HIST_COEFF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HIST_COEFF to value 0x0055_5555
impl crate::Resettable for HIST_COEFF_SPEC {
    const RESET_VALUE: u32 = 0x0055_5555;
}
