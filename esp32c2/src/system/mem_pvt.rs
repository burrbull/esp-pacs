///Register `MEM_PVT` reader
pub type R = crate::R<MEM_PVT_SPEC>;
///Register `MEM_PVT` writer
pub type W = crate::W<MEM_PVT_SPEC>;
///Field `MEM_PATH_LEN` reader - reg_mem_path_len
pub type MEM_PATH_LEN_R = crate::FieldReader;
///Field `MEM_PATH_LEN` writer - reg_mem_path_len
pub type MEM_PATH_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `MEM_ERR_CNT_CLR` writer - reg_mem_err_cnt_clr
pub type MEM_ERR_CNT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MONITOR_EN` reader - reg_mem_pvt_monitor_en
pub type MONITOR_EN_R = crate::BitReader;
///Field `MONITOR_EN` writer - reg_mem_pvt_monitor_en
pub type MONITOR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MEM_TIMING_ERR_CNT` reader - reg_mem_timing_err_cnt
pub type MEM_TIMING_ERR_CNT_R = crate::FieldReader<u16>;
///Field `MEM_VT_SEL` reader - reg_mem_vt_sel
pub type MEM_VT_SEL_R = crate::FieldReader;
///Field `MEM_VT_SEL` writer - reg_mem_vt_sel
pub type MEM_VT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:3 - reg_mem_path_len
    #[inline(always)]
    pub fn mem_path_len(&self) -> MEM_PATH_LEN_R {
        MEM_PATH_LEN_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 5 - reg_mem_pvt_monitor_en
    #[inline(always)]
    pub fn monitor_en(&self) -> MONITOR_EN_R {
        MONITOR_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:21 - reg_mem_timing_err_cnt
    #[inline(always)]
    pub fn mem_timing_err_cnt(&self) -> MEM_TIMING_ERR_CNT_R {
        MEM_TIMING_ERR_CNT_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    ///Bits 22:23 - reg_mem_vt_sel
    #[inline(always)]
    pub fn mem_vt_sel(&self) -> MEM_VT_SEL_R {
        MEM_VT_SEL_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_PVT")
            .field("mem_path_len", &self.mem_path_len())
            .field("monitor_en", &self.monitor_en())
            .field("mem_timing_err_cnt", &self.mem_timing_err_cnt())
            .field("mem_vt_sel", &self.mem_vt_sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - reg_mem_path_len
    #[inline(always)]
    #[must_use]
    pub fn mem_path_len(&mut self) -> MEM_PATH_LEN_W<MEM_PVT_SPEC> {
        MEM_PATH_LEN_W::new(self, 0)
    }
    ///Bit 4 - reg_mem_err_cnt_clr
    #[inline(always)]
    #[must_use]
    pub fn mem_err_cnt_clr(&mut self) -> MEM_ERR_CNT_CLR_W<MEM_PVT_SPEC> {
        MEM_ERR_CNT_CLR_W::new(self, 4)
    }
    ///Bit 5 - reg_mem_pvt_monitor_en
    #[inline(always)]
    #[must_use]
    pub fn monitor_en(&mut self) -> MONITOR_EN_W<MEM_PVT_SPEC> {
        MONITOR_EN_W::new(self, 5)
    }
    ///Bits 22:23 - reg_mem_vt_sel
    #[inline(always)]
    #[must_use]
    pub fn mem_vt_sel(&mut self) -> MEM_VT_SEL_W<MEM_PVT_SPEC> {
        MEM_VT_SEL_W::new(self, 22)
    }
}
/**mem pvt register

You can [`read`](crate::generic::Reg::read) this register and get [`mem_pvt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_pvt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_PVT_SPEC;
impl crate::RegisterSpec for MEM_PVT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mem_pvt::R`](R) reader structure
impl crate::Readable for MEM_PVT_SPEC {}
///`write(|w| ..)` method takes [`mem_pvt::W`](W) writer structure
impl crate::Writable for MEM_PVT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_PVT to value 0x03
impl crate::Resettable for MEM_PVT_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
