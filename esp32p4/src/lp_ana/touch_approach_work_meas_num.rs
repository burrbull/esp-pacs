///Register `TOUCH_APPROACH_WORK_MEAS_NUM` reader
pub type R = crate::R<TOUCH_APPROACH_WORK_MEAS_NUM_SPEC>;
///Register `TOUCH_APPROACH_WORK_MEAS_NUM` writer
pub type W = crate::W<TOUCH_APPROACH_WORK_MEAS_NUM_SPEC>;
///Field `TOUCH_APPROACH_MEAS_NUM2` reader - need_des
pub type TOUCH_APPROACH_MEAS_NUM2_R = crate::FieldReader<u16>;
///Field `TOUCH_APPROACH_MEAS_NUM2` writer - need_des
pub type TOUCH_APPROACH_MEAS_NUM2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TOUCH_APPROACH_MEAS_NUM1` reader - need_des
pub type TOUCH_APPROACH_MEAS_NUM1_R = crate::FieldReader<u16>;
///Field `TOUCH_APPROACH_MEAS_NUM1` writer - need_des
pub type TOUCH_APPROACH_MEAS_NUM1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
///Field `TOUCH_APPROACH_MEAS_NUM0` reader - need_des
pub type TOUCH_APPROACH_MEAS_NUM0_R = crate::FieldReader<u16>;
///Field `TOUCH_APPROACH_MEAS_NUM0` writer - need_des
pub type TOUCH_APPROACH_MEAS_NUM0_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    ///Bits 0:9 - need_des
    #[inline(always)]
    pub fn touch_approach_meas_num2(&self) -> TOUCH_APPROACH_MEAS_NUM2_R {
        TOUCH_APPROACH_MEAS_NUM2_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bits 10:19 - need_des
    #[inline(always)]
    pub fn touch_approach_meas_num1(&self) -> TOUCH_APPROACH_MEAS_NUM1_R {
        TOUCH_APPROACH_MEAS_NUM1_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    ///Bits 20:29 - need_des
    #[inline(always)]
    pub fn touch_approach_meas_num0(&self) -> TOUCH_APPROACH_MEAS_NUM0_R {
        TOUCH_APPROACH_MEAS_NUM0_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_APPROACH_WORK_MEAS_NUM")
            .field("touch_approach_meas_num2", &self.touch_approach_meas_num2())
            .field("touch_approach_meas_num1", &self.touch_approach_meas_num1())
            .field("touch_approach_meas_num0", &self.touch_approach_meas_num0())
            .finish()
    }
}
impl W {
    ///Bits 0:9 - need_des
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_meas_num2(
        &mut self,
    ) -> TOUCH_APPROACH_MEAS_NUM2_W<TOUCH_APPROACH_WORK_MEAS_NUM_SPEC> {
        TOUCH_APPROACH_MEAS_NUM2_W::new(self, 0)
    }
    ///Bits 10:19 - need_des
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_meas_num1(
        &mut self,
    ) -> TOUCH_APPROACH_MEAS_NUM1_W<TOUCH_APPROACH_WORK_MEAS_NUM_SPEC> {
        TOUCH_APPROACH_MEAS_NUM1_W::new(self, 10)
    }
    ///Bits 20:29 - need_des
    #[inline(always)]
    #[must_use]
    pub fn touch_approach_meas_num0(
        &mut self,
    ) -> TOUCH_APPROACH_MEAS_NUM0_W<TOUCH_APPROACH_WORK_MEAS_NUM_SPEC> {
        TOUCH_APPROACH_MEAS_NUM0_W::new(self, 20)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`touch_approach_work_meas_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_approach_work_meas_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TOUCH_APPROACH_WORK_MEAS_NUM_SPEC;
impl crate::RegisterSpec for TOUCH_APPROACH_WORK_MEAS_NUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`touch_approach_work_meas_num::R`](R) reader structure
impl crate::Readable for TOUCH_APPROACH_WORK_MEAS_NUM_SPEC {}
///`write(|w| ..)` method takes [`touch_approach_work_meas_num::W`](W) writer structure
impl crate::Writable for TOUCH_APPROACH_WORK_MEAS_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOUCH_APPROACH_WORK_MEAS_NUM to value 0x0641_9064
impl crate::Resettable for TOUCH_APPROACH_WORK_MEAS_NUM_SPEC {
    const RESET_VALUE: u32 = 0x0641_9064;
}
