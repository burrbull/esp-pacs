#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "SPI CS0 pin enable bit. Can be configured in CONF state\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CS0_DIS {
    #[doc = "0: SPI_CSx signal is from/to CSx pin"]
    Pin = 0,
    #[doc = "1: Disable CSx pin"]
    Disable = 1,
}
impl From<CS0_DIS> for bool {
    #[inline(always)]
    fn from(variant: CS0_DIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS_DIS(0-1)` reader - SPI CS0 pin enable bit. Can be configured in CONF state"]
pub type CS_DIS_R = crate::BitReader<CS0_DIS>;
impl CS_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CS0_DIS {
        match self.bits {
            false => CS0_DIS::Pin,
            true => CS0_DIS::Disable,
        }
    }
    #[doc = "SPI_CSx signal is from/to CSx pin"]
    #[inline(always)]
    pub fn is_pin(&self) -> bool {
        *self == CS0_DIS::Pin
    }
    #[doc = "Disable CSx pin"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CS0_DIS::Disable
    }
}
#[doc = "Field `CS_DIS(0-1)` writer - SPI CS0 pin enable bit. Can be configured in CONF state"]
pub type CS_DIS_W<'a, REG> = crate::BitWriter<'a, REG, CS0_DIS>;
impl<'a, REG> CS_DIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI_CSx signal is from/to CSx pin"]
    #[inline(always)]
    pub fn pin(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_DIS::Pin)
    }
    #[doc = "Disable CSx pin"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CS0_DIS::Disable)
    }
}
#[doc = "Field `CK_IDLE_EDGE` reader - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - spi cs line keep low when the bit is set."]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "SPI CS0 pin enable bit. Can be configured in CONF state"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field"]
    #[inline(always)]
    pub fn cs_dis(&self, n: u8) -> CS_DIS_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CS_DIS_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "SPI CS0 pin enable bit. Can be configured in CONF state"]
    #[inline(always)]
    pub fn cs_dis_iter(&self) -> impl Iterator<Item = CS_DIS_R> + '_ {
        (0..2).map(move |n| CS_DIS_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - SPI CS0 pin enable bit. Can be configured in CONF state"]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SPI CS0 pin enable bit. Can be configured in CONF state"]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS_DIS_R {
        CS_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("cs0_dis", &format_args!("{}", self.cs0_dis().bit()))
            .field("cs1_dis", &format_args!("{}", self.cs1_dis().bit()))
            .field(
                "ck_idle_edge",
                &format_args!("{}", self.ck_idle_edge().bit()),
            )
            .field(
                "cs_keep_active",
                &format_args!("{}", self.cs_keep_active().bit()),
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
    #[doc = "SPI CS0 pin enable bit. Can be configured in CONF state"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CS0_DIS` field"]
    #[inline(always)]
    #[must_use]
    pub fn cs_dis(&mut self, n: u8) -> CS_DIS_W<MISC_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CS_DIS_W::new(self, n)
    }
    #[doc = "Bit 0 - SPI CS0 pin enable bit. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn cs0_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - SPI CS0 pin enable bit. Can be configured in CONF state"]
    #[inline(always)]
    #[must_use]
    pub fn cs1_dis(&mut self) -> CS_DIS_W<MISC_SPEC> {
        CS_DIS_W::new(self, 1)
    }
    #[doc = "Bit 9 - 1: spi clk line is high when idle 0: spi clk line is low when idle"]
    #[inline(always)]
    #[must_use]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10 - spi cs line keep low when the bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 10)
    }
}
#[doc = "SPI1 misc register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
#[doc = "`reset()` method sets MISC to value 0x02"]
impl crate::Resettable for MISC_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
