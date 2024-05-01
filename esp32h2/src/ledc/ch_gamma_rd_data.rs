///Register `CH%s_GAMMA_RD_DATA` reader
pub type R = crate::R<CH_GAMMA_RD_DATA_SPEC>;
///Field `CH_GAMMA_RD_DATA` reader - Ledc ch%s gamma ram read data.
pub type CH_GAMMA_RD_DATA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:30 - Ledc ch%s gamma ram read data.
    #[inline(always)]
    pub fn ch_gamma_rd_data(&self) -> CH_GAMMA_RD_DATA_R {
        CH_GAMMA_RD_DATA_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_GAMMA_RD_DATA")
            .field("ch_gamma_rd_data", &self.ch_gamma_rd_data())
            .finish()
    }
}
/**Ledc ch%s gamma ram read data register.

You can [`read`](crate::generic::Reg::read) this register and get [`ch_gamma_rd_data::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CH_GAMMA_RD_DATA_SPEC;
impl crate::RegisterSpec for CH_GAMMA_RD_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ch_gamma_rd_data::R`](R) reader structure
impl crate::Readable for CH_GAMMA_RD_DATA_SPEC {}
///`reset()` method sets CH%s_GAMMA_RD_DATA to value 0
impl crate::Resettable for CH_GAMMA_RD_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
