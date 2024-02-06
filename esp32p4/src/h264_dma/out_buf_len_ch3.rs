#[doc = "Register `OUT_BUF_LEN_CH3` reader"]
pub type R = crate::R<OUT_BUF_LEN_CH3_SPEC>;
#[doc = "Field `OUT_CMDFIFO_BUF_LEN_HB_CH3` reader - only for debug"]
pub type OUT_CMDFIFO_BUF_LEN_HB_CH3_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - only for debug"]
    #[inline(always)]
    pub fn out_cmdfifo_buf_len_hb_ch3(&self) -> OUT_CMDFIFO_BUF_LEN_HB_CH3_R {
        OUT_CMDFIFO_BUF_LEN_HB_CH3_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_BUF_LEN_CH3")
            .field(
                "out_cmdfifo_buf_len_hb_ch3",
                &format_args!("{}", self.out_cmdfifo_buf_len_hb_ch3().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_BUF_LEN_CH3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "tx CH3 buf len register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_buf_len_ch3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_BUF_LEN_CH3_SPEC;
impl crate::RegisterSpec for OUT_BUF_LEN_CH3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_buf_len_ch3::R`](R) reader structure"]
impl crate::Readable for OUT_BUF_LEN_CH3_SPEC {}
#[doc = "`reset()` method sets OUT_BUF_LEN_CH3 to value 0"]
impl crate::Resettable for OUT_BUF_LEN_CH3_SPEC {
    const RESET_VALUE: u32 = 0;
}
