#[doc = "Register `PMC_MCKR` reader"]
pub struct R(crate::R<PMC_MCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_MCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_MCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_MCKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_MCKR` writer"]
pub struct W(crate::W<PMC_MCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_MCKR_SPEC>;
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
impl From<crate::W<PMC_MCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_MCKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSS` reader - Master/Processor Clock Source Selection"]
pub type CSS_R = crate::FieldReader<u8, CSS_A>;
#[doc = "Master/Processor Clock Source Selection\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSS_A {
    #[doc = "0: Slow Clock is selected"]
    SLOW_CLK = 0,
    #[doc = "1: Main Clock is selected"]
    MAIN_CLK = 1,
    #[doc = "2: PLLACK/PLLADIV2 is selected"]
    PLLA_CLK = 2,
    #[doc = "3: UPLL Clock is selected"]
    UPLL_CLK = 3,
}
impl From<CSS_A> for u8 {
    #[inline(always)]
    fn from(variant: CSS_A) -> Self {
        variant as _
    }
}
impl CSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSS_A {
        match self.bits {
            0 => CSS_A::SLOW_CLK,
            1 => CSS_A::MAIN_CLK,
            2 => CSS_A::PLLA_CLK,
            3 => CSS_A::UPLL_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLOW_CLK`"]
    #[inline(always)]
    pub fn is_slow_clk(&self) -> bool {
        *self == CSS_A::SLOW_CLK
    }
    #[doc = "Checks if the value of the field is `MAIN_CLK`"]
    #[inline(always)]
    pub fn is_main_clk(&self) -> bool {
        *self == CSS_A::MAIN_CLK
    }
    #[doc = "Checks if the value of the field is `PLLA_CLK`"]
    #[inline(always)]
    pub fn is_plla_clk(&self) -> bool {
        *self == CSS_A::PLLA_CLK
    }
    #[doc = "Checks if the value of the field is `UPLL_CLK`"]
    #[inline(always)]
    pub fn is_upll_clk(&self) -> bool {
        *self == CSS_A::UPLL_CLK
    }
}
#[doc = "Field `CSS` writer - Master/Processor Clock Source Selection"]
pub type CSS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMC_MCKR_SPEC, u8, CSS_A, 2, O>;
impl<'a, const O: u8> CSS_W<'a, O> {
    #[doc = "Slow Clock is selected"]
    #[inline(always)]
    pub fn slow_clk(self) -> &'a mut W {
        self.variant(CSS_A::SLOW_CLK)
    }
    #[doc = "Main Clock is selected"]
    #[inline(always)]
    pub fn main_clk(self) -> &'a mut W {
        self.variant(CSS_A::MAIN_CLK)
    }
    #[doc = "PLLACK/PLLADIV2 is selected"]
    #[inline(always)]
    pub fn plla_clk(self) -> &'a mut W {
        self.variant(CSS_A::PLLA_CLK)
    }
    #[doc = "UPLL Clock is selected"]
    #[inline(always)]
    pub fn upll_clk(self) -> &'a mut W {
        self.variant(CSS_A::UPLL_CLK)
    }
}
#[doc = "Field `PRES` reader - Master/Processor Clock Prescaler"]
pub type PRES_R = crate::FieldReader<u8, PRES_A>;
#[doc = "Master/Processor Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PRES_A {
    #[doc = "0: Selected clock"]
    CLOCK = 0,
    #[doc = "1: Selected clock divided by 2"]
    CLOCK_DIV2 = 1,
    #[doc = "2: Selected clock divided by 4"]
    CLOCK_DIV4 = 2,
    #[doc = "3: Selected clock divided by 8"]
    CLOCK_DIV8 = 3,
    #[doc = "4: Selected clock divided by 16"]
    CLOCK_DIV16 = 4,
    #[doc = "5: Selected clock divided by 32"]
    CLOCK_DIV32 = 5,
    #[doc = "6: Selected clock divided by 64"]
    CLOCK_DIV64 = 6,
    #[doc = "7: Selected clock divided by 3"]
    CLOCK_DIV3 = 7,
}
impl From<PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as _
    }
}
impl PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRES_A {
        match self.bits {
            0 => PRES_A::CLOCK,
            1 => PRES_A::CLOCK_DIV2,
            2 => PRES_A::CLOCK_DIV4,
            3 => PRES_A::CLOCK_DIV8,
            4 => PRES_A::CLOCK_DIV16,
            5 => PRES_A::CLOCK_DIV32,
            6 => PRES_A::CLOCK_DIV64,
            7 => PRES_A::CLOCK_DIV3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CLOCK`"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == PRES_A::CLOCK
    }
    #[doc = "Checks if the value of the field is `CLOCK_DIV2`"]
    #[inline(always)]
    pub fn is_clock_div2(&self) -> bool {
        *self == PRES_A::CLOCK_DIV2
    }
    #[doc = "Checks if the value of the field is `CLOCK_DIV4`"]
    #[inline(always)]
    pub fn is_clock_div4(&self) -> bool {
        *self == PRES_A::CLOCK_DIV4
    }
    #[doc = "Checks if the value of the field is `CLOCK_DIV8`"]
    #[inline(always)]
    pub fn is_clock_div8(&self) -> bool {
        *self == PRES_A::CLOCK_DIV8
    }
    #[doc = "Checks if the value of the field is `CLOCK_DIV16`"]
    #[inline(always)]
    pub fn is_clock_div16(&self) -> bool {
        *self == PRES_A::CLOCK_DIV16
    }
    #[doc = "Checks if the value of the field is `CLOCK_DIV32`"]
    #[inline(always)]
    pub fn is_clock_div32(&self) -> bool {
        *self == PRES_A::CLOCK_DIV32
    }
    #[doc = "Checks if the value of the field is `CLOCK_DIV64`"]
    #[inline(always)]
    pub fn is_clock_div64(&self) -> bool {
        *self == PRES_A::CLOCK_DIV64
    }
    #[doc = "Checks if the value of the field is `CLOCK_DIV3`"]
    #[inline(always)]
    pub fn is_clock_div3(&self) -> bool {
        *self == PRES_A::CLOCK_DIV3
    }
}
#[doc = "Field `PRES` writer - Master/Processor Clock Prescaler"]
pub type PRES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMC_MCKR_SPEC, u8, PRES_A, 3, O>;
impl<'a, const O: u8> PRES_W<'a, O> {
    #[doc = "Selected clock"]
    #[inline(always)]
    pub fn clock(self) -> &'a mut W {
        self.variant(PRES_A::CLOCK)
    }
    #[doc = "Selected clock divided by 2"]
    #[inline(always)]
    pub fn clock_div2(self) -> &'a mut W {
        self.variant(PRES_A::CLOCK_DIV2)
    }
    #[doc = "Selected clock divided by 4"]
    #[inline(always)]
    pub fn clock_div4(self) -> &'a mut W {
        self.variant(PRES_A::CLOCK_DIV4)
    }
    #[doc = "Selected clock divided by 8"]
    #[inline(always)]
    pub fn clock_div8(self) -> &'a mut W {
        self.variant(PRES_A::CLOCK_DIV8)
    }
    #[doc = "Selected clock divided by 16"]
    #[inline(always)]
    pub fn clock_div16(self) -> &'a mut W {
        self.variant(PRES_A::CLOCK_DIV16)
    }
    #[doc = "Selected clock divided by 32"]
    #[inline(always)]
    pub fn clock_div32(self) -> &'a mut W {
        self.variant(PRES_A::CLOCK_DIV32)
    }
    #[doc = "Selected clock divided by 64"]
    #[inline(always)]
    pub fn clock_div64(self) -> &'a mut W {
        self.variant(PRES_A::CLOCK_DIV64)
    }
    #[doc = "Selected clock divided by 3"]
    #[inline(always)]
    pub fn clock_div3(self) -> &'a mut W {
        self.variant(PRES_A::CLOCK_DIV3)
    }
}
#[doc = "Field `MDIV` reader - Master Clock Division"]
pub type MDIV_R = crate::FieldReader<u8, MDIV_A>;
#[doc = "Master Clock Division\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MDIV_A {
    #[doc = "0: Master Clock is Prescaler Output Clock divided by 1.Warning: DDRCK is not available."]
    EQ_PCK = 0,
    #[doc = "1: Master Clock is Prescaler Output Clock divided by 2.DDRCK is equal to MCK."]
    PCK_DIV2 = 1,
    #[doc = "2: Master Clock is Prescaler Output Clock divided by 4.DDRCK is equal to MCK."]
    PCK_DIV4 = 2,
    #[doc = "3: Master Clock is Prescaler Output Clock divided by 3.DDRCK is equal to MCK."]
    PCK_DIV3 = 3,
}
impl From<MDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: MDIV_A) -> Self {
        variant as _
    }
}
impl MDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIV_A {
        match self.bits {
            0 => MDIV_A::EQ_PCK,
            1 => MDIV_A::PCK_DIV2,
            2 => MDIV_A::PCK_DIV4,
            3 => MDIV_A::PCK_DIV3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EQ_PCK`"]
    #[inline(always)]
    pub fn is_eq_pck(&self) -> bool {
        *self == MDIV_A::EQ_PCK
    }
    #[doc = "Checks if the value of the field is `PCK_DIV2`"]
    #[inline(always)]
    pub fn is_pck_div2(&self) -> bool {
        *self == MDIV_A::PCK_DIV2
    }
    #[doc = "Checks if the value of the field is `PCK_DIV4`"]
    #[inline(always)]
    pub fn is_pck_div4(&self) -> bool {
        *self == MDIV_A::PCK_DIV4
    }
    #[doc = "Checks if the value of the field is `PCK_DIV3`"]
    #[inline(always)]
    pub fn is_pck_div3(&self) -> bool {
        *self == MDIV_A::PCK_DIV3
    }
}
#[doc = "Field `MDIV` writer - Master Clock Division"]
pub type MDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PMC_MCKR_SPEC, u8, MDIV_A, 2, O>;
impl<'a, const O: u8> MDIV_W<'a, O> {
    #[doc = "Master Clock is Prescaler Output Clock divided by 1.Warning: DDRCK is not available."]
    #[inline(always)]
    pub fn eq_pck(self) -> &'a mut W {
        self.variant(MDIV_A::EQ_PCK)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 2.DDRCK is equal to MCK."]
    #[inline(always)]
    pub fn pck_div2(self) -> &'a mut W {
        self.variant(MDIV_A::PCK_DIV2)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 4.DDRCK is equal to MCK."]
    #[inline(always)]
    pub fn pck_div4(self) -> &'a mut W {
        self.variant(MDIV_A::PCK_DIV4)
    }
    #[doc = "Master Clock is Prescaler Output Clock divided by 3.DDRCK is equal to MCK."]
    #[inline(always)]
    pub fn pck_div3(self) -> &'a mut W {
        self.variant(MDIV_A::PCK_DIV3)
    }
}
#[doc = "Field `PLLADIV2` reader - PLLA divisor by 2"]
pub type PLLADIV2_R = crate::BitReader<PLLADIV2_A>;
#[doc = "PLLA divisor by 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLADIV2_A {
    #[doc = "0: PLLA clock frequency is divided by 1."]
    NOT_DIV2 = 0,
    #[doc = "1: PLLA clock frequency is divided by 2."]
    DIV2 = 1,
}
impl From<PLLADIV2_A> for bool {
    #[inline(always)]
    fn from(variant: PLLADIV2_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLADIV2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLADIV2_A {
        match self.bits {
            false => PLLADIV2_A::NOT_DIV2,
            true => PLLADIV2_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DIV2`"]
    #[inline(always)]
    pub fn is_not_div2(&self) -> bool {
        *self == PLLADIV2_A::NOT_DIV2
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLADIV2_A::DIV2
    }
}
#[doc = "Field `PLLADIV2` writer - PLLA divisor by 2"]
pub type PLLADIV2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_MCKR_SPEC, PLLADIV2_A, O>;
impl<'a, const O: u8> PLLADIV2_W<'a, O> {
    #[doc = "PLLA clock frequency is divided by 1."]
    #[inline(always)]
    pub fn not_div2(self) -> &'a mut W {
        self.variant(PLLADIV2_A::NOT_DIV2)
    }
    #[doc = "PLLA clock frequency is divided by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLADIV2_A::DIV2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Master/Processor Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Master/Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline(always)]
    pub fn mdiv(&self) -> MDIV_R {
        MDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - PLLA divisor by 2"]
    #[inline(always)]
    pub fn plladiv2(&self) -> PLLADIV2_R {
        PLLADIV2_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master/Processor Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W<0> {
        CSS_W::new(self)
    }
    #[doc = "Bits 4:6 - Master/Processor Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W<4> {
        PRES_W::new(self)
    }
    #[doc = "Bits 8:9 - Master Clock Division"]
    #[inline(always)]
    pub fn mdiv(&mut self) -> MDIV_W<8> {
        MDIV_W::new(self)
    }
    #[doc = "Bit 12 - PLLA divisor by 2"]
    #[inline(always)]
    pub fn plladiv2(&mut self) -> PLLADIV2_W<12> {
        PLLADIV2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_mckr](index.html) module"]
pub struct PMC_MCKR_SPEC;
impl crate::RegisterSpec for PMC_MCKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_mckr::R](R) reader structure"]
impl crate::Readable for PMC_MCKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_mckr::W](W) writer structure"]
impl crate::Writable for PMC_MCKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_MCKR to value 0x01"]
impl crate::Resettable for PMC_MCKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
