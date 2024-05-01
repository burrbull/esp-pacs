///Register `SLV_WR_STATUS` reader
pub type R = crate::R<SLV_WR_STATUS_SPEC>;
///Register `SLV_WR_STATUS` writer
pub type W = crate::W<SLV_WR_STATUS_SPEC>;
///Field `SLV_WR_ST` reader - In the slave mode this register are the status register for the master to write into. In the master mode this register are the higher 32bits in the 64 bits address condition.
pub type SLV_WR_ST_R = crate::FieldReader<u32>;
///Field `SLV_WR_ST` writer - In the slave mode this register are the status register for the master to write into. In the master mode this register are the higher 32bits in the 64 bits address condition.
pub type SLV_WR_ST_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - In the slave mode this register are the status register for the master to write into. In the master mode this register are the higher 32bits in the 64 bits address condition.
    #[inline(always)]
    pub fn slv_wr_st(&self) -> SLV_WR_ST_R {
        SLV_WR_ST_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLV_WR_STATUS")
            .field("slv_wr_st", &self.slv_wr_st())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - In the slave mode this register are the status register for the master to write into. In the master mode this register are the higher 32bits in the 64 bits address condition.
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_st(&mut self) -> SLV_WR_ST_W<SLV_WR_STATUS_SPEC> {
        SLV_WR_ST_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`slv_wr_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slv_wr_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLV_WR_STATUS_SPEC;
impl crate::RegisterSpec for SLV_WR_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slv_wr_status::R`](R) reader structure
impl crate::Readable for SLV_WR_STATUS_SPEC {}
///`write(|w| ..)` method takes [`slv_wr_status::W`](W) writer structure
impl crate::Writable for SLV_WR_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLV_WR_STATUS to value 0
impl crate::Resettable for SLV_WR_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
