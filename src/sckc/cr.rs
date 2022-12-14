#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCEN` reader - Internal 32 kHz RC Oscillator"]
pub type RCEN_R = crate::BitReader<bool>;
#[doc = "Field `RCEN` writer - Internal 32 kHz RC Oscillator"]
pub type RCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OSC32EN` reader - 32,768 Hz Oscillator"]
pub type OSC32EN_R = crate::BitReader<bool>;
#[doc = "Field `OSC32EN` writer - 32,768 Hz Oscillator"]
pub type OSC32EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OSC32BYP` reader - 32,768Hz Oscillator Bypass"]
pub type OSC32BYP_R = crate::BitReader<bool>;
#[doc = "Field `OSC32BYP` writer - 32,768Hz Oscillator Bypass"]
pub type OSC32BYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OSCSEL` reader - Slow Clock Selector"]
pub type OSCSEL_R = crate::BitReader<OSCSEL_A>;
#[doc = "Slow Clock Selector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSCSEL_A {
    #[doc = "0: Slow clock is internal 32 kHz RC oscillator."]
    RC = 0,
    #[doc = "1: Slow clock is 32,768 Hz oscillator."]
    XTAL = 1,
}
impl From<OSCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: OSCSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl OSCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSCSEL_A {
        match self.bits {
            false => OSCSEL_A::RC,
            true => OSCSEL_A::XTAL,
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == OSCSEL_A::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == OSCSEL_A::XTAL
    }
}
#[doc = "Field `OSCSEL` writer - Slow Clock Selector"]
pub type OSCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, OSCSEL_A, O>;
impl<'a, const O: u8> OSCSEL_W<'a, O> {
    #[doc = "Slow clock is internal 32 kHz RC oscillator."]
    #[inline(always)]
    pub fn rc(self) -> &'a mut W {
        self.variant(OSCSEL_A::RC)
    }
    #[doc = "Slow clock is 32,768 Hz oscillator."]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(OSCSEL_A::XTAL)
    }
}
impl R {
    #[doc = "Bit 0 - Internal 32 kHz RC Oscillator"]
    #[inline(always)]
    pub fn rcen(&self) -> RCEN_R {
        RCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 32,768 Hz Oscillator"]
    #[inline(always)]
    pub fn osc32en(&self) -> OSC32EN_R {
        OSC32EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 32,768Hz Oscillator Bypass"]
    #[inline(always)]
    pub fn osc32byp(&self) -> OSC32BYP_R {
        OSC32BYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slow Clock Selector"]
    #[inline(always)]
    pub fn oscsel(&self) -> OSCSEL_R {
        OSCSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal 32 kHz RC Oscillator"]
    #[inline(always)]
    pub fn rcen(&mut self) -> RCEN_W<0> {
        RCEN_W::new(self)
    }
    #[doc = "Bit 1 - 32,768 Hz Oscillator"]
    #[inline(always)]
    pub fn osc32en(&mut self) -> OSC32EN_W<1> {
        OSC32EN_W::new(self)
    }
    #[doc = "Bit 2 - 32,768Hz Oscillator Bypass"]
    #[inline(always)]
    pub fn osc32byp(&mut self) -> OSC32BYP_W<2> {
        OSC32BYP_W::new(self)
    }
    #[doc = "Bit 3 - Slow Clock Selector"]
    #[inline(always)]
    pub fn oscsel(&mut self) -> OSCSEL_W<3> {
        OSCSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slow Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x01"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
