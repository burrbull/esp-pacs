///Register `HOST_SLCHOST_CONF_W0` reader
pub type R = crate::R<HOST_SLCHOST_CONF_W0_SPEC>;
///Register `HOST_SLCHOST_CONF_W0` writer
pub type W = crate::W<HOST_SLCHOST_CONF_W0_SPEC>;
///Field `HOST_SLCHOST_CONF0` reader -
pub type HOST_SLCHOST_CONF0_R = crate::FieldReader;
///Field `HOST_SLCHOST_CONF0` writer -
pub type HOST_SLCHOST_CONF0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HOST_SLCHOST_CONF1` reader -
pub type HOST_SLCHOST_CONF1_R = crate::FieldReader;
///Field `HOST_SLCHOST_CONF1` writer -
pub type HOST_SLCHOST_CONF1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HOST_SLCHOST_CONF2` reader -
pub type HOST_SLCHOST_CONF2_R = crate::FieldReader;
///Field `HOST_SLCHOST_CONF2` writer -
pub type HOST_SLCHOST_CONF2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `HOST_SLCHOST_CONF3` reader -
pub type HOST_SLCHOST_CONF3_R = crate::FieldReader;
///Field `HOST_SLCHOST_CONF3` writer -
pub type HOST_SLCHOST_CONF3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7
    #[inline(always)]
    pub fn host_slchost_conf0(&self) -> HOST_SLCHOST_CONF0_R {
        HOST_SLCHOST_CONF0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15
    #[inline(always)]
    pub fn host_slchost_conf1(&self) -> HOST_SLCHOST_CONF1_R {
        HOST_SLCHOST_CONF1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23
    #[inline(always)]
    pub fn host_slchost_conf2(&self) -> HOST_SLCHOST_CONF2_R {
        HOST_SLCHOST_CONF2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn host_slchost_conf3(&self) -> HOST_SLCHOST_CONF3_R {
        HOST_SLCHOST_CONF3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W0")
            .field("host_slchost_conf0", &self.host_slchost_conf0())
            .field("host_slchost_conf1", &self.host_slchost_conf1())
            .field("host_slchost_conf2", &self.host_slchost_conf2())
            .field("host_slchost_conf3", &self.host_slchost_conf3())
            .finish()
    }
}
impl W {
    ///Bits 0:7
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf0(
        &mut self,
    ) -> HOST_SLCHOST_CONF0_W<HOST_SLCHOST_CONF_W0_SPEC> {
        HOST_SLCHOST_CONF0_W::new(self, 0)
    }
    ///Bits 8:15
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf1(
        &mut self,
    ) -> HOST_SLCHOST_CONF1_W<HOST_SLCHOST_CONF_W0_SPEC> {
        HOST_SLCHOST_CONF1_W::new(self, 8)
    }
    ///Bits 16:23
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf2(
        &mut self,
    ) -> HOST_SLCHOST_CONF2_W<HOST_SLCHOST_CONF_W0_SPEC> {
        HOST_SLCHOST_CONF2_W::new(self, 16)
    }
    ///Bits 24:31
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf3(
        &mut self,
    ) -> HOST_SLCHOST_CONF3_W<HOST_SLCHOST_CONF_W0_SPEC> {
        HOST_SLCHOST_CONF3_W::new(self, 24)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HOST_SLCHOST_CONF_W0_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`host_slchost_conf_w0::R`](R) reader structure
impl crate::Readable for HOST_SLCHOST_CONF_W0_SPEC {}
///`write(|w| ..)` method takes [`host_slchost_conf_w0::W`](W) writer structure
impl crate::Writable for HOST_SLCHOST_CONF_W0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HOST_SLCHOST_CONF_W0 to value 0
impl crate::Resettable for HOST_SLCHOST_CONF_W0_SPEC {
    const RESET_VALUE: u32 = 0;
}
