///Register `LR_VALUE` reader
pub type R = crate::R<LR_VALUE_SPEC>;
///Register `LR_VALUE` writer
pub type W = crate::W<LR_VALUE_SPEC>;
///Field `GLOABLE_LR_VALUE` reader - backup gloable value
pub type GLOABLE_LR_VALUE_R = crate::FieldReader<u32>;
///Field `GLOABLE_LR_VALUE` writer - backup gloable value
pub type GLOABLE_LR_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - backup gloable value
    #[inline(always)]
    pub fn gloable_lr_value(&self) -> GLOABLE_LR_VALUE_R {
        GLOABLE_LR_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LR_VALUE")
            .field("gloable_lr_value", &self.gloable_lr_value())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - backup gloable value
    #[inline(always)]
    #[must_use]
    pub fn gloable_lr_value(&mut self) -> GLOABLE_LR_VALUE_W<LR_VALUE_SPEC> {
        GLOABLE_LR_VALUE_W::new(self, 0)
    }
}
/**gloable lr value regsiter

You can [`read`](crate::generic::Reg::read) this register and get [`lr_value::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lr_value::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LR_VALUE_SPEC;
impl crate::RegisterSpec for LR_VALUE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lr_value::R`](R) reader structure
impl crate::Readable for LR_VALUE_SPEC {}
///`write(|w| ..)` method takes [`lr_value::W`](W) writer structure
impl crate::Writable for LR_VALUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LR_VALUE to value 0
impl crate::Resettable for LR_VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
