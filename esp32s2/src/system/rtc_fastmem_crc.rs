///Register `RTC_FASTMEM_CRC` reader
pub type R = crate::R<RTC_FASTMEM_CRC_SPEC>;
///Field `RTC_MEM_CRC_RES` reader - This field stores the CRC result of RTC memory.
pub type RTC_MEM_CRC_RES_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - This field stores the CRC result of RTC memory.
    #[inline(always)]
    pub fn rtc_mem_crc_res(&self) -> RTC_MEM_CRC_RES_R {
        RTC_MEM_CRC_RES_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_FASTMEM_CRC")
            .field("rtc_mem_crc_res", &self.rtc_mem_crc_res())
            .finish()
    }
}
/**RTC fast memory CRC controlling register

You can [`read`](crate::generic::Reg::read) this register and get [`rtc_fastmem_crc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RTC_FASTMEM_CRC_SPEC;
impl crate::RegisterSpec for RTC_FASTMEM_CRC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rtc_fastmem_crc::R`](R) reader structure
impl crate::Readable for RTC_FASTMEM_CRC_SPEC {}
///`reset()` method sets RTC_FASTMEM_CRC to value 0
impl crate::Resettable for RTC_FASTMEM_CRC_SPEC {
    const RESET_VALUE: u32 = 0;
}
