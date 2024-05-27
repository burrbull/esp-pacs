///Register `TSENS_CTRL` reader
pub type R = crate::R<TSENS_CTRL_SPEC>;
///Register `TSENS_CTRL` writer
pub type W = crate::W<TSENS_CTRL_SPEC>;
///Field `OUT` reader - Need add description
pub type OUT_R = crate::FieldReader;
///Field `IN_INV` reader - Need add description
pub type IN_INV_R = crate::BitReader;
///Field `IN_INV` writer - Need add description
pub type IN_INV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLK_DIV` reader - Need add description
pub type CLK_DIV_R = crate::FieldReader;
///Field `CLK_DIV` writer - Need add description
pub type CLK_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `PU` reader - Need add description
pub type PU_R = crate::BitReader;
///Field `PU` writer - Need add description
pub type PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - Need add description
    #[inline(always)]
    pub fn out(&self) -> OUT_R {
        OUT_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 13 - Need add description
    #[inline(always)]
    pub fn in_inv(&self) -> IN_INV_R {
        IN_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:21 - Need add description
    #[inline(always)]
    pub fn clk_div(&self) -> CLK_DIV_R {
        CLK_DIV_R::new(((self.bits >> 14) & 0xff) as u8)
    }
    ///Bit 22 - Need add description
    #[inline(always)]
    pub fn pu(&self) -> PU_R {
        PU_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSENS_CTRL")
            .field("out", &self.out())
            .field("in_inv", &self.in_inv())
            .field("clk_div", &self.clk_div())
            .field("pu", &self.pu())
            .finish()
    }
}
impl W {
    ///Bit 13 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn in_inv(&mut self) -> IN_INV_W<TSENS_CTRL_SPEC> {
        IN_INV_W::new(self, 13)
    }
    ///Bits 14:21 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn clk_div(&mut self) -> CLK_DIV_W<TSENS_CTRL_SPEC> {
        CLK_DIV_W::new(self, 14)
    }
    ///Bit 22 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn pu(&mut self) -> PU_W<TSENS_CTRL_SPEC> {
        PU_W::new(self, 22)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`tsens_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsens_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TSENS_CTRL_SPEC;
impl crate::RegisterSpec for TSENS_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tsens_ctrl::R`](R) reader structure
impl crate::Readable for TSENS_CTRL_SPEC {}
///`write(|w| ..)` method takes [`tsens_ctrl::W`](W) writer structure
impl crate::Writable for TSENS_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TSENS_CTRL to value 0x0001_8000
impl crate::Resettable for TSENS_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0001_8000;
}
