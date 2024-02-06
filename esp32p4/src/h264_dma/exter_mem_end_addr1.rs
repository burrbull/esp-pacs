#[doc = "Register `EXTER_MEM_END_ADDR1` reader"]
pub type R = crate::R<EXTER_MEM_END_ADDR1_SPEC>;
#[doc = "Register `EXTER_MEM_END_ADDR1` writer"]
pub type W = crate::W<EXTER_MEM_END_ADDR1_SPEC>;
#[doc = "Field `ACCESS_EXTER_MEM_END_ADDR1` reader - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type ACCESS_EXTER_MEM_END_ADDR1_R = crate::FieldReader<u32>;
#[doc = "Field `ACCESS_EXTER_MEM_END_ADDR1` writer - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
pub type ACCESS_EXTER_MEM_END_ADDR1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    pub fn access_exter_mem_end_addr1(&self) -> ACCESS_EXTER_MEM_END_ADDR1_R {
        ACCESS_EXTER_MEM_END_ADDR1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTER_MEM_END_ADDR1")
            .field(
                "access_exter_mem_end_addr1",
                &format_args!("{}", self.access_exter_mem_end_addr1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTER_MEM_END_ADDR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The end address of accessible address space. The access address beyond this range would lead to descriptor error."]
    #[inline(always)]
    #[must_use]
    pub fn access_exter_mem_end_addr1(
        &mut self,
    ) -> ACCESS_EXTER_MEM_END_ADDR1_W<EXTER_MEM_END_ADDR1_SPEC> {
        ACCESS_EXTER_MEM_END_ADDR1_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "end address of exter memory range1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exter_mem_end_addr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exter_mem_end_addr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTER_MEM_END_ADDR1_SPEC;
impl crate::RegisterSpec for EXTER_MEM_END_ADDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exter_mem_end_addr1::R`](R) reader structure"]
impl crate::Readable for EXTER_MEM_END_ADDR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`exter_mem_end_addr1::W`](W) writer structure"]
impl crate::Writable for EXTER_MEM_END_ADDR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTER_MEM_END_ADDR1 to value 0x8fff_ffff"]
impl crate::Resettable for EXTER_MEM_END_ADDR1_SPEC {
    const RESET_VALUE: u32 = 0x8fff_ffff;
}
