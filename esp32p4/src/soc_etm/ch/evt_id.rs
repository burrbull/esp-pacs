///Register `EVT_ID` reader
pub type R = crate::R<EVT_ID_SPEC>;
///Register `EVT_ID` writer
pub type W = crate::W<EVT_ID_SPEC>;
///Field `EVT_ID` reader - Configures ch0_evt_id
pub type EVT_ID_R = crate::FieldReader;
///Field `EVT_ID` writer - Configures ch0_evt_id
pub type EVT_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Configures ch0_evt_id
    #[inline(always)]
    pub fn evt_id(&self) -> EVT_ID_R {
        EVT_ID_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ID")
            .field("evt_id", &self.evt_id())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Configures ch0_evt_id
    #[inline(always)]
    #[must_use]
    pub fn evt_id(&mut self) -> EVT_ID_W<EVT_ID_SPEC> {
        EVT_ID_W::new(self, 0)
    }
}
/**Channel0 event id register

You can [`read`](crate::generic::Reg::read) this register and get [`evt_id::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_id::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EVT_ID_SPEC;
impl crate::RegisterSpec for EVT_ID_SPEC {
    type Ux = u32;
}
///`read()` method returns [`evt_id::R`](R) reader structure
impl crate::Readable for EVT_ID_SPEC {}
///`write(|w| ..)` method takes [`evt_id::W`](W) writer structure
impl crate::Writable for EVT_ID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EVT_ID to value 0
impl crate::Resettable for EVT_ID_SPEC {
    const RESET_VALUE: u32 = 0;
}
