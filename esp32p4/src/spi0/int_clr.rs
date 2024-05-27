///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `SLV_ST_END` writer - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt.
pub type SLV_ST_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `MST_ST_END` writer - The clear bit for SPI_MEM_MST_ST_END_INT interrupt.
pub type MST_ST_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `ECC_ERR` writer - The clear bit for SPI_MEM_ECC_ERR_INT interrupt.
pub type ECC_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `PMS_REJECT` writer - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt.
pub type PMS_REJECT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `AXI_RADDR_ERR` writer - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
pub type AXI_RADDR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `AXI_WR_FLASH_ERR` writer - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
pub type AXI_WR_FLASH_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `AXI_WADDR_ERR` writer - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
pub type AXI_WADDR_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DQS0_AFIFO_OVF` writer - The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
pub type DQS0_AFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `DQS1_AFIFO_OVF` writer - The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
pub type DQS1_AFIFO_OVF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `BUS_FIFO1_UDF` writer - The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
pub type BUS_FIFO1_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `BUS_FIFO0_UDF` writer - The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
pub type BUS_FIFO0_UDF_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - The clear bit for SPI_MEM_SLV_ST_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn slv_st_end(&mut self) -> SLV_ST_END_W<INT_CLR_SPEC> {
        SLV_ST_END_W::new(self, 3)
    }
    ///Bit 4 - The clear bit for SPI_MEM_MST_ST_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn mst_st_end(&mut self) -> MST_ST_END_W<INT_CLR_SPEC> {
        MST_ST_END_W::new(self, 4)
    }
    ///Bit 5 - The clear bit for SPI_MEM_ECC_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ecc_err(&mut self) -> ECC_ERR_W<INT_CLR_SPEC> {
        ECC_ERR_W::new(self, 5)
    }
    ///Bit 6 - The clear bit for SPI_MEM_PMS_REJECT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn pms_reject(&mut self) -> PMS_REJECT_W<INT_CLR_SPEC> {
        PMS_REJECT_W::new(self, 6)
    }
    ///Bit 7 - The clear bit for SPI_MEM_AXI_RADDR_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn axi_raddr_err(&mut self) -> AXI_RADDR_ERR_W<INT_CLR_SPEC> {
        AXI_RADDR_ERR_W::new(self, 7)
    }
    ///Bit 8 - The clear bit for SPI_MEM_AXI_WR_FALSH_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn axi_wr_flash_err(&mut self) -> AXI_WR_FLASH_ERR_W<INT_CLR_SPEC> {
        AXI_WR_FLASH_ERR_W::new(self, 8)
    }
    ///Bit 9 - The clear bit for SPI_MEM_AXI_WADDR_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn axi_waddr_err(&mut self) -> AXI_WADDR_ERR_W<INT_CLR_SPEC> {
        AXI_WADDR_ERR_W::new(self, 9)
    }
    ///Bit 28 - The clear bit for SPI_MEM_DQS0_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn dqs0_afifo_ovf(&mut self) -> DQS0_AFIFO_OVF_W<INT_CLR_SPEC> {
        DQS0_AFIFO_OVF_W::new(self, 28)
    }
    ///Bit 29 - The clear bit for SPI_MEM_DQS1_AFIFO_OVF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn dqs1_afifo_ovf(&mut self) -> DQS1_AFIFO_OVF_W<INT_CLR_SPEC> {
        DQS1_AFIFO_OVF_W::new(self, 29)
    }
    ///Bit 30 - The clear bit for SPI_MEM_BUS_FIFO1_UDF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn bus_fifo1_udf(&mut self) -> BUS_FIFO1_UDF_W<INT_CLR_SPEC> {
        BUS_FIFO1_UDF_W::new(self, 30)
    }
    ///Bit 31 - The clear bit for SPI_MEM_BUS_FIFO0_UDF_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn bus_fifo0_udf(&mut self) -> BUS_FIFO0_UDF_W<INT_CLR_SPEC> {
        BUS_FIFO0_UDF_W::new(self, 31)
    }
}
/**SPI0 interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf000_03f8;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
