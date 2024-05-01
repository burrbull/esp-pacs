///Register `COMD%s` reader
pub type R = crate::R<COMD_SPEC>;
///Register `COMD%s` writer
pub type W = crate::W<COMD_SPEC>;
///Field `COMMAND` reader - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART; 1: WRITE; 2: READ; 3: STOP; 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information.
pub type COMMAND_R = crate::FieldReader<u16>;
///Field `COMMAND` writer - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART; 1: WRITE; 2: READ; 3: STOP; 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information.
pub type COMMAND_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `COMMAND_DONE` reader - When command 0 is done in I2C Master mode, this bit changes to high level.
pub type COMMAND_DONE_R = crate::BitReader;
///Field `COMMAND_DONE` writer - When command 0 is done in I2C Master mode, this bit changes to high level.
pub type COMMAND_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:13 - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART; 1: WRITE; 2: READ; 3: STOP; 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information.
    #[inline(always)]
    pub fn command(&self) -> COMMAND_R {
        COMMAND_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bit 31 - When command 0 is done in I2C Master mode, this bit changes to high level.
    #[inline(always)]
    pub fn command_done(&self) -> COMMAND_DONE_R {
        COMMAND_DONE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMD")
            .field("command", &self.command())
            .field("command_done", &self.command_done())
            .finish()
    }
}
impl W {
    ///Bits 0:13 - This is the content of command 0. It consists of three parts: op_code is the command, 0: RSTART; 1: WRITE; 2: READ; 3: STOP; 4: END. Byte_num represents the number of bytes that need to be sent or received. ack_check_en, ack_exp and ack are used to control the ACK bit. See I2C cmd structure for more Information.
    #[inline(always)]
    #[must_use]
    pub fn command(&mut self) -> COMMAND_W<COMD_SPEC> {
        COMMAND_W::new(self, 0)
    }
    ///Bit 31 - When command 0 is done in I2C Master mode, this bit changes to high level.
    #[inline(always)]
    #[must_use]
    pub fn command_done(&mut self) -> COMMAND_DONE_W<COMD_SPEC> {
        COMMAND_DONE_W::new(self, 31)
    }
}
/**I2C command register %s

You can [`read`](crate::generic::Reg::read) this register and get [`comd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct COMD_SPEC;
impl crate::RegisterSpec for COMD_SPEC {
    type Ux = u32;
}
///`read()` method returns [`comd::R`](R) reader structure
impl crate::Readable for COMD_SPEC {}
///`write(|w| ..)` method takes [`comd::W`](W) writer structure
impl crate::Writable for COMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets COMD%s to value 0
impl crate::Resettable for COMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
