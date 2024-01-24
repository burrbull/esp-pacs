#[doc = "Register `OUT_CRC_INIT_DATA_CH%s` reader"]
pub type R = crate::R<OUT_CRC_INIT_DATA_CH_SPEC>;
#[doc = "Register `OUT_CRC_INIT_DATA_CH%s` writer"]
pub type W = crate::W<OUT_CRC_INIT_DATA_CH_SPEC>;
#[doc = "Field `OUT_CRC_INIT_DATA_CH` reader - This register is used to config ch0 of tx crc initial value"]
pub type OUT_CRC_INIT_DATA_CH_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_CRC_INIT_DATA_CH` writer - This register is used to config ch0 of tx crc initial value"]
pub type OUT_CRC_INIT_DATA_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This register is used to config ch0 of tx crc initial value"]
    #[inline(always)]
    pub fn out_crc_init_data_ch(&self) -> OUT_CRC_INIT_DATA_CH_R {
        OUT_CRC_INIT_DATA_CH_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_CRC_INIT_DATA_CH")
            .field(
                "out_crc_init_data_ch",
                &format_args!("{}", self.out_crc_init_data_ch().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_CRC_INIT_DATA_CH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This register is used to config ch0 of tx crc initial value"]
    #[inline(always)]
    #[must_use]
    pub fn out_crc_init_data_ch(&mut self) -> OUT_CRC_INIT_DATA_CH_W<OUT_CRC_INIT_DATA_CH_SPEC> {
        OUT_CRC_INIT_DATA_CH_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register is used to config ch0 crc initial data(max 32 bit)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_crc_init_data_ch::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_crc_init_data_ch::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_CRC_INIT_DATA_CH_SPEC;
impl crate::RegisterSpec for OUT_CRC_INIT_DATA_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_crc_init_data_ch::R`](R) reader structure"]
impl crate::Readable for OUT_CRC_INIT_DATA_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_crc_init_data_ch::W`](W) writer structure"]
impl crate::Writable for OUT_CRC_INIT_DATA_CH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OUT_CRC_INIT_DATA_CH%s to value 0xffff_ffff"]
impl crate::Resettable for OUT_CRC_INIT_DATA_CH_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
