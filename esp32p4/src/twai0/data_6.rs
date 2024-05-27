///Register `DATA_6` reader
pub type R = crate::R<DATA_6_SPEC>;
///Register `DATA_6` writer
pub type W = crate::W<DATA_6_SPEC>;
///Field `DATA_6` reader - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
pub type DATA_6_R = crate::FieldReader;
///Field `DATA_6` writer - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
pub type DATA_6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
    #[inline(always)]
    pub fn data_6(&self) -> DATA_6_R {
        DATA_6_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_6")
            .field("data_6", &self.data_6())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - In reset mode, it is acceptance mask register 2 with R/W Permission. In operation mode, when software initiate write operation, it is tx data register 6 and when software initiate read operation, it is rx data register 6.
    #[inline(always)]
    #[must_use]
    pub fn data_6(&mut self) -> DATA_6_W<DATA_6_SPEC> {
        DATA_6_W::new(self, 0)
    }
}
/**Data register 6.

You can [`read`](crate::generic::Reg::read) this register and get [`data_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_6_SPEC;
impl crate::RegisterSpec for DATA_6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data_6::R`](R) reader structure
impl crate::Readable for DATA_6_SPEC {}
///`write(|w| ..)` method takes [`data_6::W`](W) writer structure
impl crate::Writable for DATA_6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA_6 to value 0
impl crate::Resettable for DATA_6_SPEC {
    const RESET_VALUE: u32 = 0;
}
