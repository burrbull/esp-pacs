#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    bt_select: BT_SELECT,
    out: OUT,
    out_w1ts: OUT_W1TS,
    out_w1tc: OUT_W1TC,
    _reserved4: [u8; 0x0c],
    sdio_select: SDIO_SELECT,
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    _reserved8: [u8; 0x0c],
    strap: STRAP,
    in_: IN,
    _reserved10: [u8; 0x04],
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    _reserved13: [u8; 0x0c],
    pcpu_int: PCPU_INT,
    pcpu_nmi_int: PCPU_NMI_INT,
    cpusdio_int: CPUSDIO_INT,
    _reserved16: [u8; 0x0c],
    pin: [PIN; 26],
    _reserved17: [u8; 0x70],
    status_next: STATUS_NEXT,
    _reserved18: [u8; 0x04],
    func_in_sel_cfg: [FUNC_IN_SEL_CFG; 128],
    _reserved19: [u8; 0x0200],
    func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 26],
    _reserved20: [u8; 0x70],
    clock_gate: CLOCK_GATE,
    _reserved21: [u8; 0xcc],
    reg_date: REG_DATE,
}
impl RegisterBlock {
    ///0x00 - GPIO bit select register
    #[inline(always)]
    pub const fn bt_select(&self) -> &BT_SELECT {
        &self.bt_select
    }
    ///0x04 - GPIO output register
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    ///0x08 - GPIO output set register
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    ///0x0c - GPIO output clear register
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    ///0x1c - GPIO sdio select register
    #[inline(always)]
    pub const fn sdio_select(&self) -> &SDIO_SELECT {
        &self.sdio_select
    }
    ///0x20 - GPIO output enable register
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    ///0x24 - GPIO output enable set register
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    ///0x28 - GPIO output enable clear register
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    ///0x38 - pad strapping register
    #[inline(always)]
    pub const fn strap(&self) -> &STRAP {
        &self.strap
    }
    ///0x3c - GPIO input register
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    ///0x44 - GPIO interrupt status register
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x48 - GPIO interrupt status set register
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    ///0x4c - GPIO interrupt status clear register
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    ///0x5c - GPIO PRO_CPU interrupt status register
    #[inline(always)]
    pub const fn pcpu_int(&self) -> &PCPU_INT {
        &self.pcpu_int
    }
    ///0x60 - GPIO PRO_CPU(not shielded) interrupt status register
    #[inline(always)]
    pub const fn pcpu_nmi_int(&self) -> &PCPU_NMI_INT {
        &self.pcpu_nmi_int
    }
    ///0x64 - GPIO CPUSDIO interrupt status register
    #[inline(always)]
    pub const fn cpusdio_int(&self) -> &CPUSDIO_INT {
        &self.cpusdio_int
    }
    ///0x74..0xdc - GPIO pin configuration register
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    ///Iterator for array of:
    ///0x74..0xdc - GPIO pin configuration register
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    ///0x14c - GPIO interrupt source register
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    ///0x154..0x354 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func_in_sel_cfg(&self, n: usize) -> &FUNC_IN_SEL_CFG {
        &self.func_in_sel_cfg[n]
    }
    ///Iterator for array of:
    ///0x154..0x354 - GPIO input function configuration register
    #[inline(always)]
    pub fn func_in_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_IN_SEL_CFG> {
        self.func_in_sel_cfg.iter()
    }
    ///0x154 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func0_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(0)
    }
    ///0x158 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(1)
    }
    ///0x15c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(2)
    }
    ///0x160 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(3)
    }
    ///0x164 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(4)
    }
    ///0x168 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(5)
    }
    ///0x16c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(6)
    }
    ///0x170 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(7)
    }
    ///0x174 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(8)
    }
    ///0x178 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(9)
    }
    ///0x17c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(10)
    }
    ///0x180 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(11)
    }
    ///0x184 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(12)
    }
    ///0x188 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(13)
    }
    ///0x18c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(14)
    }
    ///0x190 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(15)
    }
    ///0x194 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(16)
    }
    ///0x198 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(17)
    }
    ///0x19c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(18)
    }
    ///0x1a0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(19)
    }
    ///0x1a4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(20)
    }
    ///0x1a8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(21)
    }
    ///0x1ac - GPIO input function configuration register
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(22)
    }
    ///0x1b0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(23)
    }
    ///0x1b4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(24)
    }
    ///0x1b8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(25)
    }
    ///0x1bc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(26)
    }
    ///0x1c0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(27)
    }
    ///0x1c4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(28)
    }
    ///0x1c8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(29)
    }
    ///0x1cc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(30)
    }
    ///0x1d0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(31)
    }
    ///0x1d4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(32)
    }
    ///0x1d8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(33)
    }
    ///0x1dc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(34)
    }
    ///0x1e0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(35)
    }
    ///0x1e4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func36_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(36)
    }
    ///0x1e8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func37_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(37)
    }
    ///0x1ec - GPIO input function configuration register
    #[inline(always)]
    pub const fn func38_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(38)
    }
    ///0x1f0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func39_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(39)
    }
    ///0x1f4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(40)
    }
    ///0x1f8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(41)
    }
    ///0x1fc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(42)
    }
    ///0x200 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(43)
    }
    ///0x204 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func44_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(44)
    }
    ///0x208 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(45)
    }
    ///0x20c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func46_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(46)
    }
    ///0x210 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(47)
    }
    ///0x214 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func48_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(48)
    }
    ///0x218 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func49_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(49)
    }
    ///0x21c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func50_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(50)
    }
    ///0x220 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func51_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(51)
    }
    ///0x224 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func52_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(52)
    }
    ///0x228 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func53_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(53)
    }
    ///0x22c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func54_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(54)
    }
    ///0x230 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func55_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(55)
    }
    ///0x234 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func56_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(56)
    }
    ///0x238 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func57_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(57)
    }
    ///0x23c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func58_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(58)
    }
    ///0x240 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func59_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(59)
    }
    ///0x244 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func60_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(60)
    }
    ///0x248 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func61_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(61)
    }
    ///0x24c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func62_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(62)
    }
    ///0x250 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func63_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(63)
    }
    ///0x254 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func64_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(64)
    }
    ///0x258 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func65_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(65)
    }
    ///0x25c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func66_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(66)
    }
    ///0x260 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func67_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(67)
    }
    ///0x264 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func68_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(68)
    }
    ///0x268 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func69_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(69)
    }
    ///0x26c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func70_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(70)
    }
    ///0x270 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func71_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(71)
    }
    ///0x274 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func72_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(72)
    }
    ///0x278 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func73_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(73)
    }
    ///0x27c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func74_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(74)
    }
    ///0x280 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func75_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(75)
    }
    ///0x284 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func76_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(76)
    }
    ///0x288 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func77_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(77)
    }
    ///0x28c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func78_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(78)
    }
    ///0x290 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func79_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(79)
    }
    ///0x294 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func80_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(80)
    }
    ///0x298 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func81_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(81)
    }
    ///0x29c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func82_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(82)
    }
    ///0x2a0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func83_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(83)
    }
    ///0x2a4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func84_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(84)
    }
    ///0x2a8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func85_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(85)
    }
    ///0x2ac - GPIO input function configuration register
    #[inline(always)]
    pub const fn func86_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(86)
    }
    ///0x2b0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func87_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(87)
    }
    ///0x2b4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func88_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(88)
    }
    ///0x2b8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func89_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(89)
    }
    ///0x2bc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func90_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(90)
    }
    ///0x2c0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func91_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(91)
    }
    ///0x2c4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func92_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(92)
    }
    ///0x2c8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func93_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(93)
    }
    ///0x2cc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func94_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(94)
    }
    ///0x2d0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func95_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(95)
    }
    ///0x2d4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func96_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(96)
    }
    ///0x2d8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func97_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(97)
    }
    ///0x2dc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func98_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(98)
    }
    ///0x2e0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func99_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(99)
    }
    ///0x2e4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func100_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(100)
    }
    ///0x2e8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func101_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(101)
    }
    ///0x2ec - GPIO input function configuration register
    #[inline(always)]
    pub const fn func102_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(102)
    }
    ///0x2f0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func103_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(103)
    }
    ///0x2f4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func104_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(104)
    }
    ///0x2f8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func105_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(105)
    }
    ///0x2fc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func106_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(106)
    }
    ///0x300 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func107_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(107)
    }
    ///0x304 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func108_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(108)
    }
    ///0x308 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func109_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(109)
    }
    ///0x30c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func110_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(110)
    }
    ///0x310 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func111_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(111)
    }
    ///0x314 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func112_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(112)
    }
    ///0x318 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func113_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(113)
    }
    ///0x31c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func114_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(114)
    }
    ///0x320 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func115_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(115)
    }
    ///0x324 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func116_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(116)
    }
    ///0x328 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func117_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(117)
    }
    ///0x32c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func118_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(118)
    }
    ///0x330 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func119_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(119)
    }
    ///0x334 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func120_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(120)
    }
    ///0x338 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func121_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(121)
    }
    ///0x33c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func122_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(122)
    }
    ///0x340 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func123_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(123)
    }
    ///0x344 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func124_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(124)
    }
    ///0x348 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func125_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(125)
    }
    ///0x34c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func126_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(126)
    }
    ///0x350 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func127_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(127)
    }
    ///0x554..0x5bc - GPIO output function select register
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    ///Iterator for array of:
    ///0x554..0x5bc - GPIO output function select register
    #[inline(always)]
    pub fn func_out_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_OUT_SEL_CFG> {
        self.func_out_sel_cfg.iter()
    }
    ///0x554 - GPIO output function select register
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(0)
    }
    ///0x558 - GPIO output function select register
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(1)
    }
    ///0x55c - GPIO output function select register
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(2)
    }
    ///0x560 - GPIO output function select register
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(3)
    }
    ///0x564 - GPIO output function select register
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(4)
    }
    ///0x568 - GPIO output function select register
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(5)
    }
    ///0x56c - GPIO output function select register
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(6)
    }
    ///0x570 - GPIO output function select register
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(7)
    }
    ///0x574 - GPIO output function select register
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(8)
    }
    ///0x578 - GPIO output function select register
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(9)
    }
    ///0x57c - GPIO output function select register
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(10)
    }
    ///0x580 - GPIO output function select register
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(11)
    }
    ///0x584 - GPIO output function select register
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(12)
    }
    ///0x588 - GPIO output function select register
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(13)
    }
    ///0x58c - GPIO output function select register
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(14)
    }
    ///0x590 - GPIO output function select register
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(15)
    }
    ///0x594 - GPIO output function select register
    #[inline(always)]
    pub const fn func16_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(16)
    }
    ///0x598 - GPIO output function select register
    #[inline(always)]
    pub const fn func17_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(17)
    }
    ///0x59c - GPIO output function select register
    #[inline(always)]
    pub const fn func18_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(18)
    }
    ///0x5a0 - GPIO output function select register
    #[inline(always)]
    pub const fn func19_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(19)
    }
    ///0x5a4 - GPIO output function select register
    #[inline(always)]
    pub const fn func20_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(20)
    }
    ///0x5a8 - GPIO output function select register
    #[inline(always)]
    pub const fn func21_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(21)
    }
    ///0x5ac - GPIO output function select register
    #[inline(always)]
    pub const fn func22_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(22)
    }
    ///0x5b0 - GPIO output function select register
    #[inline(always)]
    pub const fn func23_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(23)
    }
    ///0x5b4 - GPIO output function select register
    #[inline(always)]
    pub const fn func24_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(24)
    }
    ///0x5b8 - GPIO output function select register
    #[inline(always)]
    pub const fn func25_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(25)
    }
    ///0x62c - GPIO clock gate register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x6fc - GPIO version register
    #[inline(always)]
    pub const fn reg_date(&self) -> &REG_DATE {
        &self.reg_date
    }
}
/**BT_SELECT (rw) register accessor: GPIO bit select register

You can [`read`](crate::generic::Reg::read) this register and get [`bt_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bt_select`] module*/
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
///GPIO bit select register
pub mod bt_select;
/**OUT (rw) register accessor: GPIO output register

You can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out`] module*/
pub type OUT = crate::Reg<out::OUT_SPEC>;
///GPIO output register
pub mod out;
/**OUT_W1TS (w) register accessor: GPIO output set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1ts`] module*/
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
///GPIO output set register
pub mod out_w1ts;
/**OUT_W1TC (w) register accessor: GPIO output clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1tc`] module*/
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
///GPIO output clear register
pub mod out_w1tc;
/**SDIO_SELECT (rw) register accessor: GPIO sdio select register

You can [`read`](crate::generic::Reg::read) this register and get [`sdio_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sdio_select`] module*/
pub type SDIO_SELECT = crate::Reg<sdio_select::SDIO_SELECT_SPEC>;
///GPIO sdio select register
pub mod sdio_select;
/**ENABLE (rw) register accessor: GPIO output enable register

You can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable`] module*/
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
///GPIO output enable register
pub mod enable;
/**ENABLE_W1TS (w) register accessor: GPIO output enable set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1ts`] module*/
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
///GPIO output enable set register
pub mod enable_w1ts;
/**ENABLE_W1TC (w) register accessor: GPIO output enable clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1tc`] module*/
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
///GPIO output enable clear register
pub mod enable_w1tc;
/**STRAP (r) register accessor: pad strapping register

You can [`read`](crate::generic::Reg::read) this register and get [`strap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@strap`] module*/
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
///pad strapping register
pub mod strap;
/**IN (r) register accessor: GPIO input register

You can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_`] module*/
pub type IN = crate::Reg<in_::IN_SPEC>;
///GPIO input register
pub mod in_;
/**STATUS (rw) register accessor: GPIO interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///GPIO interrupt status register
pub mod status;
/**STATUS_W1TS (w) register accessor: GPIO interrupt status set register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1ts`] module*/
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
///GPIO interrupt status set register
pub mod status_w1ts;
/**STATUS_W1TC (w) register accessor: GPIO interrupt status clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1tc`] module*/
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
///GPIO interrupt status clear register
pub mod status_w1tc;
/**PCPU_INT (r) register accessor: GPIO PRO_CPU interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pcpu_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcpu_int`] module*/
pub type PCPU_INT = crate::Reg<pcpu_int::PCPU_INT_SPEC>;
///GPIO PRO_CPU interrupt status register
pub mod pcpu_int;
/**PCPU_NMI_INT (r) register accessor: GPIO PRO_CPU(not shielded) interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`pcpu_nmi_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcpu_nmi_int`] module*/
pub type PCPU_NMI_INT = crate::Reg<pcpu_nmi_int::PCPU_NMI_INT_SPEC>;
///GPIO PRO_CPU(not shielded) interrupt status register
pub mod pcpu_nmi_int;
/**CPUSDIO_INT (r) register accessor: GPIO CPUSDIO interrupt status register

You can [`read`](crate::generic::Reg::read) this register and get [`cpusdio_int::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpusdio_int`] module*/
pub type CPUSDIO_INT = crate::Reg<cpusdio_int::CPUSDIO_INT_SPEC>;
///GPIO CPUSDIO interrupt status register
pub mod cpusdio_int;
/**PIN (rw) register accessor: GPIO pin configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin`] module*/
pub type PIN = crate::Reg<pin::PIN_SPEC>;
///GPIO pin configuration register
pub mod pin;
/**STATUS_NEXT (r) register accessor: GPIO interrupt source register

You can [`read`](crate::generic::Reg::read) this register and get [`status_next::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_next`] module*/
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
///GPIO interrupt source register
pub mod status_next;
/**FUNC_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`func_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@func_in_sel_cfg`] module*/
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
///GPIO input function configuration register
pub mod func_in_sel_cfg;
/**FUNC_OUT_SEL_CFG (rw) register accessor: GPIO output function select register

You can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@func_out_sel_cfg`] module*/
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
///GPIO output function select register
pub mod func_out_sel_cfg;
/**CLOCK_GATE (rw) register accessor: GPIO clock gate register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///GPIO clock gate register
pub mod clock_gate;
/**REG_DATE (rw) register accessor: GPIO version register

You can [`read`](crate::generic::Reg::read) this register and get [`reg_date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@reg_date`] module*/
pub type REG_DATE = crate::Reg<reg_date::REG_DATE_SPEC>;
///GPIO version register
pub mod reg_date;
