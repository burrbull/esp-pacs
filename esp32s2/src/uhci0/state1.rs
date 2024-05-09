#[doc = "Register `STATE1` reader"]
pub type R = crate::R<STATE1_SPEC>;
#[doc = "Field `OUTLINK_DSCR_ADDR` reader - This register stores the current transmit descriptor's address."]
pub type OUTLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_DSCR_STATE` reader - Reserved."]
pub type OUT_DSCR_STATE_R = crate::FieldReader;
#[doc = "Field `OUT_STATE` reader - Reserved."]
pub type OUT_STATE_R = crate::FieldReader;
#[doc = "Field `OUTFIFO_CNT` reader - This register stores the number of data bytes in TX FIFO."]
pub type OUTFIFO_CNT_R = crate::FieldReader;
#[doc = "Field `ENCODE_STATE` reader - UHCI encoder status."]
pub type ENCODE_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current transmit descriptor's address."]
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OUTLINK_DSCR_ADDR_R {
        OUTLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - Reserved."]
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OUT_DSCR_STATE_R {
        OUT_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - Reserved."]
    #[inline(always)]
    pub fn out_state(&self) -> OUT_STATE_R {
        OUT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:27 - This register stores the number of data bytes in TX FIFO."]
    #[inline(always)]
    pub fn outfifo_cnt(&self) -> OUTFIFO_CNT_R {
        OUTFIFO_CNT_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:30 - UHCI encoder status."]
    #[inline(always)]
    pub fn encode_state(&self) -> ENCODE_STATE_R {
        ENCODE_STATE_R::new(((self.bits >> 28) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE1")
            .field("outlink_dscr_addr", &self.outlink_dscr_addr().bits())
            .field("out_dscr_state", &self.out_dscr_state().bits())
            .field("out_state", &self.out_state().bits())
            .field("outfifo_cnt", &self.outfifo_cnt().bits())
            .field("encode_state", &self.encode_state().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "UHCI encoder status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATE1_SPEC;
impl crate::RegisterSpec for STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state1::R`](R) reader structure"]
impl crate::Readable for STATE1_SPEC {}
#[doc = "`reset()` method sets STATE1 to value 0"]
impl crate::Resettable for STATE1_SPEC {}
