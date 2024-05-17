///Register `L1_ICACHE1_PRELOCK_CONF` reader
pub type R = crate::R<L1_ICACHE1_PRELOCK_CONF_SPEC>;
///Field `L1_ICACHE1_PRELOCK_SCT0_EN` reader - The bit is used to enable the first section of prelock function on L1-ICache1.
pub type L1_ICACHE1_PRELOCK_SCT0_EN_R = crate::BitReader;
///Field `L1_ICACHE1_PRELOCK_SCT1_EN` reader - The bit is used to enable the second section of prelock function on L1-ICache1.
pub type L1_ICACHE1_PRELOCK_SCT1_EN_R = crate::BitReader;
///Field `L1_ICACHE1_PRELOCK_RGID` reader - The bit is used to set the gid of l1 icache1 prelock.
pub type L1_ICACHE1_PRELOCK_RGID_R = crate::FieldReader;
impl R {
    ///Bit 0 - The bit is used to enable the first section of prelock function on L1-ICache1.
    #[inline(always)]
    pub fn l1_icache1_prelock_sct0_en(&self) -> L1_ICACHE1_PRELOCK_SCT0_EN_R {
        L1_ICACHE1_PRELOCK_SCT0_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to enable the second section of prelock function on L1-ICache1.
    #[inline(always)]
    pub fn l1_icache1_prelock_sct1_en(&self) -> L1_ICACHE1_PRELOCK_SCT1_EN_R {
        L1_ICACHE1_PRELOCK_SCT1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:5 - The bit is used to set the gid of l1 icache1 prelock.
    #[inline(always)]
    pub fn l1_icache1_prelock_rgid(&self) -> L1_ICACHE1_PRELOCK_RGID_R {
        L1_ICACHE1_PRELOCK_RGID_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1_ICACHE1_PRELOCK_CONF")
            .field("l1_icache1_prelock_sct0_en", &self.l1_icache1_prelock_sct0_en())
            .field("l1_icache1_prelock_sct1_en", &self.l1_icache1_prelock_sct1_en())
            .field("l1_icache1_prelock_rgid", &self.l1_icache1_prelock_rgid())
            .finish()
    }
}
/**L1 instruction Cache 1 prelock configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_prelock_conf::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L1_ICACHE1_PRELOCK_CONF_SPEC;
impl crate::RegisterSpec for L1_ICACHE1_PRELOCK_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l1_icache1_prelock_conf::R`](R) reader structure
impl crate::Readable for L1_ICACHE1_PRELOCK_CONF_SPEC {}
///`reset()` method sets L1_ICACHE1_PRELOCK_CONF to value 0
impl crate::Resettable for L1_ICACHE1_PRELOCK_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
