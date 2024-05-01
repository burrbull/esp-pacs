///Register `INF1_EXTEND_ADDR1` reader
pub type R = crate::R<INF1_EXTEND_ADDR1_SPEC>;
///Register `INF1_EXTEND_ADDR1` writer
pub type W = crate::W<INF1_EXTEND_ADDR1_SPEC>;
///Field `MAC_INF1_EXTEND_ADDR1` reader -
pub type MAC_INF1_EXTEND_ADDR1_R = crate::FieldReader<u32>;
///Field `MAC_INF1_EXTEND_ADDR1` writer -
pub type MAC_INF1_EXTEND_ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn mac_inf1_extend_addr1(&self) -> MAC_INF1_EXTEND_ADDR1_R {
        MAC_INF1_EXTEND_ADDR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INF1_EXTEND_ADDR1")
            .field("mac_inf1_extend_addr1", &self.mac_inf1_extend_addr1())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn mac_inf1_extend_addr1(&mut self) -> MAC_INF1_EXTEND_ADDR1_W<INF1_EXTEND_ADDR1_SPEC> {
        MAC_INF1_EXTEND_ADDR1_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`inf1_extend_addr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inf1_extend_addr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INF1_EXTEND_ADDR1_SPEC;
impl crate::RegisterSpec for INF1_EXTEND_ADDR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`inf1_extend_addr1::R`](R) reader structure
impl crate::Readable for INF1_EXTEND_ADDR1_SPEC {}
///`write(|w| ..)` method takes [`inf1_extend_addr1::W`](W) writer structure
impl crate::Writable for INF1_EXTEND_ADDR1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INF1_EXTEND_ADDR1 to value 0
impl crate::Resettable for INF1_EXTEND_ADDR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
