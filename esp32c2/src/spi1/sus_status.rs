///Register `SUS_STATUS` reader
pub type R = crate::R<SUS_STATUS_SPEC>;
///Register `SUS_STATUS` writer
pub type W = crate::W<SUS_STATUS_SPEC>;
///Field `FLASH_SUS` reader - The status of flash suspend, only used in SPI1.
pub type FLASH_SUS_R = crate::BitReader;
///Field `FLASH_SUS` writer - The status of flash suspend, only used in SPI1.
pub type FLASH_SUS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAIT_PESR_CMD_2B` reader - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\[15:0\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\[7:0\] to check SUS/SUS1/SUS2 bit.
pub type WAIT_PESR_CMD_2B_R = crate::BitReader;
///Field `WAIT_PESR_CMD_2B` writer - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\[15:0\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\[7:0\] to check SUS/SUS1/SUS2 bit.
pub type WAIT_PESR_CMD_2B_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_HPM_DLY_128` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after HPM command is sent.
pub type FLASH_HPM_DLY_128_R = crate::BitReader;
///Field `FLASH_HPM_DLY_128` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after HPM command is sent.
pub type FLASH_HPM_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_RES_DLY_128` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after RES command is sent.
pub type FLASH_RES_DLY_128_R = crate::BitReader;
///Field `FLASH_RES_DLY_128` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after RES command is sent.
pub type FLASH_RES_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_DP_DLY_128` reader - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after DP command is sent.
pub type FLASH_DP_DLY_128_R = crate::BitReader;
///Field `FLASH_DP_DLY_128` writer - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after DP command is sent.
pub type FLASH_DP_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_PER_DLY_128` reader - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after PER command is sent.
pub type FLASH_PER_DLY_128_R = crate::BitReader;
///Field `FLASH_PER_DLY_128` writer - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after PER command is sent.
pub type FLASH_PER_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FLASH_PES_DLY_128` reader - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after PES command is sent.
pub type FLASH_PES_DLY_128_R = crate::BitReader;
///Field `FLASH_PES_DLY_128` writer - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after PES command is sent.
pub type FLASH_PES_DLY_128_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI0_LOCK_EN` reader - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it.
pub type SPI0_LOCK_EN_R = crate::BitReader;
///Field `SPI0_LOCK_EN` writer - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it.
pub type SPI0_LOCK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The status of flash suspend, only used in SPI1.
    #[inline(always)]
    pub fn flash_sus(&self) -> FLASH_SUS_R {
        FLASH_SUS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\[15:0\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\[7:0\] to check SUS/SUS1/SUS2 bit.
    #[inline(always)]
    pub fn wait_pesr_cmd_2b(&self) -> WAIT_PESR_CMD_2B_R {
        WAIT_PESR_CMD_2B_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after HPM command is sent.
    #[inline(always)]
    pub fn flash_hpm_dly_128(&self) -> FLASH_HPM_DLY_128_R {
        FLASH_HPM_DLY_128_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after RES command is sent.
    #[inline(always)]
    pub fn flash_res_dly_128(&self) -> FLASH_RES_DLY_128_R {
        FLASH_RES_DLY_128_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after DP command is sent.
    #[inline(always)]
    pub fn flash_dp_dly_128(&self) -> FLASH_DP_DLY_128_R {
        FLASH_DP_DLY_128_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after PER command is sent.
    #[inline(always)]
    pub fn flash_per_dly_128(&self) -> FLASH_PER_DLY_128_R {
        FLASH_PER_DLY_128_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after PES command is sent.
    #[inline(always)]
    pub fn flash_pes_dly_128(&self) -> FLASH_PES_DLY_128_R {
        FLASH_PES_DLY_128_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it.
    #[inline(always)]
    pub fn spi0_lock_en(&self) -> SPI0_LOCK_EN_R {
        SPI0_LOCK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SUS_STATUS")
            .field("flash_sus", &self.flash_sus())
            .field("wait_pesr_cmd_2b", &self.wait_pesr_cmd_2b())
            .field("flash_hpm_dly_128", &self.flash_hpm_dly_128())
            .field("flash_res_dly_128", &self.flash_res_dly_128())
            .field("flash_dp_dly_128", &self.flash_dp_dly_128())
            .field("flash_per_dly_128", &self.flash_per_dly_128())
            .field("flash_pes_dly_128", &self.flash_pes_dly_128())
            .field("spi0_lock_en", &self.spi0_lock_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - The status of flash suspend, only used in SPI1.
    #[inline(always)]
    #[must_use]
    pub fn flash_sus(&mut self) -> FLASH_SUS_W<SUS_STATUS_SPEC> {
        FLASH_SUS_W::new(self, 0)
    }
    ///Bit 1 - 1: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\[15:0\] to check SUS/SUS1/SUS2 bit. 0: SPI1 sends out SPI_MEM_WAIT_PESR_COMMAND\[7:0\] to check SUS/SUS1/SUS2 bit.
    #[inline(always)]
    #[must_use]
    pub fn wait_pesr_cmd_2b(&mut self) -> WAIT_PESR_CMD_2B_W<SUS_STATUS_SPEC> {
        WAIT_PESR_CMD_2B_W::new(self, 1)
    }
    ///Bit 2 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after HPM command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after HPM command is sent.
    #[inline(always)]
    #[must_use]
    pub fn flash_hpm_dly_128(&mut self) -> FLASH_HPM_DLY_128_W<SUS_STATUS_SPEC> {
        FLASH_HPM_DLY_128_W::new(self, 2)
    }
    ///Bit 3 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after RES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after RES command is sent.
    #[inline(always)]
    #[must_use]
    pub fn flash_res_dly_128(&mut self) -> FLASH_RES_DLY_128_W<SUS_STATUS_SPEC> {
        FLASH_RES_DLY_128_W::new(self, 3)
    }
    ///Bit 4 - 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after DP command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after DP command is sent.
    #[inline(always)]
    #[must_use]
    pub fn flash_dp_dly_128(&mut self) -> FLASH_DP_DLY_128_W<SUS_STATUS_SPEC> {
        FLASH_DP_DLY_128_W::new(self, 4)
    }
    ///Bit 5 - Valid when SPI_MEM_FLASH_PER_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after PER command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after PER command is sent.
    #[inline(always)]
    #[must_use]
    pub fn flash_per_dly_128(&mut self) -> FLASH_PER_DLY_128_W<SUS_STATUS_SPEC> {
        FLASH_PER_DLY_128_W::new(self, 5)
    }
    ///Bit 6 - Valid when SPI_MEM_FLASH_PES_WAIT_EN is 1. 1: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 128) SPI_CLK cycles after PES command is sent. 0: SPI1 waits (SPI_MEM_CS_HOLD_DELAY_RES\[9:0\] * 4) SPI_CLK cycles after PES command is sent.
    #[inline(always)]
    #[must_use]
    pub fn flash_pes_dly_128(&mut self) -> FLASH_PES_DLY_128_W<SUS_STATUS_SPEC> {
        FLASH_PES_DLY_128_W::new(self, 6)
    }
    ///Bit 7 - 1: Enable SPI0 lock SPI0/1 arbiter option. 0: Disable it.
    #[inline(always)]
    #[must_use]
    pub fn spi0_lock_en(&mut self) -> SPI0_LOCK_EN_W<SUS_STATUS_SPEC> {
        SPI0_LOCK_EN_W::new(self, 7)
    }
}
/**SPI1 flash suspend status register

You can [`read`](crate::generic::Reg::read) this register and get [`sus_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sus_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SUS_STATUS_SPEC;
impl crate::RegisterSpec for SUS_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sus_status::R`](R) reader structure
impl crate::Readable for SUS_STATUS_SPEC {}
///`write(|w| ..)` method takes [`sus_status::W`](W) writer structure
impl crate::Writable for SUS_STATUS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SUS_STATUS to value 0
impl crate::Resettable for SUS_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
