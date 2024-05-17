///Register `OUT_DATA` reader
pub type R = crate::R<OUT_DATA_SPEC>;
///Register `OUT_DATA` writer
pub type W = crate::W<OUT_DATA_SPEC>;
///Field `OUT_DATA` reader - set lp gpio output data
pub type OUT_DATA_R = crate::FieldReader;
///Field `OUT_DATA` writer - set lp gpio output data
pub type OUT_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - set lp gpio output data
    #[inline(always)]
    pub fn out_data(&self) -> OUT_DATA_R {
        OUT_DATA_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_DATA").field("out_data", &self.out_data()).finish()
    }
}
impl W {
    ///Bits 0:7 - set lp gpio output data
    #[inline(always)]
    #[must_use]
    pub fn out_data(&mut self) -> OUT_DATA_W<OUT_DATA_SPEC> {
        OUT_DATA_W::new(self, 0)
    }
}
/**need des

You can [`read`](crate::generic::Reg::read) this register and get [`out_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_DATA_SPEC;
impl crate::RegisterSpec for OUT_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_data::R`](R) reader structure
impl crate::Readable for OUT_DATA_SPEC {}
///`write(|w| ..)` method takes [`out_data::W`](W) writer structure
impl crate::Writable for OUT_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OUT_DATA to value 0
impl crate::Resettable for OUT_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
