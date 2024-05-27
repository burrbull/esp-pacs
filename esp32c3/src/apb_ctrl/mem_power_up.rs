///Register `MEM_POWER_UP` reader
pub type R = crate::R<MEM_POWER_UP_SPEC>;
///Register `MEM_POWER_UP` writer
pub type W = crate::W<MEM_POWER_UP_SPEC>;
///Field `ROM_POWER_UP` reader - reg_rom_power_up
pub type ROM_POWER_UP_R = crate::FieldReader;
///Field `ROM_POWER_UP` writer - reg_rom_power_up
pub type ROM_POWER_UP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SRAM_POWER_UP` reader - reg_sram_power_up
pub type SRAM_POWER_UP_R = crate::FieldReader;
///Field `SRAM_POWER_UP` writer - reg_sram_power_up
pub type SRAM_POWER_UP_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - reg_rom_power_up
    #[inline(always)]
    pub fn rom_power_up(&self) -> ROM_POWER_UP_R {
        ROM_POWER_UP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:5 - reg_sram_power_up
    #[inline(always)]
    pub fn sram_power_up(&self) -> SRAM_POWER_UP_R {
        SRAM_POWER_UP_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_POWER_UP")
            .field("rom_power_up", &self.rom_power_up())
            .field("sram_power_up", &self.sram_power_up())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - reg_rom_power_up
    #[inline(always)]
    #[must_use]
    pub fn rom_power_up(&mut self) -> ROM_POWER_UP_W<MEM_POWER_UP_SPEC> {
        ROM_POWER_UP_W::new(self, 0)
    }
    ///Bits 2:5 - reg_sram_power_up
    #[inline(always)]
    #[must_use]
    pub fn sram_power_up(&mut self) -> SRAM_POWER_UP_W<MEM_POWER_UP_SPEC> {
        SRAM_POWER_UP_W::new(self, 2)
    }
}
/**APB_CTRL_MEM_POWER_UP_REG

You can [`read`](crate::generic::Reg::read) this register and get [`mem_power_up::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_power_up::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MEM_POWER_UP_SPEC;
impl crate::RegisterSpec for MEM_POWER_UP_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mem_power_up::R`](R) reader structure
impl crate::Readable for MEM_POWER_UP_SPEC {}
///`write(|w| ..)` method takes [`mem_power_up::W`](W) writer structure
impl crate::Writable for MEM_POWER_UP_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MEM_POWER_UP to value 0x3f
impl crate::Resettable for MEM_POWER_UP_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}
