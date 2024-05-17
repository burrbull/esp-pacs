///Register `RD_MAC_SYS_0` reader
pub type R = crate::R<RD_MAC_SYS_0_SPEC>;
///Field `MAC_0` reader - Stores the low 32 bits of MAC address.
pub type MAC_0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Stores the low 32 bits of MAC address.
    #[inline(always)]
    pub fn mac_0(&self) -> MAC_0_R {
        MAC_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS_0").field("mac_0", &self.mac_0()).finish()
    }
}
/**BLOCK1 data register $n.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_mac_sys_0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_MAC_SYS_0_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS_0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_mac_sys_0::R`](R) reader structure
impl crate::Readable for RD_MAC_SYS_0_SPEC {}
///`reset()` method sets RD_MAC_SYS_0 to value 0
impl crate::Resettable for RD_MAC_SYS_0_SPEC {
    const RESET_VALUE: u32 = 0;
}
