#[doc = "Register `PMC_PCK[%s]` reader"]
pub struct R(crate::R<PMC_PCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_PCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_PCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_PCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_PCK[%s]` writer"]
pub struct W(crate::W<PMC_PCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PCK_SPEC>;
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
impl From<crate::W<PMC_PCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSS` reader - Master Clock Source Selection"]
pub type CSS_R = crate::FieldReader<u8, CSS_A>;
#[doc = "Master Clock Source Selection"]
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
    #[doc = "4: Master Clock is selected"]
    MCK_CLK = 4,
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
    pub fn variant(&self) -> Option<CSS_A> {
        match self.bits {
            0 => Some(CSS_A::SLOW_CLK),
            1 => Some(CSS_A::MAIN_CLK),
            2 => Some(CSS_A::PLLA_CLK),
            3 => Some(CSS_A::UPLL_CLK),
            4 => Some(CSS_A::MCK_CLK),
            _ => None,
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
    #[doc = "Checks if the value of the field is `MCK_CLK`"]
    #[inline(always)]
    pub fn is_mck_clk(&self) -> bool {
        *self == CSS_A::MCK_CLK
    }
}
#[doc = "Field `CSS` writer - Master Clock Source Selection"]
pub type CSS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_PCK_SPEC, u8, CSS_A, 3, O>;
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
    #[doc = "Master Clock is selected"]
    #[inline(always)]
    pub fn mck_clk(self) -> &'a mut W {
        self.variant(CSS_A::MCK_CLK)
    }
}
#[doc = "Field `PRES` reader - Programmable Clock Prescaler"]
pub type PRES_R = crate::FieldReader<u8, PRES_A>;
#[doc = "Programmable Clock Prescaler"]
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
    pub fn variant(&self) -> Option<PRES_A> {
        match self.bits {
            0 => Some(PRES_A::CLOCK),
            1 => Some(PRES_A::CLOCK_DIV2),
            2 => Some(PRES_A::CLOCK_DIV4),
            3 => Some(PRES_A::CLOCK_DIV8),
            4 => Some(PRES_A::CLOCK_DIV16),
            5 => Some(PRES_A::CLOCK_DIV32),
            6 => Some(PRES_A::CLOCK_DIV64),
            _ => None,
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
}
#[doc = "Field `PRES` writer - Programmable Clock Prescaler"]
pub type PRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_PCK_SPEC, u8, PRES_A, 3, O>;
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
}
impl R {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&self) -> CSS_R {
        CSS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master Clock Source Selection"]
    #[inline(always)]
    pub fn css(&mut self) -> CSS_W<0> {
        CSS_W::new(self)
    }
    #[doc = "Bits 4:6 - Programmable Clock Prescaler"]
    #[inline(always)]
    pub fn pres(&mut self) -> PRES_W<4> {
        PRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable Clock 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pck](index.html) module"]
pub struct PMC_PCK_SPEC;
impl crate::RegisterSpec for PMC_PCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_pck::R](R) reader structure"]
impl crate::Readable for PMC_PCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_pck::W](W) writer structure"]
impl crate::Writable for PMC_PCK_SPEC {
    type Writer = W;
}
