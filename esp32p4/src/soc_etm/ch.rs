#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Cluster CH%s, containing CH*_EVT_ID, CH*_TASK_ID
pub struct CH {
    evt_id: EVT_ID,
    task_id: TASK_ID,
}
impl CH {
    ///0x00 - Channel0 event id register
    #[inline(always)]
    pub const fn evt_id(&self) -> &EVT_ID {
        &self.evt_id
    }
    ///0x04 - Channel0 task id register
    #[inline(always)]
    pub const fn task_id(&self) -> &TASK_ID {
        &self.task_id
    }
}
/**EVT_ID (rw) register accessor: Channel0 event id register

You can [`read`](crate::generic::Reg::read) this register and get [`evt_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@evt_id`] module*/
pub type EVT_ID = crate::Reg<evt_id::EVT_ID_SPEC>;
///Channel0 event id register
pub mod evt_id;
/**TASK_ID (rw) register accessor: Channel0 task id register

You can [`read`](crate::generic::Reg::read) this register and get [`task_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`task_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@task_id`] module*/
pub type TASK_ID = crate::Reg<task_id::TASK_ID_SPEC>;
///Channel0 task id register
pub mod task_id;
