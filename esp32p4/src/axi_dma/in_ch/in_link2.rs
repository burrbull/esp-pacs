///Register `IN_LINK2` reader
pub type R = crate::R<IN_LINK2_SPEC>;
///Register `IN_LINK2` writer
pub type W = crate::W<IN_LINK2_SPEC>;
///Field `INLINK_ADDR` reader - This register stores the 20 least significant bits of the first inlink descriptor's address.
pub type INLINK_ADDR_R = crate::FieldReader<u32>;
///Field `INLINK_ADDR` writer - This register stores the 20 least significant bits of the first inlink descriptor's address.
pub type INLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - This register stores the 20 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    pub fn inlink_addr(&self) -> INLINK_ADDR_R {
        INLINK_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_LINK2")
            .field("inlink_addr", &self.inlink_addr())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - This register stores the 20 least significant bits of the first inlink descriptor's address.
    #[inline(always)]
    #[must_use]
    pub fn inlink_addr(&mut self) -> INLINK_ADDR_W<IN_LINK2_SPEC> {
        INLINK_ADDR_W::new(self, 0)
    }
}
/**Link descriptor configure and control register of Rx channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`in_link2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_link2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IN_LINK2_SPEC;
impl crate::RegisterSpec for IN_LINK2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`in_link2::R`](R) reader structure
impl crate::Readable for IN_LINK2_SPEC {}
///`write(|w| ..)` method takes [`in_link2::W`](W) writer structure
impl crate::Writable for IN_LINK2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IN_LINK2 to value 0
impl crate::Resettable for IN_LINK2_SPEC {
    const RESET_VALUE: u32 = 0;
}
