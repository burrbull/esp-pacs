///Register `CACHE_TAG_ACCESS_1` reader
pub type R = crate::R<CACHE_TAG_ACCESS_1_SPEC>;
///Register `CACHE_TAG_ACCESS_1` writer
pub type W = crate::W<CACHE_TAG_ACCESS_1_SPEC>;
///Field `PRO_I_TAG_RD_ACS` reader - Need add description
pub type PRO_I_TAG_RD_ACS_R = crate::BitReader;
///Field `PRO_I_TAG_RD_ACS` writer - Need add description
pub type PRO_I_TAG_RD_ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_I_TAG_WR_ACS` reader - Need add description
pub type PRO_I_TAG_WR_ACS_R = crate::BitReader;
///Field `PRO_I_TAG_WR_ACS` writer - Need add description
pub type PRO_I_TAG_WR_ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_D_TAG_RD_ACS` reader - Need add description
pub type PRO_D_TAG_RD_ACS_R = crate::BitReader;
///Field `PRO_D_TAG_RD_ACS` writer - Need add description
pub type PRO_D_TAG_RD_ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_D_TAG_WR_ACS` reader - Need add description
pub type PRO_D_TAG_WR_ACS_R = crate::BitReader;
///Field `PRO_D_TAG_WR_ACS` writer - Need add description
pub type PRO_D_TAG_WR_ACS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Need add description
    #[inline(always)]
    pub fn pro_i_tag_rd_acs(&self) -> PRO_I_TAG_RD_ACS_R {
        PRO_I_TAG_RD_ACS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Need add description
    #[inline(always)]
    pub fn pro_i_tag_wr_acs(&self) -> PRO_I_TAG_WR_ACS_R {
        PRO_I_TAG_WR_ACS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Need add description
    #[inline(always)]
    pub fn pro_d_tag_rd_acs(&self) -> PRO_D_TAG_RD_ACS_R {
        PRO_D_TAG_RD_ACS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Need add description
    #[inline(always)]
    pub fn pro_d_tag_wr_acs(&self) -> PRO_D_TAG_WR_ACS_R {
        PRO_D_TAG_WR_ACS_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_TAG_ACCESS_1")
            .field("pro_i_tag_rd_acs", &self.pro_i_tag_rd_acs())
            .field("pro_i_tag_wr_acs", &self.pro_i_tag_wr_acs())
            .field("pro_d_tag_rd_acs", &self.pro_d_tag_rd_acs())
            .field("pro_d_tag_wr_acs", &self.pro_d_tag_wr_acs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn pro_i_tag_rd_acs(&mut self) -> PRO_I_TAG_RD_ACS_W<CACHE_TAG_ACCESS_1_SPEC> {
        PRO_I_TAG_RD_ACS_W::new(self, 0)
    }
    ///Bit 1 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn pro_i_tag_wr_acs(&mut self) -> PRO_I_TAG_WR_ACS_W<CACHE_TAG_ACCESS_1_SPEC> {
        PRO_I_TAG_WR_ACS_W::new(self, 1)
    }
    ///Bit 2 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn pro_d_tag_rd_acs(&mut self) -> PRO_D_TAG_RD_ACS_W<CACHE_TAG_ACCESS_1_SPEC> {
        PRO_D_TAG_RD_ACS_W::new(self, 2)
    }
    ///Bit 3 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn pro_d_tag_wr_acs(&mut self) -> PRO_D_TAG_WR_ACS_W<CACHE_TAG_ACCESS_1_SPEC> {
        PRO_D_TAG_WR_ACS_W::new(self, 3)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`cache_tag_access_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_tag_access_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CACHE_TAG_ACCESS_1_SPEC;
impl crate::RegisterSpec for CACHE_TAG_ACCESS_1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cache_tag_access_1::R`](R) reader structure
impl crate::Readable for CACHE_TAG_ACCESS_1_SPEC {}
///`write(|w| ..)` method takes [`cache_tag_access_1::W`](W) writer structure
impl crate::Writable for CACHE_TAG_ACCESS_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CACHE_TAG_ACCESS_1 to value 0x0f
impl crate::Resettable for CACHE_TAG_ACCESS_1_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
