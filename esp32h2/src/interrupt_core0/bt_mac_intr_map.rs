#[doc = "Register `BT_MAC_INTR_MAP` reader"]
pub type R = crate::R<BT_MAC_INTR_MAP_SPEC>;
#[doc = "Register `BT_MAC_INTR_MAP` writer"]
pub type W = crate::W<BT_MAC_INTR_MAP_SPEC>;
#[doc = "Field `BT_MAC_INTR_MAP` reader - CORE0_BT_MAC_INTR mapping register"]
pub type BT_MAC_INTR_MAP_R = crate::FieldReader;
#[doc = "Field `BT_MAC_INTR_MAP` writer - CORE0_BT_MAC_INTR mapping register"]
pub type BT_MAC_INTR_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - CORE0_BT_MAC_INTR mapping register"]
    #[inline(always)]
    pub fn bt_mac_intr_map(&self) -> BT_MAC_INTR_MAP_R {
        BT_MAC_INTR_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BT_MAC_INTR_MAP")
            .field("bt_mac_intr_map", &self.bt_mac_intr_map().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BT_MAC_INTR_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - CORE0_BT_MAC_INTR mapping register"]
    #[inline(always)]
    #[must_use]
    pub fn bt_mac_intr_map(&mut self) -> BT_MAC_INTR_MAP_W<BT_MAC_INTR_MAP_SPEC> {
        BT_MAC_INTR_MAP_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bt_mac_intr_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_mac_intr_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BT_MAC_INTR_MAP_SPEC;
impl crate::RegisterSpec for BT_MAC_INTR_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bt_mac_intr_map::R`](R) reader structure"]
impl crate::Readable for BT_MAC_INTR_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bt_mac_intr_map::W`](W) writer structure"]
impl crate::Writable for BT_MAC_INTR_MAP_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BT_MAC_INTR_MAP to value 0"]
impl crate::Resettable for BT_MAC_INTR_MAP_SPEC {}
