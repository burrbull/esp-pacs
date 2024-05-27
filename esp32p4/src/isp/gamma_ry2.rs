///Register `GAMMA_RY2` reader
pub type R = crate::R<GAMMA_RY2_SPEC>;
///Register `GAMMA_RY2` writer
pub type W = crate::W<GAMMA_RY2_SPEC>;
///Field `GAMMA_R_Y07` reader - this field configures the point 7 of Y-axis of r channel gamma curve
pub type GAMMA_R_Y07_R = crate::FieldReader;
///Field `GAMMA_R_Y07` writer - this field configures the point 7 of Y-axis of r channel gamma curve
pub type GAMMA_R_Y07_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GAMMA_R_Y06` reader - this field configures the point 6 of Y-axis of r channel gamma curve
pub type GAMMA_R_Y06_R = crate::FieldReader;
///Field `GAMMA_R_Y06` writer - this field configures the point 6 of Y-axis of r channel gamma curve
pub type GAMMA_R_Y06_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GAMMA_R_Y05` reader - this field configures the point 5 of Y-axis of r channel gamma curve
pub type GAMMA_R_Y05_R = crate::FieldReader;
///Field `GAMMA_R_Y05` writer - this field configures the point 5 of Y-axis of r channel gamma curve
pub type GAMMA_R_Y05_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `GAMMA_R_Y04` reader - this field configures the point 4 of Y-axis of r channel gamma curve
pub type GAMMA_R_Y04_R = crate::FieldReader;
///Field `GAMMA_R_Y04` writer - this field configures the point 4 of Y-axis of r channel gamma curve
pub type GAMMA_R_Y04_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - this field configures the point 7 of Y-axis of r channel gamma curve
    #[inline(always)]
    pub fn gamma_r_y07(&self) -> GAMMA_R_Y07_R {
        GAMMA_R_Y07_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - this field configures the point 6 of Y-axis of r channel gamma curve
    #[inline(always)]
    pub fn gamma_r_y06(&self) -> GAMMA_R_Y06_R {
        GAMMA_R_Y06_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - this field configures the point 5 of Y-axis of r channel gamma curve
    #[inline(always)]
    pub fn gamma_r_y05(&self) -> GAMMA_R_Y05_R {
        GAMMA_R_Y05_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - this field configures the point 4 of Y-axis of r channel gamma curve
    #[inline(always)]
    pub fn gamma_r_y04(&self) -> GAMMA_R_Y04_R {
        GAMMA_R_Y04_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GAMMA_RY2")
            .field("gamma_r_y07", &self.gamma_r_y07())
            .field("gamma_r_y06", &self.gamma_r_y06())
            .field("gamma_r_y05", &self.gamma_r_y05())
            .field("gamma_r_y04", &self.gamma_r_y04())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - this field configures the point 7 of Y-axis of r channel gamma curve
    #[inline(always)]
    #[must_use]
    pub fn gamma_r_y07(&mut self) -> GAMMA_R_Y07_W<GAMMA_RY2_SPEC> {
        GAMMA_R_Y07_W::new(self, 0)
    }
    ///Bits 8:15 - this field configures the point 6 of Y-axis of r channel gamma curve
    #[inline(always)]
    #[must_use]
    pub fn gamma_r_y06(&mut self) -> GAMMA_R_Y06_W<GAMMA_RY2_SPEC> {
        GAMMA_R_Y06_W::new(self, 8)
    }
    ///Bits 16:23 - this field configures the point 5 of Y-axis of r channel gamma curve
    #[inline(always)]
    #[must_use]
    pub fn gamma_r_y05(&mut self) -> GAMMA_R_Y05_W<GAMMA_RY2_SPEC> {
        GAMMA_R_Y05_W::new(self, 16)
    }
    ///Bits 24:31 - this field configures the point 4 of Y-axis of r channel gamma curve
    #[inline(always)]
    #[must_use]
    pub fn gamma_r_y04(&mut self) -> GAMMA_R_Y04_W<GAMMA_RY2_SPEC> {
        GAMMA_R_Y04_W::new(self, 24)
    }
}
/**point of Y-axis of r channel gamma curve register 2

You can [`read`](crate::generic::Reg::read) this register and get [`gamma_ry2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gamma_ry2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct GAMMA_RY2_SPEC;
impl crate::RegisterSpec for GAMMA_RY2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`gamma_ry2::R`](R) reader structure
impl crate::Readable for GAMMA_RY2_SPEC {}
///`write(|w| ..)` method takes [`gamma_ry2::W`](W) writer structure
impl crate::Writable for GAMMA_RY2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GAMMA_RY2 to value 0x5060_7080
impl crate::Resettable for GAMMA_RY2_SPEC {
    const RESET_VALUE: u32 = 0x5060_7080;
}
