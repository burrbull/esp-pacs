#[doc = "Register `SRAM_CTRL` reader"]
pub type R = crate::R<SRAM_CTRL_SPEC>;
#[doc = "Register `SRAM_CTRL` writer"]
pub type W = crate::W<SRAM_CTRL_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_MEM_AUX_CTRL` reader - Control signals"]
pub type USB_SERIAL_JTAG_MEM_AUX_CTRL_R = crate::FieldReader<u16>;
#[doc = "Field `USB_SERIAL_JTAG_MEM_AUX_CTRL` writer - Control signals"]
pub type USB_SERIAL_JTAG_MEM_AUX_CTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Control signals"]
    #[inline(always)]
    pub fn usb_serial_jtag_mem_aux_ctrl(&self) -> USB_SERIAL_JTAG_MEM_AUX_CTRL_R {
        USB_SERIAL_JTAG_MEM_AUX_CTRL_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SRAM_CTRL")
            .field(
                "usb_serial_jtag_mem_aux_ctrl",
                &format_args!("{}", self.usb_serial_jtag_mem_aux_ctrl().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SRAM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:13 - Control signals"]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_mem_aux_ctrl(
        &mut self,
    ) -> USB_SERIAL_JTAG_MEM_AUX_CTRL_W<SRAM_CTRL_SPEC> {
        USB_SERIAL_JTAG_MEM_AUX_CTRL_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PPA SRAM Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRAM_CTRL_SPEC;
impl crate::RegisterSpec for SRAM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ctrl::R`](R) reader structure"]
impl crate::Readable for SRAM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sram_ctrl::W`](W) writer structure"]
impl crate::Writable for SRAM_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAM_CTRL to value 0x1320"]
impl crate::Resettable for SRAM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x1320;
}