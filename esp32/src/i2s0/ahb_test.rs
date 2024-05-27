///Register `AHB_TEST` reader
pub type R = crate::R<AHB_TEST_SPEC>;
///Register `AHB_TEST` writer
pub type W = crate::W<AHB_TEST_SPEC>;
///Field `AHB_TESTMODE` reader -
pub type AHB_TESTMODE_R = crate::FieldReader;
///Field `AHB_TESTMODE` writer -
pub type AHB_TESTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `AHB_TESTADDR` reader -
pub type AHB_TESTADDR_R = crate::FieldReader;
///Field `AHB_TESTADDR` writer -
pub type AHB_TESTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:2
    #[inline(always)]
    pub fn ahb_testmode(&self) -> AHB_TESTMODE_R {
        AHB_TESTMODE_R::new((self.bits & 7) as u8)
    }
    ///Bits 4:5
    #[inline(always)]
    pub fn ahb_testaddr(&self) -> AHB_TESTADDR_R {
        AHB_TESTADDR_R::new(((self.bits >> 4) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_TEST")
            .field("ahb_testmode", &self.ahb_testmode())
            .field("ahb_testaddr", &self.ahb_testaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:2
    #[inline(always)]
    #[must_use]
    pub fn ahb_testmode(&mut self) -> AHB_TESTMODE_W<AHB_TEST_SPEC> {
        AHB_TESTMODE_W::new(self, 0)
    }
    ///Bits 4:5
    #[inline(always)]
    #[must_use]
    pub fn ahb_testaddr(&mut self) -> AHB_TESTADDR_W<AHB_TEST_SPEC> {
        AHB_TESTADDR_W::new(self, 4)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ahb_test::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_test::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct AHB_TEST_SPEC;
impl crate::RegisterSpec for AHB_TEST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ahb_test::R`](R) reader structure
impl crate::Readable for AHB_TEST_SPEC {}
///`write(|w| ..)` method takes [`ahb_test::W`](W) writer structure
impl crate::Writable for AHB_TEST_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets AHB_TEST to value 0
impl crate::Resettable for AHB_TEST_SPEC {
    const RESET_VALUE: u32 = 0;
}
