///Register `BBPD_CTRL` reader
pub type R = crate::R<BBPD_CTRL_SPEC>;
///Register `BBPD_CTRL` writer
pub type W = crate::W<BBPD_CTRL_SPEC>;
///Field `DC_EST_FORCE_PD` reader -
pub type DC_EST_FORCE_PD_R = crate::BitReader;
///Field `DC_EST_FORCE_PD` writer -
pub type DC_EST_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DC_EST_FORCE_PU` reader -
pub type DC_EST_FORCE_PU_R = crate::BitReader;
///Field `DC_EST_FORCE_PU` writer -
pub type DC_EST_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FFT_FORCE_PD` reader -
pub type FFT_FORCE_PD_R = crate::BitReader;
///Field `FFT_FORCE_PD` writer -
pub type FFT_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FFT_FORCE_PU` reader -
pub type FFT_FORCE_PU_R = crate::BitReader;
///Field `FFT_FORCE_PU` writer -
pub type FFT_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0
    #[inline(always)]
    pub fn dc_est_force_pd(&self) -> DC_EST_FORCE_PD_R {
        DC_EST_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1
    #[inline(always)]
    pub fn dc_est_force_pu(&self) -> DC_EST_FORCE_PU_R {
        DC_EST_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2
    #[inline(always)]
    pub fn fft_force_pd(&self) -> FFT_FORCE_PD_R {
        FFT_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3
    #[inline(always)]
    pub fn fft_force_pu(&self) -> FFT_FORCE_PU_R {
        FFT_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BBPD_CTRL")
            .field("dc_est_force_pd", &self.dc_est_force_pd())
            .field("dc_est_force_pu", &self.dc_est_force_pu())
            .field("fft_force_pd", &self.fft_force_pd())
            .field("fft_force_pu", &self.fft_force_pu())
            .finish()
    }
}
impl W {
    ///Bit 0
    #[inline(always)]
    #[must_use]
    pub fn dc_est_force_pd(&mut self) -> DC_EST_FORCE_PD_W<BBPD_CTRL_SPEC> {
        DC_EST_FORCE_PD_W::new(self, 0)
    }
    ///Bit 1
    #[inline(always)]
    #[must_use]
    pub fn dc_est_force_pu(&mut self) -> DC_EST_FORCE_PU_W<BBPD_CTRL_SPEC> {
        DC_EST_FORCE_PU_W::new(self, 1)
    }
    ///Bit 2
    #[inline(always)]
    #[must_use]
    pub fn fft_force_pd(&mut self) -> FFT_FORCE_PD_W<BBPD_CTRL_SPEC> {
        FFT_FORCE_PD_W::new(self, 2)
    }
    ///Bit 3
    #[inline(always)]
    #[must_use]
    pub fn fft_force_pu(&mut self) -> FFT_FORCE_PU_W<BBPD_CTRL_SPEC> {
        FFT_FORCE_PU_W::new(self, 3)
    }
}
/**Baseband control register

You can [`read`](crate::generic::Reg::read) this register and get [`bbpd_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bbpd_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BBPD_CTRL_SPEC;
impl crate::RegisterSpec for BBPD_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bbpd_ctrl::R`](R) reader structure
impl crate::Readable for BBPD_CTRL_SPEC {}
///`write(|w| ..)` method takes [`bbpd_ctrl::W`](W) writer structure
impl crate::Writable for BBPD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BBPD_CTRL to value 0
impl crate::Resettable for BBPD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
