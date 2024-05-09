#[doc = "Register `SDA_SAMPLE` reader"]
pub type R = crate::R<SDA_SAMPLE_SPEC>;
#[doc = "Register `SDA_SAMPLE` writer"]
pub type W = crate::W<SDA_SAMPLE_SPEC>;
#[doc = "Field `TIME` reader - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
pub type TIME_R = crate::FieldReader<u16>;
#[doc = "Field `TIME` writer - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
pub type TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
    #[inline(always)]
    pub fn time(&self) -> TIME_R {
        TIME_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDA_SAMPLE")
            .field("time", &self.time().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SDA_SAMPLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register is used to configure the clock num I2C used to sample data on SDA after the posedge of SCL"]
    #[inline(always)]
    #[must_use]
    pub fn time(&mut self) -> TIME_W<SDA_SAMPLE_SPEC> {
        TIME_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_sample::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_sample::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDA_SAMPLE_SPEC;
impl crate::RegisterSpec for SDA_SAMPLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sda_sample::R`](R) reader structure"]
impl crate::Readable for SDA_SAMPLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sda_sample::W`](W) writer structure"]
impl crate::Writable for SDA_SAMPLE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDA_SAMPLE to value 0"]
impl crate::Resettable for SDA_SAMPLE_SPEC {}
