///Register `IDBUS_ADDRHOLE_INFO` reader
pub type R = crate::R<IDBUS_ADDRHOLE_INFO_SPEC>;
///Field `IDBUS_ADDRHOLE_ID` reader - need_des
pub type IDBUS_ADDRHOLE_ID_R = crate::FieldReader;
///Field `IDBUS_ADDRHOLE_WR` reader - need_des
pub type IDBUS_ADDRHOLE_WR_R = crate::BitReader;
///Field `IDBUS_ADDRHOLE_SECURE` reader - need_des
pub type IDBUS_ADDRHOLE_SECURE_R = crate::BitReader;
impl R {
    ///Bits 0:4 - need_des
    #[inline(always)]
    pub fn idbus_addrhole_id(&self) -> IDBUS_ADDRHOLE_ID_R {
        IDBUS_ADDRHOLE_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - need_des
    #[inline(always)]
    pub fn idbus_addrhole_wr(&self) -> IDBUS_ADDRHOLE_WR_R {
        IDBUS_ADDRHOLE_WR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - need_des
    #[inline(always)]
    pub fn idbus_addrhole_secure(&self) -> IDBUS_ADDRHOLE_SECURE_R {
        IDBUS_ADDRHOLE_SECURE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDBUS_ADDRHOLE_INFO")
            .field("idbus_addrhole_id", &self.idbus_addrhole_id())
            .field("idbus_addrhole_wr", &self.idbus_addrhole_wr())
            .field("idbus_addrhole_secure", &self.idbus_addrhole_secure())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`idbus_addrhole_info::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IDBUS_ADDRHOLE_INFO_SPEC;
impl crate::RegisterSpec for IDBUS_ADDRHOLE_INFO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`idbus_addrhole_info::R`](R) reader structure
impl crate::Readable for IDBUS_ADDRHOLE_INFO_SPEC {}
///`reset()` method sets IDBUS_ADDRHOLE_INFO to value 0
impl crate::Resettable for IDBUS_ADDRHOLE_INFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
