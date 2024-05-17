///Register `TX_ARB_WEIGH_OPT_DIR` reader
pub type R = crate::R<TX_ARB_WEIGH_OPT_DIR_SPEC>;
///Register `TX_ARB_WEIGH_OPT_DIR` writer
pub type W = crate::W<TX_ARB_WEIGH_OPT_DIR_SPEC>;
///Field `TX_ARB_WEIGH_OPT_DIR` reader - reserved
pub type TX_ARB_WEIGH_OPT_DIR_R = crate::BitReader;
///Field `TX_ARB_WEIGH_OPT_DIR` writer - reserved
pub type TX_ARB_WEIGH_OPT_DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - reserved
    #[inline(always)]
    pub fn tx_arb_weigh_opt_dir(&self) -> TX_ARB_WEIGH_OPT_DIR_R {
        TX_ARB_WEIGH_OPT_DIR_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_ARB_WEIGH_OPT_DIR")
            .field("tx_arb_weigh_opt_dir", &self.tx_arb_weigh_opt_dir())
            .finish()
    }
}
impl W {
    ///Bit 0 - reserved
    #[inline(always)]
    #[must_use]
    pub fn tx_arb_weigh_opt_dir(
        &mut self,
    ) -> TX_ARB_WEIGH_OPT_DIR_W<TX_ARB_WEIGH_OPT_DIR_SPEC> {
        TX_ARB_WEIGH_OPT_DIR_W::new(self, 0)
    }
}
/**This register is used to config off or on weigh optimization

You can [`read`](crate::generic::Reg::read) this register and get [`tx_arb_weigh_opt_dir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_arb_weigh_opt_dir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TX_ARB_WEIGH_OPT_DIR_SPEC;
impl crate::RegisterSpec for TX_ARB_WEIGH_OPT_DIR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tx_arb_weigh_opt_dir::R`](R) reader structure
impl crate::Readable for TX_ARB_WEIGH_OPT_DIR_SPEC {}
///`write(|w| ..)` method takes [`tx_arb_weigh_opt_dir::W`](W) writer structure
impl crate::Writable for TX_ARB_WEIGH_OPT_DIR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TX_ARB_WEIGH_OPT_DIR to value 0
impl crate::Resettable for TX_ARB_WEIGH_OPT_DIR_SPEC {
    const RESET_VALUE: u32 = 0;
}
