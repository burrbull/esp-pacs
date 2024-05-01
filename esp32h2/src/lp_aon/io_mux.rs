///Register `IO_MUX` reader
pub type R = crate::R<IO_MUX_SPEC>;
///Register `IO_MUX` writer
pub type W = crate::W<IO_MUX_SPEC>;
///Field `PULL_LDO` reader - need_des
pub type PULL_LDO_R = crate::FieldReader;
///Field `PULL_LDO` writer - need_des
pub type PULL_LDO_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RESET_DISABLE` reader - need_des
pub type RESET_DISABLE_R = crate::BitReader;
///Field `RESET_DISABLE` writer - need_des
pub type RESET_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 28:30 - need_des
    #[inline(always)]
    pub fn pull_ldo(&self) -> PULL_LDO_R {
        PULL_LDO_R::new(((self.bits >> 28) & 7) as u8)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    pub fn reset_disable(&self) -> RESET_DISABLE_R {
        RESET_DISABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_MUX")
            .field("pull_ldo", &self.pull_ldo())
            .field("reset_disable", &self.reset_disable())
            .finish()
    }
}
impl W {
    ///Bits 28:30 - need_des
    #[inline(always)]
    #[must_use]
    pub fn pull_ldo(&mut self) -> PULL_LDO_W<IO_MUX_SPEC> {
        PULL_LDO_W::new(self, 28)
    }
    ///Bit 31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn reset_disable(&mut self) -> RESET_DISABLE_W<IO_MUX_SPEC> {
        RESET_DISABLE_W::new(self, 31)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`io_mux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`io_mux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IO_MUX_SPEC;
impl crate::RegisterSpec for IO_MUX_SPEC {
    type Ux = u32;
}
///`read()` method returns [`io_mux::R`](R) reader structure
impl crate::Readable for IO_MUX_SPEC {}
///`write(|w| ..)` method takes [`io_mux::W`](W) writer structure
impl crate::Writable for IO_MUX_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IO_MUX to value 0
impl crate::Resettable for IO_MUX_SPEC {
    const RESET_VALUE: u32 = 0;
}
