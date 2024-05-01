///Register `SHARP_FILTER1` reader
pub type R = crate::R<SHARP_FILTER1_SPEC>;
///Register `SHARP_FILTER1` writer
pub type W = crate::W<SHARP_FILTER1_SPEC>;
///Field `SHARP_FILTER_COE10` reader - this field configures usm filter coefficient
pub type SHARP_FILTER_COE10_R = crate::FieldReader;
///Field `SHARP_FILTER_COE10` writer - this field configures usm filter coefficient
pub type SHARP_FILTER_COE10_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SHARP_FILTER_COE11` reader - this field configures usm filter coefficient
pub type SHARP_FILTER_COE11_R = crate::FieldReader;
///Field `SHARP_FILTER_COE11` writer - this field configures usm filter coefficient
pub type SHARP_FILTER_COE11_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SHARP_FILTER_COE12` reader - this field configures usm filter coefficient
pub type SHARP_FILTER_COE12_R = crate::FieldReader;
///Field `SHARP_FILTER_COE12` writer - this field configures usm filter coefficient
pub type SHARP_FILTER_COE12_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:4 - this field configures usm filter coefficient
    #[inline(always)]
    pub fn sharp_filter_coe10(&self) -> SHARP_FILTER_COE10_R {
        SHARP_FILTER_COE10_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 5:9 - this field configures usm filter coefficient
    #[inline(always)]
    pub fn sharp_filter_coe11(&self) -> SHARP_FILTER_COE11_R {
        SHARP_FILTER_COE11_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    ///Bits 10:14 - this field configures usm filter coefficient
    #[inline(always)]
    pub fn sharp_filter_coe12(&self) -> SHARP_FILTER_COE12_R {
        SHARP_FILTER_COE12_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHARP_FILTER1")
            .field("sharp_filter_coe10", &self.sharp_filter_coe10())
            .field("sharp_filter_coe11", &self.sharp_filter_coe11())
            .field("sharp_filter_coe12", &self.sharp_filter_coe12())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - this field configures usm filter coefficient
    #[inline(always)]
    #[must_use]
    pub fn sharp_filter_coe10(&mut self) -> SHARP_FILTER_COE10_W<SHARP_FILTER1_SPEC> {
        SHARP_FILTER_COE10_W::new(self, 0)
    }
    ///Bits 5:9 - this field configures usm filter coefficient
    #[inline(always)]
    #[must_use]
    pub fn sharp_filter_coe11(&mut self) -> SHARP_FILTER_COE11_W<SHARP_FILTER1_SPEC> {
        SHARP_FILTER_COE11_W::new(self, 5)
    }
    ///Bits 10:14 - this field configures usm filter coefficient
    #[inline(always)]
    #[must_use]
    pub fn sharp_filter_coe12(&mut self) -> SHARP_FILTER_COE12_W<SHARP_FILTER1_SPEC> {
        SHARP_FILTER_COE12_W::new(self, 10)
    }
}
/**sharp usm config register 1

You can [`read`](crate::generic::Reg::read) this register and get [`sharp_filter1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sharp_filter1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SHARP_FILTER1_SPEC;
impl crate::RegisterSpec for SHARP_FILTER1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sharp_filter1::R`](R) reader structure
impl crate::Readable for SHARP_FILTER1_SPEC {}
///`write(|w| ..)` method takes [`sharp_filter1::W`](W) writer structure
impl crate::Writable for SHARP_FILTER1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SHARP_FILTER1 to value 0x0882
impl crate::Resettable for SHARP_FILTER1_SPEC {
    const RESET_VALUE: u32 = 0x0882;
}
