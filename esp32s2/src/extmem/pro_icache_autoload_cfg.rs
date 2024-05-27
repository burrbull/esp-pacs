///Register `PRO_ICACHE_AUTOLOAD_CFG` reader
pub type R = crate::R<PRO_ICACHE_AUTOLOAD_CFG_SPEC>;
///Register `PRO_ICACHE_AUTOLOAD_CFG` writer
pub type W = crate::W<PRO_ICACHE_AUTOLOAD_CFG_SPEC>;
///Field `PRO_ICACHE_AUTOLOAD_MODE` reader - Reserved.
pub type PRO_ICACHE_AUTOLOAD_MODE_R = crate::BitReader;
///Field `PRO_ICACHE_AUTOLOAD_MODE` writer - Reserved.
pub type PRO_ICACHE_AUTOLOAD_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_ICACHE_AUTOLOAD_STEP` reader - Reserved.
pub type PRO_ICACHE_AUTOLOAD_STEP_R = crate::FieldReader;
///Field `PRO_ICACHE_AUTOLOAD_STEP` writer - Reserved.
pub type PRO_ICACHE_AUTOLOAD_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRO_ICACHE_AUTOLOAD_ORDER` reader - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending.
pub type PRO_ICACHE_AUTOLOAD_ORDER_R = crate::BitReader;
///Field `PRO_ICACHE_AUTOLOAD_ORDER` writer - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending.
pub type PRO_ICACHE_AUTOLOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_ICACHE_AUTOLOAD_RQST` reader - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit.
pub type PRO_ICACHE_AUTOLOAD_RQST_R = crate::FieldReader;
///Field `PRO_ICACHE_AUTOLOAD_RQST` writer - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit.
pub type PRO_ICACHE_AUTOLOAD_RQST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRO_ICACHE_AUTOLOAD_SIZE` reader - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation.
pub type PRO_ICACHE_AUTOLOAD_SIZE_R = crate::FieldReader;
///Field `PRO_ICACHE_AUTOLOAD_SIZE` writer - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation.
pub type PRO_ICACHE_AUTOLOAD_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRO_ICACHE_AUTOLOAD_SCT0_ENA` reader - The bits are used to enable the second section for conditional pre-load operation.
pub type PRO_ICACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
///Field `PRO_ICACHE_AUTOLOAD_SCT0_ENA` writer - The bits are used to enable the second section for conditional pre-load operation.
pub type PRO_ICACHE_AUTOLOAD_SCT0_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRO_ICACHE_AUTOLOAD_SCT1_ENA` reader - The bits are used to enable the first section for conditional pre-load operation.
pub type PRO_ICACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
///Field `PRO_ICACHE_AUTOLOAD_SCT1_ENA` writer - The bits are used to enable the first section for conditional pre-load operation.
pub type PRO_ICACHE_AUTOLOAD_SCT1_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Reserved.
    #[inline(always)]
    pub fn pro_icache_autoload_mode(&self) -> PRO_ICACHE_AUTOLOAD_MODE_R {
        PRO_ICACHE_AUTOLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - Reserved.
    #[inline(always)]
    pub fn pro_icache_autoload_step(&self) -> PRO_ICACHE_AUTOLOAD_STEP_R {
        PRO_ICACHE_AUTOLOAD_STEP_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending.
    #[inline(always)]
    pub fn pro_icache_autoload_order(&self) -> PRO_ICACHE_AUTOLOAD_ORDER_R {
        PRO_ICACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit.
    #[inline(always)]
    pub fn pro_icache_autoload_rqst(&self) -> PRO_ICACHE_AUTOLOAD_RQST_R {
        PRO_ICACHE_AUTOLOAD_RQST_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation.
    #[inline(always)]
    pub fn pro_icache_autoload_size(&self) -> PRO_ICACHE_AUTOLOAD_SIZE_R {
        PRO_ICACHE_AUTOLOAD_SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - The bits are used to enable the second section for conditional pre-load operation.
    #[inline(always)]
    pub fn pro_icache_autoload_sct0_ena(&self) -> PRO_ICACHE_AUTOLOAD_SCT0_ENA_R {
        PRO_ICACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - The bits are used to enable the first section for conditional pre-load operation.
    #[inline(always)]
    pub fn pro_icache_autoload_sct1_ena(&self) -> PRO_ICACHE_AUTOLOAD_SCT1_ENA_R {
        PRO_ICACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_ICACHE_AUTOLOAD_CFG")
            .field("pro_icache_autoload_mode", &self.pro_icache_autoload_mode())
            .field("pro_icache_autoload_step", &self.pro_icache_autoload_step())
            .field(
                "pro_icache_autoload_order",
                &self.pro_icache_autoload_order(),
            )
            .field("pro_icache_autoload_rqst", &self.pro_icache_autoload_rqst())
            .field("pro_icache_autoload_size", &self.pro_icache_autoload_size())
            .field(
                "pro_icache_autoload_sct0_ena",
                &self.pro_icache_autoload_sct0_ena(),
            )
            .field(
                "pro_icache_autoload_sct1_ena",
                &self.pro_icache_autoload_sct1_ena(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_mode(
        &mut self,
    ) -> PRO_ICACHE_AUTOLOAD_MODE_W<PRO_ICACHE_AUTOLOAD_CFG_SPEC> {
        PRO_ICACHE_AUTOLOAD_MODE_W::new(self, 0)
    }
    ///Bits 1:2 - Reserved.
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_step(
        &mut self,
    ) -> PRO_ICACHE_AUTOLOAD_STEP_W<PRO_ICACHE_AUTOLOAD_CFG_SPEC> {
        PRO_ICACHE_AUTOLOAD_STEP_W::new(self, 1)
    }
    ///Bit 3 - The bits are used to configure the direction of conditional pre-load operation. 1: descending, 0: ascending.
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_order(
        &mut self,
    ) -> PRO_ICACHE_AUTOLOAD_ORDER_W<PRO_ICACHE_AUTOLOAD_CFG_SPEC> {
        PRO_ICACHE_AUTOLOAD_ORDER_W::new(self, 3)
    }
    ///Bits 4:5 - The bits are used to configure trigger conditions for conditional pre-load. 0/3: cache miss, 1: cache hit, 2: both cache miss and hit.
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_rqst(
        &mut self,
    ) -> PRO_ICACHE_AUTOLOAD_RQST_W<PRO_ICACHE_AUTOLOAD_CFG_SPEC> {
        PRO_ICACHE_AUTOLOAD_RQST_W::new(self, 4)
    }
    ///Bits 6:7 - The bits are used to configure the numbers of the cache block for the issuing conditional pre-load operation.
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_size(
        &mut self,
    ) -> PRO_ICACHE_AUTOLOAD_SIZE_W<PRO_ICACHE_AUTOLOAD_CFG_SPEC> {
        PRO_ICACHE_AUTOLOAD_SIZE_W::new(self, 6)
    }
    ///Bit 8 - The bits are used to enable the second section for conditional pre-load operation.
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_sct0_ena(
        &mut self,
    ) -> PRO_ICACHE_AUTOLOAD_SCT0_ENA_W<PRO_ICACHE_AUTOLOAD_CFG_SPEC> {
        PRO_ICACHE_AUTOLOAD_SCT0_ENA_W::new(self, 8)
    }
    ///Bit 9 - The bits are used to enable the first section for conditional pre-load operation.
    #[inline(always)]
    #[must_use]
    pub fn pro_icache_autoload_sct1_ena(
        &mut self,
    ) -> PRO_ICACHE_AUTOLOAD_SCT1_ENA_W<PRO_ICACHE_AUTOLOAD_CFG_SPEC> {
        PRO_ICACHE_AUTOLOAD_SCT1_ENA_W::new(self, 9)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`pro_icache_autoload_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_icache_autoload_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_ICACHE_AUTOLOAD_CFG_SPEC;
impl crate::RegisterSpec for PRO_ICACHE_AUTOLOAD_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_icache_autoload_cfg::R`](R) reader structure
impl crate::Readable for PRO_ICACHE_AUTOLOAD_CFG_SPEC {}
///`write(|w| ..)` method takes [`pro_icache_autoload_cfg::W`](W) writer structure
impl crate::Writable for PRO_ICACHE_AUTOLOAD_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PRO_ICACHE_AUTOLOAD_CFG to value 0
impl crate::Resettable for PRO_ICACHE_AUTOLOAD_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
