///Register `LOG_CHECK_DATA` reader
pub type R = crate::R<LOG_CHECK_DATA_SPEC>;
///Register `LOG_CHECK_DATA` writer
pub type W = crate::W<LOG_CHECK_DATA_SPEC>;
///Field `LOG_CHECK_DATA` reader - The special check data, when write this special data, it will trigger logging.
pub type LOG_CHECK_DATA_R = crate::FieldReader<u32>;
///Field `LOG_CHECK_DATA` writer - The special check data, when write this special data, it will trigger logging.
pub type LOG_CHECK_DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - The special check data, when write this special data, it will trigger logging.
    #[inline(always)]
    pub fn log_check_data(&self) -> LOG_CHECK_DATA_R {
        LOG_CHECK_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG_CHECK_DATA")
            .field("log_check_data", &self.log_check_data())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - The special check data, when write this special data, it will trigger logging.
    #[inline(always)]
    #[must_use]
    pub fn log_check_data(&mut self) -> LOG_CHECK_DATA_W<LOG_CHECK_DATA_SPEC> {
        LOG_CHECK_DATA_W::new(self, 0)
    }
}
/**check data regsiter

You can [`read`](crate::generic::Reg::read) this register and get [`log_check_data::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`log_check_data::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LOG_CHECK_DATA_SPEC;
impl crate::RegisterSpec for LOG_CHECK_DATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`log_check_data::R`](R) reader structure
impl crate::Readable for LOG_CHECK_DATA_SPEC {}
///`write(|w| ..)` method takes [`log_check_data::W`](W) writer structure
impl crate::Writable for LOG_CHECK_DATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LOG_CHECK_DATA to value 0
impl crate::Resettable for LOG_CHECK_DATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
