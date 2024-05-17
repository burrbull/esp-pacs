#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    uart0_conf: UART0_CONF,
    uart0_sclk_conf: UART0_SCLK_CONF,
    uart0_pd_ctrl: UART0_PD_CTRL,
    uart1_conf: UART1_CONF,
    uart1_sclk_conf: UART1_SCLK_CONF,
    uart1_pd_ctrl: UART1_PD_CTRL,
    mspi_conf: MSPI_CONF,
    mspi_clk_conf: MSPI_CLK_CONF,
    i2c0_conf: I2C0_CONF,
    i2c0_sclk_conf: I2C0_SCLK_CONF,
    i2c1_conf: I2C1_CONF,
    i2c1_sclk_conf: I2C1_SCLK_CONF,
    uhci_conf: UHCI_CONF,
    rmt_conf: RMT_CONF,
    rmt_sclk_conf: RMT_SCLK_CONF,
    ledc_conf: LEDC_CONF,
    ledc_sclk_conf: LEDC_SCLK_CONF,
    timergroup0_conf: TIMERGROUP0_CONF,
    timergroup0_timer_clk_conf: TIMERGROUP0_TIMER_CLK_CONF,
    timergroup0_wdt_clk_conf: TIMERGROUP0_WDT_CLK_CONF,
    timergroup1_conf: TIMERGROUP1_CONF,
    timergroup1_timer_clk_conf: TIMERGROUP1_TIMER_CLK_CONF,
    timergroup1_wdt_clk_conf: TIMERGROUP1_WDT_CLK_CONF,
    systimer_conf: SYSTIMER_CONF,
    systimer_func_clk_conf: SYSTIMER_FUNC_CLK_CONF,
    twai0_conf: TWAI0_CONF,
    twai0_func_clk_conf: TWAI0_FUNC_CLK_CONF,
    i2s_conf: I2S_CONF,
    i2s_tx_clkm_conf: I2S_TX_CLKM_CONF,
    i2s_tx_clkm_div_conf: I2S_TX_CLKM_DIV_CONF,
    i2s_rx_clkm_conf: I2S_RX_CLKM_CONF,
    i2s_rx_clkm_div_conf: I2S_RX_CLKM_DIV_CONF,
    saradc_conf: SARADC_CONF,
    saradc_clkm_conf: SARADC_CLKM_CONF,
    tsens_clk_conf: TSENS_CLK_CONF,
    usb_device_conf: USB_DEVICE_CONF,
    intmtx_conf: INTMTX_CONF,
    pcnt_conf: PCNT_CONF,
    etm_conf: ETM_CONF,
    pwm_conf: PWM_CONF,
    pwm_clk_conf: PWM_CLK_CONF,
    parl_io_conf: PARL_IO_CONF,
    parl_clk_rx_conf: PARL_CLK_RX_CONF,
    parl_clk_tx_conf: PARL_CLK_TX_CONF,
    pvt_monitor_conf: PVT_MONITOR_CONF,
    pvt_monitor_func_clk_conf: PVT_MONITOR_FUNC_CLK_CONF,
    gdma_conf: GDMA_CONF,
    spi2_conf: SPI2_CONF,
    spi2_clkm_conf: SPI2_CLKM_CONF,
    aes_conf: AES_CONF,
    sha_conf: SHA_CONF,
    rsa_conf: RSA_CONF,
    rsa_pd_ctrl: RSA_PD_CTRL,
    ecc_conf: ECC_CONF,
    ecc_pd_ctrl: ECC_PD_CTRL,
    ds_conf: DS_CONF,
    hmac_conf: HMAC_CONF,
    ecdsa_conf: ECDSA_CONF,
    iomux_conf: IOMUX_CONF,
    iomux_clk_conf: IOMUX_CLK_CONF,
    mem_monitor_conf: MEM_MONITOR_CONF,
    regdma_conf: REGDMA_CONF,
    trace_conf: TRACE_CONF,
    assist_conf: ASSIST_CONF,
    cache_conf: CACHE_CONF,
    modem_conf: MODEM_CONF,
    timeout_conf: TIMEOUT_CONF,
    sysclk_conf: SYSCLK_CONF,
    cpu_waiti_conf: CPU_WAITI_CONF,
    cpu_freq_conf: CPU_FREQ_CONF,
    ahb_freq_conf: AHB_FREQ_CONF,
    apb_freq_conf: APB_FREQ_CONF,
    sysclk_freq_query_0: SYSCLK_FREQ_QUERY_0,
    pll_div_clk_en: PLL_DIV_CLK_EN,
    ctrl_clk_out_en: CTRL_CLK_OUT_EN,
    ctrl_tick_conf: CTRL_TICK_CONF,
    ctrl_32k_conf: CTRL_32K_CONF,
    sram_power_conf_0: SRAM_POWER_CONF_0,
    sram_power_conf_1: SRAM_POWER_CONF_1,
    sec_conf: SEC_CONF,
    adc_inv_phase_conf: ADC_INV_PHASE_CONF,
    sdm_inv_phase_conf: SDM_INV_PHASE_CONF,
    bus_clk_update: BUS_CLK_UPDATE,
    sar_clk_div: SAR_CLK_DIV,
    pwdet_sar_clk_conf: PWDET_SAR_CLK_CONF,
    _reserved85: [u8; 0x0e9c],
    reset_event_bypass: RESET_EVENT_BYPASS,
    fpga_debug: FPGA_DEBUG,
    clock_gate: CLOCK_GATE,
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - UART0 configuration register
    #[inline(always)]
    pub const fn uart0_conf(&self) -> &UART0_CONF {
        &self.uart0_conf
    }
    ///0x04 - UART0_SCLK configuration register
    #[inline(always)]
    pub const fn uart0_sclk_conf(&self) -> &UART0_SCLK_CONF {
        &self.uart0_sclk_conf
    }
    ///0x08 - UART0 power control register
    #[inline(always)]
    pub const fn uart0_pd_ctrl(&self) -> &UART0_PD_CTRL {
        &self.uart0_pd_ctrl
    }
    ///0x0c - UART1 configuration register
    #[inline(always)]
    pub const fn uart1_conf(&self) -> &UART1_CONF {
        &self.uart1_conf
    }
    ///0x10 - UART1_SCLK configuration register
    #[inline(always)]
    pub const fn uart1_sclk_conf(&self) -> &UART1_SCLK_CONF {
        &self.uart1_sclk_conf
    }
    ///0x14 - UART1 power control register
    #[inline(always)]
    pub const fn uart1_pd_ctrl(&self) -> &UART1_PD_CTRL {
        &self.uart1_pd_ctrl
    }
    ///0x18 - MSPI configuration register
    #[inline(always)]
    pub const fn mspi_conf(&self) -> &MSPI_CONF {
        &self.mspi_conf
    }
    ///0x1c - MSPI_CLK configuration register
    #[inline(always)]
    pub const fn mspi_clk_conf(&self) -> &MSPI_CLK_CONF {
        &self.mspi_clk_conf
    }
    ///0x20 - I2C configuration register
    #[inline(always)]
    pub const fn i2c0_conf(&self) -> &I2C0_CONF {
        &self.i2c0_conf
    }
    ///0x24 - I2C_SCLK configuration register
    #[inline(always)]
    pub const fn i2c0_sclk_conf(&self) -> &I2C0_SCLK_CONF {
        &self.i2c0_sclk_conf
    }
    ///0x28 - I2C configuration register
    #[inline(always)]
    pub const fn i2c1_conf(&self) -> &I2C1_CONF {
        &self.i2c1_conf
    }
    ///0x2c - I2C_SCLK configuration register
    #[inline(always)]
    pub const fn i2c1_sclk_conf(&self) -> &I2C1_SCLK_CONF {
        &self.i2c1_sclk_conf
    }
    ///0x30 - UHCI configuration register
    #[inline(always)]
    pub const fn uhci_conf(&self) -> &UHCI_CONF {
        &self.uhci_conf
    }
    ///0x34 - RMT configuration register
    #[inline(always)]
    pub const fn rmt_conf(&self) -> &RMT_CONF {
        &self.rmt_conf
    }
    ///0x38 - RMT_SCLK configuration register
    #[inline(always)]
    pub const fn rmt_sclk_conf(&self) -> &RMT_SCLK_CONF {
        &self.rmt_sclk_conf
    }
    ///0x3c - LEDC configuration register
    #[inline(always)]
    pub const fn ledc_conf(&self) -> &LEDC_CONF {
        &self.ledc_conf
    }
    ///0x40 - LEDC_SCLK configuration register
    #[inline(always)]
    pub const fn ledc_sclk_conf(&self) -> &LEDC_SCLK_CONF {
        &self.ledc_sclk_conf
    }
    ///0x44 - TIMERGROUP0 configuration register
    #[inline(always)]
    pub const fn timergroup0_conf(&self) -> &TIMERGROUP0_CONF {
        &self.timergroup0_conf
    }
    ///0x48 - TIMERGROUP0_TIMER_CLK configuration register
    #[inline(always)]
    pub const fn timergroup0_timer_clk_conf(&self) -> &TIMERGROUP0_TIMER_CLK_CONF {
        &self.timergroup0_timer_clk_conf
    }
    ///0x4c - TIMERGROUP0_WDT_CLK configuration register
    #[inline(always)]
    pub const fn timergroup0_wdt_clk_conf(&self) -> &TIMERGROUP0_WDT_CLK_CONF {
        &self.timergroup0_wdt_clk_conf
    }
    ///0x50 - TIMERGROUP1 configuration register
    #[inline(always)]
    pub const fn timergroup1_conf(&self) -> &TIMERGROUP1_CONF {
        &self.timergroup1_conf
    }
    ///0x54 - TIMERGROUP1_TIMER_CLK configuration register
    #[inline(always)]
    pub const fn timergroup1_timer_clk_conf(&self) -> &TIMERGROUP1_TIMER_CLK_CONF {
        &self.timergroup1_timer_clk_conf
    }
    ///0x58 - TIMERGROUP1_WDT_CLK configuration register
    #[inline(always)]
    pub const fn timergroup1_wdt_clk_conf(&self) -> &TIMERGROUP1_WDT_CLK_CONF {
        &self.timergroup1_wdt_clk_conf
    }
    ///0x5c - SYSTIMER configuration register
    #[inline(always)]
    pub const fn systimer_conf(&self) -> &SYSTIMER_CONF {
        &self.systimer_conf
    }
    ///0x60 - SYSTIMER_FUNC_CLK configuration register
    #[inline(always)]
    pub const fn systimer_func_clk_conf(&self) -> &SYSTIMER_FUNC_CLK_CONF {
        &self.systimer_func_clk_conf
    }
    ///0x64 - TWAI0 configuration register
    #[inline(always)]
    pub const fn twai0_conf(&self) -> &TWAI0_CONF {
        &self.twai0_conf
    }
    ///0x68 - TWAI0_FUNC_CLK configuration register
    #[inline(always)]
    pub const fn twai0_func_clk_conf(&self) -> &TWAI0_FUNC_CLK_CONF {
        &self.twai0_func_clk_conf
    }
    ///0x6c - I2S configuration register
    #[inline(always)]
    pub const fn i2s_conf(&self) -> &I2S_CONF {
        &self.i2s_conf
    }
    ///0x70 - I2S_TX_CLKM configuration register
    #[inline(always)]
    pub const fn i2s_tx_clkm_conf(&self) -> &I2S_TX_CLKM_CONF {
        &self.i2s_tx_clkm_conf
    }
    ///0x74 - I2S_TX_CLKM_DIV configuration register
    #[inline(always)]
    pub const fn i2s_tx_clkm_div_conf(&self) -> &I2S_TX_CLKM_DIV_CONF {
        &self.i2s_tx_clkm_div_conf
    }
    ///0x78 - I2S_RX_CLKM configuration register
    #[inline(always)]
    pub const fn i2s_rx_clkm_conf(&self) -> &I2S_RX_CLKM_CONF {
        &self.i2s_rx_clkm_conf
    }
    ///0x7c - I2S_RX_CLKM_DIV configuration register
    #[inline(always)]
    pub const fn i2s_rx_clkm_div_conf(&self) -> &I2S_RX_CLKM_DIV_CONF {
        &self.i2s_rx_clkm_div_conf
    }
    ///0x80 - SARADC configuration register
    #[inline(always)]
    pub const fn saradc_conf(&self) -> &SARADC_CONF {
        &self.saradc_conf
    }
    ///0x84 - SARADC_CLKM configuration register
    #[inline(always)]
    pub const fn saradc_clkm_conf(&self) -> &SARADC_CLKM_CONF {
        &self.saradc_clkm_conf
    }
    ///0x88 - TSENS_CLK configuration register
    #[inline(always)]
    pub const fn tsens_clk_conf(&self) -> &TSENS_CLK_CONF {
        &self.tsens_clk_conf
    }
    ///0x8c - USB_DEVICE configuration register
    #[inline(always)]
    pub const fn usb_device_conf(&self) -> &USB_DEVICE_CONF {
        &self.usb_device_conf
    }
    ///0x90 - INTMTX configuration register
    #[inline(always)]
    pub const fn intmtx_conf(&self) -> &INTMTX_CONF {
        &self.intmtx_conf
    }
    ///0x94 - PCNT configuration register
    #[inline(always)]
    pub const fn pcnt_conf(&self) -> &PCNT_CONF {
        &self.pcnt_conf
    }
    ///0x98 - ETM configuration register
    #[inline(always)]
    pub const fn etm_conf(&self) -> &ETM_CONF {
        &self.etm_conf
    }
    ///0x9c - PWM configuration register
    #[inline(always)]
    pub const fn pwm_conf(&self) -> &PWM_CONF {
        &self.pwm_conf
    }
    ///0xa0 - PWM_CLK configuration register
    #[inline(always)]
    pub const fn pwm_clk_conf(&self) -> &PWM_CLK_CONF {
        &self.pwm_clk_conf
    }
    ///0xa4 - PARL_IO configuration register
    #[inline(always)]
    pub const fn parl_io_conf(&self) -> &PARL_IO_CONF {
        &self.parl_io_conf
    }
    ///0xa8 - PARL_CLK_RX configuration register
    #[inline(always)]
    pub const fn parl_clk_rx_conf(&self) -> &PARL_CLK_RX_CONF {
        &self.parl_clk_rx_conf
    }
    ///0xac - PARL_CLK_TX configuration register
    #[inline(always)]
    pub const fn parl_clk_tx_conf(&self) -> &PARL_CLK_TX_CONF {
        &self.parl_clk_tx_conf
    }
    ///0xb0 - PVT_MONITOR configuration register
    #[inline(always)]
    pub const fn pvt_monitor_conf(&self) -> &PVT_MONITOR_CONF {
        &self.pvt_monitor_conf
    }
    ///0xb4 - PVT_MONITOR function clock configuration register
    #[inline(always)]
    pub const fn pvt_monitor_func_clk_conf(&self) -> &PVT_MONITOR_FUNC_CLK_CONF {
        &self.pvt_monitor_func_clk_conf
    }
    ///0xb8 - GDMA configuration register
    #[inline(always)]
    pub const fn gdma_conf(&self) -> &GDMA_CONF {
        &self.gdma_conf
    }
    ///0xbc - SPI2 configuration register
    #[inline(always)]
    pub const fn spi2_conf(&self) -> &SPI2_CONF {
        &self.spi2_conf
    }
    ///0xc0 - SPI2_CLKM configuration register
    #[inline(always)]
    pub const fn spi2_clkm_conf(&self) -> &SPI2_CLKM_CONF {
        &self.spi2_clkm_conf
    }
    ///0xc4 - AES configuration register
    #[inline(always)]
    pub const fn aes_conf(&self) -> &AES_CONF {
        &self.aes_conf
    }
    ///0xc8 - SHA configuration register
    #[inline(always)]
    pub const fn sha_conf(&self) -> &SHA_CONF {
        &self.sha_conf
    }
    ///0xcc - RSA configuration register
    #[inline(always)]
    pub const fn rsa_conf(&self) -> &RSA_CONF {
        &self.rsa_conf
    }
    ///0xd0 - RSA power control register
    #[inline(always)]
    pub const fn rsa_pd_ctrl(&self) -> &RSA_PD_CTRL {
        &self.rsa_pd_ctrl
    }
    ///0xd4 - ECC configuration register
    #[inline(always)]
    pub const fn ecc_conf(&self) -> &ECC_CONF {
        &self.ecc_conf
    }
    ///0xd8 - ECC power control register
    #[inline(always)]
    pub const fn ecc_pd_ctrl(&self) -> &ECC_PD_CTRL {
        &self.ecc_pd_ctrl
    }
    ///0xdc - DS configuration register
    #[inline(always)]
    pub const fn ds_conf(&self) -> &DS_CONF {
        &self.ds_conf
    }
    ///0xe0 - HMAC configuration register
    #[inline(always)]
    pub const fn hmac_conf(&self) -> &HMAC_CONF {
        &self.hmac_conf
    }
    ///0xe4 - ECDSA configuration register
    #[inline(always)]
    pub const fn ecdsa_conf(&self) -> &ECDSA_CONF {
        &self.ecdsa_conf
    }
    ///0xe8 - IOMUX configuration register
    #[inline(always)]
    pub const fn iomux_conf(&self) -> &IOMUX_CONF {
        &self.iomux_conf
    }
    ///0xec - IOMUX_CLK configuration register
    #[inline(always)]
    pub const fn iomux_clk_conf(&self) -> &IOMUX_CLK_CONF {
        &self.iomux_clk_conf
    }
    ///0xf0 - MEM_MONITOR configuration register
    #[inline(always)]
    pub const fn mem_monitor_conf(&self) -> &MEM_MONITOR_CONF {
        &self.mem_monitor_conf
    }
    ///0xf4 - REGDMA configuration register
    #[inline(always)]
    pub const fn regdma_conf(&self) -> &REGDMA_CONF {
        &self.regdma_conf
    }
    ///0xf8 - TRACE configuration register
    #[inline(always)]
    pub const fn trace_conf(&self) -> &TRACE_CONF {
        &self.trace_conf
    }
    ///0xfc - ASSIST configuration register
    #[inline(always)]
    pub const fn assist_conf(&self) -> &ASSIST_CONF {
        &self.assist_conf
    }
    ///0x100 - CACHE configuration register
    #[inline(always)]
    pub const fn cache_conf(&self) -> &CACHE_CONF {
        &self.cache_conf
    }
    ///0x104 - MODEM_APB configuration register
    #[inline(always)]
    pub const fn modem_conf(&self) -> &MODEM_CONF {
        &self.modem_conf
    }
    ///0x108 - TIMEOUT configuration register
    #[inline(always)]
    pub const fn timeout_conf(&self) -> &TIMEOUT_CONF {
        &self.timeout_conf
    }
    ///0x10c - SYSCLK configuration register
    #[inline(always)]
    pub const fn sysclk_conf(&self) -> &SYSCLK_CONF {
        &self.sysclk_conf
    }
    ///0x110 - CPU_WAITI configuration register
    #[inline(always)]
    pub const fn cpu_waiti_conf(&self) -> &CPU_WAITI_CONF {
        &self.cpu_waiti_conf
    }
    ///0x114 - CPU_FREQ configuration register
    #[inline(always)]
    pub const fn cpu_freq_conf(&self) -> &CPU_FREQ_CONF {
        &self.cpu_freq_conf
    }
    ///0x118 - AHB_FREQ configuration register
    #[inline(always)]
    pub const fn ahb_freq_conf(&self) -> &AHB_FREQ_CONF {
        &self.ahb_freq_conf
    }
    ///0x11c - APB_FREQ configuration register
    #[inline(always)]
    pub const fn apb_freq_conf(&self) -> &APB_FREQ_CONF {
        &self.apb_freq_conf
    }
    ///0x120 - SYSCLK frequency query 0 register
    #[inline(always)]
    pub const fn sysclk_freq_query_0(&self) -> &SYSCLK_FREQ_QUERY_0 {
        &self.sysclk_freq_query_0
    }
    ///0x124 - SPLL DIV clock-gating configuration register
    #[inline(always)]
    pub const fn pll_div_clk_en(&self) -> &PLL_DIV_CLK_EN {
        &self.pll_div_clk_en
    }
    ///0x128 - CLK_OUT_EN configuration register
    #[inline(always)]
    pub const fn ctrl_clk_out_en(&self) -> &CTRL_CLK_OUT_EN {
        &self.ctrl_clk_out_en
    }
    ///0x12c - TICK configuration register
    #[inline(always)]
    pub const fn ctrl_tick_conf(&self) -> &CTRL_TICK_CONF {
        &self.ctrl_tick_conf
    }
    ///0x130 - 32KHz clock configuration register
    #[inline(always)]
    pub const fn ctrl_32k_conf(&self) -> &CTRL_32K_CONF {
        &self.ctrl_32k_conf
    }
    ///0x134 - HP SRAM/ROM configuration register
    #[inline(always)]
    pub const fn sram_power_conf_0(&self) -> &SRAM_POWER_CONF_0 {
        &self.sram_power_conf_0
    }
    ///0x138 - HP SRAM/ROM configuration register
    #[inline(always)]
    pub const fn sram_power_conf_1(&self) -> &SRAM_POWER_CONF_1 {
        &self.sram_power_conf_1
    }
    ///0x13c - xxxx
    #[inline(always)]
    pub const fn sec_conf(&self) -> &SEC_CONF {
        &self.sec_conf
    }
    ///0x140 - xxxx
    #[inline(always)]
    pub const fn adc_inv_phase_conf(&self) -> &ADC_INV_PHASE_CONF {
        &self.adc_inv_phase_conf
    }
    ///0x144 - xxxx
    #[inline(always)]
    pub const fn sdm_inv_phase_conf(&self) -> &SDM_INV_PHASE_CONF {
        &self.sdm_inv_phase_conf
    }
    ///0x148 - xxxx
    #[inline(always)]
    pub const fn bus_clk_update(&self) -> &BUS_CLK_UPDATE {
        &self.bus_clk_update
    }
    ///0x14c - xxxx
    #[inline(always)]
    pub const fn sar_clk_div(&self) -> &SAR_CLK_DIV {
        &self.sar_clk_div
    }
    ///0x150 - xxxx
    #[inline(always)]
    pub const fn pwdet_sar_clk_conf(&self) -> &PWDET_SAR_CLK_CONF {
        &self.pwdet_sar_clk_conf
    }
    ///0xff0 - reset event bypass backdoor configuration register
    #[inline(always)]
    pub const fn reset_event_bypass(&self) -> &RESET_EVENT_BYPASS {
        &self.reset_event_bypass
    }
    ///0xff4 - fpga debug register
    #[inline(always)]
    pub const fn fpga_debug(&self) -> &FPGA_DEBUG {
        &self.fpga_debug
    }
    ///0xff8 - PCR clock gating configure register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0xffc - Date register.
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**UART0_CONF (rw) register accessor: UART0 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart0_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uart0_conf`] module*/
pub type UART0_CONF = crate::Reg<uart0_conf::UART0_CONF_SPEC>;
///UART0 configuration register
pub mod uart0_conf;
/**UART0_SCLK_CONF (rw) register accessor: UART0_SCLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart0_sclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_sclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uart0_sclk_conf`] module*/
pub type UART0_SCLK_CONF = crate::Reg<uart0_sclk_conf::UART0_SCLK_CONF_SPEC>;
///UART0_SCLK configuration register
pub mod uart0_sclk_conf;
/**UART0_PD_CTRL (rw) register accessor: UART0 power control register

You can [`read`](crate::generic::Reg::read) this register and get [`uart0_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart0_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uart0_pd_ctrl`] module*/
pub type UART0_PD_CTRL = crate::Reg<uart0_pd_ctrl::UART0_PD_CTRL_SPEC>;
///UART0 power control register
pub mod uart0_pd_ctrl;
/**UART1_CONF (rw) register accessor: UART1 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart1_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uart1_conf`] module*/
pub type UART1_CONF = crate::Reg<uart1_conf::UART1_CONF_SPEC>;
///UART1 configuration register
pub mod uart1_conf;
/**UART1_SCLK_CONF (rw) register accessor: UART1_SCLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uart1_sclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_sclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uart1_sclk_conf`] module*/
pub type UART1_SCLK_CONF = crate::Reg<uart1_sclk_conf::UART1_SCLK_CONF_SPEC>;
///UART1_SCLK configuration register
pub mod uart1_sclk_conf;
/**UART1_PD_CTRL (rw) register accessor: UART1 power control register

You can [`read`](crate::generic::Reg::read) this register and get [`uart1_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart1_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uart1_pd_ctrl`] module*/
pub type UART1_PD_CTRL = crate::Reg<uart1_pd_ctrl::UART1_PD_CTRL_SPEC>;
///UART1 power control register
pub mod uart1_pd_ctrl;
/**MSPI_CONF (rw) register accessor: MSPI configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`mspi_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspi_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mspi_conf`] module*/
pub type MSPI_CONF = crate::Reg<mspi_conf::MSPI_CONF_SPEC>;
///MSPI configuration register
pub mod mspi_conf;
/**MSPI_CLK_CONF (rw) register accessor: MSPI_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`mspi_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mspi_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mspi_clk_conf`] module*/
pub type MSPI_CLK_CONF = crate::Reg<mspi_clk_conf::MSPI_CLK_CONF_SPEC>;
///MSPI_CLK configuration register
pub mod mspi_clk_conf;
/**I2C0_CONF (rw) register accessor: I2C configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2c0_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2c0_conf`] module*/
pub type I2C0_CONF = crate::Reg<i2c0_conf::I2C0_CONF_SPEC>;
///I2C configuration register
pub mod i2c0_conf;
/**I2C0_SCLK_CONF (rw) register accessor: I2C_SCLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2c0_sclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c0_sclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2c0_sclk_conf`] module*/
pub type I2C0_SCLK_CONF = crate::Reg<i2c0_sclk_conf::I2C0_SCLK_CONF_SPEC>;
///I2C_SCLK configuration register
pub mod i2c0_sclk_conf;
/**I2C1_CONF (rw) register accessor: I2C configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2c1_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c1_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2c1_conf`] module*/
pub type I2C1_CONF = crate::Reg<i2c1_conf::I2C1_CONF_SPEC>;
///I2C configuration register
pub mod i2c1_conf;
/**I2C1_SCLK_CONF (rw) register accessor: I2C_SCLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2c1_sclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c1_sclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2c1_sclk_conf`] module*/
pub type I2C1_SCLK_CONF = crate::Reg<i2c1_sclk_conf::I2C1_SCLK_CONF_SPEC>;
///I2C_SCLK configuration register
pub mod i2c1_sclk_conf;
/**UHCI_CONF (rw) register accessor: UHCI configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`uhci_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uhci_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@uhci_conf`] module*/
pub type UHCI_CONF = crate::Reg<uhci_conf::UHCI_CONF_SPEC>;
///UHCI configuration register
pub mod uhci_conf;
/**RMT_CONF (rw) register accessor: RMT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rmt_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmt_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmt_conf`] module*/
pub type RMT_CONF = crate::Reg<rmt_conf::RMT_CONF_SPEC>;
///RMT configuration register
pub mod rmt_conf;
/**RMT_SCLK_CONF (rw) register accessor: RMT_SCLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rmt_sclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rmt_sclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmt_sclk_conf`] module*/
pub type RMT_SCLK_CONF = crate::Reg<rmt_sclk_conf::RMT_SCLK_CONF_SPEC>;
///RMT_SCLK configuration register
pub mod rmt_sclk_conf;
/**LEDC_CONF (rw) register accessor: LEDC configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ledc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ledc_conf`] module*/
pub type LEDC_CONF = crate::Reg<ledc_conf::LEDC_CONF_SPEC>;
///LEDC configuration register
pub mod ledc_conf;
/**LEDC_SCLK_CONF (rw) register accessor: LEDC_SCLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ledc_sclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ledc_sclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ledc_sclk_conf`] module*/
pub type LEDC_SCLK_CONF = crate::Reg<ledc_sclk_conf::LEDC_SCLK_CONF_SPEC>;
///LEDC_SCLK configuration register
pub mod ledc_sclk_conf;
/**TIMERGROUP0_CONF (rw) register accessor: TIMERGROUP0 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timergroup0_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergroup0_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timergroup0_conf`] module*/
pub type TIMERGROUP0_CONF = crate::Reg<timergroup0_conf::TIMERGROUP0_CONF_SPEC>;
///TIMERGROUP0 configuration register
pub mod timergroup0_conf;
/**TIMERGROUP0_TIMER_CLK_CONF (rw) register accessor: TIMERGROUP0_TIMER_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timergroup0_timer_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergroup0_timer_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timergroup0_timer_clk_conf`] module*/
pub type TIMERGROUP0_TIMER_CLK_CONF = crate::Reg<
    timergroup0_timer_clk_conf::TIMERGROUP0_TIMER_CLK_CONF_SPEC,
>;
///TIMERGROUP0_TIMER_CLK configuration register
pub mod timergroup0_timer_clk_conf;
/**TIMERGROUP0_WDT_CLK_CONF (rw) register accessor: TIMERGROUP0_WDT_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timergroup0_wdt_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergroup0_wdt_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timergroup0_wdt_clk_conf`] module*/
pub type TIMERGROUP0_WDT_CLK_CONF = crate::Reg<
    timergroup0_wdt_clk_conf::TIMERGROUP0_WDT_CLK_CONF_SPEC,
>;
///TIMERGROUP0_WDT_CLK configuration register
pub mod timergroup0_wdt_clk_conf;
/**TIMERGROUP1_CONF (rw) register accessor: TIMERGROUP1 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timergroup1_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergroup1_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timergroup1_conf`] module*/
pub type TIMERGROUP1_CONF = crate::Reg<timergroup1_conf::TIMERGROUP1_CONF_SPEC>;
///TIMERGROUP1 configuration register
pub mod timergroup1_conf;
/**TIMERGROUP1_TIMER_CLK_CONF (rw) register accessor: TIMERGROUP1_TIMER_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timergroup1_timer_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergroup1_timer_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timergroup1_timer_clk_conf`] module*/
pub type TIMERGROUP1_TIMER_CLK_CONF = crate::Reg<
    timergroup1_timer_clk_conf::TIMERGROUP1_TIMER_CLK_CONF_SPEC,
>;
///TIMERGROUP1_TIMER_CLK configuration register
pub mod timergroup1_timer_clk_conf;
/**TIMERGROUP1_WDT_CLK_CONF (rw) register accessor: TIMERGROUP1_WDT_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timergroup1_wdt_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timergroup1_wdt_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timergroup1_wdt_clk_conf`] module*/
pub type TIMERGROUP1_WDT_CLK_CONF = crate::Reg<
    timergroup1_wdt_clk_conf::TIMERGROUP1_WDT_CLK_CONF_SPEC,
>;
///TIMERGROUP1_WDT_CLK configuration register
pub mod timergroup1_wdt_clk_conf;
/**SYSTIMER_CONF (rw) register accessor: SYSTIMER configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`systimer_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@systimer_conf`] module*/
pub type SYSTIMER_CONF = crate::Reg<systimer_conf::SYSTIMER_CONF_SPEC>;
///SYSTIMER configuration register
pub mod systimer_conf;
/**SYSTIMER_FUNC_CLK_CONF (rw) register accessor: SYSTIMER_FUNC_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`systimer_func_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`systimer_func_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@systimer_func_clk_conf`] module*/
pub type SYSTIMER_FUNC_CLK_CONF = crate::Reg<
    systimer_func_clk_conf::SYSTIMER_FUNC_CLK_CONF_SPEC,
>;
///SYSTIMER_FUNC_CLK configuration register
pub mod systimer_func_clk_conf;
/**TWAI0_CONF (rw) register accessor: TWAI0 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`twai0_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twai0_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@twai0_conf`] module*/
pub type TWAI0_CONF = crate::Reg<twai0_conf::TWAI0_CONF_SPEC>;
///TWAI0 configuration register
pub mod twai0_conf;
/**TWAI0_FUNC_CLK_CONF (rw) register accessor: TWAI0_FUNC_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`twai0_func_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`twai0_func_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@twai0_func_clk_conf`] module*/
pub type TWAI0_FUNC_CLK_CONF = crate::Reg<twai0_func_clk_conf::TWAI0_FUNC_CLK_CONF_SPEC>;
///TWAI0_FUNC_CLK configuration register
pub mod twai0_func_clk_conf;
/**I2S_CONF (rw) register accessor: I2S configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2s_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s_conf`] module*/
pub type I2S_CONF = crate::Reg<i2s_conf::I2S_CONF_SPEC>;
///I2S configuration register
pub mod i2s_conf;
/**I2S_TX_CLKM_CONF (rw) register accessor: I2S_TX_CLKM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2s_tx_clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_tx_clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s_tx_clkm_conf`] module*/
pub type I2S_TX_CLKM_CONF = crate::Reg<i2s_tx_clkm_conf::I2S_TX_CLKM_CONF_SPEC>;
///I2S_TX_CLKM configuration register
pub mod i2s_tx_clkm_conf;
/**I2S_TX_CLKM_DIV_CONF (rw) register accessor: I2S_TX_CLKM_DIV configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2s_tx_clkm_div_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_tx_clkm_div_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s_tx_clkm_div_conf`] module*/
pub type I2S_TX_CLKM_DIV_CONF = crate::Reg<
    i2s_tx_clkm_div_conf::I2S_TX_CLKM_DIV_CONF_SPEC,
>;
///I2S_TX_CLKM_DIV configuration register
pub mod i2s_tx_clkm_div_conf;
/**I2S_RX_CLKM_CONF (rw) register accessor: I2S_RX_CLKM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2s_rx_clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_rx_clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s_rx_clkm_conf`] module*/
pub type I2S_RX_CLKM_CONF = crate::Reg<i2s_rx_clkm_conf::I2S_RX_CLKM_CONF_SPEC>;
///I2S_RX_CLKM configuration register
pub mod i2s_rx_clkm_conf;
/**I2S_RX_CLKM_DIV_CONF (rw) register accessor: I2S_RX_CLKM_DIV configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`i2s_rx_clkm_div_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_rx_clkm_div_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@i2s_rx_clkm_div_conf`] module*/
pub type I2S_RX_CLKM_DIV_CONF = crate::Reg<
    i2s_rx_clkm_div_conf::I2S_RX_CLKM_DIV_CONF_SPEC,
>;
///I2S_RX_CLKM_DIV configuration register
pub mod i2s_rx_clkm_div_conf;
/**SARADC_CONF (rw) register accessor: SARADC configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`saradc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saradc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@saradc_conf`] module*/
pub type SARADC_CONF = crate::Reg<saradc_conf::SARADC_CONF_SPEC>;
///SARADC configuration register
pub mod saradc_conf;
/**SARADC_CLKM_CONF (rw) register accessor: SARADC_CLKM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`saradc_clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`saradc_clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@saradc_clkm_conf`] module*/
pub type SARADC_CLKM_CONF = crate::Reg<saradc_clkm_conf::SARADC_CLKM_CONF_SPEC>;
///SARADC_CLKM configuration register
pub mod saradc_clkm_conf;
/**TSENS_CLK_CONF (rw) register accessor: TSENS_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`tsens_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsens_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tsens_clk_conf`] module*/
pub type TSENS_CLK_CONF = crate::Reg<tsens_clk_conf::TSENS_CLK_CONF_SPEC>;
///TSENS_CLK configuration register
pub mod tsens_clk_conf;
/**USB_DEVICE_CONF (rw) register accessor: USB_DEVICE configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`usb_device_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_device_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usb_device_conf`] module*/
pub type USB_DEVICE_CONF = crate::Reg<usb_device_conf::USB_DEVICE_CONF_SPEC>;
///USB_DEVICE configuration register
pub mod usb_device_conf;
/**INTMTX_CONF (rw) register accessor: INTMTX configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`intmtx_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intmtx_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intmtx_conf`] module*/
pub type INTMTX_CONF = crate::Reg<intmtx_conf::INTMTX_CONF_SPEC>;
///INTMTX configuration register
pub mod intmtx_conf;
/**PCNT_CONF (rw) register accessor: PCNT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pcnt_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcnt_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcnt_conf`] module*/
pub type PCNT_CONF = crate::Reg<pcnt_conf::PCNT_CONF_SPEC>;
///PCNT configuration register
pub mod pcnt_conf;
/**ETM_CONF (rw) register accessor: ETM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`etm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@etm_conf`] module*/
pub type ETM_CONF = crate::Reg<etm_conf::ETM_CONF_SPEC>;
///ETM configuration register
pub mod etm_conf;
/**PWM_CONF (rw) register accessor: PWM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pwm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwm_conf`] module*/
pub type PWM_CONF = crate::Reg<pwm_conf::PWM_CONF_SPEC>;
///PWM configuration register
pub mod pwm_conf;
/**PWM_CLK_CONF (rw) register accessor: PWM_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pwm_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwm_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwm_clk_conf`] module*/
pub type PWM_CLK_CONF = crate::Reg<pwm_clk_conf::PWM_CLK_CONF_SPEC>;
///PWM_CLK configuration register
pub mod pwm_clk_conf;
/**PARL_IO_CONF (rw) register accessor: PARL_IO configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`parl_io_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_io_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@parl_io_conf`] module*/
pub type PARL_IO_CONF = crate::Reg<parl_io_conf::PARL_IO_CONF_SPEC>;
///PARL_IO configuration register
pub mod parl_io_conf;
/**PARL_CLK_RX_CONF (rw) register accessor: PARL_CLK_RX configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`parl_clk_rx_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_clk_rx_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@parl_clk_rx_conf`] module*/
pub type PARL_CLK_RX_CONF = crate::Reg<parl_clk_rx_conf::PARL_CLK_RX_CONF_SPEC>;
///PARL_CLK_RX configuration register
pub mod parl_clk_rx_conf;
/**PARL_CLK_TX_CONF (rw) register accessor: PARL_CLK_TX configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`parl_clk_tx_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`parl_clk_tx_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@parl_clk_tx_conf`] module*/
pub type PARL_CLK_TX_CONF = crate::Reg<parl_clk_tx_conf::PARL_CLK_TX_CONF_SPEC>;
///PARL_CLK_TX configuration register
pub mod parl_clk_tx_conf;
/**PVT_MONITOR_CONF (rw) register accessor: PVT_MONITOR configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pvt_monitor_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pvt_monitor_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pvt_monitor_conf`] module*/
pub type PVT_MONITOR_CONF = crate::Reg<pvt_monitor_conf::PVT_MONITOR_CONF_SPEC>;
///PVT_MONITOR configuration register
pub mod pvt_monitor_conf;
/**PVT_MONITOR_FUNC_CLK_CONF (rw) register accessor: PVT_MONITOR function clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pvt_monitor_func_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pvt_monitor_func_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pvt_monitor_func_clk_conf`] module*/
pub type PVT_MONITOR_FUNC_CLK_CONF = crate::Reg<
    pvt_monitor_func_clk_conf::PVT_MONITOR_FUNC_CLK_CONF_SPEC,
>;
///PVT_MONITOR function clock configuration register
pub mod pvt_monitor_func_clk_conf;
/**GDMA_CONF (rw) register accessor: GDMA configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`gdma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gdma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gdma_conf`] module*/
pub type GDMA_CONF = crate::Reg<gdma_conf::GDMA_CONF_SPEC>;
///GDMA configuration register
pub mod gdma_conf;
/**SPI2_CONF (rw) register accessor: SPI2 configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi2_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi2_conf`] module*/
pub type SPI2_CONF = crate::Reg<spi2_conf::SPI2_CONF_SPEC>;
///SPI2 configuration register
pub mod spi2_conf;
/**SPI2_CLKM_CONF (rw) register accessor: SPI2_CLKM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`spi2_clkm_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spi2_clkm_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spi2_clkm_conf`] module*/
pub type SPI2_CLKM_CONF = crate::Reg<spi2_clkm_conf::SPI2_CLKM_CONF_SPEC>;
///SPI2_CLKM configuration register
pub mod spi2_clkm_conf;
/**AES_CONF (rw) register accessor: AES configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`aes_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aes_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@aes_conf`] module*/
pub type AES_CONF = crate::Reg<aes_conf::AES_CONF_SPEC>;
///AES configuration register
pub mod aes_conf;
/**SHA_CONF (rw) register accessor: SHA configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sha_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sha_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sha_conf`] module*/
pub type SHA_CONF = crate::Reg<sha_conf::SHA_CONF_SPEC>;
///SHA configuration register
pub mod sha_conf;
/**RSA_CONF (rw) register accessor: RSA configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`rsa_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rsa_conf`] module*/
pub type RSA_CONF = crate::Reg<rsa_conf::RSA_CONF_SPEC>;
///RSA configuration register
pub mod rsa_conf;
/**RSA_PD_CTRL (rw) register accessor: RSA power control register

You can [`read`](crate::generic::Reg::read) this register and get [`rsa_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rsa_pd_ctrl`] module*/
pub type RSA_PD_CTRL = crate::Reg<rsa_pd_ctrl::RSA_PD_CTRL_SPEC>;
///RSA power control register
pub mod rsa_pd_ctrl;
/**ECC_CONF (rw) register accessor: ECC configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ecc_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc_conf`] module*/
pub type ECC_CONF = crate::Reg<ecc_conf::ECC_CONF_SPEC>;
///ECC configuration register
pub mod ecc_conf;
/**ECC_PD_CTRL (rw) register accessor: ECC power control register

You can [`read`](crate::generic::Reg::read) this register and get [`ecc_pd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_pd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecc_pd_ctrl`] module*/
pub type ECC_PD_CTRL = crate::Reg<ecc_pd_ctrl::ECC_PD_CTRL_SPEC>;
///ECC power control register
pub mod ecc_pd_ctrl;
/**DS_CONF (rw) register accessor: DS configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ds_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ds_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ds_conf`] module*/
pub type DS_CONF = crate::Reg<ds_conf::DS_CONF_SPEC>;
///DS configuration register
pub mod ds_conf;
/**HMAC_CONF (rw) register accessor: HMAC configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`hmac_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hmac_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hmac_conf`] module*/
pub type HMAC_CONF = crate::Reg<hmac_conf::HMAC_CONF_SPEC>;
///HMAC configuration register
pub mod hmac_conf;
/**ECDSA_CONF (rw) register accessor: ECDSA configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ecdsa_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecdsa_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecdsa_conf`] module*/
pub type ECDSA_CONF = crate::Reg<ecdsa_conf::ECDSA_CONF_SPEC>;
///ECDSA configuration register
pub mod ecdsa_conf;
/**IOMUX_CONF (rw) register accessor: IOMUX configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`iomux_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iomux_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iomux_conf`] module*/
pub type IOMUX_CONF = crate::Reg<iomux_conf::IOMUX_CONF_SPEC>;
///IOMUX configuration register
pub mod iomux_conf;
/**IOMUX_CLK_CONF (rw) register accessor: IOMUX_CLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`iomux_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iomux_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iomux_clk_conf`] module*/
pub type IOMUX_CLK_CONF = crate::Reg<iomux_clk_conf::IOMUX_CLK_CONF_SPEC>;
///IOMUX_CLK configuration register
pub mod iomux_clk_conf;
/**MEM_MONITOR_CONF (rw) register accessor: MEM_MONITOR configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`mem_monitor_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_monitor_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mem_monitor_conf`] module*/
pub type MEM_MONITOR_CONF = crate::Reg<mem_monitor_conf::MEM_MONITOR_CONF_SPEC>;
///MEM_MONITOR configuration register
pub mod mem_monitor_conf;
/**REGDMA_CONF (rw) register accessor: REGDMA configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`regdma_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`regdma_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@regdma_conf`] module*/
pub type REGDMA_CONF = crate::Reg<regdma_conf::REGDMA_CONF_SPEC>;
///REGDMA configuration register
pub mod regdma_conf;
/**TRACE_CONF (rw) register accessor: TRACE configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`trace_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trace_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trace_conf`] module*/
pub type TRACE_CONF = crate::Reg<trace_conf::TRACE_CONF_SPEC>;
///TRACE configuration register
pub mod trace_conf;
/**ASSIST_CONF (rw) register accessor: ASSIST configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`assist_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`assist_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@assist_conf`] module*/
pub type ASSIST_CONF = crate::Reg<assist_conf::ASSIST_CONF_SPEC>;
///ASSIST configuration register
pub mod assist_conf;
/**CACHE_CONF (rw) register accessor: CACHE configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cache_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cache_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cache_conf`] module*/
pub type CACHE_CONF = crate::Reg<cache_conf::CACHE_CONF_SPEC>;
///CACHE configuration register
pub mod cache_conf;
/**MODEM_CONF (rw) register accessor: MODEM_APB configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`modem_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`modem_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@modem_conf`] module*/
pub type MODEM_CONF = crate::Reg<modem_conf::MODEM_CONF_SPEC>;
///MODEM_APB configuration register
pub mod modem_conf;
/**TIMEOUT_CONF (rw) register accessor: TIMEOUT configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`timeout_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timeout_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@timeout_conf`] module*/
pub type TIMEOUT_CONF = crate::Reg<timeout_conf::TIMEOUT_CONF_SPEC>;
///TIMEOUT configuration register
pub mod timeout_conf;
/**SYSCLK_CONF (rw) register accessor: SYSCLK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sysclk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sysclk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sysclk_conf`] module*/
pub type SYSCLK_CONF = crate::Reg<sysclk_conf::SYSCLK_CONF_SPEC>;
///SYSCLK configuration register
pub mod sysclk_conf;
/**CPU_WAITI_CONF (rw) register accessor: CPU_WAITI configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_waiti_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_waiti_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_waiti_conf`] module*/
pub type CPU_WAITI_CONF = crate::Reg<cpu_waiti_conf::CPU_WAITI_CONF_SPEC>;
///CPU_WAITI configuration register
pub mod cpu_waiti_conf;
/**CPU_FREQ_CONF (rw) register accessor: CPU_FREQ configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`cpu_freq_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cpu_freq_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpu_freq_conf`] module*/
pub type CPU_FREQ_CONF = crate::Reg<cpu_freq_conf::CPU_FREQ_CONF_SPEC>;
///CPU_FREQ configuration register
pub mod cpu_freq_conf;
/**AHB_FREQ_CONF (rw) register accessor: AHB_FREQ configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ahb_freq_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_freq_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ahb_freq_conf`] module*/
pub type AHB_FREQ_CONF = crate::Reg<ahb_freq_conf::AHB_FREQ_CONF_SPEC>;
///AHB_FREQ configuration register
pub mod ahb_freq_conf;
/**APB_FREQ_CONF (rw) register accessor: APB_FREQ configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`apb_freq_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_freq_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@apb_freq_conf`] module*/
pub type APB_FREQ_CONF = crate::Reg<apb_freq_conf::APB_FREQ_CONF_SPEC>;
///APB_FREQ configuration register
pub mod apb_freq_conf;
/**SYSCLK_FREQ_QUERY_0 (r) register accessor: SYSCLK frequency query 0 register

You can [`read`](crate::generic::Reg::read) this register and get [`sysclk_freq_query_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sysclk_freq_query_0`] module*/
pub type SYSCLK_FREQ_QUERY_0 = crate::Reg<sysclk_freq_query_0::SYSCLK_FREQ_QUERY_0_SPEC>;
///SYSCLK frequency query 0 register
pub mod sysclk_freq_query_0;
/**PLL_DIV_CLK_EN (rw) register accessor: SPLL DIV clock-gating configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pll_div_clk_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll_div_clk_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pll_div_clk_en`] module*/
pub type PLL_DIV_CLK_EN = crate::Reg<pll_div_clk_en::PLL_DIV_CLK_EN_SPEC>;
///SPLL DIV clock-gating configuration register
pub mod pll_div_clk_en;
/**CTRL_CLK_OUT_EN (rw) register accessor: CLK_OUT_EN configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl_clk_out_en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_clk_out_en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl_clk_out_en`] module*/
pub type CTRL_CLK_OUT_EN = crate::Reg<ctrl_clk_out_en::CTRL_CLK_OUT_EN_SPEC>;
///CLK_OUT_EN configuration register
pub mod ctrl_clk_out_en;
/**CTRL_TICK_CONF (rw) register accessor: TICK configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl_tick_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_tick_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl_tick_conf`] module*/
pub type CTRL_TICK_CONF = crate::Reg<ctrl_tick_conf::CTRL_TICK_CONF_SPEC>;
///TICK configuration register
pub mod ctrl_tick_conf;
/**CTRL_32K_CONF (rw) register accessor: 32KHz clock configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`ctrl_32k_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl_32k_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctrl_32k_conf`] module*/
pub type CTRL_32K_CONF = crate::Reg<ctrl_32k_conf::CTRL_32K_CONF_SPEC>;
///32KHz clock configuration register
pub mod ctrl_32k_conf;
/**SRAM_POWER_CONF_0 (rw) register accessor: HP SRAM/ROM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_power_conf_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_power_conf_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_power_conf_0`] module*/
pub type SRAM_POWER_CONF_0 = crate::Reg<sram_power_conf_0::SRAM_POWER_CONF_0_SPEC>;
///HP SRAM/ROM configuration register
pub mod sram_power_conf_0;
/**SRAM_POWER_CONF_1 (rw) register accessor: HP SRAM/ROM configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`sram_power_conf_1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_power_conf_1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sram_power_conf_1`] module*/
pub type SRAM_POWER_CONF_1 = crate::Reg<sram_power_conf_1::SRAM_POWER_CONF_1_SPEC>;
///HP SRAM/ROM configuration register
pub mod sram_power_conf_1;
/**SEC_CONF (rw) register accessor: xxxx

You can [`read`](crate::generic::Reg::read) this register and get [`sec_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sec_conf`] module*/
pub type SEC_CONF = crate::Reg<sec_conf::SEC_CONF_SPEC>;
///xxxx
pub mod sec_conf;
/**ADC_INV_PHASE_CONF (rw) register accessor: xxxx

You can [`read`](crate::generic::Reg::read) this register and get [`adc_inv_phase_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adc_inv_phase_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adc_inv_phase_conf`] module*/
pub type ADC_INV_PHASE_CONF = crate::Reg<adc_inv_phase_conf::ADC_INV_PHASE_CONF_SPEC>;
///xxxx
pub mod adc_inv_phase_conf;
/**SDM_INV_PHASE_CONF (rw) register accessor: xxxx

You can [`read`](crate::generic::Reg::read) this register and get [`sdm_inv_phase_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdm_inv_phase_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdm_inv_phase_conf`] module*/
pub type SDM_INV_PHASE_CONF = crate::Reg<sdm_inv_phase_conf::SDM_INV_PHASE_CONF_SPEC>;
///xxxx
pub mod sdm_inv_phase_conf;
/**BUS_CLK_UPDATE (rw) register accessor: xxxx

You can [`read`](crate::generic::Reg::read) this register and get [`bus_clk_update::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_clk_update::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bus_clk_update`] module*/
pub type BUS_CLK_UPDATE = crate::Reg<bus_clk_update::BUS_CLK_UPDATE_SPEC>;
///xxxx
pub mod bus_clk_update;
/**SAR_CLK_DIV (rw) register accessor: xxxx

You can [`read`](crate::generic::Reg::read) this register and get [`sar_clk_div::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_clk_div::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sar_clk_div`] module*/
pub type SAR_CLK_DIV = crate::Reg<sar_clk_div::SAR_CLK_DIV_SPEC>;
///xxxx
pub mod sar_clk_div;
/**PWDET_SAR_CLK_CONF (rw) register accessor: xxxx

You can [`read`](crate::generic::Reg::read) this register and get [`pwdet_sar_clk_conf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwdet_sar_clk_conf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwdet_sar_clk_conf`] module*/
pub type PWDET_SAR_CLK_CONF = crate::Reg<pwdet_sar_clk_conf::PWDET_SAR_CLK_CONF_SPEC>;
///xxxx
pub mod pwdet_sar_clk_conf;
/**RESET_EVENT_BYPASS (rw) register accessor: reset event bypass backdoor configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`reset_event_bypass::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_event_bypass::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reset_event_bypass`] module*/
pub type RESET_EVENT_BYPASS = crate::Reg<reset_event_bypass::RESET_EVENT_BYPASS_SPEC>;
///reset event bypass backdoor configuration register
pub mod reset_event_bypass;
/**FPGA_DEBUG (rw) register accessor: fpga debug register

You can [`read`](crate::generic::Reg::read) this register and get [`fpga_debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpga_debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fpga_debug`] module*/
pub type FPGA_DEBUG = crate::Reg<fpga_debug::FPGA_DEBUG_SPEC>;
///fpga debug register
pub mod fpga_debug;
/**CLOCK_GATE (rw) register accessor: PCR clock gating configure register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///PCR clock gating configure register
pub mod clock_gate;
/**DATE (rw) register accessor: Date register.

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///Date register.
pub mod date;
