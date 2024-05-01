///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `DONE` reader - x
pub type DONE_R = crate::BitReader;
///Field `DONE` writer - x
pub type DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR` reader - x
pub type ERR_R = crate::BitReader;
///Field `ERR` writer - x
pub type ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - x
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - x
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("done", &self.done())
            .field("err", &self.err())
            .finish()
    }
}
impl W {
    ///Bit 0 - x
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<INT_ENA_SPEC> {
        DONE_W::new(self, 0)
    }
    ///Bit 1 - x
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<INT_ENA_SPEC> {
        ERR_W::new(self, 1)
    }
}
/**x

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
