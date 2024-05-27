///Register `U%s_STATUS` reader
pub type R = crate::R<U_STATUS_SPEC>;
///Field `ZERO_MODE` reader - The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
pub type ZERO_MODE_R = crate::FieldReader;
///Field `THRES1` reader - The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others.
pub type THRES1_R = crate::BitReader;
///Field `THRES0` reader - The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others.
pub type THRES0_R = crate::BitReader;
///Field `L_LIM` reader - The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others.
pub type L_LIM_R = crate::BitReader;
///Field `H_LIM` reader - The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others.
pub type H_LIM_R = crate::BitReader;
///Field `ZERO` reader - The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others.
pub type ZERO_R = crate::BitReader;
impl R {
    ///Bits 0:1 - The pulse counter status of PCNT_U%s corresponding to 0. 0: pulse counter decreases from positive to 0. 1: pulse counter increases from negative to 0. 2: pulse counter is negative. 3: pulse counter is positive.
    #[inline(always)]
    pub fn zero_mode(&self) -> ZERO_MODE_R {
        ZERO_MODE_R::new((self.bits & 3) as u8)
    }
    ///Bit 2 - The latched value of thres1 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres1 and thres1 event is valid. 0: others.
    #[inline(always)]
    pub fn thres1(&self) -> THRES1_R {
        THRES1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The latched value of thres0 event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thres0 and thres0 event is valid. 0: others.
    #[inline(always)]
    pub fn thres0(&self) -> THRES0_R {
        THRES0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The latched value of low limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_l_lim and low limit event is valid. 0: others.
    #[inline(always)]
    pub fn l_lim(&self) -> L_LIM_R {
        L_LIM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The latched value of high limit event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to thr_h_lim and high limit event is valid. 0: others.
    #[inline(always)]
    pub fn h_lim(&self) -> H_LIM_R {
        H_LIM_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The latched value of zero threshold event of PCNT_U%s when threshold event interrupt is valid. 1: the current pulse counter equals to 0 and zero threshold event is valid. 0: others.
    #[inline(always)]
    pub fn zero(&self) -> ZERO_R {
        ZERO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_STATUS")
            .field("zero_mode", &self.zero_mode())
            .field("thres1", &self.thres1())
            .field("thres0", &self.thres0())
            .field("l_lim", &self.l_lim())
            .field("h_lim", &self.h_lim())
            .field("zero", &self.zero())
            .finish()
    }
}
/**PNCT UNIT%s status register

You can [`read`](crate::generic::Reg::read) this register and get [`u_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct U_STATUS_SPEC;
impl crate::RegisterSpec for U_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`u_status::R`](R) reader structure
impl crate::Readable for U_STATUS_SPEC {}
///`reset()` method sets U%s_STATUS to value 0
impl crate::Resettable for U_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
