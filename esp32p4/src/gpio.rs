#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
///Register block
pub struct RegisterBlock {
    bt_select: BT_SELECT,
    out: OUT,
    out_w1ts: OUT_W1TS,
    out_w1tc: OUT_W1TC,
    out1: OUT1,
    out1_w1ts: OUT1_W1TS,
    out1_w1tc: OUT1_W1TC,
    _reserved7: [u8; 0x04],
    enable: ENABLE,
    enable_w1ts: ENABLE_W1TS,
    enable_w1tc: ENABLE_W1TC,
    enable1: ENABLE1,
    enable1_w1ts: ENABLE1_W1TS,
    enable1_w1tc: ENABLE1_W1TC,
    strap: STRAP,
    in_: IN,
    in1: IN1,
    status: STATUS,
    status_w1ts: STATUS_W1TS,
    status_w1tc: STATUS_W1TC,
    status1: STATUS1,
    status1_w1ts: STATUS1_W1TS,
    status1_w1tc: STATUS1_W1TC,
    intr_0: INTR_0,
    intr1_0: INTR1_0,
    intr_1: INTR_1,
    intr1_1: INTR1_1,
    status_next: STATUS_NEXT,
    status_next1: STATUS_NEXT1,
    pin: [PIN; 57],
    _reserved29: [u8; 0x04],
    func_in_sel_cfg: [FUNC_IN_SEL_CFG; 254],
    _reserved30: [u8; 0x04],
    func_out_sel_cfg: [FUNC_OUT_SEL_CFG; 57],
    intr_2: INTR_2,
    intr1_2: INTR1_2,
    intr_3: INTR_3,
    intr1_3: INTR1_3,
    clock_gate: CLOCK_GATE,
    _reserved36: [u8; 0xb0],
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_ena: INT_ENA,
    int_clr: INT_CLR,
    zero_det0_filter_cnt: ZERO_DET0_FILTER_CNT,
    zero_det1_filter_cnt: ZERO_DET1_FILTER_CNT,
    send_seq: SEND_SEQ,
    recive_seq: RECIVE_SEQ,
    bistin_sel: BISTIN_SEL,
    bist_ctrl: BIST_CTRL,
    _reserved46: [u8; 0xd4],
    date: DATE,
}
impl RegisterBlock {
    ///0x00 - GPIO bit select register
    #[inline(always)]
    pub const fn bt_select(&self) -> &BT_SELECT {
        &self.bt_select
    }
    ///0x04 - GPIO output register for GPIO0-31
    #[inline(always)]
    pub const fn out(&self) -> &OUT {
        &self.out
    }
    ///0x08 - GPIO output set register for GPIO0-31
    #[inline(always)]
    pub const fn out_w1ts(&self) -> &OUT_W1TS {
        &self.out_w1ts
    }
    ///0x0c - GPIO output clear register for GPIO0-31
    #[inline(always)]
    pub const fn out_w1tc(&self) -> &OUT_W1TC {
        &self.out_w1tc
    }
    ///0x10 - GPIO output register for GPIO32-56
    #[inline(always)]
    pub const fn out1(&self) -> &OUT1 {
        &self.out1
    }
    ///0x14 - GPIO output set register for GPIO32-56
    #[inline(always)]
    pub const fn out1_w1ts(&self) -> &OUT1_W1TS {
        &self.out1_w1ts
    }
    ///0x18 - GPIO output clear register for GPIO32-56
    #[inline(always)]
    pub const fn out1_w1tc(&self) -> &OUT1_W1TC {
        &self.out1_w1tc
    }
    ///0x20 - GPIO output enable register for GPIO0-31
    #[inline(always)]
    pub const fn enable(&self) -> &ENABLE {
        &self.enable
    }
    ///0x24 - GPIO output enable set register for GPIO0-31
    #[inline(always)]
    pub const fn enable_w1ts(&self) -> &ENABLE_W1TS {
        &self.enable_w1ts
    }
    ///0x28 - GPIO output enable clear register for GPIO0-31
    #[inline(always)]
    pub const fn enable_w1tc(&self) -> &ENABLE_W1TC {
        &self.enable_w1tc
    }
    ///0x2c - GPIO output enable register for GPIO32-56
    #[inline(always)]
    pub const fn enable1(&self) -> &ENABLE1 {
        &self.enable1
    }
    ///0x30 - GPIO output enable set register for GPIO32-56
    #[inline(always)]
    pub const fn enable1_w1ts(&self) -> &ENABLE1_W1TS {
        &self.enable1_w1ts
    }
    ///0x34 - GPIO output enable clear register for GPIO32-56
    #[inline(always)]
    pub const fn enable1_w1tc(&self) -> &ENABLE1_W1TC {
        &self.enable1_w1tc
    }
    ///0x38 - pad strapping register
    #[inline(always)]
    pub const fn strap(&self) -> &STRAP {
        &self.strap
    }
    ///0x3c - GPIO input register for GPIO0-31
    #[inline(always)]
    pub const fn in_(&self) -> &IN {
        &self.in_
    }
    ///0x40 - GPIO input register for GPIO32-56
    #[inline(always)]
    pub const fn in1(&self) -> &IN1 {
        &self.in1
    }
    ///0x44 - GPIO interrupt status register for GPIO0-31
    #[inline(always)]
    pub const fn status(&self) -> &STATUS {
        &self.status
    }
    ///0x48 - GPIO interrupt status set register for GPIO0-31
    #[inline(always)]
    pub const fn status_w1ts(&self) -> &STATUS_W1TS {
        &self.status_w1ts
    }
    ///0x4c - GPIO interrupt status clear register for GPIO0-31
    #[inline(always)]
    pub const fn status_w1tc(&self) -> &STATUS_W1TC {
        &self.status_w1tc
    }
    ///0x50 - GPIO interrupt status register for GPIO32-56
    #[inline(always)]
    pub const fn status1(&self) -> &STATUS1 {
        &self.status1
    }
    ///0x54 - GPIO interrupt status set register for GPIO32-56
    #[inline(always)]
    pub const fn status1_w1ts(&self) -> &STATUS1_W1TS {
        &self.status1_w1ts
    }
    ///0x58 - GPIO interrupt status clear register for GPIO32-56
    #[inline(always)]
    pub const fn status1_w1tc(&self) -> &STATUS1_W1TC {
        &self.status1_w1tc
    }
    ///0x5c - GPIO interrupt 0 status register for GPIO0-31
    #[inline(always)]
    pub const fn intr_0(&self) -> &INTR_0 {
        &self.intr_0
    }
    ///0x60 - GPIO interrupt 0 status register for GPIO32-56
    #[inline(always)]
    pub const fn intr1_0(&self) -> &INTR1_0 {
        &self.intr1_0
    }
    ///0x64 - GPIO interrupt 1 status register for GPIO0-31
    #[inline(always)]
    pub const fn intr_1(&self) -> &INTR_1 {
        &self.intr_1
    }
    ///0x68 - GPIO interrupt 1 status register for GPIO32-56
    #[inline(always)]
    pub const fn intr1_1(&self) -> &INTR1_1 {
        &self.intr1_1
    }
    ///0x6c - GPIO interrupt source register for GPIO0-31
    #[inline(always)]
    pub const fn status_next(&self) -> &STATUS_NEXT {
        &self.status_next
    }
    ///0x70 - GPIO interrupt source register for GPIO32-56
    #[inline(always)]
    pub const fn status_next1(&self) -> &STATUS_NEXT1 {
        &self.status_next1
    }
    ///0x74..0x158 - GPIO pin configuration register
    #[inline(always)]
    pub const fn pin(&self, n: usize) -> &PIN {
        &self.pin[n]
    }
    ///Iterator for array of:
    ///0x74..0x158 - GPIO pin configuration register
    #[inline(always)]
    pub fn pin_iter(&self) -> impl Iterator<Item = &PIN> {
        self.pin.iter()
    }
    ///0x15c..0x554 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func_in_sel_cfg(&self, n: usize) -> &FUNC_IN_SEL_CFG {
        &self.func_in_sel_cfg[n]
    }
    ///Iterator for array of:
    ///0x15c..0x554 - GPIO input function configuration register
    #[inline(always)]
    pub fn func_in_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_IN_SEL_CFG> {
        self.func_in_sel_cfg.iter()
    }
    ///0x15c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func1_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(0)
    }
    ///0x160 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func2_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(1)
    }
    ///0x164 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func3_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(2)
    }
    ///0x168 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func4_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(3)
    }
    ///0x16c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func5_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(4)
    }
    ///0x170 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func6_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(5)
    }
    ///0x174 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func7_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(6)
    }
    ///0x178 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func8_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(7)
    }
    ///0x17c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func9_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(8)
    }
    ///0x180 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func10_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(9)
    }
    ///0x184 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func11_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(10)
    }
    ///0x188 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func12_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(11)
    }
    ///0x18c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func13_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(12)
    }
    ///0x190 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func14_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(13)
    }
    ///0x194 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func15_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(14)
    }
    ///0x198 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func16_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(15)
    }
    ///0x19c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func17_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(16)
    }
    ///0x1a0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func18_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(17)
    }
    ///0x1a4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func19_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(18)
    }
    ///0x1a8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func20_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(19)
    }
    ///0x1ac - GPIO input function configuration register
    #[inline(always)]
    pub const fn func21_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(20)
    }
    ///0x1b0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func22_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(21)
    }
    ///0x1b4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func23_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(22)
    }
    ///0x1b8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func24_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(23)
    }
    ///0x1bc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func25_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(24)
    }
    ///0x1c0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func26_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(25)
    }
    ///0x1c4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func27_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(26)
    }
    ///0x1c8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func28_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(27)
    }
    ///0x1cc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func29_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(28)
    }
    ///0x1d0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func30_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(29)
    }
    ///0x1d4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func31_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(30)
    }
    ///0x1d8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func32_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(31)
    }
    ///0x1dc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func33_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(32)
    }
    ///0x1e0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func34_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(33)
    }
    ///0x1e4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func35_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(34)
    }
    ///0x1e8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func36_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(35)
    }
    ///0x1ec - GPIO input function configuration register
    #[inline(always)]
    pub const fn func37_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(36)
    }
    ///0x1f0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func38_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(37)
    }
    ///0x1f4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func39_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(38)
    }
    ///0x1f8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func40_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(39)
    }
    ///0x1fc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func41_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(40)
    }
    ///0x200 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func42_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(41)
    }
    ///0x204 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func43_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(42)
    }
    ///0x208 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func44_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(43)
    }
    ///0x20c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func45_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(44)
    }
    ///0x210 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func46_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(45)
    }
    ///0x214 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func47_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(46)
    }
    ///0x218 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func48_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(47)
    }
    ///0x21c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func49_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(48)
    }
    ///0x220 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func50_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(49)
    }
    ///0x224 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func51_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(50)
    }
    ///0x228 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func52_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(51)
    }
    ///0x22c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func53_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(52)
    }
    ///0x230 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func54_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(53)
    }
    ///0x234 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func55_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(54)
    }
    ///0x238 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func56_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(55)
    }
    ///0x23c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func57_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(56)
    }
    ///0x240 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func58_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(57)
    }
    ///0x244 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func59_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(58)
    }
    ///0x248 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func60_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(59)
    }
    ///0x24c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func61_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(60)
    }
    ///0x250 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func62_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(61)
    }
    ///0x254 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func63_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(62)
    }
    ///0x258 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func64_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(63)
    }
    ///0x25c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func65_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(64)
    }
    ///0x260 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func66_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(65)
    }
    ///0x264 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func67_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(66)
    }
    ///0x268 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func68_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(67)
    }
    ///0x26c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func69_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(68)
    }
    ///0x270 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func70_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(69)
    }
    ///0x274 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func71_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(70)
    }
    ///0x278 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func72_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(71)
    }
    ///0x27c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func73_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(72)
    }
    ///0x280 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func74_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(73)
    }
    ///0x284 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func75_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(74)
    }
    ///0x288 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func76_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(75)
    }
    ///0x28c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func77_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(76)
    }
    ///0x290 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func78_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(77)
    }
    ///0x294 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func79_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(78)
    }
    ///0x298 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func80_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(79)
    }
    ///0x29c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func81_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(80)
    }
    ///0x2a0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func82_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(81)
    }
    ///0x2a4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func83_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(82)
    }
    ///0x2a8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func84_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(83)
    }
    ///0x2ac - GPIO input function configuration register
    #[inline(always)]
    pub const fn func85_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(84)
    }
    ///0x2b0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func86_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(85)
    }
    ///0x2b4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func87_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(86)
    }
    ///0x2b8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func88_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(87)
    }
    ///0x2bc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func89_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(88)
    }
    ///0x2c0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func90_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(89)
    }
    ///0x2c4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func91_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(90)
    }
    ///0x2c8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func92_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(91)
    }
    ///0x2cc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func93_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(92)
    }
    ///0x2d0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func94_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(93)
    }
    ///0x2d4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func95_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(94)
    }
    ///0x2d8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func96_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(95)
    }
    ///0x2dc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func97_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(96)
    }
    ///0x2e0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func98_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(97)
    }
    ///0x2e4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func99_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(98)
    }
    ///0x2e8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func100_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(99)
    }
    ///0x2ec - GPIO input function configuration register
    #[inline(always)]
    pub const fn func101_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(100)
    }
    ///0x2f0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func102_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(101)
    }
    ///0x2f4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func103_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(102)
    }
    ///0x2f8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func104_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(103)
    }
    ///0x2fc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func105_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(104)
    }
    ///0x300 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func106_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(105)
    }
    ///0x304 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func107_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(106)
    }
    ///0x308 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func108_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(107)
    }
    ///0x30c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func109_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(108)
    }
    ///0x310 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func110_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(109)
    }
    ///0x314 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func111_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(110)
    }
    ///0x318 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func112_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(111)
    }
    ///0x31c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func113_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(112)
    }
    ///0x320 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func114_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(113)
    }
    ///0x324 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func115_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(114)
    }
    ///0x328 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func116_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(115)
    }
    ///0x32c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func117_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(116)
    }
    ///0x330 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func118_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(117)
    }
    ///0x334 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func119_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(118)
    }
    ///0x338 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func120_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(119)
    }
    ///0x33c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func121_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(120)
    }
    ///0x340 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func122_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(121)
    }
    ///0x344 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func123_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(122)
    }
    ///0x348 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func124_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(123)
    }
    ///0x34c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func125_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(124)
    }
    ///0x350 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func126_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(125)
    }
    ///0x354 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func127_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(126)
    }
    ///0x358 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func128_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(127)
    }
    ///0x35c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func129_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(128)
    }
    ///0x360 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func130_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(129)
    }
    ///0x364 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func131_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(130)
    }
    ///0x368 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func132_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(131)
    }
    ///0x36c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func133_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(132)
    }
    ///0x370 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func134_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(133)
    }
    ///0x374 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func135_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(134)
    }
    ///0x378 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func136_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(135)
    }
    ///0x37c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func137_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(136)
    }
    ///0x380 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func138_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(137)
    }
    ///0x384 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func139_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(138)
    }
    ///0x388 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func140_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(139)
    }
    ///0x38c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func141_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(140)
    }
    ///0x390 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func142_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(141)
    }
    ///0x394 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func143_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(142)
    }
    ///0x398 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func144_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(143)
    }
    ///0x39c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func145_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(144)
    }
    ///0x3a0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func146_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(145)
    }
    ///0x3a4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func147_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(146)
    }
    ///0x3a8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func148_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(147)
    }
    ///0x3ac - GPIO input function configuration register
    #[inline(always)]
    pub const fn func149_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(148)
    }
    ///0x3b0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func150_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(149)
    }
    ///0x3b4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func151_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(150)
    }
    ///0x3b8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func152_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(151)
    }
    ///0x3bc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func153_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(152)
    }
    ///0x3c0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func154_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(153)
    }
    ///0x3c4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func155_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(154)
    }
    ///0x3c8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func156_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(155)
    }
    ///0x3cc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func157_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(156)
    }
    ///0x3d0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func158_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(157)
    }
    ///0x3d4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func159_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(158)
    }
    ///0x3d8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func160_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(159)
    }
    ///0x3dc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func161_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(160)
    }
    ///0x3e0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func162_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(161)
    }
    ///0x3e4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func163_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(162)
    }
    ///0x3e8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func164_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(163)
    }
    ///0x3ec - GPIO input function configuration register
    #[inline(always)]
    pub const fn func165_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(164)
    }
    ///0x3f0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func166_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(165)
    }
    ///0x3f4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func167_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(166)
    }
    ///0x3f8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func168_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(167)
    }
    ///0x3fc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func169_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(168)
    }
    ///0x400 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func170_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(169)
    }
    ///0x404 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func171_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(170)
    }
    ///0x408 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func172_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(171)
    }
    ///0x40c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func173_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(172)
    }
    ///0x410 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func174_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(173)
    }
    ///0x414 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func175_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(174)
    }
    ///0x418 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func176_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(175)
    }
    ///0x41c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func177_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(176)
    }
    ///0x420 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func178_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(177)
    }
    ///0x424 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func179_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(178)
    }
    ///0x428 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func180_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(179)
    }
    ///0x42c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func181_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(180)
    }
    ///0x430 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func182_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(181)
    }
    ///0x434 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func183_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(182)
    }
    ///0x438 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func184_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(183)
    }
    ///0x43c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func185_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(184)
    }
    ///0x440 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func186_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(185)
    }
    ///0x444 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func187_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(186)
    }
    ///0x448 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func188_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(187)
    }
    ///0x44c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func189_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(188)
    }
    ///0x450 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func190_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(189)
    }
    ///0x454 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func191_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(190)
    }
    ///0x458 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func192_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(191)
    }
    ///0x45c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func193_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(192)
    }
    ///0x460 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func194_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(193)
    }
    ///0x464 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func195_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(194)
    }
    ///0x468 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func196_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(195)
    }
    ///0x46c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func197_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(196)
    }
    ///0x470 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func198_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(197)
    }
    ///0x474 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func199_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(198)
    }
    ///0x478 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func200_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(199)
    }
    ///0x47c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func201_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(200)
    }
    ///0x480 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func202_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(201)
    }
    ///0x484 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func203_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(202)
    }
    ///0x488 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func204_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(203)
    }
    ///0x48c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func205_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(204)
    }
    ///0x490 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func206_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(205)
    }
    ///0x494 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func207_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(206)
    }
    ///0x498 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func208_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(207)
    }
    ///0x49c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func209_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(208)
    }
    ///0x4a0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func210_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(209)
    }
    ///0x4a4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func211_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(210)
    }
    ///0x4a8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func212_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(211)
    }
    ///0x4ac - GPIO input function configuration register
    #[inline(always)]
    pub const fn func213_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(212)
    }
    ///0x4b0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func214_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(213)
    }
    ///0x4b4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func215_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(214)
    }
    ///0x4b8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func216_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(215)
    }
    ///0x4bc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func217_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(216)
    }
    ///0x4c0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func218_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(217)
    }
    ///0x4c4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func219_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(218)
    }
    ///0x4c8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func220_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(219)
    }
    ///0x4cc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func221_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(220)
    }
    ///0x4d0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func222_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(221)
    }
    ///0x4d4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func223_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(222)
    }
    ///0x4d8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func224_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(223)
    }
    ///0x4dc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func225_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(224)
    }
    ///0x4e0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func226_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(225)
    }
    ///0x4e4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func227_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(226)
    }
    ///0x4e8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func228_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(227)
    }
    ///0x4ec - GPIO input function configuration register
    #[inline(always)]
    pub const fn func229_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(228)
    }
    ///0x4f0 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func230_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(229)
    }
    ///0x4f4 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func231_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(230)
    }
    ///0x4f8 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func232_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(231)
    }
    ///0x4fc - GPIO input function configuration register
    #[inline(always)]
    pub const fn func233_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(232)
    }
    ///0x500 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func234_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(233)
    }
    ///0x504 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func235_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(234)
    }
    ///0x508 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func236_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(235)
    }
    ///0x50c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func237_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(236)
    }
    ///0x510 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func238_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(237)
    }
    ///0x514 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func239_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(238)
    }
    ///0x518 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func240_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(239)
    }
    ///0x51c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func241_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(240)
    }
    ///0x520 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func242_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(241)
    }
    ///0x524 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func243_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(242)
    }
    ///0x528 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func244_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(243)
    }
    ///0x52c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func245_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(244)
    }
    ///0x530 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func246_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(245)
    }
    ///0x534 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func247_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(246)
    }
    ///0x538 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func248_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(247)
    }
    ///0x53c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func249_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(248)
    }
    ///0x540 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func250_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(249)
    }
    ///0x544 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func251_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(250)
    }
    ///0x548 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func252_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(251)
    }
    ///0x54c - GPIO input function configuration register
    #[inline(always)]
    pub const fn func253_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(252)
    }
    ///0x550 - GPIO input function configuration register
    #[inline(always)]
    pub const fn func254_in_sel_cfg(&self) -> &FUNC_IN_SEL_CFG {
        self.func_in_sel_cfg(253)
    }
    ///0x558..0x63c - GPIO output function select register
    #[inline(always)]
    pub const fn func_out_sel_cfg(&self, n: usize) -> &FUNC_OUT_SEL_CFG {
        &self.func_out_sel_cfg[n]
    }
    ///Iterator for array of:
    ///0x558..0x63c - GPIO output function select register
    #[inline(always)]
    pub fn func_out_sel_cfg_iter(&self) -> impl Iterator<Item = &FUNC_OUT_SEL_CFG> {
        self.func_out_sel_cfg.iter()
    }
    ///0x558 - GPIO output function select register
    #[inline(always)]
    pub const fn func0_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(0)
    }
    ///0x55c - GPIO output function select register
    #[inline(always)]
    pub const fn func1_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(1)
    }
    ///0x560 - GPIO output function select register
    #[inline(always)]
    pub const fn func2_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(2)
    }
    ///0x564 - GPIO output function select register
    #[inline(always)]
    pub const fn func3_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(3)
    }
    ///0x568 - GPIO output function select register
    #[inline(always)]
    pub const fn func4_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(4)
    }
    ///0x56c - GPIO output function select register
    #[inline(always)]
    pub const fn func5_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(5)
    }
    ///0x570 - GPIO output function select register
    #[inline(always)]
    pub const fn func6_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(6)
    }
    ///0x574 - GPIO output function select register
    #[inline(always)]
    pub const fn func7_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(7)
    }
    ///0x578 - GPIO output function select register
    #[inline(always)]
    pub const fn func8_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(8)
    }
    ///0x57c - GPIO output function select register
    #[inline(always)]
    pub const fn func9_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(9)
    }
    ///0x580 - GPIO output function select register
    #[inline(always)]
    pub const fn func10_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(10)
    }
    ///0x584 - GPIO output function select register
    #[inline(always)]
    pub const fn func11_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(11)
    }
    ///0x588 - GPIO output function select register
    #[inline(always)]
    pub const fn func12_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(12)
    }
    ///0x58c - GPIO output function select register
    #[inline(always)]
    pub const fn func13_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(13)
    }
    ///0x590 - GPIO output function select register
    #[inline(always)]
    pub const fn func14_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(14)
    }
    ///0x594 - GPIO output function select register
    #[inline(always)]
    pub const fn func15_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(15)
    }
    ///0x598 - GPIO output function select register
    #[inline(always)]
    pub const fn func16_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(16)
    }
    ///0x59c - GPIO output function select register
    #[inline(always)]
    pub const fn func17_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(17)
    }
    ///0x5a0 - GPIO output function select register
    #[inline(always)]
    pub const fn func18_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(18)
    }
    ///0x5a4 - GPIO output function select register
    #[inline(always)]
    pub const fn func19_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(19)
    }
    ///0x5a8 - GPIO output function select register
    #[inline(always)]
    pub const fn func20_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(20)
    }
    ///0x5ac - GPIO output function select register
    #[inline(always)]
    pub const fn func21_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(21)
    }
    ///0x5b0 - GPIO output function select register
    #[inline(always)]
    pub const fn func22_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(22)
    }
    ///0x5b4 - GPIO output function select register
    #[inline(always)]
    pub const fn func23_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(23)
    }
    ///0x5b8 - GPIO output function select register
    #[inline(always)]
    pub const fn func24_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(24)
    }
    ///0x5bc - GPIO output function select register
    #[inline(always)]
    pub const fn func25_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(25)
    }
    ///0x5c0 - GPIO output function select register
    #[inline(always)]
    pub const fn func26_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(26)
    }
    ///0x5c4 - GPIO output function select register
    #[inline(always)]
    pub const fn func27_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(27)
    }
    ///0x5c8 - GPIO output function select register
    #[inline(always)]
    pub const fn func28_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(28)
    }
    ///0x5cc - GPIO output function select register
    #[inline(always)]
    pub const fn func29_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(29)
    }
    ///0x5d0 - GPIO output function select register
    #[inline(always)]
    pub const fn func30_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(30)
    }
    ///0x5d4 - GPIO output function select register
    #[inline(always)]
    pub const fn func31_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(31)
    }
    ///0x5d8 - GPIO output function select register
    #[inline(always)]
    pub const fn func32_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(32)
    }
    ///0x5dc - GPIO output function select register
    #[inline(always)]
    pub const fn func33_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(33)
    }
    ///0x5e0 - GPIO output function select register
    #[inline(always)]
    pub const fn func34_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(34)
    }
    ///0x5e4 - GPIO output function select register
    #[inline(always)]
    pub const fn func35_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(35)
    }
    ///0x5e8 - GPIO output function select register
    #[inline(always)]
    pub const fn func36_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(36)
    }
    ///0x5ec - GPIO output function select register
    #[inline(always)]
    pub const fn func37_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(37)
    }
    ///0x5f0 - GPIO output function select register
    #[inline(always)]
    pub const fn func38_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(38)
    }
    ///0x5f4 - GPIO output function select register
    #[inline(always)]
    pub const fn func39_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(39)
    }
    ///0x5f8 - GPIO output function select register
    #[inline(always)]
    pub const fn func40_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(40)
    }
    ///0x5fc - GPIO output function select register
    #[inline(always)]
    pub const fn func41_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(41)
    }
    ///0x600 - GPIO output function select register
    #[inline(always)]
    pub const fn func42_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(42)
    }
    ///0x604 - GPIO output function select register
    #[inline(always)]
    pub const fn func43_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(43)
    }
    ///0x608 - GPIO output function select register
    #[inline(always)]
    pub const fn func44_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(44)
    }
    ///0x60c - GPIO output function select register
    #[inline(always)]
    pub const fn func45_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(45)
    }
    ///0x610 - GPIO output function select register
    #[inline(always)]
    pub const fn func46_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(46)
    }
    ///0x614 - GPIO output function select register
    #[inline(always)]
    pub const fn func47_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(47)
    }
    ///0x618 - GPIO output function select register
    #[inline(always)]
    pub const fn func48_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(48)
    }
    ///0x61c - GPIO output function select register
    #[inline(always)]
    pub const fn func49_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(49)
    }
    ///0x620 - GPIO output function select register
    #[inline(always)]
    pub const fn func50_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(50)
    }
    ///0x624 - GPIO output function select register
    #[inline(always)]
    pub const fn func51_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(51)
    }
    ///0x628 - GPIO output function select register
    #[inline(always)]
    pub const fn func52_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(52)
    }
    ///0x62c - GPIO output function select register
    #[inline(always)]
    pub const fn func53_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(53)
    }
    ///0x630 - GPIO output function select register
    #[inline(always)]
    pub const fn func54_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(54)
    }
    ///0x634 - GPIO output function select register
    #[inline(always)]
    pub const fn func55_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(55)
    }
    ///0x638 - GPIO output function select register
    #[inline(always)]
    pub const fn func56_out_sel_cfg(&self) -> &FUNC_OUT_SEL_CFG {
        self.func_out_sel_cfg(56)
    }
    ///0x63c - GPIO interrupt 2 status register for GPIO0-31
    #[inline(always)]
    pub const fn intr_2(&self) -> &INTR_2 {
        &self.intr_2
    }
    ///0x640 - GPIO interrupt 2 status register for GPIO32-56
    #[inline(always)]
    pub const fn intr1_2(&self) -> &INTR1_2 {
        &self.intr1_2
    }
    ///0x644 - GPIO interrupt 3 status register for GPIO0-31
    #[inline(always)]
    pub const fn intr_3(&self) -> &INTR_3 {
        &self.intr_3
    }
    ///0x648 - GPIO interrupt 3 status register for GPIO32-56
    #[inline(always)]
    pub const fn intr1_3(&self) -> &INTR1_3 {
        &self.intr1_3
    }
    ///0x64c - GPIO clock gate register
    #[inline(always)]
    pub const fn clock_gate(&self) -> &CLOCK_GATE {
        &self.clock_gate
    }
    ///0x700 - analog comparator interrupt raw
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    ///0x704 - analog comparator interrupt status
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    ///0x708 - analog comparator interrupt enable
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    ///0x70c - analog comparator interrupt clear
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    ///0x710 - GPIO analog comparator zero detect filter count
    #[inline(always)]
    pub const fn zero_det0_filter_cnt(&self) -> &ZERO_DET0_FILTER_CNT {
        &self.zero_det0_filter_cnt
    }
    ///0x714 - GPIO analog comparator zero detect filter count
    #[inline(always)]
    pub const fn zero_det1_filter_cnt(&self) -> &ZERO_DET1_FILTER_CNT {
        &self.zero_det1_filter_cnt
    }
    ///0x718 - High speed sdio pad bist send sequence
    #[inline(always)]
    pub const fn send_seq(&self) -> &SEND_SEQ {
        &self.send_seq
    }
    ///0x71c - High speed sdio pad bist recive sequence
    #[inline(always)]
    pub const fn recive_seq(&self) -> &RECIVE_SEQ {
        &self.recive_seq
    }
    ///0x720 - High speed sdio pad bist in pad sel
    #[inline(always)]
    pub const fn bistin_sel(&self) -> &BISTIN_SEL {
        &self.bistin_sel
    }
    ///0x724 - High speed sdio pad bist control
    #[inline(always)]
    pub const fn bist_ctrl(&self) -> &BIST_CTRL {
        &self.bist_ctrl
    }
    ///0x7fc - GPIO version register
    #[inline(always)]
    pub const fn date(&self) -> &DATE {
        &self.date
    }
}
/**BT_SELECT (rw) register accessor: GPIO bit select register

You can [`read`](crate::generic::Reg::read) this register and get [`bt_select::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bt_select::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bt_select`] module*/
pub type BT_SELECT = crate::Reg<bt_select::BT_SELECT_SPEC>;
///GPIO bit select register
pub mod bt_select;
/**OUT (rw) register accessor: GPIO output register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`out::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out`] module*/
pub type OUT = crate::Reg<out::OUT_SPEC>;
///GPIO output register for GPIO0-31
pub mod out;
/**OUT_W1TS (w) register accessor: GPIO output set register for GPIO0-31

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1ts`] module*/
pub type OUT_W1TS = crate::Reg<out_w1ts::OUT_W1TS_SPEC>;
///GPIO output set register for GPIO0-31
pub mod out_w1ts;
/**OUT_W1TC (w) register accessor: GPIO output clear register for GPIO0-31

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out_w1tc`] module*/
pub type OUT_W1TC = crate::Reg<out_w1tc::OUT_W1TC_SPEC>;
///GPIO output clear register for GPIO0-31
pub mod out_w1tc;
/**OUT1 (rw) register accessor: GPIO output register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`out1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out1`] module*/
pub type OUT1 = crate::Reg<out1::OUT1_SPEC>;
///GPIO output register for GPIO32-56
pub mod out1;
/**OUT1_W1TS (w) register accessor: GPIO output set register for GPIO32-56

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out1_w1ts`] module*/
pub type OUT1_W1TS = crate::Reg<out1_w1ts::OUT1_W1TS_SPEC>;
///GPIO output set register for GPIO32-56
pub mod out1_w1ts;
/**OUT1_W1TC (w) register accessor: GPIO output clear register for GPIO32-56

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@out1_w1tc`] module*/
pub type OUT1_W1TC = crate::Reg<out1_w1tc::OUT1_W1TC_SPEC>;
///GPIO output clear register for GPIO32-56
pub mod out1_w1tc;
/**ENABLE (rw) register accessor: GPIO output enable register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable`] module*/
pub type ENABLE = crate::Reg<enable::ENABLE_SPEC>;
///GPIO output enable register for GPIO0-31
pub mod enable;
/**ENABLE_W1TS (w) register accessor: GPIO output enable set register for GPIO0-31

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1ts`] module*/
pub type ENABLE_W1TS = crate::Reg<enable_w1ts::ENABLE_W1TS_SPEC>;
///GPIO output enable set register for GPIO0-31
pub mod enable_w1ts;
/**ENABLE_W1TC (w) register accessor: GPIO output enable clear register for GPIO0-31

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable_w1tc`] module*/
pub type ENABLE_W1TC = crate::Reg<enable_w1tc::ENABLE_W1TC_SPEC>;
///GPIO output enable clear register for GPIO0-31
pub mod enable_w1tc;
/**ENABLE1 (rw) register accessor: GPIO output enable register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`enable1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable1`] module*/
pub type ENABLE1 = crate::Reg<enable1::ENABLE1_SPEC>;
///GPIO output enable register for GPIO32-56
pub mod enable1;
/**ENABLE1_W1TS (w) register accessor: GPIO output enable set register for GPIO32-56

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable1_w1ts`] module*/
pub type ENABLE1_W1TS = crate::Reg<enable1_w1ts::ENABLE1_W1TS_SPEC>;
///GPIO output enable set register for GPIO32-56
pub mod enable1_w1ts;
/**ENABLE1_W1TC (w) register accessor: GPIO output enable clear register for GPIO32-56

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@enable1_w1tc`] module*/
pub type ENABLE1_W1TC = crate::Reg<enable1_w1tc::ENABLE1_W1TC_SPEC>;
///GPIO output enable clear register for GPIO32-56
pub mod enable1_w1tc;
/**STRAP (r) register accessor: pad strapping register

You can [`read`](crate::generic::Reg::read) this register and get [`strap::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@strap`] module*/
pub type STRAP = crate::Reg<strap::STRAP_SPEC>;
///pad strapping register
pub mod strap;
/**IN (r) register accessor: GPIO input register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`in_::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in_`] module*/
pub type IN = crate::Reg<in_::IN_SPEC>;
///GPIO input register for GPIO0-31
pub mod in_;
/**IN1 (r) register accessor: GPIO input register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`in1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@in1`] module*/
pub type IN1 = crate::Reg<in1::IN1_SPEC>;
///GPIO input register for GPIO32-56
pub mod in1;
/**STATUS (rw) register accessor: GPIO interrupt status register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status`] module*/
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
///GPIO interrupt status register for GPIO0-31
pub mod status;
/**STATUS_W1TS (w) register accessor: GPIO interrupt status set register for GPIO0-31

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1ts`] module*/
pub type STATUS_W1TS = crate::Reg<status_w1ts::STATUS_W1TS_SPEC>;
///GPIO interrupt status set register for GPIO0-31
pub mod status_w1ts;
/**STATUS_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO0-31

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_w1tc`] module*/
pub type STATUS_W1TC = crate::Reg<status_w1tc::STATUS_W1TC_SPEC>;
///GPIO interrupt status clear register for GPIO0-31
pub mod status_w1tc;
/**STATUS1 (rw) register accessor: GPIO interrupt status register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`status1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status1`] module*/
pub type STATUS1 = crate::Reg<status1::STATUS1_SPEC>;
///GPIO interrupt status register for GPIO32-56
pub mod status1;
/**STATUS1_W1TS (w) register accessor: GPIO interrupt status set register for GPIO32-56

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1ts::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status1_w1ts`] module*/
pub type STATUS1_W1TS = crate::Reg<status1_w1ts::STATUS1_W1TS_SPEC>;
///GPIO interrupt status set register for GPIO32-56
pub mod status1_w1ts;
/**STATUS1_W1TC (w) register accessor: GPIO interrupt status clear register for GPIO32-56

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`status1_w1tc::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status1_w1tc`] module*/
pub type STATUS1_W1TC = crate::Reg<status1_w1tc::STATUS1_W1TC_SPEC>;
///GPIO interrupt status clear register for GPIO32-56
pub mod status1_w1tc;
/**INTR_0 (r) register accessor: GPIO interrupt 0 status register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`intr_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_0`] module*/
pub type INTR_0 = crate::Reg<intr_0::INTR_0_SPEC>;
///GPIO interrupt 0 status register for GPIO0-31
pub mod intr_0;
/**INTR1_0 (r) register accessor: GPIO interrupt 0 status register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`intr1_0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr1_0`] module*/
pub type INTR1_0 = crate::Reg<intr1_0::INTR1_0_SPEC>;
///GPIO interrupt 0 status register for GPIO32-56
pub mod intr1_0;
/**INTR_1 (r) register accessor: GPIO interrupt 1 status register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`intr_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_1`] module*/
pub type INTR_1 = crate::Reg<intr_1::INTR_1_SPEC>;
///GPIO interrupt 1 status register for GPIO0-31
pub mod intr_1;
/**INTR1_1 (r) register accessor: GPIO interrupt 1 status register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`intr1_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr1_1`] module*/
pub type INTR1_1 = crate::Reg<intr1_1::INTR1_1_SPEC>;
///GPIO interrupt 1 status register for GPIO32-56
pub mod intr1_1;
/**STATUS_NEXT (r) register accessor: GPIO interrupt source register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`status_next::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_next`] module*/
pub type STATUS_NEXT = crate::Reg<status_next::STATUS_NEXT_SPEC>;
///GPIO interrupt source register for GPIO0-31
pub mod status_next;
/**STATUS_NEXT1 (r) register accessor: GPIO interrupt source register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`status_next1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@status_next1`] module*/
pub type STATUS_NEXT1 = crate::Reg<status_next1::STATUS_NEXT1_SPEC>;
///GPIO interrupt source register for GPIO32-56
pub mod status_next1;
/**PIN (rw) register accessor: GPIO pin configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`pin::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pin`] module*/
pub type PIN = crate::Reg<pin::PIN_SPEC>;
///GPIO pin configuration register
pub mod pin;
/**FUNC_OUT_SEL_CFG (rw) register accessor: GPIO output function select register

You can [`read`](crate::generic::Reg::read) this register and get [`func_out_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_out_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@func_out_sel_cfg`] module*/
pub type FUNC_OUT_SEL_CFG = crate::Reg<func_out_sel_cfg::FUNC_OUT_SEL_CFG_SPEC>;
///GPIO output function select register
pub mod func_out_sel_cfg;
/**INTR_2 (r) register accessor: GPIO interrupt 2 status register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`intr_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_2`] module*/
pub type INTR_2 = crate::Reg<intr_2::INTR_2_SPEC>;
///GPIO interrupt 2 status register for GPIO0-31
pub mod intr_2;
/**INTR1_2 (r) register accessor: GPIO interrupt 2 status register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`intr1_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr1_2`] module*/
pub type INTR1_2 = crate::Reg<intr1_2::INTR1_2_SPEC>;
///GPIO interrupt 2 status register for GPIO32-56
pub mod intr1_2;
/**INTR_3 (r) register accessor: GPIO interrupt 3 status register for GPIO0-31

You can [`read`](crate::generic::Reg::read) this register and get [`intr_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr_3`] module*/
pub type INTR_3 = crate::Reg<intr_3::INTR_3_SPEC>;
///GPIO interrupt 3 status register for GPIO0-31
pub mod intr_3;
/**INTR1_3 (r) register accessor: GPIO interrupt 3 status register for GPIO32-56

You can [`read`](crate::generic::Reg::read) this register and get [`intr1_3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intr1_3`] module*/
pub type INTR1_3 = crate::Reg<intr1_3::INTR1_3_SPEC>;
///GPIO interrupt 3 status register for GPIO32-56
pub mod intr1_3;
/**CLOCK_GATE (rw) register accessor: GPIO clock gate register

You can [`read`](crate::generic::Reg::read) this register and get [`clock_gate::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clock_gate::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@clock_gate`] module*/
pub type CLOCK_GATE = crate::Reg<clock_gate::CLOCK_GATE_SPEC>;
///GPIO clock gate register
pub mod clock_gate;
/**INT_RAW (rw) register accessor: analog comparator interrupt raw

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_raw`] module*/
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
///analog comparator interrupt raw
pub mod int_raw;
/**INT_ST (r) register accessor: analog comparator interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_st`] module*/
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
///analog comparator interrupt status
pub mod int_st;
/**INT_ENA (rw) register accessor: analog comparator interrupt enable

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_ena`] module*/
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
///analog comparator interrupt enable
pub mod int_ena;
/**INT_CLR (w) register accessor: analog comparator interrupt clear

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@int_clr`] module*/
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
///analog comparator interrupt clear
pub mod int_clr;
/**ZERO_DET0_FILTER_CNT (rw) register accessor: GPIO analog comparator zero detect filter count

You can [`read`](crate::generic::Reg::read) this register and get [`zero_det0_filter_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zero_det0_filter_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@zero_det0_filter_cnt`] module*/
pub type ZERO_DET0_FILTER_CNT = crate::Reg<zero_det0_filter_cnt::ZERO_DET0_FILTER_CNT_SPEC>;
///GPIO analog comparator zero detect filter count
pub mod zero_det0_filter_cnt;
/**ZERO_DET1_FILTER_CNT (rw) register accessor: GPIO analog comparator zero detect filter count

You can [`read`](crate::generic::Reg::read) this register and get [`zero_det1_filter_cnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`zero_det1_filter_cnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@zero_det1_filter_cnt`] module*/
pub type ZERO_DET1_FILTER_CNT = crate::Reg<zero_det1_filter_cnt::ZERO_DET1_FILTER_CNT_SPEC>;
///GPIO analog comparator zero detect filter count
pub mod zero_det1_filter_cnt;
/**SEND_SEQ (rw) register accessor: High speed sdio pad bist send sequence

You can [`read`](crate::generic::Reg::read) this register and get [`send_seq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`send_seq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@send_seq`] module*/
pub type SEND_SEQ = crate::Reg<send_seq::SEND_SEQ_SPEC>;
///High speed sdio pad bist send sequence
pub mod send_seq;
/**RECIVE_SEQ (r) register accessor: High speed sdio pad bist recive sequence

You can [`read`](crate::generic::Reg::read) this register and get [`recive_seq::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@recive_seq`] module*/
pub type RECIVE_SEQ = crate::Reg<recive_seq::RECIVE_SEQ_SPEC>;
///High speed sdio pad bist recive sequence
pub mod recive_seq;
/**BISTIN_SEL (rw) register accessor: High speed sdio pad bist in pad sel

You can [`read`](crate::generic::Reg::read) this register and get [`bistin_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bistin_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bistin_sel`] module*/
pub type BISTIN_SEL = crate::Reg<bistin_sel::BISTIN_SEL_SPEC>;
///High speed sdio pad bist in pad sel
pub mod bistin_sel;
/**BIST_CTRL (rw) register accessor: High speed sdio pad bist control

You can [`read`](crate::generic::Reg::read) this register and get [`bist_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bist_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bist_ctrl`] module*/
pub type BIST_CTRL = crate::Reg<bist_ctrl::BIST_CTRL_SPEC>;
///High speed sdio pad bist control
pub mod bist_ctrl;
/**DATE (rw) register accessor: GPIO version register

You can [`read`](crate::generic::Reg::read) this register and get [`date::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@date`] module*/
pub type DATE = crate::Reg<date::DATE_SPEC>;
///GPIO version register
pub mod date;
/**FUNC_IN_SEL_CFG (rw) register accessor: GPIO input function configuration register

You can [`read`](crate::generic::Reg::read) this register and get [`func_in_sel_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_in_sel_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@func_in_sel_cfg`] module*/
pub type FUNC_IN_SEL_CFG = crate::Reg<func_in_sel_cfg::FUNC_IN_SEL_CFG_SPEC>;
///GPIO input function configuration register
pub mod func_in_sel_cfg;
