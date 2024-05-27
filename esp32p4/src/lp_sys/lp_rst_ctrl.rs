///Register `LP_RST_CTRL` reader
pub type R = crate::R<LP_RST_CTRL_SPEC>;
///Register `LP_RST_CTRL` writer
pub type W = crate::W<LP_RST_CTRL_SPEC>;
///Field `ANA_RST_BYPASS` reader - analog source reset bypass : wdt,brown out,super wdt,glitch
pub type ANA_RST_BYPASS_R = crate::BitReader;
///Field `ANA_RST_BYPASS` writer - analog source reset bypass : wdt,brown out,super wdt,glitch
pub type ANA_RST_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYS_RST_BYPASS` reader - system source reset bypass : software reset,hp wdt,lp wdt,efuse
pub type SYS_RST_BYPASS_R = crate::BitReader;
///Field `SYS_RST_BYPASS` writer - system source reset bypass : software reset,hp wdt,lp wdt,efuse
pub type SYS_RST_BYPASS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EFUSE_FORCE_NORST` reader - efuse force no reset control
pub type EFUSE_FORCE_NORST_R = crate::BitReader;
///Field `EFUSE_FORCE_NORST` writer - efuse force no reset control
pub type EFUSE_FORCE_NORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - analog source reset bypass : wdt,brown out,super wdt,glitch
    #[inline(always)]
    pub fn ana_rst_bypass(&self) -> ANA_RST_BYPASS_R {
        ANA_RST_BYPASS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - system source reset bypass : software reset,hp wdt,lp wdt,efuse
    #[inline(always)]
    pub fn sys_rst_bypass(&self) -> SYS_RST_BYPASS_R {
        SYS_RST_BYPASS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - efuse force no reset control
    #[inline(always)]
    pub fn efuse_force_norst(&self) -> EFUSE_FORCE_NORST_R {
        EFUSE_FORCE_NORST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_RST_CTRL")
            .field("ana_rst_bypass", &self.ana_rst_bypass())
            .field("sys_rst_bypass", &self.sys_rst_bypass())
            .field("efuse_force_norst", &self.efuse_force_norst())
            .finish()
    }
}
impl W {
    ///Bit 0 - analog source reset bypass : wdt,brown out,super wdt,glitch
    #[inline(always)]
    #[must_use]
    pub fn ana_rst_bypass(&mut self) -> ANA_RST_BYPASS_W<LP_RST_CTRL_SPEC> {
        ANA_RST_BYPASS_W::new(self, 0)
    }
    ///Bit 1 - system source reset bypass : software reset,hp wdt,lp wdt,efuse
    #[inline(always)]
    #[must_use]
    pub fn sys_rst_bypass(&mut self) -> SYS_RST_BYPASS_W<LP_RST_CTRL_SPEC> {
        SYS_RST_BYPASS_W::new(self, 1)
    }
    ///Bit 2 - efuse force no reset control
    #[inline(always)]
    #[must_use]
    pub fn efuse_force_norst(&mut self) -> EFUSE_FORCE_NORST_W<LP_RST_CTRL_SPEC> {
        EFUSE_FORCE_NORST_W::new(self, 2)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_rst_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_rst_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_RST_CTRL_SPEC;
impl crate::RegisterSpec for LP_RST_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_rst_ctrl::R`](R) reader structure
impl crate::Readable for LP_RST_CTRL_SPEC {}
///`write(|w| ..)` method takes [`lp_rst_ctrl::W`](W) writer structure
impl crate::Writable for LP_RST_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LP_RST_CTRL to value 0x03
impl crate::Resettable for LP_RST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
