#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `CS_DIS(0-5)` reader - SPI_CS%s pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
pub type CS_DIS_R = crate::BitReader;
#[doc = "Field `CS_DIS(0-5)` writer - SPI_CS%s pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
pub type CS_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_DIS` reader - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type CK_DIS_R = crate::BitReader;
#[doc = "Field `CK_DIS` writer - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
pub type CK_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASTER_CS_POL` reader - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type MASTER_CS_POL_R = crate::FieldReader;
#[doc = "Field `MASTER_CS_POL` writer - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
pub type MASTER_CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CLK_DATA_DTR_EN` reader - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
pub type CLK_DATA_DTR_EN_R = crate::BitReader;
#[doc = "Field `DATA_DTR_EN` reader - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
pub type DATA_DTR_EN_R = crate::BitReader;
#[doc = "Field `ADDR_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
pub type ADDR_DTR_EN_R = crate::BitReader;
#[doc = "Field `CMD_DTR_EN` reader - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
pub type CMD_DTR_EN_R = crate::BitReader;
#[doc = "Field `SLAVE_CS_POL` reader - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SLAVE_CS_POL_R = crate::BitReader;
#[doc = "Field `SLAVE_CS_POL` writer - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
pub type SLAVE_CS_POL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQS_IDLE_EDGE` reader - The default value of spi_dqs. Can be configured in CONF state."]
pub type DQS_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set. Can be configured in CONF state."]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QUAD_DIN_PIN_SWAP` reader - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
pub type QUAD_DIN_PIN_SWAP_R = crate::BitReader;
#[doc = "Field `QUAD_DIN_PIN_SWAP` writer - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
pub type QUAD_DIN_PIN_SWAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "SPI_CS(0-5) pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field"]
    #[inline(always)]
    pub fn cs_dis(&self, n: u8) -> CS_DIS_R {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        CS_DIS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "SPI_CS(0-5) pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs_dis_iter(&self) -> impl Iterator<Item = CS_DIS_R> + '_ {
        (0..6).map(move |n| CS_DIS_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - SPI_CS0 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI_CS2 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs2_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SPI_CS3 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs3_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SPI_CS4 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs4_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI_CS5 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    pub fn cs5_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_dis(&self) -> CK_DIS_R {
        CK_DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    pub fn master_cs_pol(&self) -> MASTER_CS_POL_R {
        MASTER_CS_POL_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - 1: SPI master DTR mode is applied to SPI clk, data and spi_dqs. 0: SPI master DTR mode is only applied to spi_dqs. This bit should be used with bit 17/18/19."]
    #[inline(always)]
    pub fn clk_data_dtr_en(&self) -> CLK_DATA_DTR_EN_R {
        CLK_DATA_DTR_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: SPI clk and data of SPI_DOUT and SPI_DIN state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_DOUT and SPI_DIN state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn data_dtr_en(&self) -> DATA_DTR_EN_R {
        DATA_DTR_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1: SPI clk and data of SPI_SEND_ADDR state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_ADDR state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn addr_dtr_en(&self) -> ADDR_DTR_EN_R {
        ADDR_DTR_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: SPI clk and data of SPI_SEND_CMD state are in DTR mode, including master 1/2/4/8-bm. 0: SPI clk and data of SPI_SEND_CMD state are in STR mode. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cmd_dtr_en(&self) -> CMD_DTR_EN_R {
        CMD_DTR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    pub fn slave_cs_pol(&self) -> SLAVE_CS_POL_R {
        SLAVE_CS_POL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The default value of spi_dqs. Can be configured in CONF state."]
    #[inline(always)]
    pub fn dqs_idle_edge(&self) -> DQS_IDLE_EDGE_R {
        DQS_IDLE_EDGE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn quad_din_pin_swap(&self) -> QUAD_DIN_PIN_SWAP_R {
        QUAD_DIN_PIN_SWAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("cs0_dis", &format_args!("{}", self.cs0_dis().bit()))
            .field("cs1_dis", &format_args!("{}", self.cs1_dis().bit()))
            .field("cs2_dis", &format_args!("{}", self.cs2_dis().bit()))
            .field("cs3_dis", &format_args!("{}", self.cs3_dis().bit()))
            .field("cs4_dis", &format_args!("{}", self.cs4_dis().bit()))
            .field("cs5_dis", &format_args!("{}", self.cs5_dis().bit()))
            .field("ck_dis", &format_args!("{}", self.ck_dis().bit()))
            .field(
                "master_cs_pol",
                &format_args!("{}", self.master_cs_pol().bits()),
            )
            .field(
                "clk_data_dtr_en",
                &format_args!("{}", self.clk_data_dtr_en().bit()),
            )
            .field("data_dtr_en", &format_args!("{}", self.data_dtr_en().bit()))
            .field("addr_dtr_en", &format_args!("{}", self.addr_dtr_en().bit()))
            .field("cmd_dtr_en", &format_args!("{}", self.cmd_dtr_en().bit()))
            .field(
                "slave_cs_pol",
                &format_args!("{}", self.slave_cs_pol().bit()),
            )
            .field(
                "dqs_idle_edge",
                &format_args!("{}", self.dqs_idle_edge().bit()),
            )
            .field(
                "ck_idle_edge",
                &format_args!("{}", self.ck_idle_edge().bit()),
            )
            .field(
                "cs_keep_active",
                &format_args!("{}", self.cs_keep_active().bit()),
            )
            .field(
                "quad_din_pin_swap",
                &format_args!("{}", self.quad_din_pin_swap().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MISC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "SPI_CS(0-5) pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field"]
    #[inline(always)]
    #[must_use]
    pub fn cs_dis(&mut self, n: u8) -> CS_DIS_W<MISC_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 6][n as usize];
        CS_DIS_W::new(self, n)
    }
    #[doc = "Bit 0 - SPI_CS0 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    #[must_use]
    pub fn cs0_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI_CS1 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    #[must_use]
    pub fn cs1_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - SPI_CS2 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    #[must_use]
    pub fn cs2_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - SPI_CS3 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    #[must_use]
    pub fn cs3_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - SPI_CS4 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    #[must_use]
    pub fn cs4_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - SPI_CS5 pin enable, 1: disable SPI_CSx, 0: SPI_CSx pin is active to select SPI device, such as flash, external RAM and so on."]
    #[inline(always)]
    #[must_use]
    pub fn cs5_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 5)
    }
    #[doc = "Bit 6 - 1: spi clk out disable, 0: spi clk out enable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn ck_dis(&mut self) -> CK_DIS_W<MISC_SPEC> {
        CK_DIS_W::new(self, 6)
    }
    #[doc = "Bits 7:12 - In the master mode the bits are the polarity of spi cs line, the value is equivalent to spi_cs ^ spi_master_cs_pol. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn master_cs_pol(&mut self) -> MASTER_CS_POL_W<MISC_SPEC> {
        MASTER_CS_POL_W::new(self, 7)
    }
    #[doc = "Bit 23 - spi slave input cs polarity select. 1: inv 0: not change. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn slave_cs_pol(&mut self) -> SLAVE_CS_POL_W<MISC_SPEC> {
        SLAVE_CS_POL_W::new(self, 23)
    }
    #[doc = "Bit 29 - 1: spi clk line is high when idle 0: spi clk line is low when idle. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 29)
    }
    #[doc = "Bit 30 - spi cs line keep low when the bit is set. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: SPI quad input swap enable, swap FSPID with FSPIQ, swap FSPIWP with FSPIHD. 0: spi quad input swap disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn quad_din_pin_swap(&mut self) -> QUAD_DIN_PIN_SWAP_W<MISC_SPEC> {
        QUAD_DIN_PIN_SWAP_W::new(self, 31)
    }
}
#[doc = "SPI misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MISC to value 0x3e"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: u32 = 0x3e;
}
