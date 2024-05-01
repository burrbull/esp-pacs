///Register `RX_TAILING_BITS` reader
pub type R = crate::R<RX_TAILING_BITS_SPEC>;
///Register `RX_TAILING_BITS` writer
pub type W = crate::W<RX_TAILING_BITS_SPEC>;
///Field `RX_TAILING_BITS` reader - write this bits to specify the extra data bit length after getting EOF
pub type RX_TAILING_BITS_R = crate::FieldReader<u16>;
///Field `RX_TAILING_BITS` writer - write this bits to specify the extra data bit length after getting EOF
pub type RX_TAILING_BITS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    pub fn rx_tailing_bits(&self) -> RX_TAILING_BITS_R {
        RX_TAILING_BITS_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_TAILING_BITS")
            .field("rx_tailing_bits", &self.rx_tailing_bits())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - write this bits to specify the extra data bit length after getting EOF
    #[inline(always)]
    #[must_use]
    pub fn rx_tailing_bits(&mut self) -> RX_TAILING_BITS_W<RX_TAILING_BITS_SPEC> {
        RX_TAILING_BITS_W::new(self, 0)
    }
}
/**Control and configuration registers

You can [`read`](crate::generic::Reg::read) this register and get [`rx_tailing_bits::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_tailing_bits::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RX_TAILING_BITS_SPEC;
impl crate::RegisterSpec for RX_TAILING_BITS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rx_tailing_bits::R`](R) reader structure
impl crate::Readable for RX_TAILING_BITS_SPEC {}
///`write(|w| ..)` method takes [`rx_tailing_bits::W`](W) writer structure
impl crate::Writable for RX_TAILING_BITS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RX_TAILING_BITS to value 0
impl crate::Resettable for RX_TAILING_BITS_SPEC {
    const RESET_VALUE: u32 = 0;
}
