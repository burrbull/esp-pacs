///Register `DOEPCTL` reader
pub type R = crate::R<DOEPCTL_SPEC>;
///Register `DOEPCTL` writer
pub type W = crate::W<DOEPCTL_SPEC>;
///Field `MPS` reader -
pub type MPS_R = crate::FieldReader;
///Field `USBACTEP` reader -
pub type USBACTEP_R = crate::BitReader;
///Field `NAKSTS` reader -
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYPE` reader -
pub type EPTYPE_R = crate::FieldReader;
///Field `SNP` reader -
pub type SNP_R = crate::BitReader;
///Field `SNP` writer -
pub type SNP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STALL` reader -
pub type STALL_R = crate::BitReader;
///Field `STALL` writer -
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNAK` writer -
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer -
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader -
pub type EPDIS_R = crate::BitReader;
///Field `EPENA` reader -
pub type EPENA_R = crate::BitReader;
///Field `EPENA` writer -
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1
    #[inline(always)]
    pub fn mps(&self) -> MPS_R {
        MPS_R::new((self.bits & 3) as u8)
    }
    ///Bit 15
    #[inline(always)]
    pub fn usbactep(&self) -> USBACTEP_R {
        USBACTEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20
    #[inline(always)]
    pub fn snp(&self) -> SNP_R {
        SNP_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 30
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DOEPCTL")
            .field("mps", &self.mps())
            .field("usbactep", &self.usbactep())
            .field("naksts", &self.naksts())
            .field("eptype", &self.eptype())
            .field("snp", &self.snp())
            .field("stall", &self.stall())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    ///Bit 20
    #[inline(always)]
    #[must_use]
    pub fn snp(&mut self) -> SNP_W<DOEPCTL_SPEC> {
        SNP_W::new(self, 20)
    }
    ///Bit 21
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<DOEPCTL_SPEC> {
        STALL_W::new(self, 21)
    }
    ///Bit 26
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<DOEPCTL_SPEC> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<DOEPCTL_SPEC> {
        SNAK_W::new(self, 27)
    }
    ///Bit 31
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<DOEPCTL_SPEC> {
        EPENA_W::new(self, 31)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`doepctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`doepctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DOEPCTL_SPEC;
impl crate::RegisterSpec for DOEPCTL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`doepctl::R`](R) reader structure
impl crate::Readable for DOEPCTL_SPEC {}
///`write(|w| ..)` method takes [`doepctl::W`](W) writer structure
impl crate::Writable for DOEPCTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DOEPCTL to value 0x8000
impl crate::Resettable for DOEPCTL_SPEC {
    const RESET_VALUE: u32 = 0x8000;
}
