///Register `OUT_CRC_INIT_DATA` reader
pub type R = crate::R<OUT_CRC_INIT_DATA_SPEC>;
///Register `OUT_CRC_INIT_DATA` writer
pub type W = crate::W<OUT_CRC_INIT_DATA_SPEC>;
///Field `OUT_CRC_INIT_DATA` reader - This register is used to config ch0 of tx crc initial value
pub type OUT_CRC_INIT_DATA_R = crate::FieldReader<u32>;
///Field `OUT_CRC_INIT_DATA` writer - This register is used to config ch0 of tx crc initial value
pub type OUT_CRC_INIT_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    pub fn out_crc_init_data(&self) -> OUT_CRC_INIT_DATA_R {
        OUT_CRC_INIT_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CRC_INIT_DATA")
            .field("out_crc_init_data", &self.out_crc_init_data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - This register is used to config ch0 of tx crc initial value
    #[inline(always)]
    #[must_use]
    pub fn out_crc_init_data(&mut self) -> OUT_CRC_INIT_DATA_W<OUT_CRC_INIT_DATA_SPEC> {
        OUT_CRC_INIT_DATA_W::new(self, 0)
    }
}
/**This register is used to config ch0 crc initial data(max 32 bit)

You can [`read`](crate::generic::Reg::read) this register and get [`out_crc_init_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_crc_init_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_CRC_INIT_DATA_SPEC;
impl crate::RegisterSpec for OUT_CRC_INIT_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_crc_init_data::R`](R) reader structure
impl crate::Readable for OUT_CRC_INIT_DATA_SPEC {}
///`write(|w| ..)` method takes [`out_crc_init_data::W`](W) writer structure
impl crate::Writable for OUT_CRC_INIT_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OUT_CRC_INIT_DATA to value 0xffff_ffff
impl crate::Resettable for OUT_CRC_INIT_DATA_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
