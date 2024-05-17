///Register `PKT_THRES` reader
pub type R = crate::R<PKT_THRES_SPEC>;
///Register `PKT_THRES` writer
pub type W = crate::W<PKT_THRES_SPEC>;
///Field `PKT_THRS` reader - when the amount of packet payload is greater than this value the process of receiving data is done.
pub type PKT_THRS_R = crate::FieldReader<u16>;
///Field `PKT_THRS` writer - when the amount of packet payload is greater than this value the process of receiving data is done.
pub type PKT_THRS_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    ///Bits 0:12 - when the amount of packet payload is greater than this value the process of receiving data is done.
    #[inline(always)]
    pub fn pkt_thrs(&self) -> PKT_THRS_R {
        PKT_THRS_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PKT_THRES").field("pkt_thrs", &self.pkt_thrs()).finish()
    }
}
impl W {
    ///Bits 0:12 - when the amount of packet payload is greater than this value the process of receiving data is done.
    #[inline(always)]
    #[must_use]
    pub fn pkt_thrs(&mut self) -> PKT_THRS_W<PKT_THRES_SPEC> {
        PKT_THRS_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pkt_thres::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pkt_thres::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PKT_THRES_SPEC;
impl crate::RegisterSpec for PKT_THRES_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pkt_thres::R`](R) reader structure
impl crate::Readable for PKT_THRES_SPEC {}
///`write(|w| ..)` method takes [`pkt_thres::W`](W) writer structure
impl crate::Writable for PKT_THRES_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PKT_THRES to value 0x80
impl crate::Resettable for PKT_THRES_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
