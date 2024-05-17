///Register `ACK_TIMEOUT` reader
pub type R = crate::R<ACK_TIMEOUT_SPEC>;
///Register `ACK_TIMEOUT` writer
pub type W = crate::W<ACK_TIMEOUT_SPEC>;
///Field `ACK_TIMEOUT` reader -
pub type ACK_TIMEOUT_R = crate::FieldReader<u16>;
///Field `ACK_TIMEOUT` writer -
pub type ACK_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn ack_timeout(&self) -> ACK_TIMEOUT_R {
        ACK_TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACK_TIMEOUT").field("ack_timeout", &self.ack_timeout()).finish()
    }
}
impl W {
    ///Bits 0:15
    #[inline(always)]
    #[must_use]
    pub fn ack_timeout(&mut self) -> ACK_TIMEOUT_W<ACK_TIMEOUT_SPEC> {
        ACK_TIMEOUT_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`ack_timeout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_timeout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ACK_TIMEOUT_SPEC;
impl crate::RegisterSpec for ACK_TIMEOUT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ack_timeout::R`](R) reader structure
impl crate::Readable for ACK_TIMEOUT_SPEC {}
///`write(|w| ..)` method takes [`ack_timeout::W`](W) writer structure
impl crate::Writable for ACK_TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ACK_TIMEOUT to value 0
impl crate::Resettable for ACK_TIMEOUT_SPEC {
    const RESET_VALUE: u32 = 0;
}
