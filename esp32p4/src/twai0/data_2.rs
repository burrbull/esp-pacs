///Register `DATA_2` reader
pub type R = crate::R<DATA_2_SPEC>;
///Register `DATA_2` writer
pub type W = crate::W<DATA_2_SPEC>;
///Field `DATA_2` reader - In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
pub type DATA_2_R = crate::FieldReader;
///Field `DATA_2` writer - In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
pub type DATA_2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
    #[inline(always)]
    pub fn data_2(&self) -> DATA_2_R {
        DATA_2_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_2").field("data_2", &self.data_2()).finish()
    }
}
impl W {
    ///Bits 0:7 - In reset mode, it is acceptance code register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 2 and when software initiate read operation, it is rx data register 2.
    #[inline(always)]
    #[must_use]
    pub fn data_2(&mut self) -> DATA_2_W<DATA_2_SPEC> {
        DATA_2_W::new(self, 0)
    }
}
/**Data register 2.

You can [`read`](crate::generic::Reg::read) this register and get [`data_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_2_SPEC;
impl crate::RegisterSpec for DATA_2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data_2::R`](R) reader structure
impl crate::Readable for DATA_2_SPEC {}
///`write(|w| ..)` method takes [`data_2::W`](W) writer structure
impl crate::Writable for DATA_2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA_2 to value 0
impl crate::Resettable for DATA_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
