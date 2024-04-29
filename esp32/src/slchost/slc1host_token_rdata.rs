#[doc = "Register `SLC1HOST_TOKEN_RDATA` reader"]
pub type R = crate::R<SLC1HOST_TOKEN_RDATA_SPEC>;
#[doc = "Field `SLC1_TOKEN0` reader - "]
pub type SLC1_TOKEN0_R = crate::FieldReader<u16>;
#[doc = "Field `SLC1_RX_PF_VALID` reader - "]
pub type SLC1_RX_PF_VALID_R = crate::BitReader;
#[doc = "Field `HOSTSLC1_TOKEN1` reader - "]
pub type HOSTSLC1_TOKEN1_R = crate::FieldReader<u16>;
#[doc = "Field `SLC1_RX_PF_EOF` reader - "]
pub type SLC1_RX_PF_EOF_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn slc1_token0(&self) -> SLC1_TOKEN0_R {
        SLC1_TOKEN0_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc1_rx_pf_valid(&self) -> SLC1_RX_PF_VALID_R {
        SLC1_RX_PF_VALID_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:27"]
    #[inline(always)]
    pub fn hostslc1_token1(&self) -> HOSTSLC1_TOKEN1_R {
        HOSTSLC1_TOKEN1_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn slc1_rx_pf_eof(&self) -> SLC1_RX_PF_EOF_R {
        SLC1_RX_PF_EOF_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC1HOST_TOKEN_RDATA")
            .field(
                "slc1_token0",
                &format_args!("{}", self.slc1_token0().bits()),
            )
            .field(
                "slc1_rx_pf_valid",
                &format_args!("{}", self.slc1_rx_pf_valid().bit()),
            )
            .field(
                "hostslc1_token1",
                &format_args!("{}", self.hostslc1_token1().bits()),
            )
            .field(
                "slc1_rx_pf_eof",
                &format_args!("{}", self.slc1_rx_pf_eof().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLC1HOST_TOKEN_RDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slc1host_token_rdata::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLC1HOST_TOKEN_RDATA_SPEC;
impl crate::RegisterSpec for SLC1HOST_TOKEN_RDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slc1host_token_rdata::R`](R) reader structure"]
impl crate::Readable for SLC1HOST_TOKEN_RDATA_SPEC {}
#[doc = "`reset()` method sets SLC1HOST_TOKEN_RDATA to value 0"]
impl crate::Resettable for SLC1HOST_TOKEN_RDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
