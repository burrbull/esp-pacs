///Register `RD_TIM_CONF` reader
pub type R = crate::R<RD_TIM_CONF_SPEC>;
///Register `RD_TIM_CONF` writer
pub type W = crate::W<RD_TIM_CONF_SPEC>;
///Field `READ_INIT_NUM` reader - Configures the initial read time of eFuse.
pub type READ_INIT_NUM_R = crate::FieldReader;
///Field `READ_INIT_NUM` writer - Configures the initial read time of eFuse.
pub type READ_INIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 24:31 - Configures the initial read time of eFuse.
    #[inline(always)]
    pub fn read_init_num(&self) -> READ_INIT_NUM_R {
        READ_INIT_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_TIM_CONF")
            .field("read_init_num", &self.read_init_num())
            .finish()
    }
}
impl W {
    ///Bits 24:31 - Configures the initial read time of eFuse.
    #[inline(always)]
    #[must_use]
    pub fn read_init_num(&mut self) -> READ_INIT_NUM_W<RD_TIM_CONF_SPEC> {
        READ_INIT_NUM_W::new(self, 24)
    }
}
/**Configures read timing parameters.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_tim_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_tim_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_TIM_CONF_SPEC;
impl crate::RegisterSpec for RD_TIM_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_tim_conf::R`](R) reader structure
impl crate::Readable for RD_TIM_CONF_SPEC {}
///`write(|w| ..)` method takes [`rd_tim_conf::W`](W) writer structure
impl crate::Writable for RD_TIM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RD_TIM_CONF to value 0x1200_0000
impl crate::Resettable for RD_TIM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x1200_0000;
}
