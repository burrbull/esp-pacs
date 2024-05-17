///Register `DHT_VAl_AC1` reader
pub type R = crate::R<DHT_VAL_AC1_SPEC>;
///Field `DHT_VAL_AC1` reader - write codeword corresponding huffman values of ac1 table
pub type DHT_VAL_AC1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - write codeword corresponding huffman values of ac1 table
    #[inline(always)]
    pub fn dht_val_ac1(&self) -> DHT_VAL_AC1_R {
        DHT_VAL_AC1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DHT_VAl_AC1").field("dht_val_ac1", &self.dht_val_ac1()).finish()
    }
}
/**Trace and Debug registers

You can [`read`](crate::generic::Reg::read) this register and get [`dht_val_ac1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DHT_VAL_AC1_SPEC;
impl crate::RegisterSpec for DHT_VAL_AC1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dht_val_ac1::R`](R) reader structure
impl crate::Readable for DHT_VAL_AC1_SPEC {}
///`reset()` method sets DHT_VAl_AC1 to value 0
impl crate::Resettable for DHT_VAL_AC1_SPEC {
    const RESET_VALUE: u32 = 0;
}
