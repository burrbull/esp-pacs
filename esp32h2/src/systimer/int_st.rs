///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `TARGET(0-2)` reader - interupt%s status
pub type TARGET_R = crate::BitReader;
impl R {
    ///interupt(0-2) status
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `TARGET0` field
    #[inline(always)]
    pub fn target(&self, n: u8) -> TARGET_R {
        #[allow(clippy::no_effect)] [(); 3][n as usize];
        TARGET_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///interupt(0-2) status
    #[inline(always)]
    pub fn target_iter(&self) -> impl Iterator<Item = TARGET_R> + '_ {
        (0..3).map(move |n| TARGET_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - interupt0 status
    #[inline(always)]
    pub fn target0(&self) -> TARGET_R {
        TARGET_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - interupt1 status
    #[inline(always)]
    pub fn target1(&self) -> TARGET_R {
        TARGET_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - interupt2 status
    #[inline(always)]
    pub fn target2(&self) -> TARGET_R {
        TARGET_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("target0", &self.target0())
            .field("target1", &self.target1())
            .field("target2", &self.target2())
            .finish()
    }
}
/**systimer interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_st::R`](R) reader structure
impl crate::Readable for INT_ST_SPEC {}
///`reset()` method sets INT_ST to value 0
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
