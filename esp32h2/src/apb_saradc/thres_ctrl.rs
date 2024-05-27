///Register `THRES_CTRL` reader
pub type R = crate::R<THRES_CTRL_SPEC>;
///Register `THRES_CTRL` writer
pub type W = crate::W<THRES_CTRL_SPEC>;
///Field `THRES_ALL_EN` reader - enable thres to all channel
pub type THRES_ALL_EN_R = crate::BitReader;
///Field `THRES_ALL_EN` writer - enable thres to all channel
pub type THRES_ALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THRES1_EN` reader - enable thres1
pub type THRES1_EN_R = crate::BitReader;
///Field `THRES1_EN` writer - enable thres1
pub type THRES1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `THRES0_EN` reader - enable thres0
pub type THRES0_EN_R = crate::BitReader;
///Field `THRES0_EN` writer - enable thres0
pub type THRES0_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 27 - enable thres to all channel
    #[inline(always)]
    pub fn thres_all_en(&self) -> THRES_ALL_EN_R {
        THRES_ALL_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 30 - enable thres1
    #[inline(always)]
    pub fn thres1_en(&self) -> THRES1_EN_R {
        THRES1_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - enable thres0
    #[inline(always)]
    pub fn thres0_en(&self) -> THRES0_EN_R {
        THRES0_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("THRES_CTRL")
            .field("thres_all_en", &self.thres_all_en())
            .field("thres1_en", &self.thres1_en())
            .field("thres0_en", &self.thres0_en())
            .finish()
    }
}
impl W {
    ///Bit 27 - enable thres to all channel
    #[inline(always)]
    #[must_use]
    pub fn thres_all_en(&mut self) -> THRES_ALL_EN_W<THRES_CTRL_SPEC> {
        THRES_ALL_EN_W::new(self, 27)
    }
    ///Bit 30 - enable thres1
    #[inline(always)]
    #[must_use]
    pub fn thres1_en(&mut self) -> THRES1_EN_W<THRES_CTRL_SPEC> {
        THRES1_EN_W::new(self, 30)
    }
    ///Bit 31 - enable thres0
    #[inline(always)]
    #[must_use]
    pub fn thres0_en(&mut self) -> THRES0_EN_W<THRES_CTRL_SPEC> {
        THRES0_EN_W::new(self, 31)
    }
}
/**digital saradc configure register

You can [`read`](crate::generic::Reg::read) this register and get [`thres_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thres_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct THRES_CTRL_SPEC;
impl crate::RegisterSpec for THRES_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`thres_ctrl::R`](R) reader structure
impl crate::Readable for THRES_CTRL_SPEC {}
///`write(|w| ..)` method takes [`thres_ctrl::W`](W) writer structure
impl crate::Writable for THRES_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets THRES_CTRL to value 0
impl crate::Resettable for THRES_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
