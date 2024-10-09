#[doc = r"Enumeration of all the interrupts."]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - LP_WDT"]
    LP_WDT = 1,
    #[doc = "2 - LP_TIMER0"]
    LP_TIMER0 = 2,
    #[doc = "3 - LP_TIMER1"]
    LP_TIMER1 = 3,
    #[doc = "6 - PMU0"]
    PMU0 = 6,
    #[doc = "7 - PMU1"]
    PMU1 = 7,
    #[doc = "8 - LP_ANA"]
    LP_ANA = 8,
    #[doc = "9 - LP_ADC"]
    LP_ADC = 9,
    #[doc = "10 - LP_GPIO"]
    LP_GPIO = 10,
    #[doc = "11 - LP_I2C0"]
    LP_I2C0 = 11,
    #[doc = "12 - LP_I2S0"]
    LP_I2S0 = 12,
    #[doc = "14 - LP_TOUCH"]
    LP_TOUCH = 14,
    #[doc = "15 - LP_TSENS"]
    LP_TSENS = 15,
    #[doc = "16 - LP_UART"]
    LP_UART = 16,
    #[doc = "19 - LP_SYS"]
    LP_SYS = 19,
    #[doc = "20 - LP_HUK"]
    LP_HUK = 20,
    #[doc = "22 - USB_DEVICE"]
    USB_DEVICE = 22,
    #[doc = "24 - DMA"]
    DMA = 24,
    #[doc = "25 - SPI2"]
    SPI2 = 25,
    #[doc = "26 - SPI3"]
    SPI3 = 26,
    #[doc = "27 - I2S0"]
    I2S0 = 27,
    #[doc = "28 - I2S1"]
    I2S1 = 28,
    #[doc = "29 - I2S2"]
    I2S2 = 29,
    #[doc = "30 - UHCI0"]
    UHCI0 = 30,
    #[doc = "31 - UART0"]
    UART0 = 31,
    #[doc = "32 - UART1"]
    UART1 = 32,
    #[doc = "33 - UART2"]
    UART2 = 33,
    #[doc = "34 - UART3"]
    UART3 = 34,
    #[doc = "35 - UART4"]
    UART4 = 35,
    #[doc = "38 - PWM0"]
    PWM0 = 38,
    #[doc = "39 - PWM1"]
    PWM1 = 39,
    #[doc = "40 - TWAI0"]
    TWAI0 = 40,
    #[doc = "41 - TWAI1"]
    TWAI1 = 41,
    #[doc = "42 - TWAI2"]
    TWAI2 = 42,
    #[doc = "43 - RMT"]
    RMT = 43,
    #[doc = "44 - I2C0"]
    I2C0 = 44,
    #[doc = "45 - I2C1"]
    I2C1 = 45,
    #[doc = "46 - TG0_T0_LEVEL"]
    TG0_T0_LEVEL = 46,
    #[doc = "47 - TG0_T1_LEVEL"]
    TG0_T1_LEVEL = 47,
    #[doc = "48 - TG0_WDT_LEVEL"]
    TG0_WDT_LEVEL = 48,
    #[doc = "49 - TG1_T0_LEVEL"]
    TG1_T0_LEVEL = 49,
    #[doc = "50 - TG1_T1_LEVEL"]
    TG1_T1_LEVEL = 50,
    #[doc = "51 - TG1_WDT_LEVEL"]
    TG1_WDT_LEVEL = 51,
    #[doc = "52 - LEDC"]
    LEDC = 52,
    #[doc = "53 - SYSTIMER_TARGET0"]
    SYSTIMER_TARGET0 = 53,
    #[doc = "54 - SYSTIMER_TARGET1"]
    SYSTIMER_TARGET1 = 54,
    #[doc = "55 - SYSTIMER_TARGET2"]
    SYSTIMER_TARGET2 = 55,
    #[doc = "56 - AHB_PDMA_IN_CH0"]
    AHB_PDMA_IN_CH0 = 56,
    #[doc = "57 - AHB_PDMA_IN_CH1"]
    AHB_PDMA_IN_CH1 = 57,
    #[doc = "58 - AHB_PDMA_IN_CH2"]
    AHB_PDMA_IN_CH2 = 58,
    #[doc = "59 - AHB_PDMA_OUT_CH0"]
    AHB_PDMA_OUT_CH0 = 59,
    #[doc = "60 - AHB_PDMA_OUT_CH1"]
    AHB_PDMA_OUT_CH1 = 60,
    #[doc = "61 - AHB_PDMA_OUT_CH2"]
    AHB_PDMA_OUT_CH2 = 61,
    #[doc = "62 - AXI_PDMA_IN_CH0"]
    AXI_PDMA_IN_CH0 = 62,
    #[doc = "63 - AXI_PDMA_IN_CH1"]
    AXI_PDMA_IN_CH1 = 63,
    #[doc = "64 - AXI_PDMA_IN_CH2"]
    AXI_PDMA_IN_CH2 = 64,
    #[doc = "65 - AXI_PDMA_OUT_CH0"]
    AXI_PDMA_OUT_CH0 = 65,
    #[doc = "66 - AXI_PDMA_OUT_CH1"]
    AXI_PDMA_OUT_CH1 = 66,
    #[doc = "67 - AXI_PDMA_OUT_CH2"]
    AXI_PDMA_OUT_CH2 = 67,
    #[doc = "68 - RSA"]
    RSA = 68,
    #[doc = "69 - AES"]
    AES = 69,
    #[doc = "70 - SHA"]
    SHA = 70,
    #[doc = "71 - ECC"]
    ECC = 71,
    #[doc = "74 - GPIO"]
    GPIO = 74,
    #[doc = "75 - GPIO_INT1"]
    GPIO_INT1 = 75,
    #[doc = "76 - GPIO_INT2"]
    GPIO_INT2 = 76,
    #[doc = "77 - GPIO_INT3"]
    GPIO_INT3 = 77,
    #[doc = "78 - GPIO_PAD_COMP"]
    GPIO_PAD_COMP = 78,
    #[doc = "79 - FROM_CPU_INTR0"]
    FROM_CPU_INTR0 = 79,
    #[doc = "80 - FROM_CPU_INTR1"]
    FROM_CPU_INTR1 = 80,
    #[doc = "81 - FROM_CPU_INTR2"]
    FROM_CPU_INTR2 = 81,
    #[doc = "82 - FROM_CPU_INTR3"]
    FROM_CPU_INTR3 = 82,
    #[doc = "83 - CACHE"]
    CACHE = 83,
    #[doc = "85 - CSI_BRIDGE"]
    CSI_BRIDGE = 85,
    #[doc = "86 - DSI_BRIDGE"]
    DSI_BRIDGE = 86,
    #[doc = "87 - CSI"]
    CSI = 87,
    #[doc = "88 - DSI"]
    DSI = 88,
    #[doc = "95 - JPEG"]
    JPEG = 95,
    #[doc = "96 - PPA"]
    PPA = 96,
    #[doc = "100 - ISP"]
    ISP = 100,
    #[doc = "101 - I3C"]
    I3C = 101,
    #[doc = "102 - I3C_SLV"]
    I3C_SLV = 102,
    #[doc = "110 - HP_SYS"]
    HP_SYS = 110,
    #[doc = "111 - PCNT"]
    PCNT = 111,
    #[doc = "112 - PAU"]
    PAU = 112,
    #[doc = "113 - PARLIO_RX"]
    PARLIO_RX = 113,
    #[doc = "114 - PARLIO_TX"]
    PARLIO_TX = 114,
    #[doc = "115 - H264_DMA2D_OUT_CH0"]
    H264_DMA2D_OUT_CH0 = 115,
    #[doc = "116 - H264_DMA2D_OUT_CH1"]
    H264_DMA2D_OUT_CH1 = 116,
    #[doc = "117 - H264_DMA2D_OUT_CH2"]
    H264_DMA2D_OUT_CH2 = 117,
    #[doc = "118 - H264_DMA2D_OUT_CH3"]
    H264_DMA2D_OUT_CH3 = 118,
    #[doc = "119 - H264_DMA2D_OUT_CH4"]
    H264_DMA2D_OUT_CH4 = 119,
    #[doc = "120 - H264_DMA2D_IN_CH0"]
    H264_DMA2D_IN_CH0 = 120,
    #[doc = "121 - H264_DMA2D_IN_CH1"]
    H264_DMA2D_IN_CH1 = 121,
    #[doc = "122 - H264_DMA2D_IN_CH2"]
    H264_DMA2D_IN_CH2 = 122,
    #[doc = "123 - H264_DMA2D_IN_CH3"]
    H264_DMA2D_IN_CH3 = 123,
    #[doc = "124 - H264_DMA2D_IN_CH4"]
    H264_DMA2D_IN_CH4 = 124,
    #[doc = "125 - H264_DMA2D_IN_CH5"]
    H264_DMA2D_IN_CH5 = 125,
    #[doc = "126 - H264_REG"]
    H264_REG = 126,
    #[doc = "127 - ASSIST_DEBUG"]
    ASSIST_DEBUG = 127,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            1 => Ok(Interrupt::LP_WDT),
            2 => Ok(Interrupt::LP_TIMER0),
            3 => Ok(Interrupt::LP_TIMER1),
            6 => Ok(Interrupt::PMU0),
            7 => Ok(Interrupt::PMU1),
            8 => Ok(Interrupt::LP_ANA),
            9 => Ok(Interrupt::LP_ADC),
            10 => Ok(Interrupt::LP_GPIO),
            11 => Ok(Interrupt::LP_I2C0),
            12 => Ok(Interrupt::LP_I2S0),
            14 => Ok(Interrupt::LP_TOUCH),
            15 => Ok(Interrupt::LP_TSENS),
            16 => Ok(Interrupt::LP_UART),
            19 => Ok(Interrupt::LP_SYS),
            20 => Ok(Interrupt::LP_HUK),
            22 => Ok(Interrupt::USB_DEVICE),
            24 => Ok(Interrupt::DMA),
            25 => Ok(Interrupt::SPI2),
            26 => Ok(Interrupt::SPI3),
            27 => Ok(Interrupt::I2S0),
            28 => Ok(Interrupt::I2S1),
            29 => Ok(Interrupt::I2S2),
            30 => Ok(Interrupt::UHCI0),
            31 => Ok(Interrupt::UART0),
            32 => Ok(Interrupt::UART1),
            33 => Ok(Interrupt::UART2),
            34 => Ok(Interrupt::UART3),
            35 => Ok(Interrupt::UART4),
            38 => Ok(Interrupt::PWM0),
            39 => Ok(Interrupt::PWM1),
            40 => Ok(Interrupt::TWAI0),
            41 => Ok(Interrupt::TWAI1),
            42 => Ok(Interrupt::TWAI2),
            43 => Ok(Interrupt::RMT),
            44 => Ok(Interrupt::I2C0),
            45 => Ok(Interrupt::I2C1),
            46 => Ok(Interrupt::TG0_T0_LEVEL),
            47 => Ok(Interrupt::TG0_T1_LEVEL),
            48 => Ok(Interrupt::TG0_WDT_LEVEL),
            49 => Ok(Interrupt::TG1_T0_LEVEL),
            50 => Ok(Interrupt::TG1_T1_LEVEL),
            51 => Ok(Interrupt::TG1_WDT_LEVEL),
            52 => Ok(Interrupt::LEDC),
            53 => Ok(Interrupt::SYSTIMER_TARGET0),
            54 => Ok(Interrupt::SYSTIMER_TARGET1),
            55 => Ok(Interrupt::SYSTIMER_TARGET2),
            56 => Ok(Interrupt::AHB_PDMA_IN_CH0),
            57 => Ok(Interrupt::AHB_PDMA_IN_CH1),
            58 => Ok(Interrupt::AHB_PDMA_IN_CH2),
            59 => Ok(Interrupt::AHB_PDMA_OUT_CH0),
            60 => Ok(Interrupt::AHB_PDMA_OUT_CH1),
            61 => Ok(Interrupt::AHB_PDMA_OUT_CH2),
            62 => Ok(Interrupt::AXI_PDMA_IN_CH0),
            63 => Ok(Interrupt::AXI_PDMA_IN_CH1),
            64 => Ok(Interrupt::AXI_PDMA_IN_CH2),
            65 => Ok(Interrupt::AXI_PDMA_OUT_CH0),
            66 => Ok(Interrupt::AXI_PDMA_OUT_CH1),
            67 => Ok(Interrupt::AXI_PDMA_OUT_CH2),
            68 => Ok(Interrupt::RSA),
            69 => Ok(Interrupt::AES),
            70 => Ok(Interrupt::SHA),
            71 => Ok(Interrupt::ECC),
            74 => Ok(Interrupt::GPIO),
            75 => Ok(Interrupt::GPIO_INT1),
            76 => Ok(Interrupt::GPIO_INT2),
            77 => Ok(Interrupt::GPIO_INT3),
            78 => Ok(Interrupt::GPIO_PAD_COMP),
            79 => Ok(Interrupt::FROM_CPU_INTR0),
            80 => Ok(Interrupt::FROM_CPU_INTR1),
            81 => Ok(Interrupt::FROM_CPU_INTR2),
            82 => Ok(Interrupt::FROM_CPU_INTR3),
            83 => Ok(Interrupt::CACHE),
            85 => Ok(Interrupt::CSI_BRIDGE),
            86 => Ok(Interrupt::DSI_BRIDGE),
            87 => Ok(Interrupt::CSI),
            88 => Ok(Interrupt::DSI),
            95 => Ok(Interrupt::JPEG),
            96 => Ok(Interrupt::PPA),
            100 => Ok(Interrupt::ISP),
            101 => Ok(Interrupt::I3C),
            102 => Ok(Interrupt::I3C_SLV),
            110 => Ok(Interrupt::HP_SYS),
            111 => Ok(Interrupt::PCNT),
            112 => Ok(Interrupt::PAU),
            113 => Ok(Interrupt::PARLIO_RX),
            114 => Ok(Interrupt::PARLIO_TX),
            115 => Ok(Interrupt::H264_DMA2D_OUT_CH0),
            116 => Ok(Interrupt::H264_DMA2D_OUT_CH1),
            117 => Ok(Interrupt::H264_DMA2D_OUT_CH2),
            118 => Ok(Interrupt::H264_DMA2D_OUT_CH3),
            119 => Ok(Interrupt::H264_DMA2D_OUT_CH4),
            120 => Ok(Interrupt::H264_DMA2D_IN_CH0),
            121 => Ok(Interrupt::H264_DMA2D_IN_CH1),
            122 => Ok(Interrupt::H264_DMA2D_IN_CH2),
            123 => Ok(Interrupt::H264_DMA2D_IN_CH3),
            124 => Ok(Interrupt::H264_DMA2D_IN_CH4),
            125 => Ok(Interrupt::H264_DMA2D_IN_CH5),
            126 => Ok(Interrupt::H264_REG),
            127 => Ok(Interrupt::ASSIST_DEBUG),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
