///Register `SLAVE2` reader
pub type R = crate::R<SLAVE2_SPEC>;
///Register `SLAVE2` writer
pub type W = crate::W<SLAVE2_SPEC>;
///Field `SLV_RDSTA_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1).
pub type SLV_RDSTA_DUMMY_CYCLELEN_R = crate::FieldReader;
///Field `SLV_RDSTA_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1).
pub type SLV_RDSTA_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SLV_WRSTA_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1).
pub type SLV_WRSTA_DUMMY_CYCLELEN_R = crate::FieldReader;
///Field `SLV_WRSTA_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1).
pub type SLV_WRSTA_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SLV_RDBUF_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1).
pub type SLV_RDBUF_DUMMY_CYCLELEN_R = crate::FieldReader;
///Field `SLV_RDBUF_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1).
pub type SLV_RDBUF_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SLV_WRBUF_DUMMY_CYCLELEN` reader - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1).
pub type SLV_WRBUF_DUMMY_CYCLELEN_R = crate::FieldReader;
///Field `SLV_WRBUF_DUMMY_CYCLELEN` writer - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1).
pub type SLV_WRBUF_DUMMY_CYCLELEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1).
    #[inline(always)]
    pub fn slv_rdsta_dummy_cyclelen(&self) -> SLV_RDSTA_DUMMY_CYCLELEN_R {
        SLV_RDSTA_DUMMY_CYCLELEN_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1).
    #[inline(always)]
    pub fn slv_wrsta_dummy_cyclelen(&self) -> SLV_WRSTA_DUMMY_CYCLELEN_R {
        SLV_WRSTA_DUMMY_CYCLELEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1).
    #[inline(always)]
    pub fn slv_rdbuf_dummy_cyclelen(&self) -> SLV_RDBUF_DUMMY_CYCLELEN_R {
        SLV_RDBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1).
    #[inline(always)]
    pub fn slv_wrbuf_dummy_cyclelen(&self) -> SLV_WRBUF_DUMMY_CYCLELEN_R {
        SLV_WRBUF_DUMMY_CYCLELEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE2")
            .field("slv_rdsta_dummy_cyclelen", &self.slv_rdsta_dummy_cyclelen())
            .field("slv_wrsta_dummy_cyclelen", &self.slv_wrsta_dummy_cyclelen())
            .field("slv_rdbuf_dummy_cyclelen", &self.slv_rdbuf_dummy_cyclelen())
            .field("slv_wrbuf_dummy_cyclelen", &self.slv_wrbuf_dummy_cyclelen())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-status operations. The register value shall be (cycle_num-1).
    #[inline(always)]
    #[must_use]
    pub fn slv_rdsta_dummy_cyclelen(
        &mut self,
    ) -> SLV_RDSTA_DUMMY_CYCLELEN_W<SLAVE2_SPEC> {
        SLV_RDSTA_DUMMY_CYCLELEN_W::new(self, 0)
    }
    ///Bits 8:15 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-status operations. The register value shall be (cycle_num-1).
    #[inline(always)]
    #[must_use]
    pub fn slv_wrsta_dummy_cyclelen(
        &mut self,
    ) -> SLV_WRSTA_DUMMY_CYCLELEN_W<SLAVE2_SPEC> {
        SLV_WRSTA_DUMMY_CYCLELEN_W::new(self, 8)
    }
    ///Bits 16:23 - In the slave mode it is the length in spi_clk cycles of dummy phase for read-buffer operations. The register value shall be (cycle_num-1).
    #[inline(always)]
    #[must_use]
    pub fn slv_rdbuf_dummy_cyclelen(
        &mut self,
    ) -> SLV_RDBUF_DUMMY_CYCLELEN_W<SLAVE2_SPEC> {
        SLV_RDBUF_DUMMY_CYCLELEN_W::new(self, 16)
    }
    ///Bits 24:31 - In the slave mode it is the length in spi_clk cycles of dummy phase for write-buffer operations. The register value shall be (cycle_num-1).
    #[inline(always)]
    #[must_use]
    pub fn slv_wrbuf_dummy_cyclelen(
        &mut self,
    ) -> SLV_WRBUF_DUMMY_CYCLELEN_W<SLAVE2_SPEC> {
        SLV_WRBUF_DUMMY_CYCLELEN_W::new(self, 24)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`slave2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLAVE2_SPEC;
impl crate::RegisterSpec for SLAVE2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slave2::R`](R) reader structure
impl crate::Readable for SLAVE2_SPEC {}
///`write(|w| ..)` method takes [`slave2::W`](W) writer structure
impl crate::Writable for SLAVE2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLAVE2 to value 0
impl crate::Resettable for SLAVE2_SPEC {
    const RESET_VALUE: u32 = 0;
}
