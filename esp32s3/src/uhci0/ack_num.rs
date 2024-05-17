///Register `ACK_NUM` reader
pub type R = crate::R<ACK_NUM_SPEC>;
///Register `ACK_NUM` writer
pub type W = crate::W<ACK_NUM_SPEC>;
///Field `ACK_NUM` reader - This ACK number used in software flow control.
pub type ACK_NUM_R = crate::FieldReader;
///Field `ACK_NUM` writer - This ACK number used in software flow control.
pub type ACK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `LOAD` writer - Set this bit to 1, the value configured by UHCI_ACK_NUM would be loaded.
pub type LOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - This ACK number used in software flow control.
    #[inline(always)]
    pub fn ack_num(&self) -> ACK_NUM_R {
        ACK_NUM_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACK_NUM").field("ack_num", &self.ack_num()).finish()
    }
}
impl W {
    ///Bits 0:2 - This ACK number used in software flow control.
    #[inline(always)]
    #[must_use]
    pub fn ack_num(&mut self) -> ACK_NUM_W<ACK_NUM_SPEC> {
        ACK_NUM_W::new(self, 0)
    }
    ///Bit 3 - Set this bit to 1, the value configured by UHCI_ACK_NUM would be loaded.
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<ACK_NUM_SPEC> {
        LOAD_W::new(self, 3)
    }
}
/**UHCI ACK number configuration

You can [`read`](crate::generic::Reg::read) this register and get [`ack_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ack_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ACK_NUM_SPEC;
impl crate::RegisterSpec for ACK_NUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ack_num::R`](R) reader structure
impl crate::Readable for ACK_NUM_SPEC {}
///`write(|w| ..)` method takes [`ack_num::W`](W) writer structure
impl crate::Writable for ACK_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ACK_NUM to value 0x08
impl crate::Resettable for ACK_NUM_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
