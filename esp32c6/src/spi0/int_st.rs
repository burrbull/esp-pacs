#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SLV_ST_END_INT_ST` reader - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
pub type SLV_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `MST_ST_END_INT_ST` reader - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
pub type MST_ST_END_INT_ST_R = crate::BitReader;
#[doc = "Field `ECC_ERR_INT_ST` reader - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
pub type ECC_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `PMS_REJECT_INT_ST` reader - The status bit for SPI_MEM_PMS_REJECT_INT interrupt."]
pub type PMS_REJECT_INT_ST_R = crate::BitReader;
#[doc = "Field `AXI_RADDR_ERR_INT_ST` reader - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
pub type AXI_RADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `AXI_WR_FLASH_ERR_INT_ST` reader - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
pub type AXI_WR_FLASH_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `AXI_WADDR_ERR_INT_ST` reader - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
pub type AXI_WADDR_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 3 - The status bit for SPI_MEM_SLV_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn slv_st_end_int_st(&self) -> SLV_ST_END_INT_ST_R {
        SLV_ST_END_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI_MEM_MST_ST_END_INT interrupt."]
    #[inline(always)]
    pub fn mst_st_end_int_st(&self) -> MST_ST_END_INT_ST_R {
        MST_ST_END_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status bit for SPI_MEM_ECC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ecc_err_int_st(&self) -> ECC_ERR_INT_ST_R {
        ECC_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status bit for SPI_MEM_PMS_REJECT_INT interrupt."]
    #[inline(always)]
    pub fn pms_reject_int_st(&self) -> PMS_REJECT_INT_ST_R {
        PMS_REJECT_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_raddr_err_int_st(&self) -> AXI_RADDR_ERR_INT_ST_R {
        AXI_RADDR_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_wr_flash_err_int_st(&self) -> AXI_WR_FLASH_ERR_INT_ST_R {
        AXI_WR_FLASH_ERR_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn axi_waddr_err_int_st(&self) -> AXI_WADDR_ERR_INT_ST_R {
        AXI_WADDR_ERR_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "slv_st_end_int_st",
                &format_args!("{}", self.slv_st_end_int_st().bit()),
            )
            .field(
                "mst_st_end_int_st",
                &format_args!("{}", self.mst_st_end_int_st().bit()),
            )
            .field(
                "ecc_err_int_st",
                &format_args!("{}", self.ecc_err_int_st().bit()),
            )
            .field(
                "pms_reject_int_st",
                &format_args!("{}", self.pms_reject_int_st().bit()),
            )
            .field(
                "axi_raddr_err_int_st",
                &format_args!("{}", self.axi_raddr_err_int_st().bit()),
            )
            .field(
                "axi_wr_flash_err_int_st",
                &format_args!("{}", self.axi_wr_flash_err_int_st().bit()),
            )
            .field(
                "axi_waddr_err_int_st",
                &format_args!("{}", self.axi_waddr_err_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SPI0 interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
