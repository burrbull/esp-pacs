#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    l1_icache_ctrl: L1_ICACHE_CTRL,
    l1_dcache_ctrl: L1_DCACHE_CTRL,
    l1_bypass_cache_conf: L1_BYPASS_CACHE_CONF,
    l1_cache_atomic_conf: L1_CACHE_ATOMIC_CONF,
    l1_icache_cachesize_conf: L1_ICACHE_CACHESIZE_CONF,
    l1_icache_blocksize_conf: L1_ICACHE_BLOCKSIZE_CONF,
    l1_dcache_cachesize_conf: L1_DCACHE_CACHESIZE_CONF,
    l1_dcache_blocksize_conf: L1_DCACHE_BLOCKSIZE_CONF,
    l1_cache_wrap_around_ctrl: L1_CACHE_WRAP_AROUND_CTRL,
    l1_cache_tag_mem_power_ctrl: L1_CACHE_TAG_MEM_POWER_CTRL,
    l1_cache_data_mem_power_ctrl: L1_CACHE_DATA_MEM_POWER_CTRL,
    l1_cache_freeze_ctrl: L1_CACHE_FREEZE_CTRL,
    l1_cache_data_mem_acs_conf: L1_CACHE_DATA_MEM_ACS_CONF,
    l1_cache_tag_mem_acs_conf: L1_CACHE_TAG_MEM_ACS_CONF,
    l1_icache0_prelock_conf: L1_ICACHE0_PRELOCK_CONF,
    l1_icache0_prelock_sct0_addr: L1_ICACHE0_PRELOCK_SCT0_ADDR,
    l1_icache0_prelock_sct1_addr: L1_ICACHE0_PRELOCK_SCT1_ADDR,
    l1_icache0_prelock_sct_size: L1_ICACHE0_PRELOCK_SCT_SIZE,
    l1_icache1_prelock_conf: L1_ICACHE1_PRELOCK_CONF,
    l1_icache1_prelock_sct0_addr: L1_ICACHE1_PRELOCK_SCT0_ADDR,
    l1_icache1_prelock_sct1_addr: L1_ICACHE1_PRELOCK_SCT1_ADDR,
    l1_icache1_prelock_sct_size: L1_ICACHE1_PRELOCK_SCT_SIZE,
    l1_icache2_prelock_conf: L1_ICACHE2_PRELOCK_CONF,
    l1_icache2_prelock_sct0_addr: L1_ICACHE2_PRELOCK_SCT0_ADDR,
    l1_icache2_prelock_sct1_addr: L1_ICACHE2_PRELOCK_SCT1_ADDR,
    l1_icache2_prelock_sct_size: L1_ICACHE2_PRELOCK_SCT_SIZE,
    l1_icache3_prelock_conf: L1_ICACHE3_PRELOCK_CONF,
    l1_icache3_prelock_sct0_addr: L1_ICACHE3_PRELOCK_SCT0_ADDR,
    l1_icache3_prelock_sct1_addr: L1_ICACHE3_PRELOCK_SCT1_ADDR,
    l1_icache3_prelock_sct_size: L1_ICACHE3_PRELOCK_SCT_SIZE,
    l1_dcache_prelock_conf: L1_DCACHE_PRELOCK_CONF,
    l1_dcache_prelock_sct0_addr: L1_DCACHE_PRELOCK_SCT0_ADDR,
    l1_dcache_prelock_sct1_addr: L1_DCACHE_PRELOCK_SCT1_ADDR,
    l1_dcache_prelock_sct_size: L1_DCACHE_PRELOCK_SCT_SIZE,
    lock_ctrl: LOCK_CTRL,
    lock_map: LOCK_MAP,
    lock_addr: LOCK_ADDR,
    lock_size: LOCK_SIZE,
    sync_ctrl: SYNC_CTRL,
    sync_map: SYNC_MAP,
    sync_addr: SYNC_ADDR,
    sync_size: SYNC_SIZE,
    l1_icache0_preload_ctrl: L1_ICACHE0_PRELOAD_CTRL,
    l1_icache0_preload_addr: L1_ICACHE0_PRELOAD_ADDR,
    l1_icache0_preload_size: L1_ICACHE0_PRELOAD_SIZE,
    l1_icache1_preload_ctrl: L1_ICACHE1_PRELOAD_CTRL,
    l1_icache1_preload_addr: L1_ICACHE1_PRELOAD_ADDR,
    l1_icache1_preload_size: L1_ICACHE1_PRELOAD_SIZE,
    l1_icache2_preload_ctrl: L1_ICACHE2_PRELOAD_CTRL,
    l1_icache2_preload_addr: L1_ICACHE2_PRELOAD_ADDR,
    l1_icache2_preload_size: L1_ICACHE2_PRELOAD_SIZE,
    l1_icache3_preload_ctrl: L1_ICACHE3_PRELOAD_CTRL,
    l1_icache3_preload_addr: L1_ICACHE3_PRELOAD_ADDR,
    l1_icache3_preload_size: L1_ICACHE3_PRELOAD_SIZE,
    l1_dcache_preload_ctrl: L1_DCACHE_PRELOAD_CTRL,
    l1_dcache_preload_addr: L1_DCACHE_PRELOAD_ADDR,
    l1_dcache_preload_size: L1_DCACHE_PRELOAD_SIZE,
    l1_icache0_autoload_ctrl: L1_ICACHE0_AUTOLOAD_CTRL,
    l1_icache0_autoload_sct0_addr: L1_ICACHE0_AUTOLOAD_SCT0_ADDR,
    l1_icache0_autoload_sct0_size: L1_ICACHE0_AUTOLOAD_SCT0_SIZE,
    l1_icache0_autoload_sct1_addr: L1_ICACHE0_AUTOLOAD_SCT1_ADDR,
    l1_icache0_autoload_sct1_size: L1_ICACHE0_AUTOLOAD_SCT1_SIZE,
    l1_icache1_autoload_ctrl: L1_ICACHE1_AUTOLOAD_CTRL,
    l1_icache1_autoload_sct0_addr: L1_ICACHE1_AUTOLOAD_SCT0_ADDR,
    l1_icache1_autoload_sct0_size: L1_ICACHE1_AUTOLOAD_SCT0_SIZE,
    l1_icache1_autoload_sct1_addr: L1_ICACHE1_AUTOLOAD_SCT1_ADDR,
    l1_icache1_autoload_sct1_size: L1_ICACHE1_AUTOLOAD_SCT1_SIZE,
    l1_icache2_autoload_ctrl: L1_ICACHE2_AUTOLOAD_CTRL,
    l1_icache2_autoload_sct0_addr: L1_ICACHE2_AUTOLOAD_SCT0_ADDR,
    l1_icache2_autoload_sct0_size: L1_ICACHE2_AUTOLOAD_SCT0_SIZE,
    l1_icache2_autoload_sct1_addr: L1_ICACHE2_AUTOLOAD_SCT1_ADDR,
    l1_icache2_autoload_sct1_size: L1_ICACHE2_AUTOLOAD_SCT1_SIZE,
    l1_icache3_autoload_ctrl: L1_ICACHE3_AUTOLOAD_CTRL,
    l1_icache3_autoload_sct0_addr: L1_ICACHE3_AUTOLOAD_SCT0_ADDR,
    l1_icache3_autoload_sct0_size: L1_ICACHE3_AUTOLOAD_SCT0_SIZE,
    l1_icache3_autoload_sct1_addr: L1_ICACHE3_AUTOLOAD_SCT1_ADDR,
    l1_icache3_autoload_sct1_size: L1_ICACHE3_AUTOLOAD_SCT1_SIZE,
    l1_dcache_autoload_ctrl: L1_DCACHE_AUTOLOAD_CTRL,
    l1_dcache_autoload_sct0_addr: L1_DCACHE_AUTOLOAD_SCT0_ADDR,
    l1_dcache_autoload_sct0_size: L1_DCACHE_AUTOLOAD_SCT0_SIZE,
    l1_dcache_autoload_sct1_addr: L1_DCACHE_AUTOLOAD_SCT1_ADDR,
    l1_dcache_autoload_sct1_size: L1_DCACHE_AUTOLOAD_SCT1_SIZE,
    l1_dcache_autoload_sct2_addr: L1_DCACHE_AUTOLOAD_SCT2_ADDR,
    l1_dcache_autoload_sct2_size: L1_DCACHE_AUTOLOAD_SCT2_SIZE,
    l1_dcache_autoload_sct3_addr: L1_DCACHE_AUTOLOAD_SCT3_ADDR,
    l1_dcache_autoload_sct3_size: L1_DCACHE_AUTOLOAD_SCT3_SIZE,
    l1_cache_acs_cnt_int_ena: L1_CACHE_ACS_CNT_INT_ENA,
    l1_cache_acs_cnt_int_clr: L1_CACHE_ACS_CNT_INT_CLR,
    l1_cache_acs_cnt_int_raw: L1_CACHE_ACS_CNT_INT_RAW,
    l1_cache_acs_cnt_int_st: L1_CACHE_ACS_CNT_INT_ST,
    l1_cache_acs_fail_ctrl: L1_CACHE_ACS_FAIL_CTRL,
    l1_cache_acs_fail_int_ena: L1_CACHE_ACS_FAIL_INT_ENA,
    l1_cache_acs_fail_int_clr: L1_CACHE_ACS_FAIL_INT_CLR,
    l1_cache_acs_fail_int_raw: L1_CACHE_ACS_FAIL_INT_RAW,
    l1_cache_acs_fail_int_st: L1_CACHE_ACS_FAIL_INT_ST,
    l1_cache_acs_cnt_ctrl: L1_CACHE_ACS_CNT_CTRL,
    l1_ibus0_acs_hit_cnt: L1_IBUS0_ACS_HIT_CNT,
    l1_ibus0_acs_miss_cnt: L1_IBUS0_ACS_MISS_CNT,
    l1_ibus0_acs_conflict_cnt: L1_IBUS0_ACS_CONFLICT_CNT,
    l1_ibus0_acs_nxtlvl_rd_cnt: L1_IBUS0_ACS_NXTLVL_RD_CNT,
    l1_ibus1_acs_hit_cnt: L1_IBUS1_ACS_HIT_CNT,
    l1_ibus1_acs_miss_cnt: L1_IBUS1_ACS_MISS_CNT,
    l1_ibus1_acs_conflict_cnt: L1_IBUS1_ACS_CONFLICT_CNT,
    l1_ibus1_acs_nxtlvl_rd_cnt: L1_IBUS1_ACS_NXTLVL_RD_CNT,
    l1_ibus2_acs_hit_cnt: L1_IBUS2_ACS_HIT_CNT,
    l1_ibus2_acs_miss_cnt: L1_IBUS2_ACS_MISS_CNT,
    l1_ibus2_acs_conflict_cnt: L1_IBUS2_ACS_CONFLICT_CNT,
    l1_ibus2_acs_nxtlvl_rd_cnt: L1_IBUS2_ACS_NXTLVL_RD_CNT,
    l1_ibus3_acs_hit_cnt: L1_IBUS3_ACS_HIT_CNT,
    l1_ibus3_acs_miss_cnt: L1_IBUS3_ACS_MISS_CNT,
    l1_ibus3_acs_conflict_cnt: L1_IBUS3_ACS_CONFLICT_CNT,
    l1_ibus3_acs_nxtlvl_rd_cnt: L1_IBUS3_ACS_NXTLVL_RD_CNT,
    l1_dbus0_acs_hit_cnt: L1_DBUS0_ACS_HIT_CNT,
    l1_dbus0_acs_miss_cnt: L1_DBUS0_ACS_MISS_CNT,
    l1_dbus0_acs_conflict_cnt: L1_DBUS0_ACS_CONFLICT_CNT,
    l1_dbus0_acs_nxtlvl_rd_cnt: L1_DBUS0_ACS_NXTLVL_RD_CNT,
    l1_dbus0_acs_nxtlvl_wr_cnt: L1_DBUS0_ACS_NXTLVL_WR_CNT,
    l1_dbus1_acs_hit_cnt: L1_DBUS1_ACS_HIT_CNT,
    l1_dbus1_acs_miss_cnt: L1_DBUS1_ACS_MISS_CNT,
    l1_dbus1_acs_conflict_cnt: L1_DBUS1_ACS_CONFLICT_CNT,
    l1_dbus1_acs_nxtlvl_rd_cnt: L1_DBUS1_ACS_NXTLVL_RD_CNT,
    l1_dbus1_acs_nxtlvl_wr_cnt: L1_DBUS1_ACS_NXTLVL_WR_CNT,
    l1_dbus2_acs_hit_cnt: L1_DBUS2_ACS_HIT_CNT,
    l1_dbus2_acs_miss_cnt: L1_DBUS2_ACS_MISS_CNT,
    l1_dbus2_acs_conflict_cnt: L1_DBUS2_ACS_CONFLICT_CNT,
    l1_dbus2_acs_nxtlvl_rd_cnt: L1_DBUS2_ACS_NXTLVL_RD_CNT,
    l1_dbus2_acs_nxtlvl_wr_cnt: L1_DBUS2_ACS_NXTLVL_WR_CNT,
    l1_dbus3_acs_hit_cnt: L1_DBUS3_ACS_HIT_CNT,
    l1_dbus3_acs_miss_cnt: L1_DBUS3_ACS_MISS_CNT,
    l1_dbus3_acs_conflict_cnt: L1_DBUS3_ACS_CONFLICT_CNT,
    l1_dbus3_acs_nxtlvl_rd_cnt: L1_DBUS3_ACS_NXTLVL_RD_CNT,
    l1_dbus3_acs_nxtlvl_wr_cnt: L1_DBUS3_ACS_NXTLVL_WR_CNT,
    l1_icache0_acs_fail_id_attr: L1_ICACHE0_ACS_FAIL_ID_ATTR,
    l1_icache0_acs_fail_addr: L1_ICACHE0_ACS_FAIL_ADDR,
    l1_icache1_acs_fail_id_attr: L1_ICACHE1_ACS_FAIL_ID_ATTR,
    l1_icache1_acs_fail_addr: L1_ICACHE1_ACS_FAIL_ADDR,
    l1_icache2_acs_fail_id_attr: L1_ICACHE2_ACS_FAIL_ID_ATTR,
    l1_icache2_acs_fail_addr: L1_ICACHE2_ACS_FAIL_ADDR,
    l1_icache3_acs_fail_id_attr: L1_ICACHE3_ACS_FAIL_ID_ATTR,
    l1_icache3_acs_fail_addr: L1_ICACHE3_ACS_FAIL_ADDR,
    l1_dcache_acs_fail_id_attr: L1_DCACHE_ACS_FAIL_ID_ATTR,
    l1_dcache_acs_fail_addr: L1_DCACHE_ACS_FAIL_ADDR,
    sync_l1_cache_preload_int_ena: SYNC_L1_CACHE_PRELOAD_INT_ENA,
    sync_l1_cache_preload_int_clr: SYNC_L1_CACHE_PRELOAD_INT_CLR,
    sync_l1_cache_preload_int_raw: SYNC_L1_CACHE_PRELOAD_INT_RAW,
    sync_l1_cache_preload_int_st: SYNC_L1_CACHE_PRELOAD_INT_ST,
    sync_l1_cache_preload_exception: SYNC_L1_CACHE_PRELOAD_EXCEPTION,
    l1_cache_sync_rst_ctrl: L1_CACHE_SYNC_RST_CTRL,
    l1_cache_preload_rst_ctrl: L1_CACHE_PRELOAD_RST_CTRL,
    l1_cache_autoload_buf_clr_ctrl: L1_CACHE_AUTOLOAD_BUF_CLR_CTRL,
    l1_unallocate_buffer_clear: L1_UNALLOCATE_BUFFER_CLEAR,
    l1_cache_object_ctrl: L1_CACHE_OBJECT_CTRL,
    l1_cache_way_object: L1_CACHE_WAY_OBJECT,
    l1_cache_vaddr: L1_CACHE_VADDR,
    l1_cache_debug_bus: L1_CACHE_DEBUG_BUS,
    level_split0: LEVEL_SPLIT0,
    l2_cache_ctrl: L2_CACHE_CTRL,
    l2_bypass_cache_conf: L2_BYPASS_CACHE_CONF,
    l2_cache_cachesize_conf: L2_CACHE_CACHESIZE_CONF,
    l2_cache_blocksize_conf: L2_CACHE_BLOCKSIZE_CONF,
    l2_cache_wrap_around_ctrl: L2_CACHE_WRAP_AROUND_CTRL,
    l2_cache_tag_mem_power_ctrl: L2_CACHE_TAG_MEM_POWER_CTRL,
    l2_cache_data_mem_power_ctrl: L2_CACHE_DATA_MEM_POWER_CTRL,
    l2_cache_freeze_ctrl: L2_CACHE_FREEZE_CTRL,
    l2_cache_data_mem_acs_conf: L2_CACHE_DATA_MEM_ACS_CONF,
    l2_cache_tag_mem_acs_conf: L2_CACHE_TAG_MEM_ACS_CONF,
    l2_cache_prelock_conf: L2_CACHE_PRELOCK_CONF,
    l2_cache_prelock_sct0_addr: L2_CACHE_PRELOCK_SCT0_ADDR,
    l2_cache_prelock_sct1_addr: L2_CACHE_PRELOCK_SCT1_ADDR,
    l2_cache_prelock_sct_size: L2_CACHE_PRELOCK_SCT_SIZE,
    l2_cache_preload_ctrl: L2_CACHE_PRELOAD_CTRL,
    l2_cache_preload_addr: L2_CACHE_PRELOAD_ADDR,
    l2_cache_preload_size: L2_CACHE_PRELOAD_SIZE,
    l2_cache_autoload_ctrl: L2_CACHE_AUTOLOAD_CTRL,
    l2_cache_autoload_sct0_addr: L2_CACHE_AUTOLOAD_SCT0_ADDR,
    l2_cache_autoload_sct0_size: L2_CACHE_AUTOLOAD_SCT0_SIZE,
    l2_cache_autoload_sct1_addr: L2_CACHE_AUTOLOAD_SCT1_ADDR,
    l2_cache_autoload_sct1_size: L2_CACHE_AUTOLOAD_SCT1_SIZE,
    l2_cache_autoload_sct2_addr: L2_CACHE_AUTOLOAD_SCT2_ADDR,
    l2_cache_autoload_sct2_size: L2_CACHE_AUTOLOAD_SCT2_SIZE,
    l2_cache_autoload_sct3_addr: L2_CACHE_AUTOLOAD_SCT3_ADDR,
    l2_cache_autoload_sct3_size: L2_CACHE_AUTOLOAD_SCT3_SIZE,
    l2_cache_acs_cnt_int_ena: L2_CACHE_ACS_CNT_INT_ENA,
    l2_cache_acs_cnt_int_clr: L2_CACHE_ACS_CNT_INT_CLR,
    l2_cache_acs_cnt_int_raw: L2_CACHE_ACS_CNT_INT_RAW,
    l2_cache_acs_cnt_int_st: L2_CACHE_ACS_CNT_INT_ST,
    l2_cache_acs_fail_ctrl: L2_CACHE_ACS_FAIL_CTRL,
    l2_cache_acs_fail_int_ena: L2_CACHE_ACS_FAIL_INT_ENA,
    l2_cache_acs_fail_int_clr: L2_CACHE_ACS_FAIL_INT_CLR,
    l2_cache_acs_fail_int_raw: L2_CACHE_ACS_FAIL_INT_RAW,
    l2_cache_acs_fail_int_st: L2_CACHE_ACS_FAIL_INT_ST,
    l2_cache_acs_cnt_ctrl: L2_CACHE_ACS_CNT_CTRL,
    l2_ibus0_acs_hit_cnt: L2_IBUS0_ACS_HIT_CNT,
    l2_ibus0_acs_miss_cnt: L2_IBUS0_ACS_MISS_CNT,
    l2_ibus0_acs_conflict_cnt: L2_IBUS0_ACS_CONFLICT_CNT,
    l2_ibus0_acs_nxtlvl_rd_cnt: L2_IBUS0_ACS_NXTLVL_RD_CNT,
    l2_ibus1_acs_hit_cnt: L2_IBUS1_ACS_HIT_CNT,
    l2_ibus1_acs_miss_cnt: L2_IBUS1_ACS_MISS_CNT,
    l2_ibus1_acs_conflict_cnt: L2_IBUS1_ACS_CONFLICT_CNT,
    l2_ibus1_acs_nxtlvl_rd_cnt: L2_IBUS1_ACS_NXTLVL_RD_CNT,
    l2_ibus2_acs_hit_cnt: L2_IBUS2_ACS_HIT_CNT,
    l2_ibus2_acs_miss_cnt: L2_IBUS2_ACS_MISS_CNT,
    l2_ibus2_acs_conflict_cnt: L2_IBUS2_ACS_CONFLICT_CNT,
    l2_ibus2_acs_nxtlvl_rd_cnt: L2_IBUS2_ACS_NXTLVL_RD_CNT,
    l2_ibus3_acs_hit_cnt: L2_IBUS3_ACS_HIT_CNT,
    l2_ibus3_acs_miss_cnt: L2_IBUS3_ACS_MISS_CNT,
    l2_ibus3_acs_conflict_cnt: L2_IBUS3_ACS_CONFLICT_CNT,
    l2_ibus3_acs_nxtlvl_rd_cnt: L2_IBUS3_ACS_NXTLVL_RD_CNT,
    l2_dbus0_acs_hit_cnt: L2_DBUS0_ACS_HIT_CNT,
    l2_dbus0_acs_miss_cnt: L2_DBUS0_ACS_MISS_CNT,
    l2_dbus0_acs_conflict_cnt: L2_DBUS0_ACS_CONFLICT_CNT,
    l2_dbus0_acs_nxtlvl_rd_cnt: L2_DBUS0_ACS_NXTLVL_RD_CNT,
    l2_dbus0_acs_nxtlvl_wr_cnt: L2_DBUS0_ACS_NXTLVL_WR_CNT,
    l2_dbus1_acs_hit_cnt: L2_DBUS1_ACS_HIT_CNT,
    l2_dbus1_acs_miss_cnt: L2_DBUS1_ACS_MISS_CNT,
    l2_dbus1_acs_conflict_cnt: L2_DBUS1_ACS_CONFLICT_CNT,
    l2_dbus1_acs_nxtlvl_rd_cnt: L2_DBUS1_ACS_NXTLVL_RD_CNT,
    l2_dbus1_acs_nxtlvl_wr_cnt: L2_DBUS1_ACS_NXTLVL_WR_CNT,
    l2_dbus2_acs_hit_cnt: L2_DBUS2_ACS_HIT_CNT,
    l2_dbus2_acs_miss_cnt: L2_DBUS2_ACS_MISS_CNT,
    l2_dbus2_acs_conflict_cnt: L2_DBUS2_ACS_CONFLICT_CNT,
    l2_dbus2_acs_nxtlvl_rd_cnt: L2_DBUS2_ACS_NXTLVL_RD_CNT,
    l2_dbus2_acs_nxtlvl_wr_cnt: L2_DBUS2_ACS_NXTLVL_WR_CNT,
    l2_dbus3_acs_hit_cnt: L2_DBUS3_ACS_HIT_CNT,
    l2_dbus3_acs_miss_cnt: L2_DBUS3_ACS_MISS_CNT,
    l2_dbus3_acs_conflict_cnt: L2_DBUS3_ACS_CONFLICT_CNT,
    l2_dbus3_acs_nxtlvl_rd_cnt: L2_DBUS3_ACS_NXTLVL_RD_CNT,
    l2_dbus3_acs_nxtlvl_wr_cnt: L2_DBUS3_ACS_NXTLVL_WR_CNT,
    l2_cache_acs_fail_id_attr: L2_CACHE_ACS_FAIL_ID_ATTR,
    l2_cache_acs_fail_addr: L2_CACHE_ACS_FAIL_ADDR,
    l2_cache_sync_preload_int_ena: L2_CACHE_SYNC_PRELOAD_INT_ENA,
    l2_cache_sync_preload_int_clr: L2_CACHE_SYNC_PRELOAD_INT_CLR,
    l2_cache_sync_preload_int_raw: L2_CACHE_SYNC_PRELOAD_INT_RAW,
    l2_cache_sync_preload_int_st: L2_CACHE_SYNC_PRELOAD_INT_ST,
    l2_cache_sync_preload_exception: L2_CACHE_SYNC_PRELOAD_EXCEPTION,
    l2_cache_sync_rst_ctrl: L2_CACHE_SYNC_RST_CTRL,
    l2_cache_preload_rst_ctrl: L2_CACHE_PRELOAD_RST_CTRL,
    l2_cache_autoload_buf_clr_ctrl: L2_CACHE_AUTOLOAD_BUF_CLR_CTRL,
    l2_unallocate_buffer_clear: L2_UNALLOCATE_BUFFER_CLEAR,
    l2_cache_access_attr_ctrl: L2_CACHE_ACCESS_ATTR_CTRL,
    l2_cache_object_ctrl: L2_CACHE_OBJECT_CTRL,
    l2_cache_way_object: L2_CACHE_WAY_OBJECT,
    l2_cache_vaddr: L2_CACHE_VADDR,
    l2_cache_debug_bus: L2_CACHE_DEBUG_BUS,
    level_split1: LEVEL_SPLIT1,
    clock_gate: CLOCK_GATE,
    redundancy_sig0: REDUNDANCY_SIG0,
    redundancy_sig1: REDUNDANCY_SIG1,
    redundancy_sig2: REDUNDANCY_SIG2,
    redundancy_sig3: REDUNDANCY_SIG3,
    redundancy_sig4: REDUNDANCY_SIG4,
    _reserved251: [u8; 0x10],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - L1 instruction Cache(L1-ICache) control register
    #[inline(always)]
    pub const fn l1_icache_ctrl(&self) -> &L1_ICACHE_CTRL {
        &self.l1_icache_ctrl
    }
    ///0x04 - L1 data Cache(L1-DCache) control register
    #[inline(always)]
    pub const fn l1_dcache_ctrl(&self) -> &L1_DCACHE_CTRL {
        &self.l1_dcache_ctrl
    }
    ///0x08 - Bypass Cache configure register
    #[inline(always)]
    pub const fn l1_bypass_cache_conf(&self) -> &L1_BYPASS_CACHE_CONF {
        &self.l1_bypass_cache_conf
    }
    ///0x0c - L1 Cache atomic feature configure register
    #[inline(always)]
    pub const fn l1_cache_atomic_conf(&self) -> &L1_CACHE_ATOMIC_CONF {
        &self.l1_cache_atomic_conf
    }
    ///0x10 - L1 instruction Cache CacheSize mode configure register
    #[inline(always)]
    pub const fn l1_icache_cachesize_conf(&self) -> &L1_ICACHE_CACHESIZE_CONF {
        &self.l1_icache_cachesize_conf
    }
    ///0x14 - L1 instruction Cache BlockSize mode configure register
    #[inline(always)]
    pub const fn l1_icache_blocksize_conf(&self) -> &L1_ICACHE_BLOCKSIZE_CONF {
        &self.l1_icache_blocksize_conf
    }
    ///0x18 - L1 data Cache CacheSize mode configure register
    #[inline(always)]
    pub const fn l1_dcache_cachesize_conf(&self) -> &L1_DCACHE_CACHESIZE_CONF {
        &self.l1_dcache_cachesize_conf
    }
    ///0x1c - L1 data Cache BlockSize mode configure register
    #[inline(always)]
    pub const fn l1_dcache_blocksize_conf(&self) -> &L1_DCACHE_BLOCKSIZE_CONF {
        &self.l1_dcache_blocksize_conf
    }
    ///0x20 - Cache wrap around control register
    #[inline(always)]
    pub const fn l1_cache_wrap_around_ctrl(&self) -> &L1_CACHE_WRAP_AROUND_CTRL {
        &self.l1_cache_wrap_around_ctrl
    }
    ///0x24 - Cache tag memory power control register
    #[inline(always)]
    pub const fn l1_cache_tag_mem_power_ctrl(&self) -> &L1_CACHE_TAG_MEM_POWER_CTRL {
        &self.l1_cache_tag_mem_power_ctrl
    }
    ///0x28 - Cache data memory power control register
    #[inline(always)]
    pub const fn l1_cache_data_mem_power_ctrl(&self) -> &L1_CACHE_DATA_MEM_POWER_CTRL {
        &self.l1_cache_data_mem_power_ctrl
    }
    ///0x2c - Cache Freeze control register
    #[inline(always)]
    pub const fn l1_cache_freeze_ctrl(&self) -> &L1_CACHE_FREEZE_CTRL {
        &self.l1_cache_freeze_ctrl
    }
    ///0x30 - Cache data memory access configure register
    #[inline(always)]
    pub const fn l1_cache_data_mem_acs_conf(&self) -> &L1_CACHE_DATA_MEM_ACS_CONF {
        &self.l1_cache_data_mem_acs_conf
    }
    ///0x34 - Cache tag memory access configure register
    #[inline(always)]
    pub const fn l1_cache_tag_mem_acs_conf(&self) -> &L1_CACHE_TAG_MEM_ACS_CONF {
        &self.l1_cache_tag_mem_acs_conf
    }
    ///0x38 - L1 instruction Cache 0 prelock configure register
    #[inline(always)]
    pub const fn l1_icache0_prelock_conf(&self) -> &L1_ICACHE0_PRELOCK_CONF {
        &self.l1_icache0_prelock_conf
    }
    ///0x3c - L1 instruction Cache 0 prelock section0 address configure register
    #[inline(always)]
    pub const fn l1_icache0_prelock_sct0_addr(&self) -> &L1_ICACHE0_PRELOCK_SCT0_ADDR {
        &self.l1_icache0_prelock_sct0_addr
    }
    ///0x40 - L1 instruction Cache 0 prelock section1 address configure register
    #[inline(always)]
    pub const fn l1_icache0_prelock_sct1_addr(&self) -> &L1_ICACHE0_PRELOCK_SCT1_ADDR {
        &self.l1_icache0_prelock_sct1_addr
    }
    ///0x44 - L1 instruction Cache 0 prelock section size configure register
    #[inline(always)]
    pub const fn l1_icache0_prelock_sct_size(&self) -> &L1_ICACHE0_PRELOCK_SCT_SIZE {
        &self.l1_icache0_prelock_sct_size
    }
    ///0x48 - L1 instruction Cache 1 prelock configure register
    #[inline(always)]
    pub const fn l1_icache1_prelock_conf(&self) -> &L1_ICACHE1_PRELOCK_CONF {
        &self.l1_icache1_prelock_conf
    }
    ///0x4c - L1 instruction Cache 1 prelock section0 address configure register
    #[inline(always)]
    pub const fn l1_icache1_prelock_sct0_addr(&self) -> &L1_ICACHE1_PRELOCK_SCT0_ADDR {
        &self.l1_icache1_prelock_sct0_addr
    }
    ///0x50 - L1 instruction Cache 1 prelock section1 address configure register
    #[inline(always)]
    pub const fn l1_icache1_prelock_sct1_addr(&self) -> &L1_ICACHE1_PRELOCK_SCT1_ADDR {
        &self.l1_icache1_prelock_sct1_addr
    }
    ///0x54 - L1 instruction Cache 1 prelock section size configure register
    #[inline(always)]
    pub const fn l1_icache1_prelock_sct_size(&self) -> &L1_ICACHE1_PRELOCK_SCT_SIZE {
        &self.l1_icache1_prelock_sct_size
    }
    ///0x58 - L1 instruction Cache 2 prelock configure register
    #[inline(always)]
    pub const fn l1_icache2_prelock_conf(&self) -> &L1_ICACHE2_PRELOCK_CONF {
        &self.l1_icache2_prelock_conf
    }
    ///0x5c - L1 instruction Cache 2 prelock section0 address configure register
    #[inline(always)]
    pub const fn l1_icache2_prelock_sct0_addr(&self) -> &L1_ICACHE2_PRELOCK_SCT0_ADDR {
        &self.l1_icache2_prelock_sct0_addr
    }
    ///0x60 - L1 instruction Cache 2 prelock section1 address configure register
    #[inline(always)]
    pub const fn l1_icache2_prelock_sct1_addr(&self) -> &L1_ICACHE2_PRELOCK_SCT1_ADDR {
        &self.l1_icache2_prelock_sct1_addr
    }
    ///0x64 - L1 instruction Cache 2 prelock section size configure register
    #[inline(always)]
    pub const fn l1_icache2_prelock_sct_size(&self) -> &L1_ICACHE2_PRELOCK_SCT_SIZE {
        &self.l1_icache2_prelock_sct_size
    }
    ///0x68 - L1 instruction Cache 3 prelock configure register
    #[inline(always)]
    pub const fn l1_icache3_prelock_conf(&self) -> &L1_ICACHE3_PRELOCK_CONF {
        &self.l1_icache3_prelock_conf
    }
    ///0x6c - L1 instruction Cache 3 prelock section0 address configure register
    #[inline(always)]
    pub const fn l1_icache3_prelock_sct0_addr(&self) -> &L1_ICACHE3_PRELOCK_SCT0_ADDR {
        &self.l1_icache3_prelock_sct0_addr
    }
    ///0x70 - L1 instruction Cache 3 prelock section1 address configure register
    #[inline(always)]
    pub const fn l1_icache3_prelock_sct1_addr(&self) -> &L1_ICACHE3_PRELOCK_SCT1_ADDR {
        &self.l1_icache3_prelock_sct1_addr
    }
    ///0x74 - L1 instruction Cache 3 prelock section size configure register
    #[inline(always)]
    pub const fn l1_icache3_prelock_sct_size(&self) -> &L1_ICACHE3_PRELOCK_SCT_SIZE {
        &self.l1_icache3_prelock_sct_size
    }
    ///0x78 - L1 data Cache prelock configure register
    #[inline(always)]
    pub const fn l1_dcache_prelock_conf(&self) -> &L1_DCACHE_PRELOCK_CONF {
        &self.l1_dcache_prelock_conf
    }
    ///0x7c - L1 data Cache prelock section0 address configure register
    #[inline(always)]
    pub const fn l1_dcache_prelock_sct0_addr(&self) -> &L1_DCACHE_PRELOCK_SCT0_ADDR {
        &self.l1_dcache_prelock_sct0_addr
    }
    ///0x80 - L1 data Cache prelock section1 address configure register
    #[inline(always)]
    pub const fn l1_dcache_prelock_sct1_addr(&self) -> &L1_DCACHE_PRELOCK_SCT1_ADDR {
        &self.l1_dcache_prelock_sct1_addr
    }
    ///0x84 - L1 data Cache prelock section size configure register
    #[inline(always)]
    pub const fn l1_dcache_prelock_sct_size(&self) -> &L1_DCACHE_PRELOCK_SCT_SIZE {
        &self.l1_dcache_prelock_sct_size
    }
    ///0x88 - Lock-class (manual lock) operation control register
    #[inline(always)]
    pub const fn lock_ctrl(&self) -> &LOCK_CTRL {
        &self.lock_ctrl
    }
    ///0x8c - Lock (manual lock) map configure register
    #[inline(always)]
    pub const fn lock_map(&self) -> &LOCK_MAP {
        &self.lock_map
    }
    ///0x90 - Lock (manual lock) address configure register
    #[inline(always)]
    pub const fn lock_addr(&self) -> &LOCK_ADDR {
        &self.lock_addr
    }
    ///0x94 - Lock (manual lock) size configure register
    #[inline(always)]
    pub const fn lock_size(&self) -> &LOCK_SIZE {
        &self.lock_size
    }
    ///0x98 - Sync-class operation control register
    #[inline(always)]
    pub const fn sync_ctrl(&self) -> &SYNC_CTRL {
        &self.sync_ctrl
    }
    ///0x9c - Sync map configure register
    #[inline(always)]
    pub const fn sync_map(&self) -> &SYNC_MAP {
        &self.sync_map
    }
    ///0xa0 - Sync address configure register
    #[inline(always)]
    pub const fn sync_addr(&self) -> &SYNC_ADDR {
        &self.sync_addr
    }
    ///0xa4 - Sync size configure register
    #[inline(always)]
    pub const fn sync_size(&self) -> &SYNC_SIZE {
        &self.sync_size
    }
    ///0xa8 - L1 instruction Cache 0 preload-operation control register
    #[inline(always)]
    pub const fn l1_icache0_preload_ctrl(&self) -> &L1_ICACHE0_PRELOAD_CTRL {
        &self.l1_icache0_preload_ctrl
    }
    ///0xac - L1 instruction Cache 0 preload address configure register
    #[inline(always)]
    pub const fn l1_icache0_preload_addr(&self) -> &L1_ICACHE0_PRELOAD_ADDR {
        &self.l1_icache0_preload_addr
    }
    ///0xb0 - L1 instruction Cache 0 preload size configure register
    #[inline(always)]
    pub const fn l1_icache0_preload_size(&self) -> &L1_ICACHE0_PRELOAD_SIZE {
        &self.l1_icache0_preload_size
    }
    ///0xb4 - L1 instruction Cache 1 preload-operation control register
    #[inline(always)]
    pub const fn l1_icache1_preload_ctrl(&self) -> &L1_ICACHE1_PRELOAD_CTRL {
        &self.l1_icache1_preload_ctrl
    }
    ///0xb8 - L1 instruction Cache 1 preload address configure register
    #[inline(always)]
    pub const fn l1_icache1_preload_addr(&self) -> &L1_ICACHE1_PRELOAD_ADDR {
        &self.l1_icache1_preload_addr
    }
    ///0xbc - L1 instruction Cache 1 preload size configure register
    #[inline(always)]
    pub const fn l1_icache1_preload_size(&self) -> &L1_ICACHE1_PRELOAD_SIZE {
        &self.l1_icache1_preload_size
    }
    ///0xc0 - L1 instruction Cache 2 preload-operation control register
    #[inline(always)]
    pub const fn l1_icache2_preload_ctrl(&self) -> &L1_ICACHE2_PRELOAD_CTRL {
        &self.l1_icache2_preload_ctrl
    }
    ///0xc4 - L1 instruction Cache 2 preload address configure register
    #[inline(always)]
    pub const fn l1_icache2_preload_addr(&self) -> &L1_ICACHE2_PRELOAD_ADDR {
        &self.l1_icache2_preload_addr
    }
    ///0xc8 - L1 instruction Cache 2 preload size configure register
    #[inline(always)]
    pub const fn l1_icache2_preload_size(&self) -> &L1_ICACHE2_PRELOAD_SIZE {
        &self.l1_icache2_preload_size
    }
    ///0xcc - L1 instruction Cache 3 preload-operation control register
    #[inline(always)]
    pub const fn l1_icache3_preload_ctrl(&self) -> &L1_ICACHE3_PRELOAD_CTRL {
        &self.l1_icache3_preload_ctrl
    }
    ///0xd0 - L1 instruction Cache 3 preload address configure register
    #[inline(always)]
    pub const fn l1_icache3_preload_addr(&self) -> &L1_ICACHE3_PRELOAD_ADDR {
        &self.l1_icache3_preload_addr
    }
    ///0xd4 - L1 instruction Cache 3 preload size configure register
    #[inline(always)]
    pub const fn l1_icache3_preload_size(&self) -> &L1_ICACHE3_PRELOAD_SIZE {
        &self.l1_icache3_preload_size
    }
    ///0xd8 - L1 data Cache preload-operation control register
    #[inline(always)]
    pub const fn l1_dcache_preload_ctrl(&self) -> &L1_DCACHE_PRELOAD_CTRL {
        &self.l1_dcache_preload_ctrl
    }
    ///0xdc - L1 data Cache preload address configure register
    #[inline(always)]
    pub const fn l1_dcache_preload_addr(&self) -> &L1_DCACHE_PRELOAD_ADDR {
        &self.l1_dcache_preload_addr
    }
    ///0xe0 - L1 data Cache preload size configure register
    #[inline(always)]
    pub const fn l1_dcache_preload_size(&self) -> &L1_DCACHE_PRELOAD_SIZE {
        &self.l1_dcache_preload_size
    }
    ///0xe4 - L1 instruction Cache 0 autoload-operation control register
    #[inline(always)]
    pub const fn l1_icache0_autoload_ctrl(&self) -> &L1_ICACHE0_AUTOLOAD_CTRL {
        &self.l1_icache0_autoload_ctrl
    }
    ///0xe8 - L1 instruction Cache 0 autoload section 0 address configure register
    #[inline(always)]
    pub const fn l1_icache0_autoload_sct0_addr(&self) -> &L1_ICACHE0_AUTOLOAD_SCT0_ADDR {
        &self.l1_icache0_autoload_sct0_addr
    }
    ///0xec - L1 instruction Cache 0 autoload section 0 size configure register
    #[inline(always)]
    pub const fn l1_icache0_autoload_sct0_size(&self) -> &L1_ICACHE0_AUTOLOAD_SCT0_SIZE {
        &self.l1_icache0_autoload_sct0_size
    }
    ///0xf0 - L1 instruction Cache 0 autoload section 1 address configure register
    #[inline(always)]
    pub const fn l1_icache0_autoload_sct1_addr(&self) -> &L1_ICACHE0_AUTOLOAD_SCT1_ADDR {
        &self.l1_icache0_autoload_sct1_addr
    }
    ///0xf4 - L1 instruction Cache 0 autoload section 1 size configure register
    #[inline(always)]
    pub const fn l1_icache0_autoload_sct1_size(&self) -> &L1_ICACHE0_AUTOLOAD_SCT1_SIZE {
        &self.l1_icache0_autoload_sct1_size
    }
    ///0xf8 - L1 instruction Cache 1 autoload-operation control register
    #[inline(always)]
    pub const fn l1_icache1_autoload_ctrl(&self) -> &L1_ICACHE1_AUTOLOAD_CTRL {
        &self.l1_icache1_autoload_ctrl
    }
    ///0xfc - L1 instruction Cache 1 autoload section 0 address configure register
    #[inline(always)]
    pub const fn l1_icache1_autoload_sct0_addr(&self) -> &L1_ICACHE1_AUTOLOAD_SCT0_ADDR {
        &self.l1_icache1_autoload_sct0_addr
    }
    ///0x100 - L1 instruction Cache 1 autoload section 0 size configure register
    #[inline(always)]
    pub const fn l1_icache1_autoload_sct0_size(&self) -> &L1_ICACHE1_AUTOLOAD_SCT0_SIZE {
        &self.l1_icache1_autoload_sct0_size
    }
    ///0x104 - L1 instruction Cache 1 autoload section 1 address configure register
    #[inline(always)]
    pub const fn l1_icache1_autoload_sct1_addr(&self) -> &L1_ICACHE1_AUTOLOAD_SCT1_ADDR {
        &self.l1_icache1_autoload_sct1_addr
    }
    ///0x108 - L1 instruction Cache 1 autoload section 1 size configure register
    #[inline(always)]
    pub const fn l1_icache1_autoload_sct1_size(&self) -> &L1_ICACHE1_AUTOLOAD_SCT1_SIZE {
        &self.l1_icache1_autoload_sct1_size
    }
    ///0x10c - L1 instruction Cache 2 autoload-operation control register
    #[inline(always)]
    pub const fn l1_icache2_autoload_ctrl(&self) -> &L1_ICACHE2_AUTOLOAD_CTRL {
        &self.l1_icache2_autoload_ctrl
    }
    ///0x110 - L1 instruction Cache 2 autoload section 0 address configure register
    #[inline(always)]
    pub const fn l1_icache2_autoload_sct0_addr(&self) -> &L1_ICACHE2_AUTOLOAD_SCT0_ADDR {
        &self.l1_icache2_autoload_sct0_addr
    }
    ///0x114 - L1 instruction Cache 2 autoload section 0 size configure register
    #[inline(always)]
    pub const fn l1_icache2_autoload_sct0_size(&self) -> &L1_ICACHE2_AUTOLOAD_SCT0_SIZE {
        &self.l1_icache2_autoload_sct0_size
    }
    ///0x118 - L1 instruction Cache 2 autoload section 1 address configure register
    #[inline(always)]
    pub const fn l1_icache2_autoload_sct1_addr(&self) -> &L1_ICACHE2_AUTOLOAD_SCT1_ADDR {
        &self.l1_icache2_autoload_sct1_addr
    }
    ///0x11c - L1 instruction Cache 2 autoload section 1 size configure register
    #[inline(always)]
    pub const fn l1_icache2_autoload_sct1_size(&self) -> &L1_ICACHE2_AUTOLOAD_SCT1_SIZE {
        &self.l1_icache2_autoload_sct1_size
    }
    ///0x120 - L1 instruction Cache 3 autoload-operation control register
    #[inline(always)]
    pub const fn l1_icache3_autoload_ctrl(&self) -> &L1_ICACHE3_AUTOLOAD_CTRL {
        &self.l1_icache3_autoload_ctrl
    }
    ///0x124 - L1 instruction Cache 3 autoload section 0 address configure register
    #[inline(always)]
    pub const fn l1_icache3_autoload_sct0_addr(&self) -> &L1_ICACHE3_AUTOLOAD_SCT0_ADDR {
        &self.l1_icache3_autoload_sct0_addr
    }
    ///0x128 - L1 instruction Cache 3 autoload section 0 size configure register
    #[inline(always)]
    pub const fn l1_icache3_autoload_sct0_size(&self) -> &L1_ICACHE3_AUTOLOAD_SCT0_SIZE {
        &self.l1_icache3_autoload_sct0_size
    }
    ///0x12c - L1 instruction Cache 3 autoload section 1 address configure register
    #[inline(always)]
    pub const fn l1_icache3_autoload_sct1_addr(&self) -> &L1_ICACHE3_AUTOLOAD_SCT1_ADDR {
        &self.l1_icache3_autoload_sct1_addr
    }
    ///0x130 - L1 instruction Cache 3 autoload section 1 size configure register
    #[inline(always)]
    pub const fn l1_icache3_autoload_sct1_size(&self) -> &L1_ICACHE3_AUTOLOAD_SCT1_SIZE {
        &self.l1_icache3_autoload_sct1_size
    }
    ///0x134 - L1 data Cache autoload-operation control register
    #[inline(always)]
    pub const fn l1_dcache_autoload_ctrl(&self) -> &L1_DCACHE_AUTOLOAD_CTRL {
        &self.l1_dcache_autoload_ctrl
    }
    ///0x138 - L1 data Cache autoload section 0 address configure register
    #[inline(always)]
    pub const fn l1_dcache_autoload_sct0_addr(&self) -> &L1_DCACHE_AUTOLOAD_SCT0_ADDR {
        &self.l1_dcache_autoload_sct0_addr
    }
    ///0x13c - L1 data Cache autoload section 0 size configure register
    #[inline(always)]
    pub const fn l1_dcache_autoload_sct0_size(&self) -> &L1_DCACHE_AUTOLOAD_SCT0_SIZE {
        &self.l1_dcache_autoload_sct0_size
    }
    ///0x140 - L1 data Cache autoload section 1 address configure register
    #[inline(always)]
    pub const fn l1_dcache_autoload_sct1_addr(&self) -> &L1_DCACHE_AUTOLOAD_SCT1_ADDR {
        &self.l1_dcache_autoload_sct1_addr
    }
    ///0x144 - L1 data Cache autoload section 1 size configure register
    #[inline(always)]
    pub const fn l1_dcache_autoload_sct1_size(&self) -> &L1_DCACHE_AUTOLOAD_SCT1_SIZE {
        &self.l1_dcache_autoload_sct1_size
    }
    ///0x148 - L1 data Cache autoload section 2 address configure register
    #[inline(always)]
    pub const fn l1_dcache_autoload_sct2_addr(&self) -> &L1_DCACHE_AUTOLOAD_SCT2_ADDR {
        &self.l1_dcache_autoload_sct2_addr
    }
    ///0x14c - L1 data Cache autoload section 2 size configure register
    #[inline(always)]
    pub const fn l1_dcache_autoload_sct2_size(&self) -> &L1_DCACHE_AUTOLOAD_SCT2_SIZE {
        &self.l1_dcache_autoload_sct2_size
    }
    ///0x150 - L1 data Cache autoload section 1 address configure register
    #[inline(always)]
    pub const fn l1_dcache_autoload_sct3_addr(&self) -> &L1_DCACHE_AUTOLOAD_SCT3_ADDR {
        &self.l1_dcache_autoload_sct3_addr
    }
    ///0x154 - L1 data Cache autoload section 1 size configure register
    #[inline(always)]
    pub const fn l1_dcache_autoload_sct3_size(&self) -> &L1_DCACHE_AUTOLOAD_SCT3_SIZE {
        &self.l1_dcache_autoload_sct3_size
    }
    ///0x158 - Cache Access Counter Interrupt enable register
    #[inline(always)]
    pub const fn l1_cache_acs_cnt_int_ena(&self) -> &L1_CACHE_ACS_CNT_INT_ENA {
        &self.l1_cache_acs_cnt_int_ena
    }
    ///0x15c - Cache Access Counter Interrupt clear register
    #[inline(always)]
    pub const fn l1_cache_acs_cnt_int_clr(&self) -> &L1_CACHE_ACS_CNT_INT_CLR {
        &self.l1_cache_acs_cnt_int_clr
    }
    ///0x160 - Cache Access Counter Interrupt raw register
    #[inline(always)]
    pub const fn l1_cache_acs_cnt_int_raw(&self) -> &L1_CACHE_ACS_CNT_INT_RAW {
        &self.l1_cache_acs_cnt_int_raw
    }
    ///0x164 - Cache Access Counter Interrupt status register
    #[inline(always)]
    pub const fn l1_cache_acs_cnt_int_st(&self) -> &L1_CACHE_ACS_CNT_INT_ST {
        &self.l1_cache_acs_cnt_int_st
    }
    ///0x168 - Cache Access Fail Configuration register
    #[inline(always)]
    pub const fn l1_cache_acs_fail_ctrl(&self) -> &L1_CACHE_ACS_FAIL_CTRL {
        &self.l1_cache_acs_fail_ctrl
    }
    ///0x16c - Cache Access Fail Interrupt enable register
    #[inline(always)]
    pub const fn l1_cache_acs_fail_int_ena(&self) -> &L1_CACHE_ACS_FAIL_INT_ENA {
        &self.l1_cache_acs_fail_int_ena
    }
    ///0x170 - L1-Cache Access Fail Interrupt clear register
    #[inline(always)]
    pub const fn l1_cache_acs_fail_int_clr(&self) -> &L1_CACHE_ACS_FAIL_INT_CLR {
        &self.l1_cache_acs_fail_int_clr
    }
    ///0x174 - Cache Access Fail Interrupt raw register
    #[inline(always)]
    pub const fn l1_cache_acs_fail_int_raw(&self) -> &L1_CACHE_ACS_FAIL_INT_RAW {
        &self.l1_cache_acs_fail_int_raw
    }
    ///0x178 - Cache Access Fail Interrupt status register
    #[inline(always)]
    pub const fn l1_cache_acs_fail_int_st(&self) -> &L1_CACHE_ACS_FAIL_INT_ST {
        &self.l1_cache_acs_fail_int_st
    }
    ///0x17c - Cache Access Counter enable and clear register
    #[inline(always)]
    pub const fn l1_cache_acs_cnt_ctrl(&self) -> &L1_CACHE_ACS_CNT_CTRL {
        &self.l1_cache_acs_cnt_ctrl
    }
    ///0x180 - L1-ICache bus0 Hit-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus0_acs_hit_cnt(&self) -> &L1_IBUS0_ACS_HIT_CNT {
        &self.l1_ibus0_acs_hit_cnt
    }
    ///0x184 - L1-ICache bus0 Miss-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus0_acs_miss_cnt(&self) -> &L1_IBUS0_ACS_MISS_CNT {
        &self.l1_ibus0_acs_miss_cnt
    }
    ///0x188 - L1-ICache bus0 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus0_acs_conflict_cnt(&self) -> &L1_IBUS0_ACS_CONFLICT_CNT {
        &self.l1_ibus0_acs_conflict_cnt
    }
    ///0x18c - L1-ICache bus0 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus0_acs_nxtlvl_rd_cnt(&self) -> &L1_IBUS0_ACS_NXTLVL_RD_CNT {
        &self.l1_ibus0_acs_nxtlvl_rd_cnt
    }
    ///0x190 - L1-ICache bus1 Hit-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus1_acs_hit_cnt(&self) -> &L1_IBUS1_ACS_HIT_CNT {
        &self.l1_ibus1_acs_hit_cnt
    }
    ///0x194 - L1-ICache bus1 Miss-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus1_acs_miss_cnt(&self) -> &L1_IBUS1_ACS_MISS_CNT {
        &self.l1_ibus1_acs_miss_cnt
    }
    ///0x198 - L1-ICache bus1 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus1_acs_conflict_cnt(&self) -> &L1_IBUS1_ACS_CONFLICT_CNT {
        &self.l1_ibus1_acs_conflict_cnt
    }
    ///0x19c - L1-ICache bus1 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus1_acs_nxtlvl_rd_cnt(&self) -> &L1_IBUS1_ACS_NXTLVL_RD_CNT {
        &self.l1_ibus1_acs_nxtlvl_rd_cnt
    }
    ///0x1a0 - L1-ICache bus2 Hit-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus2_acs_hit_cnt(&self) -> &L1_IBUS2_ACS_HIT_CNT {
        &self.l1_ibus2_acs_hit_cnt
    }
    ///0x1a4 - L1-ICache bus2 Miss-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus2_acs_miss_cnt(&self) -> &L1_IBUS2_ACS_MISS_CNT {
        &self.l1_ibus2_acs_miss_cnt
    }
    ///0x1a8 - L1-ICache bus2 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus2_acs_conflict_cnt(&self) -> &L1_IBUS2_ACS_CONFLICT_CNT {
        &self.l1_ibus2_acs_conflict_cnt
    }
    ///0x1ac - L1-ICache bus2 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus2_acs_nxtlvl_rd_cnt(&self) -> &L1_IBUS2_ACS_NXTLVL_RD_CNT {
        &self.l1_ibus2_acs_nxtlvl_rd_cnt
    }
    ///0x1b0 - L1-ICache bus3 Hit-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus3_acs_hit_cnt(&self) -> &L1_IBUS3_ACS_HIT_CNT {
        &self.l1_ibus3_acs_hit_cnt
    }
    ///0x1b4 - L1-ICache bus3 Miss-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus3_acs_miss_cnt(&self) -> &L1_IBUS3_ACS_MISS_CNT {
        &self.l1_ibus3_acs_miss_cnt
    }
    ///0x1b8 - L1-ICache bus3 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus3_acs_conflict_cnt(&self) -> &L1_IBUS3_ACS_CONFLICT_CNT {
        &self.l1_ibus3_acs_conflict_cnt
    }
    ///0x1bc - L1-ICache bus3 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l1_ibus3_acs_nxtlvl_rd_cnt(&self) -> &L1_IBUS3_ACS_NXTLVL_RD_CNT {
        &self.l1_ibus3_acs_nxtlvl_rd_cnt
    }
    ///0x1c0 - L1-DCache bus0 Hit-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus0_acs_hit_cnt(&self) -> &L1_DBUS0_ACS_HIT_CNT {
        &self.l1_dbus0_acs_hit_cnt
    }
    ///0x1c4 - L1-DCache bus0 Miss-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus0_acs_miss_cnt(&self) -> &L1_DBUS0_ACS_MISS_CNT {
        &self.l1_dbus0_acs_miss_cnt
    }
    ///0x1c8 - L1-DCache bus0 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus0_acs_conflict_cnt(&self) -> &L1_DBUS0_ACS_CONFLICT_CNT {
        &self.l1_dbus0_acs_conflict_cnt
    }
    ///0x1cc - L1-DCache bus0 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus0_acs_nxtlvl_rd_cnt(&self) -> &L1_DBUS0_ACS_NXTLVL_RD_CNT {
        &self.l1_dbus0_acs_nxtlvl_rd_cnt
    }
    ///0x1d0 - L1-DCache bus0 WB-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus0_acs_nxtlvl_wr_cnt(&self) -> &L1_DBUS0_ACS_NXTLVL_WR_CNT {
        &self.l1_dbus0_acs_nxtlvl_wr_cnt
    }
    ///0x1d4 - L1-DCache bus1 Hit-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus1_acs_hit_cnt(&self) -> &L1_DBUS1_ACS_HIT_CNT {
        &self.l1_dbus1_acs_hit_cnt
    }
    ///0x1d8 - L1-DCache bus1 Miss-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus1_acs_miss_cnt(&self) -> &L1_DBUS1_ACS_MISS_CNT {
        &self.l1_dbus1_acs_miss_cnt
    }
    ///0x1dc - L1-DCache bus1 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus1_acs_conflict_cnt(&self) -> &L1_DBUS1_ACS_CONFLICT_CNT {
        &self.l1_dbus1_acs_conflict_cnt
    }
    ///0x1e0 - L1-DCache bus1 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus1_acs_nxtlvl_rd_cnt(&self) -> &L1_DBUS1_ACS_NXTLVL_RD_CNT {
        &self.l1_dbus1_acs_nxtlvl_rd_cnt
    }
    ///0x1e4 - L1-DCache bus1 WB-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus1_acs_nxtlvl_wr_cnt(&self) -> &L1_DBUS1_ACS_NXTLVL_WR_CNT {
        &self.l1_dbus1_acs_nxtlvl_wr_cnt
    }
    ///0x1e8 - L1-DCache bus2 Hit-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus2_acs_hit_cnt(&self) -> &L1_DBUS2_ACS_HIT_CNT {
        &self.l1_dbus2_acs_hit_cnt
    }
    ///0x1ec - L1-DCache bus2 Miss-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus2_acs_miss_cnt(&self) -> &L1_DBUS2_ACS_MISS_CNT {
        &self.l1_dbus2_acs_miss_cnt
    }
    ///0x1f0 - L1-DCache bus2 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus2_acs_conflict_cnt(&self) -> &L1_DBUS2_ACS_CONFLICT_CNT {
        &self.l1_dbus2_acs_conflict_cnt
    }
    ///0x1f4 - L1-DCache bus2 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus2_acs_nxtlvl_rd_cnt(&self) -> &L1_DBUS2_ACS_NXTLVL_RD_CNT {
        &self.l1_dbus2_acs_nxtlvl_rd_cnt
    }
    ///0x1f8 - L1-DCache bus2 WB-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus2_acs_nxtlvl_wr_cnt(&self) -> &L1_DBUS2_ACS_NXTLVL_WR_CNT {
        &self.l1_dbus2_acs_nxtlvl_wr_cnt
    }
    ///0x1fc - L1-DCache bus3 Hit-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus3_acs_hit_cnt(&self) -> &L1_DBUS3_ACS_HIT_CNT {
        &self.l1_dbus3_acs_hit_cnt
    }
    ///0x200 - L1-DCache bus3 Miss-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus3_acs_miss_cnt(&self) -> &L1_DBUS3_ACS_MISS_CNT {
        &self.l1_dbus3_acs_miss_cnt
    }
    ///0x204 - L1-DCache bus3 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus3_acs_conflict_cnt(&self) -> &L1_DBUS3_ACS_CONFLICT_CNT {
        &self.l1_dbus3_acs_conflict_cnt
    }
    ///0x208 - L1-DCache bus3 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus3_acs_nxtlvl_rd_cnt(&self) -> &L1_DBUS3_ACS_NXTLVL_RD_CNT {
        &self.l1_dbus3_acs_nxtlvl_rd_cnt
    }
    ///0x20c - L1-DCache bus3 WB-Access Counter register
    #[inline(always)]
    pub const fn l1_dbus3_acs_nxtlvl_wr_cnt(&self) -> &L1_DBUS3_ACS_NXTLVL_WR_CNT {
        &self.l1_dbus3_acs_nxtlvl_wr_cnt
    }
    ///0x210 - L1-ICache0 Access Fail ID/attribution information register
    #[inline(always)]
    pub const fn l1_icache0_acs_fail_id_attr(&self) -> &L1_ICACHE0_ACS_FAIL_ID_ATTR {
        &self.l1_icache0_acs_fail_id_attr
    }
    ///0x214 - L1-ICache0 Access Fail Address information register
    #[inline(always)]
    pub const fn l1_icache0_acs_fail_addr(&self) -> &L1_ICACHE0_ACS_FAIL_ADDR {
        &self.l1_icache0_acs_fail_addr
    }
    ///0x218 - L1-ICache0 Access Fail ID/attribution information register
    #[inline(always)]
    pub const fn l1_icache1_acs_fail_id_attr(&self) -> &L1_ICACHE1_ACS_FAIL_ID_ATTR {
        &self.l1_icache1_acs_fail_id_attr
    }
    ///0x21c - L1-ICache0 Access Fail Address information register
    #[inline(always)]
    pub const fn l1_icache1_acs_fail_addr(&self) -> &L1_ICACHE1_ACS_FAIL_ADDR {
        &self.l1_icache1_acs_fail_addr
    }
    ///0x220 - L1-ICache0 Access Fail ID/attribution information register
    #[inline(always)]
    pub const fn l1_icache2_acs_fail_id_attr(&self) -> &L1_ICACHE2_ACS_FAIL_ID_ATTR {
        &self.l1_icache2_acs_fail_id_attr
    }
    ///0x224 - L1-ICache0 Access Fail Address information register
    #[inline(always)]
    pub const fn l1_icache2_acs_fail_addr(&self) -> &L1_ICACHE2_ACS_FAIL_ADDR {
        &self.l1_icache2_acs_fail_addr
    }
    ///0x228 - L1-ICache0 Access Fail ID/attribution information register
    #[inline(always)]
    pub const fn l1_icache3_acs_fail_id_attr(&self) -> &L1_ICACHE3_ACS_FAIL_ID_ATTR {
        &self.l1_icache3_acs_fail_id_attr
    }
    ///0x22c - L1-ICache0 Access Fail Address information register
    #[inline(always)]
    pub const fn l1_icache3_acs_fail_addr(&self) -> &L1_ICACHE3_ACS_FAIL_ADDR {
        &self.l1_icache3_acs_fail_addr
    }
    ///0x230 - L1-DCache Access Fail ID/attribution information register
    #[inline(always)]
    pub const fn l1_dcache_acs_fail_id_attr(&self) -> &L1_DCACHE_ACS_FAIL_ID_ATTR {
        &self.l1_dcache_acs_fail_id_attr
    }
    ///0x234 - L1-DCache Access Fail Address information register
    #[inline(always)]
    pub const fn l1_dcache_acs_fail_addr(&self) -> &L1_DCACHE_ACS_FAIL_ADDR {
        &self.l1_dcache_acs_fail_addr
    }
    ///0x238 - L1-Cache Access Fail Interrupt enable register
    #[inline(always)]
    pub const fn sync_l1_cache_preload_int_ena(&self) -> &SYNC_L1_CACHE_PRELOAD_INT_ENA {
        &self.sync_l1_cache_preload_int_ena
    }
    ///0x23c - Sync Preload operation Interrupt clear register
    #[inline(always)]
    pub const fn sync_l1_cache_preload_int_clr(&self) -> &SYNC_L1_CACHE_PRELOAD_INT_CLR {
        &self.sync_l1_cache_preload_int_clr
    }
    ///0x240 - Sync Preload operation Interrupt raw register
    #[inline(always)]
    pub const fn sync_l1_cache_preload_int_raw(&self) -> &SYNC_L1_CACHE_PRELOAD_INT_RAW {
        &self.sync_l1_cache_preload_int_raw
    }
    ///0x244 - L1-Cache Access Fail Interrupt status register
    #[inline(always)]
    pub const fn sync_l1_cache_preload_int_st(&self) -> &SYNC_L1_CACHE_PRELOAD_INT_ST {
        &self.sync_l1_cache_preload_int_st
    }
    ///0x248 - Cache Sync/Preload Operation exception register
    #[inline(always)]
    pub const fn sync_l1_cache_preload_exception(
        &self,
    ) -> &SYNC_L1_CACHE_PRELOAD_EXCEPTION {
        &self.sync_l1_cache_preload_exception
    }
    ///0x24c - Cache Sync Reset control register
    #[inline(always)]
    pub const fn l1_cache_sync_rst_ctrl(&self) -> &L1_CACHE_SYNC_RST_CTRL {
        &self.l1_cache_sync_rst_ctrl
    }
    ///0x250 - Cache Preload Reset control register
    #[inline(always)]
    pub const fn l1_cache_preload_rst_ctrl(&self) -> &L1_CACHE_PRELOAD_RST_CTRL {
        &self.l1_cache_preload_rst_ctrl
    }
    ///0x254 - Cache Autoload buffer clear control register
    #[inline(always)]
    pub const fn l1_cache_autoload_buf_clr_ctrl(
        &self,
    ) -> &L1_CACHE_AUTOLOAD_BUF_CLR_CTRL {
        &self.l1_cache_autoload_buf_clr_ctrl
    }
    ///0x258 - Unallocate request buffer clear registers
    #[inline(always)]
    pub const fn l1_unallocate_buffer_clear(&self) -> &L1_UNALLOCATE_BUFFER_CLEAR {
        &self.l1_unallocate_buffer_clear
    }
    ///0x25c - Cache Tag and Data memory Object control register
    #[inline(always)]
    pub const fn l1_cache_object_ctrl(&self) -> &L1_CACHE_OBJECT_CTRL {
        &self.l1_cache_object_ctrl
    }
    ///0x260 - Cache Tag and Data memory way register
    #[inline(always)]
    pub const fn l1_cache_way_object(&self) -> &L1_CACHE_WAY_OBJECT {
        &self.l1_cache_way_object
    }
    ///0x264 - Cache Vaddr register
    #[inline(always)]
    pub const fn l1_cache_vaddr(&self) -> &L1_CACHE_VADDR {
        &self.l1_cache_vaddr
    }
    ///0x268 - Cache Tag/data memory content register
    #[inline(always)]
    pub const fn l1_cache_debug_bus(&self) -> &L1_CACHE_DEBUG_BUS {
        &self.l1_cache_debug_bus
    }
    ///0x26c - USED TO SPLIT L1 CACHE AND L2 CACHE
    #[inline(always)]
    pub const fn level_split0(&self) -> &LEVEL_SPLIT0 {
        &self.level_split0
    }
    ///0x270 - L2 Cache(L2-Cache) control register
    #[inline(always)]
    pub const fn l2_cache_ctrl(&self) -> &L2_CACHE_CTRL {
        &self.l2_cache_ctrl
    }
    ///0x274 - Bypass Cache configure register
    #[inline(always)]
    pub const fn l2_bypass_cache_conf(&self) -> &L2_BYPASS_CACHE_CONF {
        &self.l2_bypass_cache_conf
    }
    ///0x278 - L2 Cache CacheSize mode configure register
    #[inline(always)]
    pub const fn l2_cache_cachesize_conf(&self) -> &L2_CACHE_CACHESIZE_CONF {
        &self.l2_cache_cachesize_conf
    }
    ///0x27c - L2 Cache BlockSize mode configure register
    #[inline(always)]
    pub const fn l2_cache_blocksize_conf(&self) -> &L2_CACHE_BLOCKSIZE_CONF {
        &self.l2_cache_blocksize_conf
    }
    ///0x280 - Cache wrap around control register
    #[inline(always)]
    pub const fn l2_cache_wrap_around_ctrl(&self) -> &L2_CACHE_WRAP_AROUND_CTRL {
        &self.l2_cache_wrap_around_ctrl
    }
    ///0x284 - Cache tag memory power control register
    #[inline(always)]
    pub const fn l2_cache_tag_mem_power_ctrl(&self) -> &L2_CACHE_TAG_MEM_POWER_CTRL {
        &self.l2_cache_tag_mem_power_ctrl
    }
    ///0x288 - Cache data memory power control register
    #[inline(always)]
    pub const fn l2_cache_data_mem_power_ctrl(&self) -> &L2_CACHE_DATA_MEM_POWER_CTRL {
        &self.l2_cache_data_mem_power_ctrl
    }
    ///0x28c - Cache Freeze control register
    #[inline(always)]
    pub const fn l2_cache_freeze_ctrl(&self) -> &L2_CACHE_FREEZE_CTRL {
        &self.l2_cache_freeze_ctrl
    }
    ///0x290 - Cache data memory access configure register
    #[inline(always)]
    pub const fn l2_cache_data_mem_acs_conf(&self) -> &L2_CACHE_DATA_MEM_ACS_CONF {
        &self.l2_cache_data_mem_acs_conf
    }
    ///0x294 - Cache tag memory access configure register
    #[inline(always)]
    pub const fn l2_cache_tag_mem_acs_conf(&self) -> &L2_CACHE_TAG_MEM_ACS_CONF {
        &self.l2_cache_tag_mem_acs_conf
    }
    ///0x298 - L2 Cache prelock configure register
    #[inline(always)]
    pub const fn l2_cache_prelock_conf(&self) -> &L2_CACHE_PRELOCK_CONF {
        &self.l2_cache_prelock_conf
    }
    ///0x29c - L2 Cache prelock section0 address configure register
    #[inline(always)]
    pub const fn l2_cache_prelock_sct0_addr(&self) -> &L2_CACHE_PRELOCK_SCT0_ADDR {
        &self.l2_cache_prelock_sct0_addr
    }
    ///0x2a0 - L2 Cache prelock section1 address configure register
    #[inline(always)]
    pub const fn l2_cache_prelock_sct1_addr(&self) -> &L2_CACHE_PRELOCK_SCT1_ADDR {
        &self.l2_cache_prelock_sct1_addr
    }
    ///0x2a4 - L2 Cache prelock section size configure register
    #[inline(always)]
    pub const fn l2_cache_prelock_sct_size(&self) -> &L2_CACHE_PRELOCK_SCT_SIZE {
        &self.l2_cache_prelock_sct_size
    }
    ///0x2a8 - L2 Cache preload-operation control register
    #[inline(always)]
    pub const fn l2_cache_preload_ctrl(&self) -> &L2_CACHE_PRELOAD_CTRL {
        &self.l2_cache_preload_ctrl
    }
    ///0x2ac - L2 Cache preload address configure register
    #[inline(always)]
    pub const fn l2_cache_preload_addr(&self) -> &L2_CACHE_PRELOAD_ADDR {
        &self.l2_cache_preload_addr
    }
    ///0x2b0 - L2 Cache preload size configure register
    #[inline(always)]
    pub const fn l2_cache_preload_size(&self) -> &L2_CACHE_PRELOAD_SIZE {
        &self.l2_cache_preload_size
    }
    ///0x2b4 - L2 Cache autoload-operation control register
    #[inline(always)]
    pub const fn l2_cache_autoload_ctrl(&self) -> &L2_CACHE_AUTOLOAD_CTRL {
        &self.l2_cache_autoload_ctrl
    }
    ///0x2b8 - L2 Cache autoload section 0 address configure register
    #[inline(always)]
    pub const fn l2_cache_autoload_sct0_addr(&self) -> &L2_CACHE_AUTOLOAD_SCT0_ADDR {
        &self.l2_cache_autoload_sct0_addr
    }
    ///0x2bc - L2 Cache autoload section 0 size configure register
    #[inline(always)]
    pub const fn l2_cache_autoload_sct0_size(&self) -> &L2_CACHE_AUTOLOAD_SCT0_SIZE {
        &self.l2_cache_autoload_sct0_size
    }
    ///0x2c0 - L2 Cache autoload section 1 address configure register
    #[inline(always)]
    pub const fn l2_cache_autoload_sct1_addr(&self) -> &L2_CACHE_AUTOLOAD_SCT1_ADDR {
        &self.l2_cache_autoload_sct1_addr
    }
    ///0x2c4 - L2 Cache autoload section 1 size configure register
    #[inline(always)]
    pub const fn l2_cache_autoload_sct1_size(&self) -> &L2_CACHE_AUTOLOAD_SCT1_SIZE {
        &self.l2_cache_autoload_sct1_size
    }
    ///0x2c8 - L2 Cache autoload section 2 address configure register
    #[inline(always)]
    pub const fn l2_cache_autoload_sct2_addr(&self) -> &L2_CACHE_AUTOLOAD_SCT2_ADDR {
        &self.l2_cache_autoload_sct2_addr
    }
    ///0x2cc - L2 Cache autoload section 2 size configure register
    #[inline(always)]
    pub const fn l2_cache_autoload_sct2_size(&self) -> &L2_CACHE_AUTOLOAD_SCT2_SIZE {
        &self.l2_cache_autoload_sct2_size
    }
    ///0x2d0 - L2 Cache autoload section 3 address configure register
    #[inline(always)]
    pub const fn l2_cache_autoload_sct3_addr(&self) -> &L2_CACHE_AUTOLOAD_SCT3_ADDR {
        &self.l2_cache_autoload_sct3_addr
    }
    ///0x2d4 - L2 Cache autoload section 3 size configure register
    #[inline(always)]
    pub const fn l2_cache_autoload_sct3_size(&self) -> &L2_CACHE_AUTOLOAD_SCT3_SIZE {
        &self.l2_cache_autoload_sct3_size
    }
    ///0x2d8 - Cache Access Counter Interrupt enable register
    #[inline(always)]
    pub const fn l2_cache_acs_cnt_int_ena(&self) -> &L2_CACHE_ACS_CNT_INT_ENA {
        &self.l2_cache_acs_cnt_int_ena
    }
    ///0x2dc - Cache Access Counter Interrupt clear register
    #[inline(always)]
    pub const fn l2_cache_acs_cnt_int_clr(&self) -> &L2_CACHE_ACS_CNT_INT_CLR {
        &self.l2_cache_acs_cnt_int_clr
    }
    ///0x2e0 - Cache Access Counter Interrupt raw register
    #[inline(always)]
    pub const fn l2_cache_acs_cnt_int_raw(&self) -> &L2_CACHE_ACS_CNT_INT_RAW {
        &self.l2_cache_acs_cnt_int_raw
    }
    ///0x2e4 - Cache Access Counter Interrupt status register
    #[inline(always)]
    pub const fn l2_cache_acs_cnt_int_st(&self) -> &L2_CACHE_ACS_CNT_INT_ST {
        &self.l2_cache_acs_cnt_int_st
    }
    ///0x2e8 - Cache Access Fail Configuration register
    #[inline(always)]
    pub const fn l2_cache_acs_fail_ctrl(&self) -> &L2_CACHE_ACS_FAIL_CTRL {
        &self.l2_cache_acs_fail_ctrl
    }
    ///0x2ec - Cache Access Fail Interrupt enable register
    #[inline(always)]
    pub const fn l2_cache_acs_fail_int_ena(&self) -> &L2_CACHE_ACS_FAIL_INT_ENA {
        &self.l2_cache_acs_fail_int_ena
    }
    ///0x2f0 - L1-Cache Access Fail Interrupt clear register
    #[inline(always)]
    pub const fn l2_cache_acs_fail_int_clr(&self) -> &L2_CACHE_ACS_FAIL_INT_CLR {
        &self.l2_cache_acs_fail_int_clr
    }
    ///0x2f4 - Cache Access Fail Interrupt raw register
    #[inline(always)]
    pub const fn l2_cache_acs_fail_int_raw(&self) -> &L2_CACHE_ACS_FAIL_INT_RAW {
        &self.l2_cache_acs_fail_int_raw
    }
    ///0x2f8 - Cache Access Fail Interrupt status register
    #[inline(always)]
    pub const fn l2_cache_acs_fail_int_st(&self) -> &L2_CACHE_ACS_FAIL_INT_ST {
        &self.l2_cache_acs_fail_int_st
    }
    ///0x2fc - Cache Access Counter enable and clear register
    #[inline(always)]
    pub const fn l2_cache_acs_cnt_ctrl(&self) -> &L2_CACHE_ACS_CNT_CTRL {
        &self.l2_cache_acs_cnt_ctrl
    }
    ///0x300 - L2-Cache bus0 Hit-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus0_acs_hit_cnt(&self) -> &L2_IBUS0_ACS_HIT_CNT {
        &self.l2_ibus0_acs_hit_cnt
    }
    ///0x304 - L2-Cache bus0 Miss-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus0_acs_miss_cnt(&self) -> &L2_IBUS0_ACS_MISS_CNT {
        &self.l2_ibus0_acs_miss_cnt
    }
    ///0x308 - L2-Cache bus0 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus0_acs_conflict_cnt(&self) -> &L2_IBUS0_ACS_CONFLICT_CNT {
        &self.l2_ibus0_acs_conflict_cnt
    }
    ///0x30c - L2-Cache bus0 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus0_acs_nxtlvl_rd_cnt(&self) -> &L2_IBUS0_ACS_NXTLVL_RD_CNT {
        &self.l2_ibus0_acs_nxtlvl_rd_cnt
    }
    ///0x310 - L2-Cache bus1 Hit-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus1_acs_hit_cnt(&self) -> &L2_IBUS1_ACS_HIT_CNT {
        &self.l2_ibus1_acs_hit_cnt
    }
    ///0x314 - L2-Cache bus1 Miss-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus1_acs_miss_cnt(&self) -> &L2_IBUS1_ACS_MISS_CNT {
        &self.l2_ibus1_acs_miss_cnt
    }
    ///0x318 - L2-Cache bus1 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus1_acs_conflict_cnt(&self) -> &L2_IBUS1_ACS_CONFLICT_CNT {
        &self.l2_ibus1_acs_conflict_cnt
    }
    ///0x31c - L2-Cache bus1 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus1_acs_nxtlvl_rd_cnt(&self) -> &L2_IBUS1_ACS_NXTLVL_RD_CNT {
        &self.l2_ibus1_acs_nxtlvl_rd_cnt
    }
    ///0x320 - L2-Cache bus2 Hit-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus2_acs_hit_cnt(&self) -> &L2_IBUS2_ACS_HIT_CNT {
        &self.l2_ibus2_acs_hit_cnt
    }
    ///0x324 - L2-Cache bus2 Miss-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus2_acs_miss_cnt(&self) -> &L2_IBUS2_ACS_MISS_CNT {
        &self.l2_ibus2_acs_miss_cnt
    }
    ///0x328 - L2-Cache bus2 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus2_acs_conflict_cnt(&self) -> &L2_IBUS2_ACS_CONFLICT_CNT {
        &self.l2_ibus2_acs_conflict_cnt
    }
    ///0x32c - L2-Cache bus2 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus2_acs_nxtlvl_rd_cnt(&self) -> &L2_IBUS2_ACS_NXTLVL_RD_CNT {
        &self.l2_ibus2_acs_nxtlvl_rd_cnt
    }
    ///0x330 - L2-Cache bus3 Hit-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus3_acs_hit_cnt(&self) -> &L2_IBUS3_ACS_HIT_CNT {
        &self.l2_ibus3_acs_hit_cnt
    }
    ///0x334 - L2-Cache bus3 Miss-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus3_acs_miss_cnt(&self) -> &L2_IBUS3_ACS_MISS_CNT {
        &self.l2_ibus3_acs_miss_cnt
    }
    ///0x338 - L2-Cache bus3 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus3_acs_conflict_cnt(&self) -> &L2_IBUS3_ACS_CONFLICT_CNT {
        &self.l2_ibus3_acs_conflict_cnt
    }
    ///0x33c - L2-Cache bus3 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l2_ibus3_acs_nxtlvl_rd_cnt(&self) -> &L2_IBUS3_ACS_NXTLVL_RD_CNT {
        &self.l2_ibus3_acs_nxtlvl_rd_cnt
    }
    ///0x340 - L2-Cache bus0 Hit-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus0_acs_hit_cnt(&self) -> &L2_DBUS0_ACS_HIT_CNT {
        &self.l2_dbus0_acs_hit_cnt
    }
    ///0x344 - L2-Cache bus0 Miss-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus0_acs_miss_cnt(&self) -> &L2_DBUS0_ACS_MISS_CNT {
        &self.l2_dbus0_acs_miss_cnt
    }
    ///0x348 - L2-Cache bus0 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus0_acs_conflict_cnt(&self) -> &L2_DBUS0_ACS_CONFLICT_CNT {
        &self.l2_dbus0_acs_conflict_cnt
    }
    ///0x34c - L2-Cache bus0 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus0_acs_nxtlvl_rd_cnt(&self) -> &L2_DBUS0_ACS_NXTLVL_RD_CNT {
        &self.l2_dbus0_acs_nxtlvl_rd_cnt
    }
    ///0x350 - L2-Cache bus0 WB-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus0_acs_nxtlvl_wr_cnt(&self) -> &L2_DBUS0_ACS_NXTLVL_WR_CNT {
        &self.l2_dbus0_acs_nxtlvl_wr_cnt
    }
    ///0x354 - L2-Cache bus1 Hit-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus1_acs_hit_cnt(&self) -> &L2_DBUS1_ACS_HIT_CNT {
        &self.l2_dbus1_acs_hit_cnt
    }
    ///0x358 - L2-Cache bus1 Miss-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus1_acs_miss_cnt(&self) -> &L2_DBUS1_ACS_MISS_CNT {
        &self.l2_dbus1_acs_miss_cnt
    }
    ///0x35c - L2-Cache bus1 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus1_acs_conflict_cnt(&self) -> &L2_DBUS1_ACS_CONFLICT_CNT {
        &self.l2_dbus1_acs_conflict_cnt
    }
    ///0x360 - L2-Cache bus1 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus1_acs_nxtlvl_rd_cnt(&self) -> &L2_DBUS1_ACS_NXTLVL_RD_CNT {
        &self.l2_dbus1_acs_nxtlvl_rd_cnt
    }
    ///0x364 - L2-Cache bus1 WB-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus1_acs_nxtlvl_wr_cnt(&self) -> &L2_DBUS1_ACS_NXTLVL_WR_CNT {
        &self.l2_dbus1_acs_nxtlvl_wr_cnt
    }
    ///0x368 - L2-Cache bus2 Hit-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus2_acs_hit_cnt(&self) -> &L2_DBUS2_ACS_HIT_CNT {
        &self.l2_dbus2_acs_hit_cnt
    }
    ///0x36c - L2-Cache bus2 Miss-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus2_acs_miss_cnt(&self) -> &L2_DBUS2_ACS_MISS_CNT {
        &self.l2_dbus2_acs_miss_cnt
    }
    ///0x370 - L2-Cache bus2 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus2_acs_conflict_cnt(&self) -> &L2_DBUS2_ACS_CONFLICT_CNT {
        &self.l2_dbus2_acs_conflict_cnt
    }
    ///0x374 - L2-Cache bus2 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus2_acs_nxtlvl_rd_cnt(&self) -> &L2_DBUS2_ACS_NXTLVL_RD_CNT {
        &self.l2_dbus2_acs_nxtlvl_rd_cnt
    }
    ///0x378 - L2-Cache bus2 WB-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus2_acs_nxtlvl_wr_cnt(&self) -> &L2_DBUS2_ACS_NXTLVL_WR_CNT {
        &self.l2_dbus2_acs_nxtlvl_wr_cnt
    }
    ///0x37c - L2-Cache bus3 Hit-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus3_acs_hit_cnt(&self) -> &L2_DBUS3_ACS_HIT_CNT {
        &self.l2_dbus3_acs_hit_cnt
    }
    ///0x380 - L2-Cache bus3 Miss-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus3_acs_miss_cnt(&self) -> &L2_DBUS3_ACS_MISS_CNT {
        &self.l2_dbus3_acs_miss_cnt
    }
    ///0x384 - L2-Cache bus3 Conflict-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus3_acs_conflict_cnt(&self) -> &L2_DBUS3_ACS_CONFLICT_CNT {
        &self.l2_dbus3_acs_conflict_cnt
    }
    ///0x388 - L2-Cache bus3 Next-Level-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus3_acs_nxtlvl_rd_cnt(&self) -> &L2_DBUS3_ACS_NXTLVL_RD_CNT {
        &self.l2_dbus3_acs_nxtlvl_rd_cnt
    }
    ///0x38c - L2-Cache bus3 WB-Access Counter register
    #[inline(always)]
    pub const fn l2_dbus3_acs_nxtlvl_wr_cnt(&self) -> &L2_DBUS3_ACS_NXTLVL_WR_CNT {
        &self.l2_dbus3_acs_nxtlvl_wr_cnt
    }
    ///0x390 - L2-Cache Access Fail ID/attribution information register
    #[inline(always)]
    pub const fn l2_cache_acs_fail_id_attr(&self) -> &L2_CACHE_ACS_FAIL_ID_ATTR {
        &self.l2_cache_acs_fail_id_attr
    }
    ///0x394 - L2-Cache Access Fail Address information register
    #[inline(always)]
    pub const fn l2_cache_acs_fail_addr(&self) -> &L2_CACHE_ACS_FAIL_ADDR {
        &self.l2_cache_acs_fail_addr
    }
    ///0x398 - L1-Cache Access Fail Interrupt enable register
    #[inline(always)]
    pub const fn l2_cache_sync_preload_int_ena(&self) -> &L2_CACHE_SYNC_PRELOAD_INT_ENA {
        &self.l2_cache_sync_preload_int_ena
    }
    ///0x39c - Sync Preload operation Interrupt clear register
    #[inline(always)]
    pub const fn l2_cache_sync_preload_int_clr(&self) -> &L2_CACHE_SYNC_PRELOAD_INT_CLR {
        &self.l2_cache_sync_preload_int_clr
    }
    ///0x3a0 - Sync Preload operation Interrupt raw register
    #[inline(always)]
    pub const fn l2_cache_sync_preload_int_raw(&self) -> &L2_CACHE_SYNC_PRELOAD_INT_RAW {
        &self.l2_cache_sync_preload_int_raw
    }
    ///0x3a4 - L1-Cache Access Fail Interrupt status register
    #[inline(always)]
    pub const fn l2_cache_sync_preload_int_st(&self) -> &L2_CACHE_SYNC_PRELOAD_INT_ST {
        &self.l2_cache_sync_preload_int_st
    }
    ///0x3a8 - Cache Sync/Preload Operation exception register
    #[inline(always)]
    pub const fn l2_cache_sync_preload_exception(
        &self,
    ) -> &L2_CACHE_SYNC_PRELOAD_EXCEPTION {
        &self.l2_cache_sync_preload_exception
    }
    ///0x3ac - Cache Sync Reset control register
    #[inline(always)]
    pub const fn l2_cache_sync_rst_ctrl(&self) -> &L2_CACHE_SYNC_RST_CTRL {
        &self.l2_cache_sync_rst_ctrl
    }
    ///0x3b0 - Cache Preload Reset control register
    #[inline(always)]
    pub const fn l2_cache_preload_rst_ctrl(&self) -> &L2_CACHE_PRELOAD_RST_CTRL {
        &self.l2_cache_preload_rst_ctrl
    }
    ///0x3b4 - Cache Autoload buffer clear control register
    #[inline(always)]
    pub const fn l2_cache_autoload_buf_clr_ctrl(
        &self,
    ) -> &L2_CACHE_AUTOLOAD_BUF_CLR_CTRL {
        &self.l2_cache_autoload_buf_clr_ctrl
    }
    ///0x3b8 - Unallocate request buffer clear registers
    #[inline(always)]
    pub const fn l2_unallocate_buffer_clear(&self) -> &L2_UNALLOCATE_BUFFER_CLEAR {
        &self.l2_unallocate_buffer_clear
    }
    ///0x3bc - L2 cache access attribute control register
    #[inline(always)]
    pub const fn l2_cache_access_attr_ctrl(&self) -> &L2_CACHE_ACCESS_ATTR_CTRL {
        &self.l2_cache_access_attr_ctrl
    }
    ///0x3c0 - Cache Tag and Data memory Object control register
    #[inline(always)]
    pub const fn l2_cache_object_ctrl(&self) -> &L2_CACHE_OBJECT_CTRL {
        &self.l2_cache_object_ctrl
    }
    ///0x3c4 - Cache Tag and Data memory way register
    #[inline(always)]
    pub const fn l2_cache_way_object(&self) -> &L2_CACHE_WAY_OBJECT {
        &self.l2_cache_way_object
    }
    ///0x3c8 - Cache Vaddr register
    #[inline(always)]
    pub const fn l2_cache_vaddr(&self) -> &L2_CACHE_VADDR {
        &self.l2_cache_vaddr
    }
    ///0x3cc - Cache Tag/data memory content register
    #[inline(always)]
    pub const fn l2_cache_debug_bus(&self) -> &L2_CACHE_DEBUG_BUS {
        &self.l2_cache_debug_bus
    }
    ///0x3d0 - USED TO SPLIT L1 CACHE AND L2 CACHE
    #[inline(always)]
    pub const fn level_split1(&self) -> &LEVEL_SPLIT1 {
        &self.level_split1
    }
    ///0x3d4 - Clock gate control register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x3d8 - Cache redundancy signal 0 register
    #[inline(always)]
    pub const fn redundancy_sig0(&self) -> &REDUNDANCY_SIG0 {
        &self.redundancy_sig0
    }
    ///0x3dc - Cache redundancy signal 1 register
    #[inline(always)]
    pub const fn redundancy_sig1(&self) -> &REDUNDANCY_SIG1 {
        &self.redundancy_sig1
    }
    ///0x3e0 - Cache redundancy signal 2 register
    #[inline(always)]
    pub const fn redundancy_sig2(&self) -> &REDUNDANCY_SIG2 {
        &self.redundancy_sig2
    }
    ///0x3e4 - Cache redundancy signal 3 register
    #[inline(always)]
    pub const fn redundancy_sig3(&self) -> &REDUNDANCY_SIG3 {
        &self.redundancy_sig3
    }
    ///0x3e8 - Cache redundancy signal 0 register
    #[inline(always)]
    pub const fn redundancy_sig4(&self) -> &REDUNDANCY_SIG4 {
        &self.redundancy_sig4
    }
    ///0x3fc - Version control register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**L1_ICACHE_CTRL (rw) register accessor: L1 instruction Cache(L1-ICache) control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache_ctrl`] module*/
pub type L1_ICACHE_CTRL = crate::Reg<l1_icache_ctrl::L1_ICACHE_CTRL_SPEC>;
///L1 instruction Cache(L1-ICache) control register
pub mod l1_icache_ctrl;
/**L1_DCACHE_CTRL (rw) register accessor: L1 data Cache(L1-DCache) control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_ctrl`] module*/
pub type L1_DCACHE_CTRL = crate::Reg<l1_dcache_ctrl::L1_DCACHE_CTRL_SPEC>;
///L1 data Cache(L1-DCache) control register
pub mod l1_dcache_ctrl;
/**L1_BYPASS_CACHE_CONF (rw) register accessor: Bypass Cache configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_bypass_cache_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_bypass_cache_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_bypass_cache_conf`] module*/
pub type L1_BYPASS_CACHE_CONF = crate::Reg<
    l1_bypass_cache_conf::L1_BYPASS_CACHE_CONF_SPEC,
>;
///Bypass Cache configure register
pub mod l1_bypass_cache_conf;
/**L1_CACHE_ATOMIC_CONF (rw) register accessor: L1 Cache atomic feature configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_atomic_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_atomic_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_atomic_conf`] module*/
pub type L1_CACHE_ATOMIC_CONF = crate::Reg<
    l1_cache_atomic_conf::L1_CACHE_ATOMIC_CONF_SPEC,
>;
///L1 Cache atomic feature configure register
pub mod l1_cache_atomic_conf;
/**L1_ICACHE_CACHESIZE_CONF (r) register accessor: L1 instruction Cache CacheSize mode configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache_cachesize_conf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache_cachesize_conf`] module*/
pub type L1_ICACHE_CACHESIZE_CONF = crate::Reg<
    l1_icache_cachesize_conf::L1_ICACHE_CACHESIZE_CONF_SPEC,
>;
///L1 instruction Cache CacheSize mode configure register
pub mod l1_icache_cachesize_conf;
/**L1_ICACHE_BLOCKSIZE_CONF (r) register accessor: L1 instruction Cache BlockSize mode configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache_blocksize_conf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache_blocksize_conf`] module*/
pub type L1_ICACHE_BLOCKSIZE_CONF = crate::Reg<
    l1_icache_blocksize_conf::L1_ICACHE_BLOCKSIZE_CONF_SPEC,
>;
///L1 instruction Cache BlockSize mode configure register
pub mod l1_icache_blocksize_conf;
/**L1_DCACHE_CACHESIZE_CONF (r) register accessor: L1 data Cache CacheSize mode configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_cachesize_conf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_cachesize_conf`] module*/
pub type L1_DCACHE_CACHESIZE_CONF = crate::Reg<
    l1_dcache_cachesize_conf::L1_DCACHE_CACHESIZE_CONF_SPEC,
>;
///L1 data Cache CacheSize mode configure register
pub mod l1_dcache_cachesize_conf;
/**L1_DCACHE_BLOCKSIZE_CONF (r) register accessor: L1 data Cache BlockSize mode configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_blocksize_conf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_blocksize_conf`] module*/
pub type L1_DCACHE_BLOCKSIZE_CONF = crate::Reg<
    l1_dcache_blocksize_conf::L1_DCACHE_BLOCKSIZE_CONF_SPEC,
>;
///L1 data Cache BlockSize mode configure register
pub mod l1_dcache_blocksize_conf;
/**L1_CACHE_WRAP_AROUND_CTRL (rw) register accessor: Cache wrap around control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_wrap_around_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_wrap_around_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_wrap_around_ctrl`] module*/
pub type L1_CACHE_WRAP_AROUND_CTRL = crate::Reg<
    l1_cache_wrap_around_ctrl::L1_CACHE_WRAP_AROUND_CTRL_SPEC,
>;
///Cache wrap around control register
pub mod l1_cache_wrap_around_ctrl;
/**L1_CACHE_TAG_MEM_POWER_CTRL (rw) register accessor: Cache tag memory power control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_tag_mem_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_tag_mem_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_tag_mem_power_ctrl`] module*/
pub type L1_CACHE_TAG_MEM_POWER_CTRL = crate::Reg<
    l1_cache_tag_mem_power_ctrl::L1_CACHE_TAG_MEM_POWER_CTRL_SPEC,
>;
///Cache tag memory power control register
pub mod l1_cache_tag_mem_power_ctrl;
/**L1_CACHE_DATA_MEM_POWER_CTRL (rw) register accessor: Cache data memory power control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_data_mem_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_data_mem_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_data_mem_power_ctrl`] module*/
pub type L1_CACHE_DATA_MEM_POWER_CTRL = crate::Reg<
    l1_cache_data_mem_power_ctrl::L1_CACHE_DATA_MEM_POWER_CTRL_SPEC,
>;
///Cache data memory power control register
pub mod l1_cache_data_mem_power_ctrl;
/**L1_CACHE_FREEZE_CTRL (rw) register accessor: Cache Freeze control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_freeze_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_freeze_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_freeze_ctrl`] module*/
pub type L1_CACHE_FREEZE_CTRL = crate::Reg<
    l1_cache_freeze_ctrl::L1_CACHE_FREEZE_CTRL_SPEC,
>;
///Cache Freeze control register
pub mod l1_cache_freeze_ctrl;
/**L1_CACHE_DATA_MEM_ACS_CONF (rw) register accessor: Cache data memory access configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_data_mem_acs_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_data_mem_acs_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_data_mem_acs_conf`] module*/
pub type L1_CACHE_DATA_MEM_ACS_CONF = crate::Reg<
    l1_cache_data_mem_acs_conf::L1_CACHE_DATA_MEM_ACS_CONF_SPEC,
>;
///Cache data memory access configure register
pub mod l1_cache_data_mem_acs_conf;
/**L1_CACHE_TAG_MEM_ACS_CONF (rw) register accessor: Cache tag memory access configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_tag_mem_acs_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_tag_mem_acs_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_tag_mem_acs_conf`] module*/
pub type L1_CACHE_TAG_MEM_ACS_CONF = crate::Reg<
    l1_cache_tag_mem_acs_conf::L1_CACHE_TAG_MEM_ACS_CONF_SPEC,
>;
///Cache tag memory access configure register
pub mod l1_cache_tag_mem_acs_conf;
/**L1_ICACHE0_PRELOCK_CONF (rw) register accessor: L1 instruction Cache 0 prelock configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_prelock_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_prelock_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_prelock_conf`] module*/
pub type L1_ICACHE0_PRELOCK_CONF = crate::Reg<
    l1_icache0_prelock_conf::L1_ICACHE0_PRELOCK_CONF_SPEC,
>;
///L1 instruction Cache 0 prelock configure register
pub mod l1_icache0_prelock_conf;
/**L1_ICACHE0_PRELOCK_SCT0_ADDR (rw) register accessor: L1 instruction Cache 0 prelock section0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_prelock_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_prelock_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_prelock_sct0_addr`] module*/
pub type L1_ICACHE0_PRELOCK_SCT0_ADDR = crate::Reg<
    l1_icache0_prelock_sct0_addr::L1_ICACHE0_PRELOCK_SCT0_ADDR_SPEC,
>;
///L1 instruction Cache 0 prelock section0 address configure register
pub mod l1_icache0_prelock_sct0_addr;
/**L1_ICACHE0_PRELOCK_SCT1_ADDR (rw) register accessor: L1 instruction Cache 0 prelock section1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_prelock_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_prelock_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_prelock_sct1_addr`] module*/
pub type L1_ICACHE0_PRELOCK_SCT1_ADDR = crate::Reg<
    l1_icache0_prelock_sct1_addr::L1_ICACHE0_PRELOCK_SCT1_ADDR_SPEC,
>;
///L1 instruction Cache 0 prelock section1 address configure register
pub mod l1_icache0_prelock_sct1_addr;
/**L1_ICACHE0_PRELOCK_SCT_SIZE (rw) register accessor: L1 instruction Cache 0 prelock section size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_prelock_sct_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_prelock_sct_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_prelock_sct_size`] module*/
pub type L1_ICACHE0_PRELOCK_SCT_SIZE = crate::Reg<
    l1_icache0_prelock_sct_size::L1_ICACHE0_PRELOCK_SCT_SIZE_SPEC,
>;
///L1 instruction Cache 0 prelock section size configure register
pub mod l1_icache0_prelock_sct_size;
/**L1_ICACHE1_PRELOCK_CONF (rw) register accessor: L1 instruction Cache 1 prelock configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_prelock_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_prelock_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_prelock_conf`] module*/
pub type L1_ICACHE1_PRELOCK_CONF = crate::Reg<
    l1_icache1_prelock_conf::L1_ICACHE1_PRELOCK_CONF_SPEC,
>;
///L1 instruction Cache 1 prelock configure register
pub mod l1_icache1_prelock_conf;
/**L1_ICACHE1_PRELOCK_SCT0_ADDR (rw) register accessor: L1 instruction Cache 1 prelock section0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_prelock_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_prelock_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_prelock_sct0_addr`] module*/
pub type L1_ICACHE1_PRELOCK_SCT0_ADDR = crate::Reg<
    l1_icache1_prelock_sct0_addr::L1_ICACHE1_PRELOCK_SCT0_ADDR_SPEC,
>;
///L1 instruction Cache 1 prelock section0 address configure register
pub mod l1_icache1_prelock_sct0_addr;
/**L1_ICACHE1_PRELOCK_SCT1_ADDR (rw) register accessor: L1 instruction Cache 1 prelock section1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_prelock_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_prelock_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_prelock_sct1_addr`] module*/
pub type L1_ICACHE1_PRELOCK_SCT1_ADDR = crate::Reg<
    l1_icache1_prelock_sct1_addr::L1_ICACHE1_PRELOCK_SCT1_ADDR_SPEC,
>;
///L1 instruction Cache 1 prelock section1 address configure register
pub mod l1_icache1_prelock_sct1_addr;
/**L1_ICACHE1_PRELOCK_SCT_SIZE (rw) register accessor: L1 instruction Cache 1 prelock section size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_prelock_sct_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_prelock_sct_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_prelock_sct_size`] module*/
pub type L1_ICACHE1_PRELOCK_SCT_SIZE = crate::Reg<
    l1_icache1_prelock_sct_size::L1_ICACHE1_PRELOCK_SCT_SIZE_SPEC,
>;
///L1 instruction Cache 1 prelock section size configure register
pub mod l1_icache1_prelock_sct_size;
/**L1_ICACHE2_PRELOCK_CONF (r) register accessor: L1 instruction Cache 2 prelock configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_prelock_conf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_prelock_conf`] module*/
pub type L1_ICACHE2_PRELOCK_CONF = crate::Reg<
    l1_icache2_prelock_conf::L1_ICACHE2_PRELOCK_CONF_SPEC,
>;
///L1 instruction Cache 2 prelock configure register
pub mod l1_icache2_prelock_conf;
/**L1_ICACHE2_PRELOCK_SCT0_ADDR (r) register accessor: L1 instruction Cache 2 prelock section0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_prelock_sct0_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_prelock_sct0_addr`] module*/
pub type L1_ICACHE2_PRELOCK_SCT0_ADDR = crate::Reg<
    l1_icache2_prelock_sct0_addr::L1_ICACHE2_PRELOCK_SCT0_ADDR_SPEC,
>;
///L1 instruction Cache 2 prelock section0 address configure register
pub mod l1_icache2_prelock_sct0_addr;
/**L1_ICACHE2_PRELOCK_SCT1_ADDR (r) register accessor: L1 instruction Cache 2 prelock section1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_prelock_sct1_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_prelock_sct1_addr`] module*/
pub type L1_ICACHE2_PRELOCK_SCT1_ADDR = crate::Reg<
    l1_icache2_prelock_sct1_addr::L1_ICACHE2_PRELOCK_SCT1_ADDR_SPEC,
>;
///L1 instruction Cache 2 prelock section1 address configure register
pub mod l1_icache2_prelock_sct1_addr;
/**L1_ICACHE2_PRELOCK_SCT_SIZE (r) register accessor: L1 instruction Cache 2 prelock section size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_prelock_sct_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_prelock_sct_size`] module*/
pub type L1_ICACHE2_PRELOCK_SCT_SIZE = crate::Reg<
    l1_icache2_prelock_sct_size::L1_ICACHE2_PRELOCK_SCT_SIZE_SPEC,
>;
///L1 instruction Cache 2 prelock section size configure register
pub mod l1_icache2_prelock_sct_size;
/**L1_ICACHE3_PRELOCK_CONF (r) register accessor: L1 instruction Cache 3 prelock configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_prelock_conf::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_prelock_conf`] module*/
pub type L1_ICACHE3_PRELOCK_CONF = crate::Reg<
    l1_icache3_prelock_conf::L1_ICACHE3_PRELOCK_CONF_SPEC,
>;
///L1 instruction Cache 3 prelock configure register
pub mod l1_icache3_prelock_conf;
/**L1_ICACHE3_PRELOCK_SCT0_ADDR (r) register accessor: L1 instruction Cache 3 prelock section0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_prelock_sct0_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_prelock_sct0_addr`] module*/
pub type L1_ICACHE3_PRELOCK_SCT0_ADDR = crate::Reg<
    l1_icache3_prelock_sct0_addr::L1_ICACHE3_PRELOCK_SCT0_ADDR_SPEC,
>;
///L1 instruction Cache 3 prelock section0 address configure register
pub mod l1_icache3_prelock_sct0_addr;
/**L1_ICACHE3_PRELOCK_SCT1_ADDR (r) register accessor: L1 instruction Cache 3 prelock section1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_prelock_sct1_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_prelock_sct1_addr`] module*/
pub type L1_ICACHE3_PRELOCK_SCT1_ADDR = crate::Reg<
    l1_icache3_prelock_sct1_addr::L1_ICACHE3_PRELOCK_SCT1_ADDR_SPEC,
>;
///L1 instruction Cache 3 prelock section1 address configure register
pub mod l1_icache3_prelock_sct1_addr;
/**L1_ICACHE3_PRELOCK_SCT_SIZE (r) register accessor: L1 instruction Cache 3 prelock section size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_prelock_sct_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_prelock_sct_size`] module*/
pub type L1_ICACHE3_PRELOCK_SCT_SIZE = crate::Reg<
    l1_icache3_prelock_sct_size::L1_ICACHE3_PRELOCK_SCT_SIZE_SPEC,
>;
///L1 instruction Cache 3 prelock section size configure register
pub mod l1_icache3_prelock_sct_size;
/**L1_DCACHE_PRELOCK_CONF (rw) register accessor: L1 data Cache prelock configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_prelock_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_prelock_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_prelock_conf`] module*/
pub type L1_DCACHE_PRELOCK_CONF = crate::Reg<
    l1_dcache_prelock_conf::L1_DCACHE_PRELOCK_CONF_SPEC,
>;
///L1 data Cache prelock configure register
pub mod l1_dcache_prelock_conf;
/**L1_DCACHE_PRELOCK_SCT0_ADDR (rw) register accessor: L1 data Cache prelock section0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_prelock_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_prelock_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_prelock_sct0_addr`] module*/
pub type L1_DCACHE_PRELOCK_SCT0_ADDR = crate::Reg<
    l1_dcache_prelock_sct0_addr::L1_DCACHE_PRELOCK_SCT0_ADDR_SPEC,
>;
///L1 data Cache prelock section0 address configure register
pub mod l1_dcache_prelock_sct0_addr;
/**L1_DCACHE_PRELOCK_SCT1_ADDR (rw) register accessor: L1 data Cache prelock section1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_prelock_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_prelock_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_prelock_sct1_addr`] module*/
pub type L1_DCACHE_PRELOCK_SCT1_ADDR = crate::Reg<
    l1_dcache_prelock_sct1_addr::L1_DCACHE_PRELOCK_SCT1_ADDR_SPEC,
>;
///L1 data Cache prelock section1 address configure register
pub mod l1_dcache_prelock_sct1_addr;
/**L1_DCACHE_PRELOCK_SCT_SIZE (rw) register accessor: L1 data Cache prelock section size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_prelock_sct_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_prelock_sct_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_prelock_sct_size`] module*/
pub type L1_DCACHE_PRELOCK_SCT_SIZE = crate::Reg<
    l1_dcache_prelock_sct_size::L1_DCACHE_PRELOCK_SCT_SIZE_SPEC,
>;
///L1 data Cache prelock section size configure register
pub mod l1_dcache_prelock_sct_size;
/**LOCK_CTRL (rw) register accessor: Lock-class (manual lock) operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`lock_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lock_ctrl`] module*/
pub type LOCK_CTRL = crate::Reg<lock_ctrl::LOCK_CTRL_SPEC>;
///Lock-class (manual lock) operation control register
pub mod lock_ctrl;
/**LOCK_MAP (rw) register accessor: Lock (manual lock) map configure register

You can [`read`](crate::generic::Reg::read) this register and get [`lock_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lock_map`] module*/
pub type LOCK_MAP = crate::Reg<lock_map::LOCK_MAP_SPEC>;
///Lock (manual lock) map configure register
pub mod lock_map;
/**LOCK_ADDR (rw) register accessor: Lock (manual lock) address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`lock_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lock_addr`] module*/
pub type LOCK_ADDR = crate::Reg<lock_addr::LOCK_ADDR_SPEC>;
///Lock (manual lock) address configure register
pub mod lock_addr;
/**LOCK_SIZE (rw) register accessor: Lock (manual lock) size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`lock_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lock_size`] module*/
pub type LOCK_SIZE = crate::Reg<lock_size::LOCK_SIZE_SPEC>;
///Lock (manual lock) size configure register
pub mod lock_size;
/**SYNC_CTRL (rw) register accessor: Sync-class operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync_ctrl`] module*/
pub type SYNC_CTRL = crate::Reg<sync_ctrl::SYNC_CTRL_SPEC>;
///Sync-class operation control register
pub mod sync_ctrl;
/**SYNC_MAP (rw) register accessor: Sync map configure register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_map::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_map::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync_map`] module*/
pub type SYNC_MAP = crate::Reg<sync_map::SYNC_MAP_SPEC>;
///Sync map configure register
pub mod sync_map;
/**SYNC_ADDR (rw) register accessor: Sync address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync_addr`] module*/
pub type SYNC_ADDR = crate::Reg<sync_addr::SYNC_ADDR_SPEC>;
///Sync address configure register
pub mod sync_addr;
/**SYNC_SIZE (rw) register accessor: Sync size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync_size`] module*/
pub type SYNC_SIZE = crate::Reg<sync_size::SYNC_SIZE_SPEC>;
///Sync size configure register
pub mod sync_size;
/**L1_ICACHE0_PRELOAD_CTRL (rw) register accessor: L1 instruction Cache 0 preload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_preload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_preload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_preload_ctrl`] module*/
pub type L1_ICACHE0_PRELOAD_CTRL = crate::Reg<
    l1_icache0_preload_ctrl::L1_ICACHE0_PRELOAD_CTRL_SPEC,
>;
///L1 instruction Cache 0 preload-operation control register
pub mod l1_icache0_preload_ctrl;
/**L1_ICACHE0_PRELOAD_ADDR (rw) register accessor: L1 instruction Cache 0 preload address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_preload_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_preload_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_preload_addr`] module*/
pub type L1_ICACHE0_PRELOAD_ADDR = crate::Reg<
    l1_icache0_preload_addr::L1_ICACHE0_PRELOAD_ADDR_SPEC,
>;
///L1 instruction Cache 0 preload address configure register
pub mod l1_icache0_preload_addr;
/**L1_ICACHE0_PRELOAD_SIZE (rw) register accessor: L1 instruction Cache 0 preload size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_preload_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_preload_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_preload_size`] module*/
pub type L1_ICACHE0_PRELOAD_SIZE = crate::Reg<
    l1_icache0_preload_size::L1_ICACHE0_PRELOAD_SIZE_SPEC,
>;
///L1 instruction Cache 0 preload size configure register
pub mod l1_icache0_preload_size;
/**L1_ICACHE1_PRELOAD_CTRL (rw) register accessor: L1 instruction Cache 1 preload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_preload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_preload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_preload_ctrl`] module*/
pub type L1_ICACHE1_PRELOAD_CTRL = crate::Reg<
    l1_icache1_preload_ctrl::L1_ICACHE1_PRELOAD_CTRL_SPEC,
>;
///L1 instruction Cache 1 preload-operation control register
pub mod l1_icache1_preload_ctrl;
/**L1_ICACHE1_PRELOAD_ADDR (rw) register accessor: L1 instruction Cache 1 preload address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_preload_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_preload_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_preload_addr`] module*/
pub type L1_ICACHE1_PRELOAD_ADDR = crate::Reg<
    l1_icache1_preload_addr::L1_ICACHE1_PRELOAD_ADDR_SPEC,
>;
///L1 instruction Cache 1 preload address configure register
pub mod l1_icache1_preload_addr;
/**L1_ICACHE1_PRELOAD_SIZE (rw) register accessor: L1 instruction Cache 1 preload size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_preload_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_preload_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_preload_size`] module*/
pub type L1_ICACHE1_PRELOAD_SIZE = crate::Reg<
    l1_icache1_preload_size::L1_ICACHE1_PRELOAD_SIZE_SPEC,
>;
///L1 instruction Cache 1 preload size configure register
pub mod l1_icache1_preload_size;
/**L1_ICACHE2_PRELOAD_CTRL (r) register accessor: L1 instruction Cache 2 preload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_preload_ctrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_preload_ctrl`] module*/
pub type L1_ICACHE2_PRELOAD_CTRL = crate::Reg<
    l1_icache2_preload_ctrl::L1_ICACHE2_PRELOAD_CTRL_SPEC,
>;
///L1 instruction Cache 2 preload-operation control register
pub mod l1_icache2_preload_ctrl;
/**L1_ICACHE2_PRELOAD_ADDR (r) register accessor: L1 instruction Cache 2 preload address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_preload_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_preload_addr`] module*/
pub type L1_ICACHE2_PRELOAD_ADDR = crate::Reg<
    l1_icache2_preload_addr::L1_ICACHE2_PRELOAD_ADDR_SPEC,
>;
///L1 instruction Cache 2 preload address configure register
pub mod l1_icache2_preload_addr;
/**L1_ICACHE2_PRELOAD_SIZE (r) register accessor: L1 instruction Cache 2 preload size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_preload_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_preload_size`] module*/
pub type L1_ICACHE2_PRELOAD_SIZE = crate::Reg<
    l1_icache2_preload_size::L1_ICACHE2_PRELOAD_SIZE_SPEC,
>;
///L1 instruction Cache 2 preload size configure register
pub mod l1_icache2_preload_size;
/**L1_ICACHE3_PRELOAD_CTRL (r) register accessor: L1 instruction Cache 3 preload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_preload_ctrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_preload_ctrl`] module*/
pub type L1_ICACHE3_PRELOAD_CTRL = crate::Reg<
    l1_icache3_preload_ctrl::L1_ICACHE3_PRELOAD_CTRL_SPEC,
>;
///L1 instruction Cache 3 preload-operation control register
pub mod l1_icache3_preload_ctrl;
/**L1_ICACHE3_PRELOAD_ADDR (r) register accessor: L1 instruction Cache 3 preload address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_preload_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_preload_addr`] module*/
pub type L1_ICACHE3_PRELOAD_ADDR = crate::Reg<
    l1_icache3_preload_addr::L1_ICACHE3_PRELOAD_ADDR_SPEC,
>;
///L1 instruction Cache 3 preload address configure register
pub mod l1_icache3_preload_addr;
/**L1_ICACHE3_PRELOAD_SIZE (r) register accessor: L1 instruction Cache 3 preload size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_preload_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_preload_size`] module*/
pub type L1_ICACHE3_PRELOAD_SIZE = crate::Reg<
    l1_icache3_preload_size::L1_ICACHE3_PRELOAD_SIZE_SPEC,
>;
///L1 instruction Cache 3 preload size configure register
pub mod l1_icache3_preload_size;
/**L1_DCACHE_PRELOAD_CTRL (rw) register accessor: L1 data Cache preload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_preload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_preload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_preload_ctrl`] module*/
pub type L1_DCACHE_PRELOAD_CTRL = crate::Reg<
    l1_dcache_preload_ctrl::L1_DCACHE_PRELOAD_CTRL_SPEC,
>;
///L1 data Cache preload-operation control register
pub mod l1_dcache_preload_ctrl;
/**L1_DCACHE_PRELOAD_ADDR (rw) register accessor: L1 data Cache preload address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_preload_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_preload_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_preload_addr`] module*/
pub type L1_DCACHE_PRELOAD_ADDR = crate::Reg<
    l1_dcache_preload_addr::L1_DCACHE_PRELOAD_ADDR_SPEC,
>;
///L1 data Cache preload address configure register
pub mod l1_dcache_preload_addr;
/**L1_DCACHE_PRELOAD_SIZE (rw) register accessor: L1 data Cache preload size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_preload_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_preload_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_preload_size`] module*/
pub type L1_DCACHE_PRELOAD_SIZE = crate::Reg<
    l1_dcache_preload_size::L1_DCACHE_PRELOAD_SIZE_SPEC,
>;
///L1 data Cache preload size configure register
pub mod l1_dcache_preload_size;
/**L1_ICACHE0_AUTOLOAD_CTRL (rw) register accessor: L1 instruction Cache 0 autoload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_autoload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_autoload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_autoload_ctrl`] module*/
pub type L1_ICACHE0_AUTOLOAD_CTRL = crate::Reg<
    l1_icache0_autoload_ctrl::L1_ICACHE0_AUTOLOAD_CTRL_SPEC,
>;
///L1 instruction Cache 0 autoload-operation control register
pub mod l1_icache0_autoload_ctrl;
/**L1_ICACHE0_AUTOLOAD_SCT0_ADDR (rw) register accessor: L1 instruction Cache 0 autoload section 0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_autoload_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_autoload_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_autoload_sct0_addr`] module*/
pub type L1_ICACHE0_AUTOLOAD_SCT0_ADDR = crate::Reg<
    l1_icache0_autoload_sct0_addr::L1_ICACHE0_AUTOLOAD_SCT0_ADDR_SPEC,
>;
///L1 instruction Cache 0 autoload section 0 address configure register
pub mod l1_icache0_autoload_sct0_addr;
/**L1_ICACHE0_AUTOLOAD_SCT0_SIZE (rw) register accessor: L1 instruction Cache 0 autoload section 0 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_autoload_sct0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_autoload_sct0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_autoload_sct0_size`] module*/
pub type L1_ICACHE0_AUTOLOAD_SCT0_SIZE = crate::Reg<
    l1_icache0_autoload_sct0_size::L1_ICACHE0_AUTOLOAD_SCT0_SIZE_SPEC,
>;
///L1 instruction Cache 0 autoload section 0 size configure register
pub mod l1_icache0_autoload_sct0_size;
/**L1_ICACHE0_AUTOLOAD_SCT1_ADDR (rw) register accessor: L1 instruction Cache 0 autoload section 1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_autoload_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_autoload_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_autoload_sct1_addr`] module*/
pub type L1_ICACHE0_AUTOLOAD_SCT1_ADDR = crate::Reg<
    l1_icache0_autoload_sct1_addr::L1_ICACHE0_AUTOLOAD_SCT1_ADDR_SPEC,
>;
///L1 instruction Cache 0 autoload section 1 address configure register
pub mod l1_icache0_autoload_sct1_addr;
/**L1_ICACHE0_AUTOLOAD_SCT1_SIZE (rw) register accessor: L1 instruction Cache 0 autoload section 1 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_autoload_sct1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache0_autoload_sct1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_autoload_sct1_size`] module*/
pub type L1_ICACHE0_AUTOLOAD_SCT1_SIZE = crate::Reg<
    l1_icache0_autoload_sct1_size::L1_ICACHE0_AUTOLOAD_SCT1_SIZE_SPEC,
>;
///L1 instruction Cache 0 autoload section 1 size configure register
pub mod l1_icache0_autoload_sct1_size;
/**L1_ICACHE1_AUTOLOAD_CTRL (rw) register accessor: L1 instruction Cache 1 autoload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_autoload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_autoload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_autoload_ctrl`] module*/
pub type L1_ICACHE1_AUTOLOAD_CTRL = crate::Reg<
    l1_icache1_autoload_ctrl::L1_ICACHE1_AUTOLOAD_CTRL_SPEC,
>;
///L1 instruction Cache 1 autoload-operation control register
pub mod l1_icache1_autoload_ctrl;
/**L1_ICACHE1_AUTOLOAD_SCT0_ADDR (rw) register accessor: L1 instruction Cache 1 autoload section 0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_autoload_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_autoload_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_autoload_sct0_addr`] module*/
pub type L1_ICACHE1_AUTOLOAD_SCT0_ADDR = crate::Reg<
    l1_icache1_autoload_sct0_addr::L1_ICACHE1_AUTOLOAD_SCT0_ADDR_SPEC,
>;
///L1 instruction Cache 1 autoload section 0 address configure register
pub mod l1_icache1_autoload_sct0_addr;
/**L1_ICACHE1_AUTOLOAD_SCT0_SIZE (rw) register accessor: L1 instruction Cache 1 autoload section 0 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_autoload_sct0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_autoload_sct0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_autoload_sct0_size`] module*/
pub type L1_ICACHE1_AUTOLOAD_SCT0_SIZE = crate::Reg<
    l1_icache1_autoload_sct0_size::L1_ICACHE1_AUTOLOAD_SCT0_SIZE_SPEC,
>;
///L1 instruction Cache 1 autoload section 0 size configure register
pub mod l1_icache1_autoload_sct0_size;
/**L1_ICACHE1_AUTOLOAD_SCT1_ADDR (rw) register accessor: L1 instruction Cache 1 autoload section 1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_autoload_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_autoload_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_autoload_sct1_addr`] module*/
pub type L1_ICACHE1_AUTOLOAD_SCT1_ADDR = crate::Reg<
    l1_icache1_autoload_sct1_addr::L1_ICACHE1_AUTOLOAD_SCT1_ADDR_SPEC,
>;
///L1 instruction Cache 1 autoload section 1 address configure register
pub mod l1_icache1_autoload_sct1_addr;
/**L1_ICACHE1_AUTOLOAD_SCT1_SIZE (rw) register accessor: L1 instruction Cache 1 autoload section 1 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_autoload_sct1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_icache1_autoload_sct1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_autoload_sct1_size`] module*/
pub type L1_ICACHE1_AUTOLOAD_SCT1_SIZE = crate::Reg<
    l1_icache1_autoload_sct1_size::L1_ICACHE1_AUTOLOAD_SCT1_SIZE_SPEC,
>;
///L1 instruction Cache 1 autoload section 1 size configure register
pub mod l1_icache1_autoload_sct1_size;
/**L1_ICACHE2_AUTOLOAD_CTRL (r) register accessor: L1 instruction Cache 2 autoload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_autoload_ctrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_autoload_ctrl`] module*/
pub type L1_ICACHE2_AUTOLOAD_CTRL = crate::Reg<
    l1_icache2_autoload_ctrl::L1_ICACHE2_AUTOLOAD_CTRL_SPEC,
>;
///L1 instruction Cache 2 autoload-operation control register
pub mod l1_icache2_autoload_ctrl;
/**L1_ICACHE2_AUTOLOAD_SCT0_ADDR (r) register accessor: L1 instruction Cache 2 autoload section 0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_autoload_sct0_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_autoload_sct0_addr`] module*/
pub type L1_ICACHE2_AUTOLOAD_SCT0_ADDR = crate::Reg<
    l1_icache2_autoload_sct0_addr::L1_ICACHE2_AUTOLOAD_SCT0_ADDR_SPEC,
>;
///L1 instruction Cache 2 autoload section 0 address configure register
pub mod l1_icache2_autoload_sct0_addr;
/**L1_ICACHE2_AUTOLOAD_SCT0_SIZE (r) register accessor: L1 instruction Cache 2 autoload section 0 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_autoload_sct0_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_autoload_sct0_size`] module*/
pub type L1_ICACHE2_AUTOLOAD_SCT0_SIZE = crate::Reg<
    l1_icache2_autoload_sct0_size::L1_ICACHE2_AUTOLOAD_SCT0_SIZE_SPEC,
>;
///L1 instruction Cache 2 autoload section 0 size configure register
pub mod l1_icache2_autoload_sct0_size;
/**L1_ICACHE2_AUTOLOAD_SCT1_ADDR (r) register accessor: L1 instruction Cache 2 autoload section 1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_autoload_sct1_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_autoload_sct1_addr`] module*/
pub type L1_ICACHE2_AUTOLOAD_SCT1_ADDR = crate::Reg<
    l1_icache2_autoload_sct1_addr::L1_ICACHE2_AUTOLOAD_SCT1_ADDR_SPEC,
>;
///L1 instruction Cache 2 autoload section 1 address configure register
pub mod l1_icache2_autoload_sct1_addr;
/**L1_ICACHE2_AUTOLOAD_SCT1_SIZE (r) register accessor: L1 instruction Cache 2 autoload section 1 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_autoload_sct1_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_autoload_sct1_size`] module*/
pub type L1_ICACHE2_AUTOLOAD_SCT1_SIZE = crate::Reg<
    l1_icache2_autoload_sct1_size::L1_ICACHE2_AUTOLOAD_SCT1_SIZE_SPEC,
>;
///L1 instruction Cache 2 autoload section 1 size configure register
pub mod l1_icache2_autoload_sct1_size;
/**L1_ICACHE3_AUTOLOAD_CTRL (r) register accessor: L1 instruction Cache 3 autoload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_autoload_ctrl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_autoload_ctrl`] module*/
pub type L1_ICACHE3_AUTOLOAD_CTRL = crate::Reg<
    l1_icache3_autoload_ctrl::L1_ICACHE3_AUTOLOAD_CTRL_SPEC,
>;
///L1 instruction Cache 3 autoload-operation control register
pub mod l1_icache3_autoload_ctrl;
/**L1_ICACHE3_AUTOLOAD_SCT0_ADDR (r) register accessor: L1 instruction Cache 3 autoload section 0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_autoload_sct0_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_autoload_sct0_addr`] module*/
pub type L1_ICACHE3_AUTOLOAD_SCT0_ADDR = crate::Reg<
    l1_icache3_autoload_sct0_addr::L1_ICACHE3_AUTOLOAD_SCT0_ADDR_SPEC,
>;
///L1 instruction Cache 3 autoload section 0 address configure register
pub mod l1_icache3_autoload_sct0_addr;
/**L1_ICACHE3_AUTOLOAD_SCT0_SIZE (r) register accessor: L1 instruction Cache 3 autoload section 0 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_autoload_sct0_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_autoload_sct0_size`] module*/
pub type L1_ICACHE3_AUTOLOAD_SCT0_SIZE = crate::Reg<
    l1_icache3_autoload_sct0_size::L1_ICACHE3_AUTOLOAD_SCT0_SIZE_SPEC,
>;
///L1 instruction Cache 3 autoload section 0 size configure register
pub mod l1_icache3_autoload_sct0_size;
/**L1_ICACHE3_AUTOLOAD_SCT1_ADDR (r) register accessor: L1 instruction Cache 3 autoload section 1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_autoload_sct1_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_autoload_sct1_addr`] module*/
pub type L1_ICACHE3_AUTOLOAD_SCT1_ADDR = crate::Reg<
    l1_icache3_autoload_sct1_addr::L1_ICACHE3_AUTOLOAD_SCT1_ADDR_SPEC,
>;
///L1 instruction Cache 3 autoload section 1 address configure register
pub mod l1_icache3_autoload_sct1_addr;
/**L1_ICACHE3_AUTOLOAD_SCT1_SIZE (r) register accessor: L1 instruction Cache 3 autoload section 1 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_autoload_sct1_size::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_autoload_sct1_size`] module*/
pub type L1_ICACHE3_AUTOLOAD_SCT1_SIZE = crate::Reg<
    l1_icache3_autoload_sct1_size::L1_ICACHE3_AUTOLOAD_SCT1_SIZE_SPEC,
>;
///L1 instruction Cache 3 autoload section 1 size configure register
pub mod l1_icache3_autoload_sct1_size;
/**L1_DCACHE_AUTOLOAD_CTRL (rw) register accessor: L1 data Cache autoload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_autoload_ctrl`] module*/
pub type L1_DCACHE_AUTOLOAD_CTRL = crate::Reg<
    l1_dcache_autoload_ctrl::L1_DCACHE_AUTOLOAD_CTRL_SPEC,
>;
///L1 data Cache autoload-operation control register
pub mod l1_dcache_autoload_ctrl;
/**L1_DCACHE_AUTOLOAD_SCT0_ADDR (rw) register accessor: L1 data Cache autoload section 0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_autoload_sct0_addr`] module*/
pub type L1_DCACHE_AUTOLOAD_SCT0_ADDR = crate::Reg<
    l1_dcache_autoload_sct0_addr::L1_DCACHE_AUTOLOAD_SCT0_ADDR_SPEC,
>;
///L1 data Cache autoload section 0 address configure register
pub mod l1_dcache_autoload_sct0_addr;
/**L1_DCACHE_AUTOLOAD_SCT0_SIZE (rw) register accessor: L1 data Cache autoload section 0 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_sct0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_autoload_sct0_size`] module*/
pub type L1_DCACHE_AUTOLOAD_SCT0_SIZE = crate::Reg<
    l1_dcache_autoload_sct0_size::L1_DCACHE_AUTOLOAD_SCT0_SIZE_SPEC,
>;
///L1 data Cache autoload section 0 size configure register
pub mod l1_dcache_autoload_sct0_size;
/**L1_DCACHE_AUTOLOAD_SCT1_ADDR (rw) register accessor: L1 data Cache autoload section 1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_autoload_sct1_addr`] module*/
pub type L1_DCACHE_AUTOLOAD_SCT1_ADDR = crate::Reg<
    l1_dcache_autoload_sct1_addr::L1_DCACHE_AUTOLOAD_SCT1_ADDR_SPEC,
>;
///L1 data Cache autoload section 1 address configure register
pub mod l1_dcache_autoload_sct1_addr;
/**L1_DCACHE_AUTOLOAD_SCT1_SIZE (rw) register accessor: L1 data Cache autoload section 1 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_sct1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_autoload_sct1_size`] module*/
pub type L1_DCACHE_AUTOLOAD_SCT1_SIZE = crate::Reg<
    l1_dcache_autoload_sct1_size::L1_DCACHE_AUTOLOAD_SCT1_SIZE_SPEC,
>;
///L1 data Cache autoload section 1 size configure register
pub mod l1_dcache_autoload_sct1_size;
/**L1_DCACHE_AUTOLOAD_SCT2_ADDR (rw) register accessor: L1 data Cache autoload section 2 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_sct2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_autoload_sct2_addr`] module*/
pub type L1_DCACHE_AUTOLOAD_SCT2_ADDR = crate::Reg<
    l1_dcache_autoload_sct2_addr::L1_DCACHE_AUTOLOAD_SCT2_ADDR_SPEC,
>;
///L1 data Cache autoload section 2 address configure register
pub mod l1_dcache_autoload_sct2_addr;
/**L1_DCACHE_AUTOLOAD_SCT2_SIZE (rw) register accessor: L1 data Cache autoload section 2 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_sct2_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct2_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_autoload_sct2_size`] module*/
pub type L1_DCACHE_AUTOLOAD_SCT2_SIZE = crate::Reg<
    l1_dcache_autoload_sct2_size::L1_DCACHE_AUTOLOAD_SCT2_SIZE_SPEC,
>;
///L1 data Cache autoload section 2 size configure register
pub mod l1_dcache_autoload_sct2_size;
/**L1_DCACHE_AUTOLOAD_SCT3_ADDR (rw) register accessor: L1 data Cache autoload section 1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_sct3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_autoload_sct3_addr`] module*/
pub type L1_DCACHE_AUTOLOAD_SCT3_ADDR = crate::Reg<
    l1_dcache_autoload_sct3_addr::L1_DCACHE_AUTOLOAD_SCT3_ADDR_SPEC,
>;
///L1 data Cache autoload section 1 address configure register
pub mod l1_dcache_autoload_sct3_addr;
/**L1_DCACHE_AUTOLOAD_SCT3_SIZE (rw) register accessor: L1 data Cache autoload section 1 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_autoload_sct3_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_dcache_autoload_sct3_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_autoload_sct3_size`] module*/
pub type L1_DCACHE_AUTOLOAD_SCT3_SIZE = crate::Reg<
    l1_dcache_autoload_sct3_size::L1_DCACHE_AUTOLOAD_SCT3_SIZE_SPEC,
>;
///L1 data Cache autoload section 1 size configure register
pub mod l1_dcache_autoload_sct3_size;
/**L1_CACHE_ACS_CNT_INT_ENA (rw) register accessor: Cache Access Counter Interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_cnt_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_cnt_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_cnt_int_ena`] module*/
pub type L1_CACHE_ACS_CNT_INT_ENA = crate::Reg<
    l1_cache_acs_cnt_int_ena::L1_CACHE_ACS_CNT_INT_ENA_SPEC,
>;
///Cache Access Counter Interrupt enable register
pub mod l1_cache_acs_cnt_int_ena;
/**L1_CACHE_ACS_CNT_INT_CLR (rw) register accessor: Cache Access Counter Interrupt clear register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_cnt_int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_cnt_int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_cnt_int_clr`] module*/
pub type L1_CACHE_ACS_CNT_INT_CLR = crate::Reg<
    l1_cache_acs_cnt_int_clr::L1_CACHE_ACS_CNT_INT_CLR_SPEC,
>;
///Cache Access Counter Interrupt clear register
pub mod l1_cache_acs_cnt_int_clr;
/**L1_CACHE_ACS_CNT_INT_RAW (rw) register accessor: Cache Access Counter Interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_cnt_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_cnt_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_cnt_int_raw`] module*/
pub type L1_CACHE_ACS_CNT_INT_RAW = crate::Reg<
    l1_cache_acs_cnt_int_raw::L1_CACHE_ACS_CNT_INT_RAW_SPEC,
>;
///Cache Access Counter Interrupt raw register
pub mod l1_cache_acs_cnt_int_raw;
/**L1_CACHE_ACS_CNT_INT_ST (r) register accessor: Cache Access Counter Interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_cnt_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_cnt_int_st`] module*/
pub type L1_CACHE_ACS_CNT_INT_ST = crate::Reg<
    l1_cache_acs_cnt_int_st::L1_CACHE_ACS_CNT_INT_ST_SPEC,
>;
///Cache Access Counter Interrupt status register
pub mod l1_cache_acs_cnt_int_st;
/**L1_CACHE_ACS_FAIL_CTRL (rw) register accessor: Cache Access Fail Configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_fail_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_fail_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_fail_ctrl`] module*/
pub type L1_CACHE_ACS_FAIL_CTRL = crate::Reg<
    l1_cache_acs_fail_ctrl::L1_CACHE_ACS_FAIL_CTRL_SPEC,
>;
///Cache Access Fail Configuration register
pub mod l1_cache_acs_fail_ctrl;
/**L1_CACHE_ACS_FAIL_INT_ENA (rw) register accessor: Cache Access Fail Interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_fail_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_fail_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_fail_int_ena`] module*/
pub type L1_CACHE_ACS_FAIL_INT_ENA = crate::Reg<
    l1_cache_acs_fail_int_ena::L1_CACHE_ACS_FAIL_INT_ENA_SPEC,
>;
///Cache Access Fail Interrupt enable register
pub mod l1_cache_acs_fail_int_ena;
/**L1_CACHE_ACS_FAIL_INT_CLR (rw) register accessor: L1-Cache Access Fail Interrupt clear register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_fail_int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_fail_int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_fail_int_clr`] module*/
pub type L1_CACHE_ACS_FAIL_INT_CLR = crate::Reg<
    l1_cache_acs_fail_int_clr::L1_CACHE_ACS_FAIL_INT_CLR_SPEC,
>;
///L1-Cache Access Fail Interrupt clear register
pub mod l1_cache_acs_fail_int_clr;
/**L1_CACHE_ACS_FAIL_INT_RAW (rw) register accessor: Cache Access Fail Interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_fail_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_fail_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_fail_int_raw`] module*/
pub type L1_CACHE_ACS_FAIL_INT_RAW = crate::Reg<
    l1_cache_acs_fail_int_raw::L1_CACHE_ACS_FAIL_INT_RAW_SPEC,
>;
///Cache Access Fail Interrupt raw register
pub mod l1_cache_acs_fail_int_raw;
/**L1_CACHE_ACS_FAIL_INT_ST (r) register accessor: Cache Access Fail Interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_fail_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_fail_int_st`] module*/
pub type L1_CACHE_ACS_FAIL_INT_ST = crate::Reg<
    l1_cache_acs_fail_int_st::L1_CACHE_ACS_FAIL_INT_ST_SPEC,
>;
///Cache Access Fail Interrupt status register
pub mod l1_cache_acs_fail_int_st;
/**L1_CACHE_ACS_CNT_CTRL (rw) register accessor: Cache Access Counter enable and clear register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_acs_cnt_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_acs_cnt_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_acs_cnt_ctrl`] module*/
pub type L1_CACHE_ACS_CNT_CTRL = crate::Reg<
    l1_cache_acs_cnt_ctrl::L1_CACHE_ACS_CNT_CTRL_SPEC,
>;
///Cache Access Counter enable and clear register
pub mod l1_cache_acs_cnt_ctrl;
/**L1_IBUS0_ACS_HIT_CNT (r) register accessor: L1-ICache bus0 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus0_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus0_acs_hit_cnt`] module*/
pub type L1_IBUS0_ACS_HIT_CNT = crate::Reg<
    l1_ibus0_acs_hit_cnt::L1_IBUS0_ACS_HIT_CNT_SPEC,
>;
///L1-ICache bus0 Hit-Access Counter register
pub mod l1_ibus0_acs_hit_cnt;
/**L1_IBUS0_ACS_MISS_CNT (r) register accessor: L1-ICache bus0 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus0_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus0_acs_miss_cnt`] module*/
pub type L1_IBUS0_ACS_MISS_CNT = crate::Reg<
    l1_ibus0_acs_miss_cnt::L1_IBUS0_ACS_MISS_CNT_SPEC,
>;
///L1-ICache bus0 Miss-Access Counter register
pub mod l1_ibus0_acs_miss_cnt;
/**L1_IBUS0_ACS_CONFLICT_CNT (r) register accessor: L1-ICache bus0 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus0_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus0_acs_conflict_cnt`] module*/
pub type L1_IBUS0_ACS_CONFLICT_CNT = crate::Reg<
    l1_ibus0_acs_conflict_cnt::L1_IBUS0_ACS_CONFLICT_CNT_SPEC,
>;
///L1-ICache bus0 Conflict-Access Counter register
pub mod l1_ibus0_acs_conflict_cnt;
/**L1_IBUS0_ACS_NXTLVL_RD_CNT (r) register accessor: L1-ICache bus0 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus0_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus0_acs_nxtlvl_rd_cnt`] module*/
pub type L1_IBUS0_ACS_NXTLVL_RD_CNT = crate::Reg<
    l1_ibus0_acs_nxtlvl_rd_cnt::L1_IBUS0_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L1-ICache bus0 Next-Level-Access Counter register
pub mod l1_ibus0_acs_nxtlvl_rd_cnt;
/**L1_IBUS1_ACS_HIT_CNT (r) register accessor: L1-ICache bus1 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus1_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus1_acs_hit_cnt`] module*/
pub type L1_IBUS1_ACS_HIT_CNT = crate::Reg<
    l1_ibus1_acs_hit_cnt::L1_IBUS1_ACS_HIT_CNT_SPEC,
>;
///L1-ICache bus1 Hit-Access Counter register
pub mod l1_ibus1_acs_hit_cnt;
/**L1_IBUS1_ACS_MISS_CNT (r) register accessor: L1-ICache bus1 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus1_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus1_acs_miss_cnt`] module*/
pub type L1_IBUS1_ACS_MISS_CNT = crate::Reg<
    l1_ibus1_acs_miss_cnt::L1_IBUS1_ACS_MISS_CNT_SPEC,
>;
///L1-ICache bus1 Miss-Access Counter register
pub mod l1_ibus1_acs_miss_cnt;
/**L1_IBUS1_ACS_CONFLICT_CNT (r) register accessor: L1-ICache bus1 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus1_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus1_acs_conflict_cnt`] module*/
pub type L1_IBUS1_ACS_CONFLICT_CNT = crate::Reg<
    l1_ibus1_acs_conflict_cnt::L1_IBUS1_ACS_CONFLICT_CNT_SPEC,
>;
///L1-ICache bus1 Conflict-Access Counter register
pub mod l1_ibus1_acs_conflict_cnt;
/**L1_IBUS1_ACS_NXTLVL_RD_CNT (r) register accessor: L1-ICache bus1 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus1_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus1_acs_nxtlvl_rd_cnt`] module*/
pub type L1_IBUS1_ACS_NXTLVL_RD_CNT = crate::Reg<
    l1_ibus1_acs_nxtlvl_rd_cnt::L1_IBUS1_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L1-ICache bus1 Next-Level-Access Counter register
pub mod l1_ibus1_acs_nxtlvl_rd_cnt;
/**L1_IBUS2_ACS_HIT_CNT (r) register accessor: L1-ICache bus2 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus2_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus2_acs_hit_cnt`] module*/
pub type L1_IBUS2_ACS_HIT_CNT = crate::Reg<
    l1_ibus2_acs_hit_cnt::L1_IBUS2_ACS_HIT_CNT_SPEC,
>;
///L1-ICache bus2 Hit-Access Counter register
pub mod l1_ibus2_acs_hit_cnt;
/**L1_IBUS2_ACS_MISS_CNT (r) register accessor: L1-ICache bus2 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus2_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus2_acs_miss_cnt`] module*/
pub type L1_IBUS2_ACS_MISS_CNT = crate::Reg<
    l1_ibus2_acs_miss_cnt::L1_IBUS2_ACS_MISS_CNT_SPEC,
>;
///L1-ICache bus2 Miss-Access Counter register
pub mod l1_ibus2_acs_miss_cnt;
/**L1_IBUS2_ACS_CONFLICT_CNT (r) register accessor: L1-ICache bus2 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus2_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus2_acs_conflict_cnt`] module*/
pub type L1_IBUS2_ACS_CONFLICT_CNT = crate::Reg<
    l1_ibus2_acs_conflict_cnt::L1_IBUS2_ACS_CONFLICT_CNT_SPEC,
>;
///L1-ICache bus2 Conflict-Access Counter register
pub mod l1_ibus2_acs_conflict_cnt;
/**L1_IBUS2_ACS_NXTLVL_RD_CNT (r) register accessor: L1-ICache bus2 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus2_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus2_acs_nxtlvl_rd_cnt`] module*/
pub type L1_IBUS2_ACS_NXTLVL_RD_CNT = crate::Reg<
    l1_ibus2_acs_nxtlvl_rd_cnt::L1_IBUS2_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L1-ICache bus2 Next-Level-Access Counter register
pub mod l1_ibus2_acs_nxtlvl_rd_cnt;
/**L1_IBUS3_ACS_HIT_CNT (r) register accessor: L1-ICache bus3 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus3_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus3_acs_hit_cnt`] module*/
pub type L1_IBUS3_ACS_HIT_CNT = crate::Reg<
    l1_ibus3_acs_hit_cnt::L1_IBUS3_ACS_HIT_CNT_SPEC,
>;
///L1-ICache bus3 Hit-Access Counter register
pub mod l1_ibus3_acs_hit_cnt;
/**L1_IBUS3_ACS_MISS_CNT (r) register accessor: L1-ICache bus3 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus3_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus3_acs_miss_cnt`] module*/
pub type L1_IBUS3_ACS_MISS_CNT = crate::Reg<
    l1_ibus3_acs_miss_cnt::L1_IBUS3_ACS_MISS_CNT_SPEC,
>;
///L1-ICache bus3 Miss-Access Counter register
pub mod l1_ibus3_acs_miss_cnt;
/**L1_IBUS3_ACS_CONFLICT_CNT (r) register accessor: L1-ICache bus3 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus3_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus3_acs_conflict_cnt`] module*/
pub type L1_IBUS3_ACS_CONFLICT_CNT = crate::Reg<
    l1_ibus3_acs_conflict_cnt::L1_IBUS3_ACS_CONFLICT_CNT_SPEC,
>;
///L1-ICache bus3 Conflict-Access Counter register
pub mod l1_ibus3_acs_conflict_cnt;
/**L1_IBUS3_ACS_NXTLVL_RD_CNT (r) register accessor: L1-ICache bus3 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_ibus3_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_ibus3_acs_nxtlvl_rd_cnt`] module*/
pub type L1_IBUS3_ACS_NXTLVL_RD_CNT = crate::Reg<
    l1_ibus3_acs_nxtlvl_rd_cnt::L1_IBUS3_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L1-ICache bus3 Next-Level-Access Counter register
pub mod l1_ibus3_acs_nxtlvl_rd_cnt;
/**L1_DBUS0_ACS_HIT_CNT (r) register accessor: L1-DCache bus0 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus0_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus0_acs_hit_cnt`] module*/
pub type L1_DBUS0_ACS_HIT_CNT = crate::Reg<
    l1_dbus0_acs_hit_cnt::L1_DBUS0_ACS_HIT_CNT_SPEC,
>;
///L1-DCache bus0 Hit-Access Counter register
pub mod l1_dbus0_acs_hit_cnt;
/**L1_DBUS0_ACS_MISS_CNT (r) register accessor: L1-DCache bus0 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus0_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus0_acs_miss_cnt`] module*/
pub type L1_DBUS0_ACS_MISS_CNT = crate::Reg<
    l1_dbus0_acs_miss_cnt::L1_DBUS0_ACS_MISS_CNT_SPEC,
>;
///L1-DCache bus0 Miss-Access Counter register
pub mod l1_dbus0_acs_miss_cnt;
/**L1_DBUS0_ACS_CONFLICT_CNT (r) register accessor: L1-DCache bus0 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus0_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus0_acs_conflict_cnt`] module*/
pub type L1_DBUS0_ACS_CONFLICT_CNT = crate::Reg<
    l1_dbus0_acs_conflict_cnt::L1_DBUS0_ACS_CONFLICT_CNT_SPEC,
>;
///L1-DCache bus0 Conflict-Access Counter register
pub mod l1_dbus0_acs_conflict_cnt;
/**L1_DBUS0_ACS_NXTLVL_RD_CNT (r) register accessor: L1-DCache bus0 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus0_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus0_acs_nxtlvl_rd_cnt`] module*/
pub type L1_DBUS0_ACS_NXTLVL_RD_CNT = crate::Reg<
    l1_dbus0_acs_nxtlvl_rd_cnt::L1_DBUS0_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L1-DCache bus0 Next-Level-Access Counter register
pub mod l1_dbus0_acs_nxtlvl_rd_cnt;
/**L1_DBUS0_ACS_NXTLVL_WR_CNT (r) register accessor: L1-DCache bus0 WB-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus0_acs_nxtlvl_wr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus0_acs_nxtlvl_wr_cnt`] module*/
pub type L1_DBUS0_ACS_NXTLVL_WR_CNT = crate::Reg<
    l1_dbus0_acs_nxtlvl_wr_cnt::L1_DBUS0_ACS_NXTLVL_WR_CNT_SPEC,
>;
///L1-DCache bus0 WB-Access Counter register
pub mod l1_dbus0_acs_nxtlvl_wr_cnt;
/**L1_DBUS1_ACS_HIT_CNT (r) register accessor: L1-DCache bus1 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus1_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus1_acs_hit_cnt`] module*/
pub type L1_DBUS1_ACS_HIT_CNT = crate::Reg<
    l1_dbus1_acs_hit_cnt::L1_DBUS1_ACS_HIT_CNT_SPEC,
>;
///L1-DCache bus1 Hit-Access Counter register
pub mod l1_dbus1_acs_hit_cnt;
/**L1_DBUS1_ACS_MISS_CNT (r) register accessor: L1-DCache bus1 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus1_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus1_acs_miss_cnt`] module*/
pub type L1_DBUS1_ACS_MISS_CNT = crate::Reg<
    l1_dbus1_acs_miss_cnt::L1_DBUS1_ACS_MISS_CNT_SPEC,
>;
///L1-DCache bus1 Miss-Access Counter register
pub mod l1_dbus1_acs_miss_cnt;
/**L1_DBUS1_ACS_CONFLICT_CNT (r) register accessor: L1-DCache bus1 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus1_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus1_acs_conflict_cnt`] module*/
pub type L1_DBUS1_ACS_CONFLICT_CNT = crate::Reg<
    l1_dbus1_acs_conflict_cnt::L1_DBUS1_ACS_CONFLICT_CNT_SPEC,
>;
///L1-DCache bus1 Conflict-Access Counter register
pub mod l1_dbus1_acs_conflict_cnt;
/**L1_DBUS1_ACS_NXTLVL_RD_CNT (r) register accessor: L1-DCache bus1 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus1_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus1_acs_nxtlvl_rd_cnt`] module*/
pub type L1_DBUS1_ACS_NXTLVL_RD_CNT = crate::Reg<
    l1_dbus1_acs_nxtlvl_rd_cnt::L1_DBUS1_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L1-DCache bus1 Next-Level-Access Counter register
pub mod l1_dbus1_acs_nxtlvl_rd_cnt;
/**L1_DBUS1_ACS_NXTLVL_WR_CNT (r) register accessor: L1-DCache bus1 WB-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus1_acs_nxtlvl_wr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus1_acs_nxtlvl_wr_cnt`] module*/
pub type L1_DBUS1_ACS_NXTLVL_WR_CNT = crate::Reg<
    l1_dbus1_acs_nxtlvl_wr_cnt::L1_DBUS1_ACS_NXTLVL_WR_CNT_SPEC,
>;
///L1-DCache bus1 WB-Access Counter register
pub mod l1_dbus1_acs_nxtlvl_wr_cnt;
/**L1_DBUS2_ACS_HIT_CNT (r) register accessor: L1-DCache bus2 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus2_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus2_acs_hit_cnt`] module*/
pub type L1_DBUS2_ACS_HIT_CNT = crate::Reg<
    l1_dbus2_acs_hit_cnt::L1_DBUS2_ACS_HIT_CNT_SPEC,
>;
///L1-DCache bus2 Hit-Access Counter register
pub mod l1_dbus2_acs_hit_cnt;
/**L1_DBUS2_ACS_MISS_CNT (r) register accessor: L1-DCache bus2 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus2_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus2_acs_miss_cnt`] module*/
pub type L1_DBUS2_ACS_MISS_CNT = crate::Reg<
    l1_dbus2_acs_miss_cnt::L1_DBUS2_ACS_MISS_CNT_SPEC,
>;
///L1-DCache bus2 Miss-Access Counter register
pub mod l1_dbus2_acs_miss_cnt;
/**L1_DBUS2_ACS_CONFLICT_CNT (r) register accessor: L1-DCache bus2 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus2_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus2_acs_conflict_cnt`] module*/
pub type L1_DBUS2_ACS_CONFLICT_CNT = crate::Reg<
    l1_dbus2_acs_conflict_cnt::L1_DBUS2_ACS_CONFLICT_CNT_SPEC,
>;
///L1-DCache bus2 Conflict-Access Counter register
pub mod l1_dbus2_acs_conflict_cnt;
/**L1_DBUS2_ACS_NXTLVL_RD_CNT (r) register accessor: L1-DCache bus2 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus2_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus2_acs_nxtlvl_rd_cnt`] module*/
pub type L1_DBUS2_ACS_NXTLVL_RD_CNT = crate::Reg<
    l1_dbus2_acs_nxtlvl_rd_cnt::L1_DBUS2_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L1-DCache bus2 Next-Level-Access Counter register
pub mod l1_dbus2_acs_nxtlvl_rd_cnt;
/**L1_DBUS2_ACS_NXTLVL_WR_CNT (r) register accessor: L1-DCache bus2 WB-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus2_acs_nxtlvl_wr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus2_acs_nxtlvl_wr_cnt`] module*/
pub type L1_DBUS2_ACS_NXTLVL_WR_CNT = crate::Reg<
    l1_dbus2_acs_nxtlvl_wr_cnt::L1_DBUS2_ACS_NXTLVL_WR_CNT_SPEC,
>;
///L1-DCache bus2 WB-Access Counter register
pub mod l1_dbus2_acs_nxtlvl_wr_cnt;
/**L1_DBUS3_ACS_HIT_CNT (r) register accessor: L1-DCache bus3 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus3_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus3_acs_hit_cnt`] module*/
pub type L1_DBUS3_ACS_HIT_CNT = crate::Reg<
    l1_dbus3_acs_hit_cnt::L1_DBUS3_ACS_HIT_CNT_SPEC,
>;
///L1-DCache bus3 Hit-Access Counter register
pub mod l1_dbus3_acs_hit_cnt;
/**L1_DBUS3_ACS_MISS_CNT (r) register accessor: L1-DCache bus3 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus3_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus3_acs_miss_cnt`] module*/
pub type L1_DBUS3_ACS_MISS_CNT = crate::Reg<
    l1_dbus3_acs_miss_cnt::L1_DBUS3_ACS_MISS_CNT_SPEC,
>;
///L1-DCache bus3 Miss-Access Counter register
pub mod l1_dbus3_acs_miss_cnt;
/**L1_DBUS3_ACS_CONFLICT_CNT (r) register accessor: L1-DCache bus3 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus3_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus3_acs_conflict_cnt`] module*/
pub type L1_DBUS3_ACS_CONFLICT_CNT = crate::Reg<
    l1_dbus3_acs_conflict_cnt::L1_DBUS3_ACS_CONFLICT_CNT_SPEC,
>;
///L1-DCache bus3 Conflict-Access Counter register
pub mod l1_dbus3_acs_conflict_cnt;
/**L1_DBUS3_ACS_NXTLVL_RD_CNT (r) register accessor: L1-DCache bus3 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus3_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus3_acs_nxtlvl_rd_cnt`] module*/
pub type L1_DBUS3_ACS_NXTLVL_RD_CNT = crate::Reg<
    l1_dbus3_acs_nxtlvl_rd_cnt::L1_DBUS3_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L1-DCache bus3 Next-Level-Access Counter register
pub mod l1_dbus3_acs_nxtlvl_rd_cnt;
/**L1_DBUS3_ACS_NXTLVL_WR_CNT (r) register accessor: L1-DCache bus3 WB-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dbus3_acs_nxtlvl_wr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dbus3_acs_nxtlvl_wr_cnt`] module*/
pub type L1_DBUS3_ACS_NXTLVL_WR_CNT = crate::Reg<
    l1_dbus3_acs_nxtlvl_wr_cnt::L1_DBUS3_ACS_NXTLVL_WR_CNT_SPEC,
>;
///L1-DCache bus3 WB-Access Counter register
pub mod l1_dbus3_acs_nxtlvl_wr_cnt;
/**L1_ICACHE0_ACS_FAIL_ID_ATTR (r) register accessor: L1-ICache0 Access Fail ID/attribution information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_acs_fail_id_attr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_acs_fail_id_attr`] module*/
pub type L1_ICACHE0_ACS_FAIL_ID_ATTR = crate::Reg<
    l1_icache0_acs_fail_id_attr::L1_ICACHE0_ACS_FAIL_ID_ATTR_SPEC,
>;
///L1-ICache0 Access Fail ID/attribution information register
pub mod l1_icache0_acs_fail_id_attr;
/**L1_ICACHE0_ACS_FAIL_ADDR (r) register accessor: L1-ICache0 Access Fail Address information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache0_acs_fail_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache0_acs_fail_addr`] module*/
pub type L1_ICACHE0_ACS_FAIL_ADDR = crate::Reg<
    l1_icache0_acs_fail_addr::L1_ICACHE0_ACS_FAIL_ADDR_SPEC,
>;
///L1-ICache0 Access Fail Address information register
pub mod l1_icache0_acs_fail_addr;
/**L1_ICACHE1_ACS_FAIL_ID_ATTR (r) register accessor: L1-ICache0 Access Fail ID/attribution information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_acs_fail_id_attr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_acs_fail_id_attr`] module*/
pub type L1_ICACHE1_ACS_FAIL_ID_ATTR = crate::Reg<
    l1_icache1_acs_fail_id_attr::L1_ICACHE1_ACS_FAIL_ID_ATTR_SPEC,
>;
///L1-ICache0 Access Fail ID/attribution information register
pub mod l1_icache1_acs_fail_id_attr;
/**L1_ICACHE1_ACS_FAIL_ADDR (r) register accessor: L1-ICache0 Access Fail Address information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache1_acs_fail_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache1_acs_fail_addr`] module*/
pub type L1_ICACHE1_ACS_FAIL_ADDR = crate::Reg<
    l1_icache1_acs_fail_addr::L1_ICACHE1_ACS_FAIL_ADDR_SPEC,
>;
///L1-ICache0 Access Fail Address information register
pub mod l1_icache1_acs_fail_addr;
/**L1_ICACHE2_ACS_FAIL_ID_ATTR (r) register accessor: L1-ICache0 Access Fail ID/attribution information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_acs_fail_id_attr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_acs_fail_id_attr`] module*/
pub type L1_ICACHE2_ACS_FAIL_ID_ATTR = crate::Reg<
    l1_icache2_acs_fail_id_attr::L1_ICACHE2_ACS_FAIL_ID_ATTR_SPEC,
>;
///L1-ICache0 Access Fail ID/attribution information register
pub mod l1_icache2_acs_fail_id_attr;
/**L1_ICACHE2_ACS_FAIL_ADDR (r) register accessor: L1-ICache0 Access Fail Address information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache2_acs_fail_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache2_acs_fail_addr`] module*/
pub type L1_ICACHE2_ACS_FAIL_ADDR = crate::Reg<
    l1_icache2_acs_fail_addr::L1_ICACHE2_ACS_FAIL_ADDR_SPEC,
>;
///L1-ICache0 Access Fail Address information register
pub mod l1_icache2_acs_fail_addr;
/**L1_ICACHE3_ACS_FAIL_ID_ATTR (r) register accessor: L1-ICache0 Access Fail ID/attribution information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_acs_fail_id_attr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_acs_fail_id_attr`] module*/
pub type L1_ICACHE3_ACS_FAIL_ID_ATTR = crate::Reg<
    l1_icache3_acs_fail_id_attr::L1_ICACHE3_ACS_FAIL_ID_ATTR_SPEC,
>;
///L1-ICache0 Access Fail ID/attribution information register
pub mod l1_icache3_acs_fail_id_attr;
/**L1_ICACHE3_ACS_FAIL_ADDR (r) register accessor: L1-ICache0 Access Fail Address information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_icache3_acs_fail_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_icache3_acs_fail_addr`] module*/
pub type L1_ICACHE3_ACS_FAIL_ADDR = crate::Reg<
    l1_icache3_acs_fail_addr::L1_ICACHE3_ACS_FAIL_ADDR_SPEC,
>;
///L1-ICache0 Access Fail Address information register
pub mod l1_icache3_acs_fail_addr;
/**L1_DCACHE_ACS_FAIL_ID_ATTR (r) register accessor: L1-DCache Access Fail ID/attribution information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_acs_fail_id_attr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_acs_fail_id_attr`] module*/
pub type L1_DCACHE_ACS_FAIL_ID_ATTR = crate::Reg<
    l1_dcache_acs_fail_id_attr::L1_DCACHE_ACS_FAIL_ID_ATTR_SPEC,
>;
///L1-DCache Access Fail ID/attribution information register
pub mod l1_dcache_acs_fail_id_attr;
/**L1_DCACHE_ACS_FAIL_ADDR (r) register accessor: L1-DCache Access Fail Address information register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_dcache_acs_fail_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_dcache_acs_fail_addr`] module*/
pub type L1_DCACHE_ACS_FAIL_ADDR = crate::Reg<
    l1_dcache_acs_fail_addr::L1_DCACHE_ACS_FAIL_ADDR_SPEC,
>;
///L1-DCache Access Fail Address information register
pub mod l1_dcache_acs_fail_addr;
/**SYNC_L1_CACHE_PRELOAD_INT_ENA (rw) register accessor: L1-Cache Access Fail Interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_l1_cache_preload_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_l1_cache_preload_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync_l1_cache_preload_int_ena`] module*/
pub type SYNC_L1_CACHE_PRELOAD_INT_ENA = crate::Reg<
    sync_l1_cache_preload_int_ena::SYNC_L1_CACHE_PRELOAD_INT_ENA_SPEC,
>;
///L1-Cache Access Fail Interrupt enable register
pub mod sync_l1_cache_preload_int_ena;
/**SYNC_L1_CACHE_PRELOAD_INT_CLR (rw) register accessor: Sync Preload operation Interrupt clear register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_l1_cache_preload_int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_l1_cache_preload_int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync_l1_cache_preload_int_clr`] module*/
pub type SYNC_L1_CACHE_PRELOAD_INT_CLR = crate::Reg<
    sync_l1_cache_preload_int_clr::SYNC_L1_CACHE_PRELOAD_INT_CLR_SPEC,
>;
///Sync Preload operation Interrupt clear register
pub mod sync_l1_cache_preload_int_clr;
/**SYNC_L1_CACHE_PRELOAD_INT_RAW (rw) register accessor: Sync Preload operation Interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_l1_cache_preload_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync_l1_cache_preload_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync_l1_cache_preload_int_raw`] module*/
pub type SYNC_L1_CACHE_PRELOAD_INT_RAW = crate::Reg<
    sync_l1_cache_preload_int_raw::SYNC_L1_CACHE_PRELOAD_INT_RAW_SPEC,
>;
///Sync Preload operation Interrupt raw register
pub mod sync_l1_cache_preload_int_raw;
/**SYNC_L1_CACHE_PRELOAD_INT_ST (r) register accessor: L1-Cache Access Fail Interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_l1_cache_preload_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync_l1_cache_preload_int_st`] module*/
pub type SYNC_L1_CACHE_PRELOAD_INT_ST = crate::Reg<
    sync_l1_cache_preload_int_st::SYNC_L1_CACHE_PRELOAD_INT_ST_SPEC,
>;
///L1-Cache Access Fail Interrupt status register
pub mod sync_l1_cache_preload_int_st;
/**SYNC_L1_CACHE_PRELOAD_EXCEPTION (r) register accessor: Cache Sync/Preload Operation exception register

You can [`read`](crate::generic::Reg::read) this register and get [`sync_l1_cache_preload_exception::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sync_l1_cache_preload_exception`] module*/
pub type SYNC_L1_CACHE_PRELOAD_EXCEPTION = crate::Reg<
    sync_l1_cache_preload_exception::SYNC_L1_CACHE_PRELOAD_EXCEPTION_SPEC,
>;
///Cache Sync/Preload Operation exception register
pub mod sync_l1_cache_preload_exception;
/**L1_CACHE_SYNC_RST_CTRL (rw) register accessor: Cache Sync Reset control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_sync_rst_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_sync_rst_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_sync_rst_ctrl`] module*/
pub type L1_CACHE_SYNC_RST_CTRL = crate::Reg<
    l1_cache_sync_rst_ctrl::L1_CACHE_SYNC_RST_CTRL_SPEC,
>;
///Cache Sync Reset control register
pub mod l1_cache_sync_rst_ctrl;
/**L1_CACHE_PRELOAD_RST_CTRL (rw) register accessor: Cache Preload Reset control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_preload_rst_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_preload_rst_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_preload_rst_ctrl`] module*/
pub type L1_CACHE_PRELOAD_RST_CTRL = crate::Reg<
    l1_cache_preload_rst_ctrl::L1_CACHE_PRELOAD_RST_CTRL_SPEC,
>;
///Cache Preload Reset control register
pub mod l1_cache_preload_rst_ctrl;
/**L1_CACHE_AUTOLOAD_BUF_CLR_CTRL (rw) register accessor: Cache Autoload buffer clear control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_autoload_buf_clr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_autoload_buf_clr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_autoload_buf_clr_ctrl`] module*/
pub type L1_CACHE_AUTOLOAD_BUF_CLR_CTRL = crate::Reg<
    l1_cache_autoload_buf_clr_ctrl::L1_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC,
>;
///Cache Autoload buffer clear control register
pub mod l1_cache_autoload_buf_clr_ctrl;
/**L1_UNALLOCATE_BUFFER_CLEAR (rw) register accessor: Unallocate request buffer clear registers

You can [`read`](crate::generic::Reg::read) this register and get [`l1_unallocate_buffer_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_unallocate_buffer_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_unallocate_buffer_clear`] module*/
pub type L1_UNALLOCATE_BUFFER_CLEAR = crate::Reg<
    l1_unallocate_buffer_clear::L1_UNALLOCATE_BUFFER_CLEAR_SPEC,
>;
///Unallocate request buffer clear registers
pub mod l1_unallocate_buffer_clear;
/**L1_CACHE_OBJECT_CTRL (rw) register accessor: Cache Tag and Data memory Object control register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_object_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_object_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_object_ctrl`] module*/
pub type L1_CACHE_OBJECT_CTRL = crate::Reg<
    l1_cache_object_ctrl::L1_CACHE_OBJECT_CTRL_SPEC,
>;
///Cache Tag and Data memory Object control register
pub mod l1_cache_object_ctrl;
/**L1_CACHE_WAY_OBJECT (rw) register accessor: Cache Tag and Data memory way register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_way_object::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_way_object::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_way_object`] module*/
pub type L1_CACHE_WAY_OBJECT = crate::Reg<l1_cache_way_object::L1_CACHE_WAY_OBJECT_SPEC>;
///Cache Tag and Data memory way register
pub mod l1_cache_way_object;
/**L1_CACHE_VADDR (rw) register accessor: Cache Vaddr register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_vaddr`] module*/
pub type L1_CACHE_VADDR = crate::Reg<l1_cache_vaddr::L1_CACHE_VADDR_SPEC>;
///Cache Vaddr register
pub mod l1_cache_vaddr;
/**L1_CACHE_DEBUG_BUS (rw) register accessor: Cache Tag/data memory content register

You can [`read`](crate::generic::Reg::read) this register and get [`l1_cache_debug_bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_cache_debug_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l1_cache_debug_bus`] module*/
pub type L1_CACHE_DEBUG_BUS = crate::Reg<l1_cache_debug_bus::L1_CACHE_DEBUG_BUS_SPEC>;
///Cache Tag/data memory content register
pub mod l1_cache_debug_bus;
/**LEVEL_SPLIT0 (r) register accessor: USED TO SPLIT L1 CACHE AND L2 CACHE

You can [`read`](crate::generic::Reg::read) this register and get [`level_split0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@level_split0`] module*/
pub type LEVEL_SPLIT0 = crate::Reg<level_split0::LEVEL_SPLIT0_SPEC>;
///USED TO SPLIT L1 CACHE AND L2 CACHE
pub mod level_split0;
/**L2_CACHE_CTRL (rw) register accessor: L2 Cache(L2-Cache) control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_ctrl`] module*/
pub type L2_CACHE_CTRL = crate::Reg<l2_cache_ctrl::L2_CACHE_CTRL_SPEC>;
///L2 Cache(L2-Cache) control register
pub mod l2_cache_ctrl;
/**L2_BYPASS_CACHE_CONF (rw) register accessor: Bypass Cache configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_bypass_cache_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_bypass_cache_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_bypass_cache_conf`] module*/
pub type L2_BYPASS_CACHE_CONF = crate::Reg<
    l2_bypass_cache_conf::L2_BYPASS_CACHE_CONF_SPEC,
>;
///Bypass Cache configure register
pub mod l2_bypass_cache_conf;
/**L2_CACHE_CACHESIZE_CONF (rw) register accessor: L2 Cache CacheSize mode configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_cachesize_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_cachesize_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_cachesize_conf`] module*/
pub type L2_CACHE_CACHESIZE_CONF = crate::Reg<
    l2_cache_cachesize_conf::L2_CACHE_CACHESIZE_CONF_SPEC,
>;
///L2 Cache CacheSize mode configure register
pub mod l2_cache_cachesize_conf;
/**L2_CACHE_BLOCKSIZE_CONF (rw) register accessor: L2 Cache BlockSize mode configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_blocksize_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_blocksize_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_blocksize_conf`] module*/
pub type L2_CACHE_BLOCKSIZE_CONF = crate::Reg<
    l2_cache_blocksize_conf::L2_CACHE_BLOCKSIZE_CONF_SPEC,
>;
///L2 Cache BlockSize mode configure register
pub mod l2_cache_blocksize_conf;
/**L2_CACHE_WRAP_AROUND_CTRL (rw) register accessor: Cache wrap around control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_wrap_around_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_wrap_around_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_wrap_around_ctrl`] module*/
pub type L2_CACHE_WRAP_AROUND_CTRL = crate::Reg<
    l2_cache_wrap_around_ctrl::L2_CACHE_WRAP_AROUND_CTRL_SPEC,
>;
///Cache wrap around control register
pub mod l2_cache_wrap_around_ctrl;
/**L2_CACHE_TAG_MEM_POWER_CTRL (rw) register accessor: Cache tag memory power control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_tag_mem_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_tag_mem_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_tag_mem_power_ctrl`] module*/
pub type L2_CACHE_TAG_MEM_POWER_CTRL = crate::Reg<
    l2_cache_tag_mem_power_ctrl::L2_CACHE_TAG_MEM_POWER_CTRL_SPEC,
>;
///Cache tag memory power control register
pub mod l2_cache_tag_mem_power_ctrl;
/**L2_CACHE_DATA_MEM_POWER_CTRL (rw) register accessor: Cache data memory power control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_data_mem_power_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_data_mem_power_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_data_mem_power_ctrl`] module*/
pub type L2_CACHE_DATA_MEM_POWER_CTRL = crate::Reg<
    l2_cache_data_mem_power_ctrl::L2_CACHE_DATA_MEM_POWER_CTRL_SPEC,
>;
///Cache data memory power control register
pub mod l2_cache_data_mem_power_ctrl;
/**L2_CACHE_FREEZE_CTRL (rw) register accessor: Cache Freeze control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_freeze_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_freeze_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_freeze_ctrl`] module*/
pub type L2_CACHE_FREEZE_CTRL = crate::Reg<
    l2_cache_freeze_ctrl::L2_CACHE_FREEZE_CTRL_SPEC,
>;
///Cache Freeze control register
pub mod l2_cache_freeze_ctrl;
/**L2_CACHE_DATA_MEM_ACS_CONF (rw) register accessor: Cache data memory access configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_data_mem_acs_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_data_mem_acs_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_data_mem_acs_conf`] module*/
pub type L2_CACHE_DATA_MEM_ACS_CONF = crate::Reg<
    l2_cache_data_mem_acs_conf::L2_CACHE_DATA_MEM_ACS_CONF_SPEC,
>;
///Cache data memory access configure register
pub mod l2_cache_data_mem_acs_conf;
/**L2_CACHE_TAG_MEM_ACS_CONF (rw) register accessor: Cache tag memory access configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_tag_mem_acs_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_tag_mem_acs_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_tag_mem_acs_conf`] module*/
pub type L2_CACHE_TAG_MEM_ACS_CONF = crate::Reg<
    l2_cache_tag_mem_acs_conf::L2_CACHE_TAG_MEM_ACS_CONF_SPEC,
>;
///Cache tag memory access configure register
pub mod l2_cache_tag_mem_acs_conf;
/**L2_CACHE_PRELOCK_CONF (rw) register accessor: L2 Cache prelock configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_prelock_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_prelock_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_prelock_conf`] module*/
pub type L2_CACHE_PRELOCK_CONF = crate::Reg<
    l2_cache_prelock_conf::L2_CACHE_PRELOCK_CONF_SPEC,
>;
///L2 Cache prelock configure register
pub mod l2_cache_prelock_conf;
/**L2_CACHE_PRELOCK_SCT0_ADDR (rw) register accessor: L2 Cache prelock section0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_prelock_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_prelock_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_prelock_sct0_addr`] module*/
pub type L2_CACHE_PRELOCK_SCT0_ADDR = crate::Reg<
    l2_cache_prelock_sct0_addr::L2_CACHE_PRELOCK_SCT0_ADDR_SPEC,
>;
///L2 Cache prelock section0 address configure register
pub mod l2_cache_prelock_sct0_addr;
/**L2_CACHE_PRELOCK_SCT1_ADDR (rw) register accessor: L2 Cache prelock section1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_prelock_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_prelock_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_prelock_sct1_addr`] module*/
pub type L2_CACHE_PRELOCK_SCT1_ADDR = crate::Reg<
    l2_cache_prelock_sct1_addr::L2_CACHE_PRELOCK_SCT1_ADDR_SPEC,
>;
///L2 Cache prelock section1 address configure register
pub mod l2_cache_prelock_sct1_addr;
/**L2_CACHE_PRELOCK_SCT_SIZE (rw) register accessor: L2 Cache prelock section size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_prelock_sct_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_prelock_sct_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_prelock_sct_size`] module*/
pub type L2_CACHE_PRELOCK_SCT_SIZE = crate::Reg<
    l2_cache_prelock_sct_size::L2_CACHE_PRELOCK_SCT_SIZE_SPEC,
>;
///L2 Cache prelock section size configure register
pub mod l2_cache_prelock_sct_size;
/**L2_CACHE_PRELOAD_CTRL (rw) register accessor: L2 Cache preload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_preload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_preload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_preload_ctrl`] module*/
pub type L2_CACHE_PRELOAD_CTRL = crate::Reg<
    l2_cache_preload_ctrl::L2_CACHE_PRELOAD_CTRL_SPEC,
>;
///L2 Cache preload-operation control register
pub mod l2_cache_preload_ctrl;
/**L2_CACHE_PRELOAD_ADDR (rw) register accessor: L2 Cache preload address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_preload_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_preload_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_preload_addr`] module*/
pub type L2_CACHE_PRELOAD_ADDR = crate::Reg<
    l2_cache_preload_addr::L2_CACHE_PRELOAD_ADDR_SPEC,
>;
///L2 Cache preload address configure register
pub mod l2_cache_preload_addr;
/**L2_CACHE_PRELOAD_SIZE (rw) register accessor: L2 Cache preload size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_preload_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_preload_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_preload_size`] module*/
pub type L2_CACHE_PRELOAD_SIZE = crate::Reg<
    l2_cache_preload_size::L2_CACHE_PRELOAD_SIZE_SPEC,
>;
///L2 Cache preload size configure register
pub mod l2_cache_preload_size;
/**L2_CACHE_AUTOLOAD_CTRL (rw) register accessor: L2 Cache autoload-operation control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_ctrl`] module*/
pub type L2_CACHE_AUTOLOAD_CTRL = crate::Reg<
    l2_cache_autoload_ctrl::L2_CACHE_AUTOLOAD_CTRL_SPEC,
>;
///L2 Cache autoload-operation control register
pub mod l2_cache_autoload_ctrl;
/**L2_CACHE_AUTOLOAD_SCT0_ADDR (rw) register accessor: L2 Cache autoload section 0 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct0_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_sct0_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_sct0_addr`] module*/
pub type L2_CACHE_AUTOLOAD_SCT0_ADDR = crate::Reg<
    l2_cache_autoload_sct0_addr::L2_CACHE_AUTOLOAD_SCT0_ADDR_SPEC,
>;
///L2 Cache autoload section 0 address configure register
pub mod l2_cache_autoload_sct0_addr;
/**L2_CACHE_AUTOLOAD_SCT0_SIZE (rw) register accessor: L2 Cache autoload section 0 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct0_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_sct0_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_sct0_size`] module*/
pub type L2_CACHE_AUTOLOAD_SCT0_SIZE = crate::Reg<
    l2_cache_autoload_sct0_size::L2_CACHE_AUTOLOAD_SCT0_SIZE_SPEC,
>;
///L2 Cache autoload section 0 size configure register
pub mod l2_cache_autoload_sct0_size;
/**L2_CACHE_AUTOLOAD_SCT1_ADDR (rw) register accessor: L2 Cache autoload section 1 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct1_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_sct1_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_sct1_addr`] module*/
pub type L2_CACHE_AUTOLOAD_SCT1_ADDR = crate::Reg<
    l2_cache_autoload_sct1_addr::L2_CACHE_AUTOLOAD_SCT1_ADDR_SPEC,
>;
///L2 Cache autoload section 1 address configure register
pub mod l2_cache_autoload_sct1_addr;
/**L2_CACHE_AUTOLOAD_SCT1_SIZE (rw) register accessor: L2 Cache autoload section 1 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct1_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_sct1_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_sct1_size`] module*/
pub type L2_CACHE_AUTOLOAD_SCT1_SIZE = crate::Reg<
    l2_cache_autoload_sct1_size::L2_CACHE_AUTOLOAD_SCT1_SIZE_SPEC,
>;
///L2 Cache autoload section 1 size configure register
pub mod l2_cache_autoload_sct1_size;
/**L2_CACHE_AUTOLOAD_SCT2_ADDR (rw) register accessor: L2 Cache autoload section 2 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct2_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_sct2_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_sct2_addr`] module*/
pub type L2_CACHE_AUTOLOAD_SCT2_ADDR = crate::Reg<
    l2_cache_autoload_sct2_addr::L2_CACHE_AUTOLOAD_SCT2_ADDR_SPEC,
>;
///L2 Cache autoload section 2 address configure register
pub mod l2_cache_autoload_sct2_addr;
/**L2_CACHE_AUTOLOAD_SCT2_SIZE (rw) register accessor: L2 Cache autoload section 2 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct2_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_sct2_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_sct2_size`] module*/
pub type L2_CACHE_AUTOLOAD_SCT2_SIZE = crate::Reg<
    l2_cache_autoload_sct2_size::L2_CACHE_AUTOLOAD_SCT2_SIZE_SPEC,
>;
///L2 Cache autoload section 2 size configure register
pub mod l2_cache_autoload_sct2_size;
/**L2_CACHE_AUTOLOAD_SCT3_ADDR (rw) register accessor: L2 Cache autoload section 3 address configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct3_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_sct3_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_sct3_addr`] module*/
pub type L2_CACHE_AUTOLOAD_SCT3_ADDR = crate::Reg<
    l2_cache_autoload_sct3_addr::L2_CACHE_AUTOLOAD_SCT3_ADDR_SPEC,
>;
///L2 Cache autoload section 3 address configure register
pub mod l2_cache_autoload_sct3_addr;
/**L2_CACHE_AUTOLOAD_SCT3_SIZE (rw) register accessor: L2 Cache autoload section 3 size configure register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_sct3_size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_sct3_size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_sct3_size`] module*/
pub type L2_CACHE_AUTOLOAD_SCT3_SIZE = crate::Reg<
    l2_cache_autoload_sct3_size::L2_CACHE_AUTOLOAD_SCT3_SIZE_SPEC,
>;
///L2 Cache autoload section 3 size configure register
pub mod l2_cache_autoload_sct3_size;
/**L2_CACHE_ACS_CNT_INT_ENA (rw) register accessor: Cache Access Counter Interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_cnt_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_cnt_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_cnt_int_ena`] module*/
pub type L2_CACHE_ACS_CNT_INT_ENA = crate::Reg<
    l2_cache_acs_cnt_int_ena::L2_CACHE_ACS_CNT_INT_ENA_SPEC,
>;
///Cache Access Counter Interrupt enable register
pub mod l2_cache_acs_cnt_int_ena;
/**L2_CACHE_ACS_CNT_INT_CLR (rw) register accessor: Cache Access Counter Interrupt clear register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_cnt_int_clr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_cnt_int_clr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_cnt_int_clr`] module*/
pub type L2_CACHE_ACS_CNT_INT_CLR = crate::Reg<
    l2_cache_acs_cnt_int_clr::L2_CACHE_ACS_CNT_INT_CLR_SPEC,
>;
///Cache Access Counter Interrupt clear register
pub mod l2_cache_acs_cnt_int_clr;
/**L2_CACHE_ACS_CNT_INT_RAW (rw) register accessor: Cache Access Counter Interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_cnt_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_cnt_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_cnt_int_raw`] module*/
pub type L2_CACHE_ACS_CNT_INT_RAW = crate::Reg<
    l2_cache_acs_cnt_int_raw::L2_CACHE_ACS_CNT_INT_RAW_SPEC,
>;
///Cache Access Counter Interrupt raw register
pub mod l2_cache_acs_cnt_int_raw;
/**L2_CACHE_ACS_CNT_INT_ST (r) register accessor: Cache Access Counter Interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_cnt_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_cnt_int_st`] module*/
pub type L2_CACHE_ACS_CNT_INT_ST = crate::Reg<
    l2_cache_acs_cnt_int_st::L2_CACHE_ACS_CNT_INT_ST_SPEC,
>;
///Cache Access Counter Interrupt status register
pub mod l2_cache_acs_cnt_int_st;
/**L2_CACHE_ACS_FAIL_CTRL (rw) register accessor: Cache Access Fail Configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_fail_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_fail_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_fail_ctrl`] module*/
pub type L2_CACHE_ACS_FAIL_CTRL = crate::Reg<
    l2_cache_acs_fail_ctrl::L2_CACHE_ACS_FAIL_CTRL_SPEC,
>;
///Cache Access Fail Configuration register
pub mod l2_cache_acs_fail_ctrl;
/**L2_CACHE_ACS_FAIL_INT_ENA (rw) register accessor: Cache Access Fail Interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_fail_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_fail_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_fail_int_ena`] module*/
pub type L2_CACHE_ACS_FAIL_INT_ENA = crate::Reg<
    l2_cache_acs_fail_int_ena::L2_CACHE_ACS_FAIL_INT_ENA_SPEC,
>;
///Cache Access Fail Interrupt enable register
pub mod l2_cache_acs_fail_int_ena;
/**L2_CACHE_ACS_FAIL_INT_CLR (w) register accessor: L1-Cache Access Fail Interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_fail_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_fail_int_clr`] module*/
pub type L2_CACHE_ACS_FAIL_INT_CLR = crate::Reg<
    l2_cache_acs_fail_int_clr::L2_CACHE_ACS_FAIL_INT_CLR_SPEC,
>;
///L1-Cache Access Fail Interrupt clear register
pub mod l2_cache_acs_fail_int_clr;
/**L2_CACHE_ACS_FAIL_INT_RAW (rw) register accessor: Cache Access Fail Interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_fail_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_fail_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_fail_int_raw`] module*/
pub type L2_CACHE_ACS_FAIL_INT_RAW = crate::Reg<
    l2_cache_acs_fail_int_raw::L2_CACHE_ACS_FAIL_INT_RAW_SPEC,
>;
///Cache Access Fail Interrupt raw register
pub mod l2_cache_acs_fail_int_raw;
/**L2_CACHE_ACS_FAIL_INT_ST (r) register accessor: Cache Access Fail Interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_fail_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_fail_int_st`] module*/
pub type L2_CACHE_ACS_FAIL_INT_ST = crate::Reg<
    l2_cache_acs_fail_int_st::L2_CACHE_ACS_FAIL_INT_ST_SPEC,
>;
///Cache Access Fail Interrupt status register
pub mod l2_cache_acs_fail_int_st;
/**L2_CACHE_ACS_CNT_CTRL (rw) register accessor: Cache Access Counter enable and clear register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_cnt_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_acs_cnt_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_cnt_ctrl`] module*/
pub type L2_CACHE_ACS_CNT_CTRL = crate::Reg<
    l2_cache_acs_cnt_ctrl::L2_CACHE_ACS_CNT_CTRL_SPEC,
>;
///Cache Access Counter enable and clear register
pub mod l2_cache_acs_cnt_ctrl;
/**L2_IBUS0_ACS_HIT_CNT (r) register accessor: L2-Cache bus0 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus0_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus0_acs_hit_cnt`] module*/
pub type L2_IBUS0_ACS_HIT_CNT = crate::Reg<
    l2_ibus0_acs_hit_cnt::L2_IBUS0_ACS_HIT_CNT_SPEC,
>;
///L2-Cache bus0 Hit-Access Counter register
pub mod l2_ibus0_acs_hit_cnt;
/**L2_IBUS0_ACS_MISS_CNT (r) register accessor: L2-Cache bus0 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus0_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus0_acs_miss_cnt`] module*/
pub type L2_IBUS0_ACS_MISS_CNT = crate::Reg<
    l2_ibus0_acs_miss_cnt::L2_IBUS0_ACS_MISS_CNT_SPEC,
>;
///L2-Cache bus0 Miss-Access Counter register
pub mod l2_ibus0_acs_miss_cnt;
/**L2_IBUS0_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus0 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus0_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus0_acs_conflict_cnt`] module*/
pub type L2_IBUS0_ACS_CONFLICT_CNT = crate::Reg<
    l2_ibus0_acs_conflict_cnt::L2_IBUS0_ACS_CONFLICT_CNT_SPEC,
>;
///L2-Cache bus0 Conflict-Access Counter register
pub mod l2_ibus0_acs_conflict_cnt;
/**L2_IBUS0_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus0 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus0_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus0_acs_nxtlvl_rd_cnt`] module*/
pub type L2_IBUS0_ACS_NXTLVL_RD_CNT = crate::Reg<
    l2_ibus0_acs_nxtlvl_rd_cnt::L2_IBUS0_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L2-Cache bus0 Next-Level-Access Counter register
pub mod l2_ibus0_acs_nxtlvl_rd_cnt;
/**L2_IBUS1_ACS_HIT_CNT (r) register accessor: L2-Cache bus1 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus1_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus1_acs_hit_cnt`] module*/
pub type L2_IBUS1_ACS_HIT_CNT = crate::Reg<
    l2_ibus1_acs_hit_cnt::L2_IBUS1_ACS_HIT_CNT_SPEC,
>;
///L2-Cache bus1 Hit-Access Counter register
pub mod l2_ibus1_acs_hit_cnt;
/**L2_IBUS1_ACS_MISS_CNT (r) register accessor: L2-Cache bus1 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus1_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus1_acs_miss_cnt`] module*/
pub type L2_IBUS1_ACS_MISS_CNT = crate::Reg<
    l2_ibus1_acs_miss_cnt::L2_IBUS1_ACS_MISS_CNT_SPEC,
>;
///L2-Cache bus1 Miss-Access Counter register
pub mod l2_ibus1_acs_miss_cnt;
/**L2_IBUS1_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus1 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus1_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus1_acs_conflict_cnt`] module*/
pub type L2_IBUS1_ACS_CONFLICT_CNT = crate::Reg<
    l2_ibus1_acs_conflict_cnt::L2_IBUS1_ACS_CONFLICT_CNT_SPEC,
>;
///L2-Cache bus1 Conflict-Access Counter register
pub mod l2_ibus1_acs_conflict_cnt;
/**L2_IBUS1_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus1 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus1_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus1_acs_nxtlvl_rd_cnt`] module*/
pub type L2_IBUS1_ACS_NXTLVL_RD_CNT = crate::Reg<
    l2_ibus1_acs_nxtlvl_rd_cnt::L2_IBUS1_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L2-Cache bus1 Next-Level-Access Counter register
pub mod l2_ibus1_acs_nxtlvl_rd_cnt;
/**L2_IBUS2_ACS_HIT_CNT (r) register accessor: L2-Cache bus2 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus2_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus2_acs_hit_cnt`] module*/
pub type L2_IBUS2_ACS_HIT_CNT = crate::Reg<
    l2_ibus2_acs_hit_cnt::L2_IBUS2_ACS_HIT_CNT_SPEC,
>;
///L2-Cache bus2 Hit-Access Counter register
pub mod l2_ibus2_acs_hit_cnt;
/**L2_IBUS2_ACS_MISS_CNT (r) register accessor: L2-Cache bus2 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus2_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus2_acs_miss_cnt`] module*/
pub type L2_IBUS2_ACS_MISS_CNT = crate::Reg<
    l2_ibus2_acs_miss_cnt::L2_IBUS2_ACS_MISS_CNT_SPEC,
>;
///L2-Cache bus2 Miss-Access Counter register
pub mod l2_ibus2_acs_miss_cnt;
/**L2_IBUS2_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus2 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus2_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus2_acs_conflict_cnt`] module*/
pub type L2_IBUS2_ACS_CONFLICT_CNT = crate::Reg<
    l2_ibus2_acs_conflict_cnt::L2_IBUS2_ACS_CONFLICT_CNT_SPEC,
>;
///L2-Cache bus2 Conflict-Access Counter register
pub mod l2_ibus2_acs_conflict_cnt;
/**L2_IBUS2_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus2 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus2_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus2_acs_nxtlvl_rd_cnt`] module*/
pub type L2_IBUS2_ACS_NXTLVL_RD_CNT = crate::Reg<
    l2_ibus2_acs_nxtlvl_rd_cnt::L2_IBUS2_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L2-Cache bus2 Next-Level-Access Counter register
pub mod l2_ibus2_acs_nxtlvl_rd_cnt;
/**L2_IBUS3_ACS_HIT_CNT (r) register accessor: L2-Cache bus3 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus3_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus3_acs_hit_cnt`] module*/
pub type L2_IBUS3_ACS_HIT_CNT = crate::Reg<
    l2_ibus3_acs_hit_cnt::L2_IBUS3_ACS_HIT_CNT_SPEC,
>;
///L2-Cache bus3 Hit-Access Counter register
pub mod l2_ibus3_acs_hit_cnt;
/**L2_IBUS3_ACS_MISS_CNT (r) register accessor: L2-Cache bus3 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus3_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus3_acs_miss_cnt`] module*/
pub type L2_IBUS3_ACS_MISS_CNT = crate::Reg<
    l2_ibus3_acs_miss_cnt::L2_IBUS3_ACS_MISS_CNT_SPEC,
>;
///L2-Cache bus3 Miss-Access Counter register
pub mod l2_ibus3_acs_miss_cnt;
/**L2_IBUS3_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus3 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus3_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus3_acs_conflict_cnt`] module*/
pub type L2_IBUS3_ACS_CONFLICT_CNT = crate::Reg<
    l2_ibus3_acs_conflict_cnt::L2_IBUS3_ACS_CONFLICT_CNT_SPEC,
>;
///L2-Cache bus3 Conflict-Access Counter register
pub mod l2_ibus3_acs_conflict_cnt;
/**L2_IBUS3_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus3 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_ibus3_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_ibus3_acs_nxtlvl_rd_cnt`] module*/
pub type L2_IBUS3_ACS_NXTLVL_RD_CNT = crate::Reg<
    l2_ibus3_acs_nxtlvl_rd_cnt::L2_IBUS3_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L2-Cache bus3 Next-Level-Access Counter register
pub mod l2_ibus3_acs_nxtlvl_rd_cnt;
/**L2_DBUS0_ACS_HIT_CNT (r) register accessor: L2-Cache bus0 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus0_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus0_acs_hit_cnt`] module*/
pub type L2_DBUS0_ACS_HIT_CNT = crate::Reg<
    l2_dbus0_acs_hit_cnt::L2_DBUS0_ACS_HIT_CNT_SPEC,
>;
///L2-Cache bus0 Hit-Access Counter register
pub mod l2_dbus0_acs_hit_cnt;
/**L2_DBUS0_ACS_MISS_CNT (r) register accessor: L2-Cache bus0 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus0_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus0_acs_miss_cnt`] module*/
pub type L2_DBUS0_ACS_MISS_CNT = crate::Reg<
    l2_dbus0_acs_miss_cnt::L2_DBUS0_ACS_MISS_CNT_SPEC,
>;
///L2-Cache bus0 Miss-Access Counter register
pub mod l2_dbus0_acs_miss_cnt;
/**L2_DBUS0_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus0 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus0_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus0_acs_conflict_cnt`] module*/
pub type L2_DBUS0_ACS_CONFLICT_CNT = crate::Reg<
    l2_dbus0_acs_conflict_cnt::L2_DBUS0_ACS_CONFLICT_CNT_SPEC,
>;
///L2-Cache bus0 Conflict-Access Counter register
pub mod l2_dbus0_acs_conflict_cnt;
/**L2_DBUS0_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus0 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus0_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus0_acs_nxtlvl_rd_cnt`] module*/
pub type L2_DBUS0_ACS_NXTLVL_RD_CNT = crate::Reg<
    l2_dbus0_acs_nxtlvl_rd_cnt::L2_DBUS0_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L2-Cache bus0 Next-Level-Access Counter register
pub mod l2_dbus0_acs_nxtlvl_rd_cnt;
/**L2_DBUS0_ACS_NXTLVL_WR_CNT (r) register accessor: L2-Cache bus0 WB-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus0_acs_nxtlvl_wr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus0_acs_nxtlvl_wr_cnt`] module*/
pub type L2_DBUS0_ACS_NXTLVL_WR_CNT = crate::Reg<
    l2_dbus0_acs_nxtlvl_wr_cnt::L2_DBUS0_ACS_NXTLVL_WR_CNT_SPEC,
>;
///L2-Cache bus0 WB-Access Counter register
pub mod l2_dbus0_acs_nxtlvl_wr_cnt;
/**L2_DBUS1_ACS_HIT_CNT (r) register accessor: L2-Cache bus1 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus1_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus1_acs_hit_cnt`] module*/
pub type L2_DBUS1_ACS_HIT_CNT = crate::Reg<
    l2_dbus1_acs_hit_cnt::L2_DBUS1_ACS_HIT_CNT_SPEC,
>;
///L2-Cache bus1 Hit-Access Counter register
pub mod l2_dbus1_acs_hit_cnt;
/**L2_DBUS1_ACS_MISS_CNT (r) register accessor: L2-Cache bus1 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus1_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus1_acs_miss_cnt`] module*/
pub type L2_DBUS1_ACS_MISS_CNT = crate::Reg<
    l2_dbus1_acs_miss_cnt::L2_DBUS1_ACS_MISS_CNT_SPEC,
>;
///L2-Cache bus1 Miss-Access Counter register
pub mod l2_dbus1_acs_miss_cnt;
/**L2_DBUS1_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus1 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus1_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus1_acs_conflict_cnt`] module*/
pub type L2_DBUS1_ACS_CONFLICT_CNT = crate::Reg<
    l2_dbus1_acs_conflict_cnt::L2_DBUS1_ACS_CONFLICT_CNT_SPEC,
>;
///L2-Cache bus1 Conflict-Access Counter register
pub mod l2_dbus1_acs_conflict_cnt;
/**L2_DBUS1_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus1 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus1_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus1_acs_nxtlvl_rd_cnt`] module*/
pub type L2_DBUS1_ACS_NXTLVL_RD_CNT = crate::Reg<
    l2_dbus1_acs_nxtlvl_rd_cnt::L2_DBUS1_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L2-Cache bus1 Next-Level-Access Counter register
pub mod l2_dbus1_acs_nxtlvl_rd_cnt;
/**L2_DBUS1_ACS_NXTLVL_WR_CNT (r) register accessor: L2-Cache bus1 WB-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus1_acs_nxtlvl_wr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus1_acs_nxtlvl_wr_cnt`] module*/
pub type L2_DBUS1_ACS_NXTLVL_WR_CNT = crate::Reg<
    l2_dbus1_acs_nxtlvl_wr_cnt::L2_DBUS1_ACS_NXTLVL_WR_CNT_SPEC,
>;
///L2-Cache bus1 WB-Access Counter register
pub mod l2_dbus1_acs_nxtlvl_wr_cnt;
/**L2_DBUS2_ACS_HIT_CNT (r) register accessor: L2-Cache bus2 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus2_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus2_acs_hit_cnt`] module*/
pub type L2_DBUS2_ACS_HIT_CNT = crate::Reg<
    l2_dbus2_acs_hit_cnt::L2_DBUS2_ACS_HIT_CNT_SPEC,
>;
///L2-Cache bus2 Hit-Access Counter register
pub mod l2_dbus2_acs_hit_cnt;
/**L2_DBUS2_ACS_MISS_CNT (r) register accessor: L2-Cache bus2 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus2_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus2_acs_miss_cnt`] module*/
pub type L2_DBUS2_ACS_MISS_CNT = crate::Reg<
    l2_dbus2_acs_miss_cnt::L2_DBUS2_ACS_MISS_CNT_SPEC,
>;
///L2-Cache bus2 Miss-Access Counter register
pub mod l2_dbus2_acs_miss_cnt;
/**L2_DBUS2_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus2 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus2_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus2_acs_conflict_cnt`] module*/
pub type L2_DBUS2_ACS_CONFLICT_CNT = crate::Reg<
    l2_dbus2_acs_conflict_cnt::L2_DBUS2_ACS_CONFLICT_CNT_SPEC,
>;
///L2-Cache bus2 Conflict-Access Counter register
pub mod l2_dbus2_acs_conflict_cnt;
/**L2_DBUS2_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus2 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus2_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus2_acs_nxtlvl_rd_cnt`] module*/
pub type L2_DBUS2_ACS_NXTLVL_RD_CNT = crate::Reg<
    l2_dbus2_acs_nxtlvl_rd_cnt::L2_DBUS2_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L2-Cache bus2 Next-Level-Access Counter register
pub mod l2_dbus2_acs_nxtlvl_rd_cnt;
/**L2_DBUS2_ACS_NXTLVL_WR_CNT (r) register accessor: L2-Cache bus2 WB-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus2_acs_nxtlvl_wr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus2_acs_nxtlvl_wr_cnt`] module*/
pub type L2_DBUS2_ACS_NXTLVL_WR_CNT = crate::Reg<
    l2_dbus2_acs_nxtlvl_wr_cnt::L2_DBUS2_ACS_NXTLVL_WR_CNT_SPEC,
>;
///L2-Cache bus2 WB-Access Counter register
pub mod l2_dbus2_acs_nxtlvl_wr_cnt;
/**L2_DBUS3_ACS_HIT_CNT (r) register accessor: L2-Cache bus3 Hit-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus3_acs_hit_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus3_acs_hit_cnt`] module*/
pub type L2_DBUS3_ACS_HIT_CNT = crate::Reg<
    l2_dbus3_acs_hit_cnt::L2_DBUS3_ACS_HIT_CNT_SPEC,
>;
///L2-Cache bus3 Hit-Access Counter register
pub mod l2_dbus3_acs_hit_cnt;
/**L2_DBUS3_ACS_MISS_CNT (r) register accessor: L2-Cache bus3 Miss-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus3_acs_miss_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus3_acs_miss_cnt`] module*/
pub type L2_DBUS3_ACS_MISS_CNT = crate::Reg<
    l2_dbus3_acs_miss_cnt::L2_DBUS3_ACS_MISS_CNT_SPEC,
>;
///L2-Cache bus3 Miss-Access Counter register
pub mod l2_dbus3_acs_miss_cnt;
/**L2_DBUS3_ACS_CONFLICT_CNT (r) register accessor: L2-Cache bus3 Conflict-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus3_acs_conflict_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus3_acs_conflict_cnt`] module*/
pub type L2_DBUS3_ACS_CONFLICT_CNT = crate::Reg<
    l2_dbus3_acs_conflict_cnt::L2_DBUS3_ACS_CONFLICT_CNT_SPEC,
>;
///L2-Cache bus3 Conflict-Access Counter register
pub mod l2_dbus3_acs_conflict_cnt;
/**L2_DBUS3_ACS_NXTLVL_RD_CNT (r) register accessor: L2-Cache bus3 Next-Level-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus3_acs_nxtlvl_rd_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus3_acs_nxtlvl_rd_cnt`] module*/
pub type L2_DBUS3_ACS_NXTLVL_RD_CNT = crate::Reg<
    l2_dbus3_acs_nxtlvl_rd_cnt::L2_DBUS3_ACS_NXTLVL_RD_CNT_SPEC,
>;
///L2-Cache bus3 Next-Level-Access Counter register
pub mod l2_dbus3_acs_nxtlvl_rd_cnt;
/**L2_DBUS3_ACS_NXTLVL_WR_CNT (r) register accessor: L2-Cache bus3 WB-Access Counter register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_dbus3_acs_nxtlvl_wr_cnt::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_dbus3_acs_nxtlvl_wr_cnt`] module*/
pub type L2_DBUS3_ACS_NXTLVL_WR_CNT = crate::Reg<
    l2_dbus3_acs_nxtlvl_wr_cnt::L2_DBUS3_ACS_NXTLVL_WR_CNT_SPEC,
>;
///L2-Cache bus3 WB-Access Counter register
pub mod l2_dbus3_acs_nxtlvl_wr_cnt;
/**L2_CACHE_ACS_FAIL_ID_ATTR (r) register accessor: L2-Cache Access Fail ID/attribution information register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_fail_id_attr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_fail_id_attr`] module*/
pub type L2_CACHE_ACS_FAIL_ID_ATTR = crate::Reg<
    l2_cache_acs_fail_id_attr::L2_CACHE_ACS_FAIL_ID_ATTR_SPEC,
>;
///L2-Cache Access Fail ID/attribution information register
pub mod l2_cache_acs_fail_id_attr;
/**L2_CACHE_ACS_FAIL_ADDR (r) register accessor: L2-Cache Access Fail Address information register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_acs_fail_addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_acs_fail_addr`] module*/
pub type L2_CACHE_ACS_FAIL_ADDR = crate::Reg<
    l2_cache_acs_fail_addr::L2_CACHE_ACS_FAIL_ADDR_SPEC,
>;
///L2-Cache Access Fail Address information register
pub mod l2_cache_acs_fail_addr;
/**L2_CACHE_SYNC_PRELOAD_INT_ENA (rw) register accessor: L1-Cache Access Fail Interrupt enable register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_sync_preload_int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_sync_preload_int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_sync_preload_int_ena`] module*/
pub type L2_CACHE_SYNC_PRELOAD_INT_ENA = crate::Reg<
    l2_cache_sync_preload_int_ena::L2_CACHE_SYNC_PRELOAD_INT_ENA_SPEC,
>;
///L1-Cache Access Fail Interrupt enable register
pub mod l2_cache_sync_preload_int_ena;
/**L2_CACHE_SYNC_PRELOAD_INT_CLR (w) register accessor: Sync Preload operation Interrupt clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_sync_preload_int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_sync_preload_int_clr`] module*/
pub type L2_CACHE_SYNC_PRELOAD_INT_CLR = crate::Reg<
    l2_cache_sync_preload_int_clr::L2_CACHE_SYNC_PRELOAD_INT_CLR_SPEC,
>;
///Sync Preload operation Interrupt clear register
pub mod l2_cache_sync_preload_int_clr;
/**L2_CACHE_SYNC_PRELOAD_INT_RAW (rw) register accessor: Sync Preload operation Interrupt raw register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_sync_preload_int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_sync_preload_int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_sync_preload_int_raw`] module*/
pub type L2_CACHE_SYNC_PRELOAD_INT_RAW = crate::Reg<
    l2_cache_sync_preload_int_raw::L2_CACHE_SYNC_PRELOAD_INT_RAW_SPEC,
>;
///Sync Preload operation Interrupt raw register
pub mod l2_cache_sync_preload_int_raw;
/**L2_CACHE_SYNC_PRELOAD_INT_ST (r) register accessor: L1-Cache Access Fail Interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_sync_preload_int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_sync_preload_int_st`] module*/
pub type L2_CACHE_SYNC_PRELOAD_INT_ST = crate::Reg<
    l2_cache_sync_preload_int_st::L2_CACHE_SYNC_PRELOAD_INT_ST_SPEC,
>;
///L1-Cache Access Fail Interrupt status register
pub mod l2_cache_sync_preload_int_st;
/**L2_CACHE_SYNC_PRELOAD_EXCEPTION (r) register accessor: Cache Sync/Preload Operation exception register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_sync_preload_exception::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_sync_preload_exception`] module*/
pub type L2_CACHE_SYNC_PRELOAD_EXCEPTION = crate::Reg<
    l2_cache_sync_preload_exception::L2_CACHE_SYNC_PRELOAD_EXCEPTION_SPEC,
>;
///Cache Sync/Preload Operation exception register
pub mod l2_cache_sync_preload_exception;
/**L2_CACHE_SYNC_RST_CTRL (rw) register accessor: Cache Sync Reset control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_sync_rst_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_sync_rst_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_sync_rst_ctrl`] module*/
pub type L2_CACHE_SYNC_RST_CTRL = crate::Reg<
    l2_cache_sync_rst_ctrl::L2_CACHE_SYNC_RST_CTRL_SPEC,
>;
///Cache Sync Reset control register
pub mod l2_cache_sync_rst_ctrl;
/**L2_CACHE_PRELOAD_RST_CTRL (rw) register accessor: Cache Preload Reset control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_preload_rst_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_preload_rst_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_preload_rst_ctrl`] module*/
pub type L2_CACHE_PRELOAD_RST_CTRL = crate::Reg<
    l2_cache_preload_rst_ctrl::L2_CACHE_PRELOAD_RST_CTRL_SPEC,
>;
///Cache Preload Reset control register
pub mod l2_cache_preload_rst_ctrl;
/**L2_CACHE_AUTOLOAD_BUF_CLR_CTRL (rw) register accessor: Cache Autoload buffer clear control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_autoload_buf_clr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_autoload_buf_clr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_autoload_buf_clr_ctrl`] module*/
pub type L2_CACHE_AUTOLOAD_BUF_CLR_CTRL = crate::Reg<
    l2_cache_autoload_buf_clr_ctrl::L2_CACHE_AUTOLOAD_BUF_CLR_CTRL_SPEC,
>;
///Cache Autoload buffer clear control register
pub mod l2_cache_autoload_buf_clr_ctrl;
/**L2_UNALLOCATE_BUFFER_CLEAR (rw) register accessor: Unallocate request buffer clear registers

You can [`read`](crate::generic::Reg::read) this register and get [`l2_unallocate_buffer_clear::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_unallocate_buffer_clear::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_unallocate_buffer_clear`] module*/
pub type L2_UNALLOCATE_BUFFER_CLEAR = crate::Reg<
    l2_unallocate_buffer_clear::L2_UNALLOCATE_BUFFER_CLEAR_SPEC,
>;
///Unallocate request buffer clear registers
pub mod l2_unallocate_buffer_clear;
/**L2_CACHE_ACCESS_ATTR_CTRL (rw) register accessor: L2 cache access attribute control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_access_attr_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_access_attr_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_access_attr_ctrl`] module*/
pub type L2_CACHE_ACCESS_ATTR_CTRL = crate::Reg<
    l2_cache_access_attr_ctrl::L2_CACHE_ACCESS_ATTR_CTRL_SPEC,
>;
///L2 cache access attribute control register
pub mod l2_cache_access_attr_ctrl;
/**L2_CACHE_OBJECT_CTRL (rw) register accessor: Cache Tag and Data memory Object control register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_object_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_object_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_object_ctrl`] module*/
pub type L2_CACHE_OBJECT_CTRL = crate::Reg<
    l2_cache_object_ctrl::L2_CACHE_OBJECT_CTRL_SPEC,
>;
///Cache Tag and Data memory Object control register
pub mod l2_cache_object_ctrl;
/**L2_CACHE_WAY_OBJECT (rw) register accessor: Cache Tag and Data memory way register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_way_object::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_way_object::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_way_object`] module*/
pub type L2_CACHE_WAY_OBJECT = crate::Reg<l2_cache_way_object::L2_CACHE_WAY_OBJECT_SPEC>;
///Cache Tag and Data memory way register
pub mod l2_cache_way_object;
/**L2_CACHE_VADDR (rw) register accessor: Cache Vaddr register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_vaddr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_vaddr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_vaddr`] module*/
pub type L2_CACHE_VADDR = crate::Reg<l2_cache_vaddr::L2_CACHE_VADDR_SPEC>;
///Cache Vaddr register
pub mod l2_cache_vaddr;
/**L2_CACHE_DEBUG_BUS (rw) register accessor: Cache Tag/data memory content register

You can [`read`](crate::generic::Reg::read) this register and get [`l2_cache_debug_bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l2_cache_debug_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@l2_cache_debug_bus`] module*/
pub type L2_CACHE_DEBUG_BUS = crate::Reg<l2_cache_debug_bus::L2_CACHE_DEBUG_BUS_SPEC>;
///Cache Tag/data memory content register
pub mod l2_cache_debug_bus;
/**LEVEL_SPLIT1 (r) register accessor: USED TO SPLIT L1 CACHE AND L2 CACHE

You can [`read`](crate::generic::Reg::read) this register and get [`level_split1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@level_split1`] module*/
pub type LEVEL_SPLIT1 = crate::Reg<level_split1::LEVEL_SPLIT1_SPEC>;
///USED TO SPLIT L1 CACHE AND L2 CACHE
pub mod level_split1;
/**CLOCK_GATE (rw) register accessor: Clock gate control register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///Clock gate control register
pub mod clock_gate;
/**REDUNDANCY_SIG0 (rw) register accessor: Cache redundancy signal 0 register

You can [`read`](crate::generic::Reg::read) this register and get [`redundancy_sig0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundancy_sig0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@redundancy_sig0`] module*/
pub type REDUNDANCY_SIG0 = crate::Reg<redundancy_sig0::REDUNDANCY_SIG0_SPEC>;
///Cache redundancy signal 0 register
pub mod redundancy_sig0;
/**REDUNDANCY_SIG1 (rw) register accessor: Cache redundancy signal 1 register

You can [`read`](crate::generic::Reg::read) this register and get [`redundancy_sig1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundancy_sig1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@redundancy_sig1`] module*/
pub type REDUNDANCY_SIG1 = crate::Reg<redundancy_sig1::REDUNDANCY_SIG1_SPEC>;
///Cache redundancy signal 1 register
pub mod redundancy_sig1;
/**REDUNDANCY_SIG2 (rw) register accessor: Cache redundancy signal 2 register

You can [`read`](crate::generic::Reg::read) this register and get [`redundancy_sig2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundancy_sig2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@redundancy_sig2`] module*/
pub type REDUNDANCY_SIG2 = crate::Reg<redundancy_sig2::REDUNDANCY_SIG2_SPEC>;
///Cache redundancy signal 2 register
pub mod redundancy_sig2;
/**REDUNDANCY_SIG3 (rw) register accessor: Cache redundancy signal 3 register

You can [`read`](crate::generic::Reg::read) this register and get [`redundancy_sig3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`redundancy_sig3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@redundancy_sig3`] module*/
pub type REDUNDANCY_SIG3 = crate::Reg<redundancy_sig3::REDUNDANCY_SIG3_SPEC>;
///Cache redundancy signal 3 register
pub mod redundancy_sig3;
/**REDUNDANCY_SIG4 (r) register accessor: Cache redundancy signal 0 register

You can [`read`](crate::generic::Reg::read) this register and get [`redundancy_sig4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@redundancy_sig4`] module*/
pub type REDUNDANCY_SIG4 = crate::Reg<redundancy_sig4::REDUNDANCY_SIG4_SPEC>;
///Cache redundancy signal 0 register
pub mod redundancy_sig4;
/**DATE (rw) register accessor: Version control register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Version control register
pub mod date;
