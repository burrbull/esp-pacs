///Register `SAR_DAC_CTRL2` reader
pub type R = crate::R<SAR_DAC_CTRL2_SPEC>;
///Register `SAR_DAC_CTRL2` writer
pub type W = crate::W<SAR_DAC_CTRL2_SPEC>;
///Field `DAC_DC1` reader - DC offset for DAC1 CW generator.
pub type DAC_DC1_R = crate::FieldReader;
///Field `DAC_DC1` writer - DC offset for DAC1 CW generator.
pub type DAC_DC1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DAC_DC2` reader - DC offset for DAC2 CW generator.
pub type DAC_DC2_R = crate::FieldReader;
///Field `DAC_DC2` writer - DC offset for DAC2 CW generator.
pub type DAC_DC2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `DAC_SCALE1` reader - DAC1 scaling. 00: no scale. 01: scale to 1/2. 10: scale to 1/4. 11: scale to 1/8.
pub type DAC_SCALE1_R = crate::FieldReader;
///Field `DAC_SCALE1` writer - DAC1 scaling. 00: no scale. 01: scale to 1/2. 10: scale to 1/4. 11: scale to 1/8.
pub type DAC_SCALE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DAC_SCALE2` reader - DAC2 scaling. 00: no scale. 01: scale to 1/2. 10: scale to 1/4. 11: scale to 1/8.
pub type DAC_SCALE2_R = crate::FieldReader;
///Field `DAC_SCALE2` writer - DAC2 scaling. 00: no scale. 01: scale to 1/2. 10: scale to 1/4. 11: scale to 1/8.
pub type DAC_SCALE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DAC_INV1` reader - Invert DAC1. 00: do not invert any bits. 01: invert all bits. 10: invert MSB. 11: invert all bits except MSB.
pub type DAC_INV1_R = crate::FieldReader;
///Field `DAC_INV1` writer - Invert DAC1. 00: do not invert any bits. 01: invert all bits. 10: invert MSB. 11: invert all bits except MSB.
pub type DAC_INV1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DAC_INV2` reader - Invert DAC2. 00: do not invert any bits. 01: invert all bits. 10: invert MSB. 11: invert all bits except MSB.
pub type DAC_INV2_R = crate::FieldReader;
///Field `DAC_INV2` writer - Invert DAC2. 00: do not invert any bits. 01: invert all bits. 10: invert MSB. 11: invert all bits except MSB.
pub type DAC_INV2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DAC_CW_EN1` reader - 1: select CW generator as source for PDAC1_DAC. 0: select register RT- CIO_PDAC1_DAC as source for PDAC1_DAC.
pub type DAC_CW_EN1_R = crate::BitReader;
///Field `DAC_CW_EN1` writer - 1: select CW generator as source for PDAC1_DAC. 0: select register RT- CIO_PDAC1_DAC as source for PDAC1_DAC.
pub type DAC_CW_EN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC_CW_EN2` reader - 1: select CW generator as source for PDAC2_DAC. 0: select register RT- CIO_PDAC2_DAC as source for PDAC2_DAC.
pub type DAC_CW_EN2_R = crate::BitReader;
///Field `DAC_CW_EN2` writer - 1: select CW generator as source for PDAC2_DAC. 0: select register RT- CIO_PDAC2_DAC as source for PDAC2_DAC.
pub type DAC_CW_EN2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - DC offset for DAC1 CW generator.
    #[inline(always)]
    pub fn dac_dc1(&self) -> DAC_DC1_R {
        DAC_DC1_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - DC offset for DAC2 CW generator.
    #[inline(always)]
    pub fn dac_dc2(&self) -> DAC_DC2_R {
        DAC_DC2_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:17 - DAC1 scaling. 00: no scale. 01: scale to 1/2. 10: scale to 1/4. 11: scale to 1/8.
    #[inline(always)]
    pub fn dac_scale1(&self) -> DAC_SCALE1_R {
        DAC_SCALE1_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:19 - DAC2 scaling. 00: no scale. 01: scale to 1/2. 10: scale to 1/4. 11: scale to 1/8.
    #[inline(always)]
    pub fn dac_scale2(&self) -> DAC_SCALE2_R {
        DAC_SCALE2_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:21 - Invert DAC1. 00: do not invert any bits. 01: invert all bits. 10: invert MSB. 11: invert all bits except MSB.
    #[inline(always)]
    pub fn dac_inv1(&self) -> DAC_INV1_R {
        DAC_INV1_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bits 22:23 - Invert DAC2. 00: do not invert any bits. 01: invert all bits. 10: invert MSB. 11: invert all bits except MSB.
    #[inline(always)]
    pub fn dac_inv2(&self) -> DAC_INV2_R {
        DAC_INV2_R::new(((self.bits >> 22) & 3) as u8)
    }
    ///Bit 24 - 1: select CW generator as source for PDAC1_DAC. 0: select register RT- CIO_PDAC1_DAC as source for PDAC1_DAC.
    #[inline(always)]
    pub fn dac_cw_en1(&self) -> DAC_CW_EN1_R {
        DAC_CW_EN1_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - 1: select CW generator as source for PDAC2_DAC. 0: select register RT- CIO_PDAC2_DAC as source for PDAC2_DAC.
    #[inline(always)]
    pub fn dac_cw_en2(&self) -> DAC_CW_EN2_R {
        DAC_CW_EN2_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_DAC_CTRL2")
            .field("dac_dc1", &self.dac_dc1())
            .field("dac_dc2", &self.dac_dc2())
            .field("dac_scale1", &self.dac_scale1())
            .field("dac_scale2", &self.dac_scale2())
            .field("dac_inv1", &self.dac_inv1())
            .field("dac_inv2", &self.dac_inv2())
            .field("dac_cw_en1", &self.dac_cw_en1())
            .field("dac_cw_en2", &self.dac_cw_en2())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - DC offset for DAC1 CW generator.
    #[inline(always)]
    #[must_use]
    pub fn dac_dc1(&mut self) -> DAC_DC1_W<SAR_DAC_CTRL2_SPEC> {
        DAC_DC1_W::new(self, 0)
    }
    ///Bits 8:15 - DC offset for DAC2 CW generator.
    #[inline(always)]
    #[must_use]
    pub fn dac_dc2(&mut self) -> DAC_DC2_W<SAR_DAC_CTRL2_SPEC> {
        DAC_DC2_W::new(self, 8)
    }
    ///Bits 16:17 - DAC1 scaling. 00: no scale. 01: scale to 1/2. 10: scale to 1/4. 11: scale to 1/8.
    #[inline(always)]
    #[must_use]
    pub fn dac_scale1(&mut self) -> DAC_SCALE1_W<SAR_DAC_CTRL2_SPEC> {
        DAC_SCALE1_W::new(self, 16)
    }
    ///Bits 18:19 - DAC2 scaling. 00: no scale. 01: scale to 1/2. 10: scale to 1/4. 11: scale to 1/8.
    #[inline(always)]
    #[must_use]
    pub fn dac_scale2(&mut self) -> DAC_SCALE2_W<SAR_DAC_CTRL2_SPEC> {
        DAC_SCALE2_W::new(self, 18)
    }
    ///Bits 20:21 - Invert DAC1. 00: do not invert any bits. 01: invert all bits. 10: invert MSB. 11: invert all bits except MSB.
    #[inline(always)]
    #[must_use]
    pub fn dac_inv1(&mut self) -> DAC_INV1_W<SAR_DAC_CTRL2_SPEC> {
        DAC_INV1_W::new(self, 20)
    }
    ///Bits 22:23 - Invert DAC2. 00: do not invert any bits. 01: invert all bits. 10: invert MSB. 11: invert all bits except MSB.
    #[inline(always)]
    #[must_use]
    pub fn dac_inv2(&mut self) -> DAC_INV2_W<SAR_DAC_CTRL2_SPEC> {
        DAC_INV2_W::new(self, 22)
    }
    ///Bit 24 - 1: select CW generator as source for PDAC1_DAC. 0: select register RT- CIO_PDAC1_DAC as source for PDAC1_DAC.
    #[inline(always)]
    #[must_use]
    pub fn dac_cw_en1(&mut self) -> DAC_CW_EN1_W<SAR_DAC_CTRL2_SPEC> {
        DAC_CW_EN1_W::new(self, 24)
    }
    ///Bit 25 - 1: select CW generator as source for PDAC2_DAC. 0: select register RT- CIO_PDAC2_DAC as source for PDAC2_DAC.
    #[inline(always)]
    #[must_use]
    pub fn dac_cw_en2(&mut self) -> DAC_CW_EN2_W<SAR_DAC_CTRL2_SPEC> {
        DAC_CW_EN2_W::new(self, 25)
    }
}
/**DAC output control

You can [`read`](crate::generic::Reg::read) this register and get [`sar_dac_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_dac_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SAR_DAC_CTRL2_SPEC;
impl crate::RegisterSpec for SAR_DAC_CTRL2_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sar_dac_ctrl2::R`](R) reader structure
impl crate::Readable for SAR_DAC_CTRL2_SPEC {}
///`write(|w| ..)` method takes [`sar_dac_ctrl2::W`](W) writer structure
impl crate::Writable for SAR_DAC_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAR_DAC_CTRL2 to value 0x0300_0000
impl crate::Resettable for SAR_DAC_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x0300_0000;
}
