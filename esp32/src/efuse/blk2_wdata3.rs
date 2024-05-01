///Register `BLK2_WDATA3` reader
pub type R = crate::R<BLK2_WDATA3_SPEC>;
///Register `BLK2_WDATA3` writer
pub type W = crate::W<BLK2_WDATA3_SPEC>;
///Field `BLK2_DIN3` reader -
pub type BLK2_DIN3_R = crate::FieldReader<u32>;
///Field `BLK2_DIN3` writer -
pub type BLK2_DIN3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn blk2_din3(&self) -> BLK2_DIN3_R {
        BLK2_DIN3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK2_WDATA3")
            .field("blk2_din3", &self.blk2_din3())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn blk2_din3(&mut self) -> BLK2_DIN3_W<BLK2_WDATA3_SPEC> {
        BLK2_DIN3_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`blk2_wdata3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk2_wdata3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BLK2_WDATA3_SPEC;
impl crate::RegisterSpec for BLK2_WDATA3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`blk2_wdata3::R`](R) reader structure
impl crate::Readable for BLK2_WDATA3_SPEC {}
///`write(|w| ..)` method takes [`blk2_wdata3::W`](W) writer structure
impl crate::Writable for BLK2_WDATA3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BLK2_WDATA3 to value 0
impl crate::Resettable for BLK2_WDATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
