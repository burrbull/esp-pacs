///Register `RINTSTS` reader
pub type R = crate::R<RINTSTS_SPEC>;
///Register `RINTSTS` writer
pub type W = crate::W<RINTSTS_SPEC>;
///Field `INT_STATUS_RAW` reader - Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect.
pub type INT_STATUS_RAW_R = crate::FieldReader<u16>;
///Field `INT_STATUS_RAW` writer - Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect.
pub type INT_STATUS_RAW_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `SDIO_INTERRUPT_RAW` reader - Interrupt from SDIO card, one bit for each card. Bit\[17:16\] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect. 0: No SDIO interrupt from card; 1: SDIO interrupt from card.
pub type SDIO_INTERRUPT_RAW_R = crate::FieldReader;
///Field `SDIO_INTERRUPT_RAW` writer - Interrupt from SDIO card, one bit for each card. Bit\[17:16\] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect. 0: No SDIO interrupt from card; 1: SDIO interrupt from card.
pub type SDIO_INTERRUPT_RAW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bits 0:15 - Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect.
    #[inline(always)]
    pub fn int_status_raw(&self) -> INT_STATUS_RAW_R {
        INT_STATUS_RAW_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:17 - Interrupt from SDIO card, one bit for each card. Bit\[17:16\] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect. 0: No SDIO interrupt from card; 1: SDIO interrupt from card.
    #[inline(always)]
    pub fn sdio_interrupt_raw(&self) -> SDIO_INTERRUPT_RAW_R {
        SDIO_INTERRUPT_RAW_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RINTSTS")
            .field("int_status_raw", &self.int_status_raw())
            .field("sdio_interrupt_raw", &self.sdio_interrupt_raw())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Setting a bit clears the corresponding interrupt and writing 0 has no effect. Bits are logged regardless of interrupt mask status. Bit 15 (EBE): End-bit error/no CRC error; Bit 14 (ACD): Auto command done; Bit 13 (SBE/BCI): RX Start Bit Error; Bit 12 (HLE): Hardware locked write error; Bit 11 (FRUN): FIFO underrun/overrun error; Bit 10 (HTO): Data starvation by host timeout (HTO); Bit 9 (DTRO): Data read timeout; Bit 8 (RTO): Response timeout; Bit 7 (DCRC): Data CRC error; Bit 6 (RCRC): Response CRC error; Bit 5 (RXDR): Receive FIFO data request; Bit 4 (TXDR): Transmit FIFO data request; Bit 3 (DTO): Data transfer over; Bit 2 (CD): Command done; Bit 1 (RE): Response error; Bit 0 (CD): Card detect.
    #[inline(always)]
    #[must_use]
    pub fn int_status_raw(&mut self) -> INT_STATUS_RAW_W<RINTSTS_SPEC> {
        INT_STATUS_RAW_W::new(self, 0)
    }
    ///Bits 16:17 - Interrupt from SDIO card, one bit for each card. Bit\[17:16\] correspond to card1 and card0, respectively. Setting a bit clears the corresponding interrupt bit and writing 0 has no effect. 0: No SDIO interrupt from card; 1: SDIO interrupt from card.
    #[inline(always)]
    #[must_use]
    pub fn sdio_interrupt_raw(&mut self) -> SDIO_INTERRUPT_RAW_W<RINTSTS_SPEC> {
        SDIO_INTERRUPT_RAW_W::new(self, 16)
    }
}
/**Raw interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`rintsts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rintsts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RINTSTS_SPEC;
impl crate::RegisterSpec for RINTSTS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rintsts::R`](R) reader structure
impl crate::Readable for RINTSTS_SPEC {}
///`write(|w| ..)` method takes [`rintsts::W`](W) writer structure
impl crate::Writable for RINTSTS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RINTSTS to value 0
impl crate::Resettable for RINTSTS_SPEC {
    const RESET_VALUE: u32 = 0;
}
