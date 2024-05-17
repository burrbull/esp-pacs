///Register `BIST_CTRL` reader
pub type R = crate::R<BIST_CTRL_SPEC>;
///Register `BIST_CTRL` writer
pub type W = crate::W<BIST_CTRL_SPEC>;
///Field `BIST_PAD_OE` reader - High speed sdio pad bist out pad oe
pub type BIST_PAD_OE_R = crate::BitReader;
///Field `BIST_PAD_OE` writer - High speed sdio pad bist out pad oe
pub type BIST_PAD_OE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIST_START` writer - High speed sdio pad bist start
pub type BIST_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - High speed sdio pad bist out pad oe
    #[inline(always)]
    pub fn bist_pad_oe(&self) -> BIST_PAD_OE_R {
        BIST_PAD_OE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BIST_CTRL").field("bist_pad_oe", &self.bist_pad_oe()).finish()
    }
}
impl W {
    ///Bit 0 - High speed sdio pad bist out pad oe
    #[inline(always)]
    #[must_use]
    pub fn bist_pad_oe(&mut self) -> BIST_PAD_OE_W<BIST_CTRL_SPEC> {
        BIST_PAD_OE_W::new(self, 0)
    }
    ///Bit 1 - High speed sdio pad bist start
    #[inline(always)]
    #[must_use]
    pub fn bist_start(&mut self) -> BIST_START_W<BIST_CTRL_SPEC> {
        BIST_START_W::new(self, 1)
    }
}
/**High speed sdio pad bist control

You can [`read`](crate::generic::Reg::read) this register and get [`bist_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bist_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct BIST_CTRL_SPEC;
impl crate::RegisterSpec for BIST_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`bist_ctrl::R`](R) reader structure
impl crate::Readable for BIST_CTRL_SPEC {}
///`write(|w| ..)` method takes [`bist_ctrl::W`](W) writer structure
impl crate::Writable for BIST_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets BIST_CTRL to value 0x01
impl crate::Resettable for BIST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
