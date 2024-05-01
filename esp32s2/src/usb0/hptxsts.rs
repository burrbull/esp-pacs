///Register `HPTXSTS` reader
pub type R = crate::R<HPTXSTS_SPEC>;
///Field `PTXFSPCAVAIL` reader -
pub type PTXFSPCAVAIL_R = crate::FieldReader<u16>;
///Field `PTXQSPCAVAIL` reader -
pub type PTXQSPCAVAIL_R = crate::FieldReader;
///Field `PTXQTOP` reader -
pub type PTXQTOP_R = crate::FieldReader;
impl R {
    ///Bits 0:15
    #[inline(always)]
    pub fn ptxfspcavail(&self) -> PTXFSPCAVAIL_R {
        PTXFSPCAVAIL_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:20
    #[inline(always)]
    pub fn ptxqspcavail(&self) -> PTXQSPCAVAIL_R {
        PTXQSPCAVAIL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:31
    #[inline(always)]
    pub fn ptxqtop(&self) -> PTXQTOP_R {
        PTXQTOP_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPTXSTS")
            .field("ptxfspcavail", &self.ptxfspcavail())
            .field("ptxqspcavail", &self.ptxqspcavail())
            .field("ptxqtop", &self.ptxqtop())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`hptxsts::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HPTXSTS_SPEC;
impl crate::RegisterSpec for HPTXSTS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hptxsts::R`](R) reader structure
impl crate::Readable for HPTXSTS_SPEC {}
///`reset()` method sets HPTXSTS to value 0x0008_0100
impl crate::Resettable for HPTXSTS_SPEC {
    const RESET_VALUE: u32 = 0x0008_0100;
}
