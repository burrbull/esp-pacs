///Register `INTR_RCGN` reader
pub type R = crate::R<INTR_RCGN_SPEC>;
///Register `INTR_RCGN` writer
pub type W = crate::W<INTR_RCGN_SPEC>;
///Field `INTR_MODE_CH(0-7)` reader - Configure channel %s interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
pub type INTR_MODE_CH_R = crate::FieldReader;
///Field `INTR_MODE_CH(0-7)` writer - Configure channel %s interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
pub type INTR_MODE_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Configure channel (0-7) interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `INTR_MODE_CH0` field
    #[inline(always)]
    pub fn intr_mode_ch(&self, n: u8) -> INTR_MODE_CH_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        INTR_MODE_CH_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    ///Iterator for array of:
    ///Configure channel (0-7) interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    pub fn intr_mode_ch_iter(&self) -> impl Iterator<Item = INTR_MODE_CH_R> + '_ {
        (0..8).map(move |n| INTR_MODE_CH_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    ///Bits 0:2 - Configure channel 0 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    pub fn intr_mode_ch0(&self) -> INTR_MODE_CH_R {
        INTR_MODE_CH_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:5 - Configure channel 1 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    pub fn intr_mode_ch1(&self) -> INTR_MODE_CH_R {
        INTR_MODE_CH_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bits 6:8 - Configure channel 2 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    pub fn intr_mode_ch2(&self) -> INTR_MODE_CH_R {
        INTR_MODE_CH_R::new(((self.bits >> 6) & 7) as u8)
    }
    ///Bits 9:11 - Configure channel 3 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    pub fn intr_mode_ch3(&self) -> INTR_MODE_CH_R {
        INTR_MODE_CH_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:14 - Configure channel 4 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    pub fn intr_mode_ch4(&self) -> INTR_MODE_CH_R {
        INTR_MODE_CH_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 15:17 - Configure channel 5 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    pub fn intr_mode_ch5(&self) -> INTR_MODE_CH_R {
        INTR_MODE_CH_R::new(((self.bits >> 15) & 7) as u8)
    }
    ///Bits 18:20 - Configure channel 6 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    pub fn intr_mode_ch6(&self) -> INTR_MODE_CH_R {
        INTR_MODE_CH_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bits 21:23 - Configure channel 7 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    pub fn intr_mode_ch7(&self) -> INTR_MODE_CH_R {
        INTR_MODE_CH_R::new(((self.bits >> 21) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_RCGN")
            .field("intr_mode_ch0", &self.intr_mode_ch0())
            .field("intr_mode_ch1", &self.intr_mode_ch1())
            .field("intr_mode_ch2", &self.intr_mode_ch2())
            .field("intr_mode_ch3", &self.intr_mode_ch3())
            .field("intr_mode_ch4", &self.intr_mode_ch4())
            .field("intr_mode_ch5", &self.intr_mode_ch5())
            .field("intr_mode_ch6", &self.intr_mode_ch6())
            .field("intr_mode_ch7", &self.intr_mode_ch7())
            .finish()
    }
}
impl W {
    ///Configure channel (0-7) interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `INTR_MODE_CH0` field
    #[inline(always)]
    #[must_use]
    pub fn intr_mode_ch(&mut self, n: u8) -> INTR_MODE_CH_W<INTR_RCGN_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        INTR_MODE_CH_W::new(self, n * 3)
    }
    ///Bits 0:2 - Configure channel 0 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    #[must_use]
    pub fn intr_mode_ch0(&mut self) -> INTR_MODE_CH_W<INTR_RCGN_SPEC> {
        INTR_MODE_CH_W::new(self, 0)
    }
    ///Bits 3:5 - Configure channel 1 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    #[must_use]
    pub fn intr_mode_ch1(&mut self) -> INTR_MODE_CH_W<INTR_RCGN_SPEC> {
        INTR_MODE_CH_W::new(self, 3)
    }
    ///Bits 6:8 - Configure channel 2 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    #[must_use]
    pub fn intr_mode_ch2(&mut self) -> INTR_MODE_CH_W<INTR_RCGN_SPEC> {
        INTR_MODE_CH_W::new(self, 6)
    }
    ///Bits 9:11 - Configure channel 3 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    #[must_use]
    pub fn intr_mode_ch3(&mut self) -> INTR_MODE_CH_W<INTR_RCGN_SPEC> {
        INTR_MODE_CH_W::new(self, 9)
    }
    ///Bits 12:14 - Configure channel 4 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    #[must_use]
    pub fn intr_mode_ch4(&mut self) -> INTR_MODE_CH_W<INTR_RCGN_SPEC> {
        INTR_MODE_CH_W::new(self, 12)
    }
    ///Bits 15:17 - Configure channel 5 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    #[must_use]
    pub fn intr_mode_ch5(&mut self) -> INTR_MODE_CH_W<INTR_RCGN_SPEC> {
        INTR_MODE_CH_W::new(self, 15)
    }
    ///Bits 18:20 - Configure channel 6 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    #[must_use]
    pub fn intr_mode_ch6(&mut self) -> INTR_MODE_CH_W<INTR_RCGN_SPEC> {
        INTR_MODE_CH_W::new(self, 18)
    }
    ///Bits 21:23 - Configure channel 7 interrupt generate mode. 0/1: do not generate interrupt. 2: low level trigger. 3: high level trigger. 4: falling edge trigger. 5: raising edge trigger. 6/7: falling and raising edge trigger.
    #[inline(always)]
    #[must_use]
    pub fn intr_mode_ch7(&mut self) -> INTR_MODE_CH_W<INTR_RCGN_SPEC> {
        INTR_MODE_CH_W::new(self, 21)
    }
}
/**Dedicated GPIO interrupts generation mode register

You can [`read`](crate::generic::Reg::read) this register and get [`intr_rcgn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_rcgn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INTR_RCGN_SPEC;
impl crate::RegisterSpec for INTR_RCGN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`intr_rcgn::R`](R) reader structure
impl crate::Readable for INTR_RCGN_SPEC {}
///`write(|w| ..)` method takes [`intr_rcgn::W`](W) writer structure
impl crate::Writable for INTR_RCGN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INTR_RCGN to value 0
impl crate::Resettable for INTR_RCGN_SPEC {
    const RESET_VALUE: u32 = 0;
}
