///Register `ETM_TASK_P1_CFG` reader
pub type R = crate::R<ETM_TASK_P1_CFG_SPEC>;
///Register `ETM_TASK_P1_CFG` writer
pub type W = crate::W<ETM_TASK_P1_CFG_SPEC>;
///Field `GPIO_EN(4-7)` reader - Enable bit of GPIO response etm task.
pub type GPIO_EN_R = crate::BitReader;
///Field `GPIO_EN(4-7)` writer - Enable bit of GPIO response etm task.
pub type GPIO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIO_SEL(4-7)` reader - GPIO choose a etm task channel.
pub type GPIO_SEL_R = crate::FieldReader;
///Field `GPIO_SEL(4-7)` writer - GPIO choose a etm task channel.
pub type GPIO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Enable bit of GPIO response etm task.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO4_EN` field
    #[inline(always)]
    pub fn gpio_en(&self, n: u8) -> GPIO_EN_R {
        #[allow(clippy::no_effect)] [(); 4][n as usize];
        GPIO_EN_R::new(((self.bits >> (n * 8)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn gpio_en_iter(&self) -> impl Iterator<Item = GPIO_EN_R> + '_ {
        (0..4).map(move |n| GPIO_EN_R::new(((self.bits >> (n * 8)) & 1) != 0))
    }
    ///Bit 0 - Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn gpio4_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 8 - Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn gpio5_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn gpio6_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Enable bit of GPIO response etm task.
    #[inline(always)]
    pub fn gpio7_en(&self) -> GPIO_EN_R {
        GPIO_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///GPIO choose a etm task channel.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO4_SEL` field
    #[inline(always)]
    pub fn gpio_sel(&self, n: u8) -> GPIO_SEL_R {
        #[allow(clippy::no_effect)] [(); 4][n as usize];
        GPIO_SEL_R::new(((self.bits >> (n * 8 + 1)) & 7) as u8)
    }
    ///Iterator for array of:
    ///GPIO choose a etm task channel.
    #[inline(always)]
    pub fn gpio_sel_iter(&self) -> impl Iterator<Item = GPIO_SEL_R> + '_ {
        (0..4).map(move |n| GPIO_SEL_R::new(((self.bits >> (n * 8 + 1)) & 7) as u8))
    }
    ///Bits 1:3 - GPIO choose a etm task channel.
    #[inline(always)]
    pub fn gpio4_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    ///Bits 9:11 - GPIO choose a etm task channel.
    #[inline(always)]
    pub fn gpio5_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 17:19 - GPIO choose a etm task channel.
    #[inline(always)]
    pub fn gpio6_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 25:27 - GPIO choose a etm task channel.
    #[inline(always)]
    pub fn gpio7_sel(&self) -> GPIO_SEL_R {
        GPIO_SEL_R::new(((self.bits >> 25) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_TASK_P1_CFG")
            .field("gpio4_en", &self.gpio4_en())
            .field("gpio5_en", &self.gpio5_en())
            .field("gpio6_en", &self.gpio6_en())
            .field("gpio7_en", &self.gpio7_en())
            .field("gpio4_sel", &self.gpio4_sel())
            .field("gpio5_sel", &self.gpio5_sel())
            .field("gpio6_sel", &self.gpio6_sel())
            .field("gpio7_sel", &self.gpio7_sel())
            .finish()
    }
}
impl W {
    ///Enable bit of GPIO response etm task.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO4_EN` field
    #[inline(always)]
    #[must_use]
    pub fn gpio_en(&mut self, n: u8) -> GPIO_EN_W<ETM_TASK_P1_CFG_SPEC> {
        #[allow(clippy::no_effect)] [(); 4][n as usize];
        GPIO_EN_W::new(self, n * 8)
    }
    ///Bit 0 - Enable bit of GPIO response etm task.
    #[inline(always)]
    #[must_use]
    pub fn gpio4_en(&mut self) -> GPIO_EN_W<ETM_TASK_P1_CFG_SPEC> {
        GPIO_EN_W::new(self, 0)
    }
    ///Bit 8 - Enable bit of GPIO response etm task.
    #[inline(always)]
    #[must_use]
    pub fn gpio5_en(&mut self) -> GPIO_EN_W<ETM_TASK_P1_CFG_SPEC> {
        GPIO_EN_W::new(self, 8)
    }
    ///Bit 16 - Enable bit of GPIO response etm task.
    #[inline(always)]
    #[must_use]
    pub fn gpio6_en(&mut self) -> GPIO_EN_W<ETM_TASK_P1_CFG_SPEC> {
        GPIO_EN_W::new(self, 16)
    }
    ///Bit 24 - Enable bit of GPIO response etm task.
    #[inline(always)]
    #[must_use]
    pub fn gpio7_en(&mut self) -> GPIO_EN_W<ETM_TASK_P1_CFG_SPEC> {
        GPIO_EN_W::new(self, 24)
    }
    ///GPIO choose a etm task channel.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `GPIO4_SEL` field
    #[inline(always)]
    #[must_use]
    pub fn gpio_sel(&mut self, n: u8) -> GPIO_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        #[allow(clippy::no_effect)] [(); 4][n as usize];
        GPIO_SEL_W::new(self, n * 8 + 1)
    }
    ///Bits 1:3 - GPIO choose a etm task channel.
    #[inline(always)]
    #[must_use]
    pub fn gpio4_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        GPIO_SEL_W::new(self, 1)
    }
    ///Bits 9:11 - GPIO choose a etm task channel.
    #[inline(always)]
    #[must_use]
    pub fn gpio5_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        GPIO_SEL_W::new(self, 9)
    }
    ///Bits 17:19 - GPIO choose a etm task channel.
    #[inline(always)]
    #[must_use]
    pub fn gpio6_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        GPIO_SEL_W::new(self, 17)
    }
    ///Bits 25:27 - GPIO choose a etm task channel.
    #[inline(always)]
    #[must_use]
    pub fn gpio7_sel(&mut self) -> GPIO_SEL_W<ETM_TASK_P1_CFG_SPEC> {
        GPIO_SEL_W::new(self, 25)
    }
}
/**Etm Configure Register to decide which GPIO been chosen

You can [`read`](crate::generic::Reg::read) this register and get [`etm_task_p1_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_task_p1_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ETM_TASK_P1_CFG_SPEC;
impl crate::RegisterSpec for ETM_TASK_P1_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`etm_task_p1_cfg::R`](R) reader structure
impl crate::Readable for ETM_TASK_P1_CFG_SPEC {}
///`write(|w| ..)` method takes [`etm_task_p1_cfg::W`](W) writer structure
impl crate::Writable for ETM_TASK_P1_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETM_TASK_P1_CFG to value 0
impl crate::Resettable for ETM_TASK_P1_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
