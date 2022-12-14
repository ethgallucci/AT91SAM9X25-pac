#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOWRES` reader - Resolution"]
pub type LOWRES_R = crate::BitReader<LOWRES_A>;
#[doc = "Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOWRES_A {
    #[doc = "0: 10-bit resolution"]
    BITS_10 = 0,
    #[doc = "1: 8-bit resolution"]
    BITS_8 = 1,
}
impl From<LOWRES_A> for bool {
    #[inline(always)]
    fn from(variant: LOWRES_A) -> Self {
        variant as u8 != 0
    }
}
impl LOWRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOWRES_A {
        match self.bits {
            false => LOWRES_A::BITS_10,
            true => LOWRES_A::BITS_8,
        }
    }
    #[doc = "Checks if the value of the field is `BITS_10`"]
    #[inline(always)]
    pub fn is_bits_10(&self) -> bool {
        *self == LOWRES_A::BITS_10
    }
    #[doc = "Checks if the value of the field is `BITS_8`"]
    #[inline(always)]
    pub fn is_bits_8(&self) -> bool {
        *self == LOWRES_A::BITS_8
    }
}
#[doc = "Field `LOWRES` writer - Resolution"]
pub type LOWRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, LOWRES_A, O>;
impl<'a, const O: u8> LOWRES_W<'a, O> {
    #[doc = "10-bit resolution"]
    #[inline(always)]
    pub fn bits_10(self) -> &'a mut W {
        self.variant(LOWRES_A::BITS_10)
    }
    #[doc = "8-bit resolution"]
    #[inline(always)]
    pub fn bits_8(self) -> &'a mut W {
        self.variant(LOWRES_A::BITS_8)
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode"]
pub type SLEEP_R = crate::BitReader<SLEEP_A>;
#[doc = "Sleep Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SLEEP_A {
    #[doc = "0: Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    NORMAL = 0,
    #[doc = "1: Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    SLEEP = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::NORMAL,
            true => SLEEP_A::SLEEP,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == SLEEP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SLEEP`"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == SLEEP_A::SLEEP
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, SLEEP_A, O>;
impl<'a, const O: u8> SLEEP_W<'a, O> {
    #[doc = "Normal Mode: The ADC Core and reference voltage circuitry are kept ON between conversions"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(SLEEP_A::NORMAL)
    }
    #[doc = "Sleep Mode: The ADC Core and reference voltage circuitry are OFF between conversions"]
    #[inline(always)]
    pub fn sleep(self) -> &'a mut W {
        self.variant(SLEEP_A::SLEEP)
    }
}
#[doc = "Field `FWUP` reader - Fast Wake Up"]
pub type FWUP_R = crate::BitReader<FWUP_A>;
#[doc = "Fast Wake Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FWUP_A {
    #[doc = "0: Normal Sleep Mode: The sleep mode is defined by the SLEEP bit"]
    OFF = 0,
    #[doc = "1: Fast Wake Up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    ON = 1,
}
impl From<FWUP_A> for bool {
    #[inline(always)]
    fn from(variant: FWUP_A) -> Self {
        variant as u8 != 0
    }
}
impl FWUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWUP_A {
        match self.bits {
            false => FWUP_A::OFF,
            true => FWUP_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == FWUP_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == FWUP_A::ON
    }
}
#[doc = "Field `FWUP` writer - Fast Wake Up"]
pub type FWUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, FWUP_A, O>;
impl<'a, const O: u8> FWUP_W<'a, O> {
    #[doc = "Normal Sleep Mode: The sleep mode is defined by the SLEEP bit"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(FWUP_A::OFF)
    }
    #[doc = "Fast Wake Up Sleep Mode: The Voltage reference is ON between conversions and ADC Core is OFF"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(FWUP_A::ON)
    }
}
#[doc = "Field `PRESCAL` reader - Prescaler Rate Selection"]
pub type PRESCAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRESCAL` writer - Prescaler Rate Selection"]
pub type PRESCAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 8, O>;
#[doc = "Field `STARTUP` reader - Start Up Time"]
pub type STARTUP_R = crate::FieldReader<u8, STARTUP_A>;
#[doc = "Start Up Time\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum STARTUP_A {
    #[doc = "0: 0 periods of ADCClock"]
    SUT0 = 0,
    #[doc = "1: 8 periods of ADCClock"]
    SUT8 = 1,
    #[doc = "2: 16 periods of ADCClock"]
    SUT16 = 2,
    #[doc = "3: 24 periods of ADCClock"]
    SUT24 = 3,
    #[doc = "4: 64 periods of ADCClock"]
    SUT64 = 4,
    #[doc = "5: 80 periods of ADCClock"]
    SUT80 = 5,
    #[doc = "6: 96 periods of ADCClock"]
    SUT96 = 6,
    #[doc = "7: 112 periods of ADCClock"]
    SUT112 = 7,
    #[doc = "8: 512 periods of ADCClock"]
    SUT512 = 8,
    #[doc = "9: 576 periods of ADCClock"]
    SUT576 = 9,
    #[doc = "10: 640 periods of ADCClock"]
    SUT640 = 10,
    #[doc = "11: 704 periods of ADCClock"]
    SUT704 = 11,
    #[doc = "12: 768 periods of ADCClock"]
    SUT768 = 12,
    #[doc = "13: 832 periods of ADCClock"]
    SUT832 = 13,
    #[doc = "14: 896 periods of ADCClock"]
    SUT896 = 14,
    #[doc = "15: 960 periods of ADCClock"]
    SUT960 = 15,
}
impl From<STARTUP_A> for u8 {
    #[inline(always)]
    fn from(variant: STARTUP_A) -> Self {
        variant as _
    }
}
impl STARTUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STARTUP_A {
        match self.bits {
            0 => STARTUP_A::SUT0,
            1 => STARTUP_A::SUT8,
            2 => STARTUP_A::SUT16,
            3 => STARTUP_A::SUT24,
            4 => STARTUP_A::SUT64,
            5 => STARTUP_A::SUT80,
            6 => STARTUP_A::SUT96,
            7 => STARTUP_A::SUT112,
            8 => STARTUP_A::SUT512,
            9 => STARTUP_A::SUT576,
            10 => STARTUP_A::SUT640,
            11 => STARTUP_A::SUT704,
            12 => STARTUP_A::SUT768,
            13 => STARTUP_A::SUT832,
            14 => STARTUP_A::SUT896,
            15 => STARTUP_A::SUT960,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SUT0`"]
    #[inline(always)]
    pub fn is_sut0(&self) -> bool {
        *self == STARTUP_A::SUT0
    }
    #[doc = "Checks if the value of the field is `SUT8`"]
    #[inline(always)]
    pub fn is_sut8(&self) -> bool {
        *self == STARTUP_A::SUT8
    }
    #[doc = "Checks if the value of the field is `SUT16`"]
    #[inline(always)]
    pub fn is_sut16(&self) -> bool {
        *self == STARTUP_A::SUT16
    }
    #[doc = "Checks if the value of the field is `SUT24`"]
    #[inline(always)]
    pub fn is_sut24(&self) -> bool {
        *self == STARTUP_A::SUT24
    }
    #[doc = "Checks if the value of the field is `SUT64`"]
    #[inline(always)]
    pub fn is_sut64(&self) -> bool {
        *self == STARTUP_A::SUT64
    }
    #[doc = "Checks if the value of the field is `SUT80`"]
    #[inline(always)]
    pub fn is_sut80(&self) -> bool {
        *self == STARTUP_A::SUT80
    }
    #[doc = "Checks if the value of the field is `SUT96`"]
    #[inline(always)]
    pub fn is_sut96(&self) -> bool {
        *self == STARTUP_A::SUT96
    }
    #[doc = "Checks if the value of the field is `SUT112`"]
    #[inline(always)]
    pub fn is_sut112(&self) -> bool {
        *self == STARTUP_A::SUT112
    }
    #[doc = "Checks if the value of the field is `SUT512`"]
    #[inline(always)]
    pub fn is_sut512(&self) -> bool {
        *self == STARTUP_A::SUT512
    }
    #[doc = "Checks if the value of the field is `SUT576`"]
    #[inline(always)]
    pub fn is_sut576(&self) -> bool {
        *self == STARTUP_A::SUT576
    }
    #[doc = "Checks if the value of the field is `SUT640`"]
    #[inline(always)]
    pub fn is_sut640(&self) -> bool {
        *self == STARTUP_A::SUT640
    }
    #[doc = "Checks if the value of the field is `SUT704`"]
    #[inline(always)]
    pub fn is_sut704(&self) -> bool {
        *self == STARTUP_A::SUT704
    }
    #[doc = "Checks if the value of the field is `SUT768`"]
    #[inline(always)]
    pub fn is_sut768(&self) -> bool {
        *self == STARTUP_A::SUT768
    }
    #[doc = "Checks if the value of the field is `SUT832`"]
    #[inline(always)]
    pub fn is_sut832(&self) -> bool {
        *self == STARTUP_A::SUT832
    }
    #[doc = "Checks if the value of the field is `SUT896`"]
    #[inline(always)]
    pub fn is_sut896(&self) -> bool {
        *self == STARTUP_A::SUT896
    }
    #[doc = "Checks if the value of the field is `SUT960`"]
    #[inline(always)]
    pub fn is_sut960(&self) -> bool {
        *self == STARTUP_A::SUT960
    }
}
#[doc = "Field `STARTUP` writer - Start Up Time"]
pub type STARTUP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MR_SPEC, u8, STARTUP_A, 4, O>;
impl<'a, const O: u8> STARTUP_W<'a, O> {
    #[doc = "0 periods of ADCClock"]
    #[inline(always)]
    pub fn sut0(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT0)
    }
    #[doc = "8 periods of ADCClock"]
    #[inline(always)]
    pub fn sut8(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT8)
    }
    #[doc = "16 periods of ADCClock"]
    #[inline(always)]
    pub fn sut16(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT16)
    }
    #[doc = "24 periods of ADCClock"]
    #[inline(always)]
    pub fn sut24(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT24)
    }
    #[doc = "64 periods of ADCClock"]
    #[inline(always)]
    pub fn sut64(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT64)
    }
    #[doc = "80 periods of ADCClock"]
    #[inline(always)]
    pub fn sut80(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT80)
    }
    #[doc = "96 periods of ADCClock"]
    #[inline(always)]
    pub fn sut96(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT96)
    }
    #[doc = "112 periods of ADCClock"]
    #[inline(always)]
    pub fn sut112(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT112)
    }
    #[doc = "512 periods of ADCClock"]
    #[inline(always)]
    pub fn sut512(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT512)
    }
    #[doc = "576 periods of ADCClock"]
    #[inline(always)]
    pub fn sut576(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT576)
    }
    #[doc = "640 periods of ADCClock"]
    #[inline(always)]
    pub fn sut640(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT640)
    }
    #[doc = "704 periods of ADCClock"]
    #[inline(always)]
    pub fn sut704(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT704)
    }
    #[doc = "768 periods of ADCClock"]
    #[inline(always)]
    pub fn sut768(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT768)
    }
    #[doc = "832 periods of ADCClock"]
    #[inline(always)]
    pub fn sut832(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT832)
    }
    #[doc = "896 periods of ADCClock"]
    #[inline(always)]
    pub fn sut896(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT896)
    }
    #[doc = "960 periods of ADCClock"]
    #[inline(always)]
    pub fn sut960(self) -> &'a mut W {
        self.variant(STARTUP_A::SUT960)
    }
}
#[doc = "Field `TRACKTIM` reader - Tracking Time"]
pub type TRACKTIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRACKTIM` writer - Tracking Time"]
pub type TRACKTIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 4, O>;
#[doc = "Field `USEQ` reader - Use Sequence Enable"]
pub type USEQ_R = crate::BitReader<USEQ_A>;
#[doc = "Use Sequence Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USEQ_A {
    #[doc = "0: Normal Mode: The controller converts channels in a simple numeric order depending only on the channel index."]
    NUM_ORDER = 0,
    #[doc = "1: User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers and can be used to convert several times the same channel."]
    REG_ORDER = 1,
}
impl From<USEQ_A> for bool {
    #[inline(always)]
    fn from(variant: USEQ_A) -> Self {
        variant as u8 != 0
    }
}
impl USEQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USEQ_A {
        match self.bits {
            false => USEQ_A::NUM_ORDER,
            true => USEQ_A::REG_ORDER,
        }
    }
    #[doc = "Checks if the value of the field is `NUM_ORDER`"]
    #[inline(always)]
    pub fn is_num_order(&self) -> bool {
        *self == USEQ_A::NUM_ORDER
    }
    #[doc = "Checks if the value of the field is `REG_ORDER`"]
    #[inline(always)]
    pub fn is_reg_order(&self) -> bool {
        *self == USEQ_A::REG_ORDER
    }
}
#[doc = "Field `USEQ` writer - Use Sequence Enable"]
pub type USEQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, USEQ_A, O>;
impl<'a, const O: u8> USEQ_W<'a, O> {
    #[doc = "Normal Mode: The controller converts channels in a simple numeric order depending only on the channel index."]
    #[inline(always)]
    pub fn num_order(self) -> &'a mut W {
        self.variant(USEQ_A::NUM_ORDER)
    }
    #[doc = "User Sequence Mode: The sequence respects what is defined in ADC_SEQR1 and ADC_SEQR2 registers and can be used to convert several times the same channel."]
    #[inline(always)]
    pub fn reg_order(self) -> &'a mut W {
        self.variant(USEQ_A::REG_ORDER)
    }
}
impl R {
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    pub fn lowres(&self) -> LOWRES_R {
        LOWRES_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Wake Up"]
    #[inline(always)]
    pub fn fwup(&self) -> FWUP_R {
        FWUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&self) -> PRESCAL_R {
        PRESCAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    pub fn startup(&self) -> STARTUP_R {
        STARTUP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&self) -> TRACKTIM_R {
        TRACKTIM_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    pub fn useq(&self) -> USEQ_R {
        USEQ_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Resolution"]
    #[inline(always)]
    pub fn lowres(&mut self) -> LOWRES_W<4> {
        LOWRES_W::new(self)
    }
    #[doc = "Bit 5 - Sleep Mode"]
    #[inline(always)]
    pub fn sleep(&mut self) -> SLEEP_W<5> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 6 - Fast Wake Up"]
    #[inline(always)]
    pub fn fwup(&mut self) -> FWUP_W<6> {
        FWUP_W::new(self)
    }
    #[doc = "Bits 8:15 - Prescaler Rate Selection"]
    #[inline(always)]
    pub fn prescal(&mut self) -> PRESCAL_W<8> {
        PRESCAL_W::new(self)
    }
    #[doc = "Bits 16:19 - Start Up Time"]
    #[inline(always)]
    pub fn startup(&mut self) -> STARTUP_W<16> {
        STARTUP_W::new(self)
    }
    #[doc = "Bits 24:27 - Tracking Time"]
    #[inline(always)]
    pub fn tracktim(&mut self) -> TRACKTIM_W<24> {
        TRACKTIM_W::new(self)
    }
    #[doc = "Bit 31 - Use Sequence Enable"]
    #[inline(always)]
    pub fn useq(&mut self) -> USEQ_W<31> {
        USEQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
