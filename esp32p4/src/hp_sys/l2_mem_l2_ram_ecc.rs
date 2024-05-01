///Register `L2_MEM_L2_RAM_ECC` reader
pub type R = crate::R<L2_MEM_L2_RAM_ECC_SPEC>;
///Register `L2_MEM_L2_RAM_ECC` writer
pub type W = crate::W<L2_MEM_L2_RAM_ECC_SPEC>;
///Field `REG_L2_RAM_UNIT0_ECC_EN` reader - NA
pub type REG_L2_RAM_UNIT0_ECC_EN_R = crate::BitReader;
///Field `REG_L2_RAM_UNIT0_ECC_EN` writer - NA
pub type REG_L2_RAM_UNIT0_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L2_RAM_UNIT1_ECC_EN` reader - NA
pub type REG_L2_RAM_UNIT1_ECC_EN_R = crate::BitReader;
///Field `REG_L2_RAM_UNIT1_ECC_EN` writer - NA
pub type REG_L2_RAM_UNIT1_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L2_RAM_UNIT2_ECC_EN` reader - NA
pub type REG_L2_RAM_UNIT2_ECC_EN_R = crate::BitReader;
///Field `REG_L2_RAM_UNIT2_ECC_EN` writer - NA
pub type REG_L2_RAM_UNIT2_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L2_RAM_UNIT3_ECC_EN` reader - NA
pub type REG_L2_RAM_UNIT3_ECC_EN_R = crate::BitReader;
///Field `REG_L2_RAM_UNIT3_ECC_EN` writer - NA
pub type REG_L2_RAM_UNIT3_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L2_RAM_UNIT4_ECC_EN` reader - NA
pub type REG_L2_RAM_UNIT4_ECC_EN_R = crate::BitReader;
///Field `REG_L2_RAM_UNIT4_ECC_EN` writer - NA
pub type REG_L2_RAM_UNIT4_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `REG_L2_RAM_UNIT5_ECC_EN` reader - NA
pub type REG_L2_RAM_UNIT5_ECC_EN_R = crate::BitReader;
///Field `REG_L2_RAM_UNIT5_ECC_EN` writer - NA
pub type REG_L2_RAM_UNIT5_ECC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - NA
    #[inline(always)]
    pub fn reg_l2_ram_unit0_ecc_en(&self) -> REG_L2_RAM_UNIT0_ECC_EN_R {
        REG_L2_RAM_UNIT0_ECC_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    pub fn reg_l2_ram_unit1_ecc_en(&self) -> REG_L2_RAM_UNIT1_ECC_EN_R {
        REG_L2_RAM_UNIT1_ECC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - NA
    #[inline(always)]
    pub fn reg_l2_ram_unit2_ecc_en(&self) -> REG_L2_RAM_UNIT2_ECC_EN_R {
        REG_L2_RAM_UNIT2_ECC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NA
    #[inline(always)]
    pub fn reg_l2_ram_unit3_ecc_en(&self) -> REG_L2_RAM_UNIT3_ECC_EN_R {
        REG_L2_RAM_UNIT3_ECC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - NA
    #[inline(always)]
    pub fn reg_l2_ram_unit4_ecc_en(&self) -> REG_L2_RAM_UNIT4_ECC_EN_R {
        REG_L2_RAM_UNIT4_ECC_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - NA
    #[inline(always)]
    pub fn reg_l2_ram_unit5_ecc_en(&self) -> REG_L2_RAM_UNIT5_ECC_EN_R {
        REG_L2_RAM_UNIT5_ECC_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_MEM_L2_RAM_ECC")
            .field("reg_l2_ram_unit0_ecc_en", &self.reg_l2_ram_unit0_ecc_en())
            .field("reg_l2_ram_unit1_ecc_en", &self.reg_l2_ram_unit1_ecc_en())
            .field("reg_l2_ram_unit2_ecc_en", &self.reg_l2_ram_unit2_ecc_en())
            .field("reg_l2_ram_unit3_ecc_en", &self.reg_l2_ram_unit3_ecc_en())
            .field("reg_l2_ram_unit4_ecc_en", &self.reg_l2_ram_unit4_ecc_en())
            .field("reg_l2_ram_unit5_ecc_en", &self.reg_l2_ram_unit5_ecc_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_ram_unit0_ecc_en(&mut self) -> REG_L2_RAM_UNIT0_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT0_ECC_EN_W::new(self, 0)
    }
    ///Bit 1 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_ram_unit1_ecc_en(&mut self) -> REG_L2_RAM_UNIT1_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT1_ECC_EN_W::new(self, 1)
    }
    ///Bit 2 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_ram_unit2_ecc_en(&mut self) -> REG_L2_RAM_UNIT2_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT2_ECC_EN_W::new(self, 2)
    }
    ///Bit 3 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_ram_unit3_ecc_en(&mut self) -> REG_L2_RAM_UNIT3_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT3_ECC_EN_W::new(self, 3)
    }
    ///Bit 4 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_ram_unit4_ecc_en(&mut self) -> REG_L2_RAM_UNIT4_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT4_ECC_EN_W::new(self, 4)
    }
    ///Bit 5 - NA
    #[inline(always)]
    #[must_use]
    pub fn reg_l2_ram_unit5_ecc_en(&mut self) -> REG_L2_RAM_UNIT5_ECC_EN_W<L2_MEM_L2_RAM_ECC_SPEC> {
        REG_L2_RAM_UNIT5_ECC_EN_W::new(self, 5)
    }
}
/**NA

You can [`read`](crate::generic::Reg::read) this register and get [`l2_mem_l2_ram_ecc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_mem_l2_ram_ecc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct L2_MEM_L2_RAM_ECC_SPEC;
impl crate::RegisterSpec for L2_MEM_L2_RAM_ECC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`l2_mem_l2_ram_ecc::R`](R) reader structure
impl crate::Readable for L2_MEM_L2_RAM_ECC_SPEC {}
///`write(|w| ..)` method takes [`l2_mem_l2_ram_ecc::W`](W) writer structure
impl crate::Writable for L2_MEM_L2_RAM_ECC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets L2_MEM_L2_RAM_ECC to value 0
impl crate::Resettable for L2_MEM_L2_RAM_ECC_SPEC {
    const RESET_VALUE: u32 = 0;
}
