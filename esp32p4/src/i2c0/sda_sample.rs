///Register `SDA_SAMPLE` reader
pub type R = crate::R<SDA_SAMPLE_SPEC>;
///Register `SDA_SAMPLE` writer
pub type W = crate::W<SDA_SAMPLE_SPEC>;
///Field `TIME` reader - Configures the sample time after a positive SCL edge. Measurement unit: i2c_sclk
pub type TIME_R = crate::FieldReader<u16>;
///Field `TIME` writer - Configures the sample time after a positive SCL edge. Measurement unit: i2c_sclk
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    ///Bits 0:8 - Configures the sample time after a positive SCL edge. Measurement unit: i2c_sclk
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_SAMPLE")
            .field("time", &self.time())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - Configures the sample time after a positive SCL edge. Measurement unit: i2c_sclk
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<SDA_SAMPLE_SPEC> {
        TIME_W::new(self, 0)
    }
}
/**Configures the sample time after a positive SCL edge.

You can [`read`](crate::generic::Reg::read) this register and get [`sda_sample::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_sample::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SDA_SAMPLE_SPEC;
impl crate::RegisterSpec for SDA_SAMPLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sda_sample::R`](R) reader structure
impl crate::Readable for SDA_SAMPLE_SPEC {}
///`write(|w| ..)` method takes [`sda_sample::W`](W) writer structure
impl crate::Writable for SDA_SAMPLE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDA_SAMPLE to value 0
impl crate::Resettable for SDA_SAMPLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
