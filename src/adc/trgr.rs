#[doc = "Register `TRGR` reader"]
pub struct R(crate::R<TRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRGR` writer"]
pub struct W(crate::W<TRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRGR_SPEC>;
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
impl From<crate::W<TRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGMOD` reader - Trigger Mode"]
pub type TRGMOD_R = crate::FieldReader<u8, TRGMOD_A>;
#[doc = "Trigger Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TRGMOD_A {
    #[doc = "0: No trigger, only software trigger can start conversions"]
    NO_TRIGGER = 0,
    #[doc = "1: External Trigger Rising Edge"]
    EXT_TRIG_RISE = 1,
    #[doc = "2: External Trigger Falling Edge"]
    EXT_TRIG_FALL = 2,
    #[doc = "3: External Trigger Any Edge"]
    EXT_TRIG_ANY = 3,
    #[doc = "5: Periodic Trigger (TRGPER shall be initiated appropriately)"]
    PERIOD_TRIG = 5,
    #[doc = "6: Continuous Mode"]
    CONTINUOUS = 6,
}
impl From<TRGMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGMOD_A) -> Self {
        variant as _
    }
}
impl TRGMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRGMOD_A> {
        match self.bits {
            0 => Some(TRGMOD_A::NO_TRIGGER),
            1 => Some(TRGMOD_A::EXT_TRIG_RISE),
            2 => Some(TRGMOD_A::EXT_TRIG_FALL),
            3 => Some(TRGMOD_A::EXT_TRIG_ANY),
            5 => Some(TRGMOD_A::PERIOD_TRIG),
            6 => Some(TRGMOD_A::CONTINUOUS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TRIGGER`"]
    #[inline(always)]
    pub fn is_no_trigger(&self) -> bool {
        *self == TRGMOD_A::NO_TRIGGER
    }
    #[doc = "Checks if the value of the field is `EXT_TRIG_RISE`"]
    #[inline(always)]
    pub fn is_ext_trig_rise(&self) -> bool {
        *self == TRGMOD_A::EXT_TRIG_RISE
    }
    #[doc = "Checks if the value of the field is `EXT_TRIG_FALL`"]
    #[inline(always)]
    pub fn is_ext_trig_fall(&self) -> bool {
        *self == TRGMOD_A::EXT_TRIG_FALL
    }
    #[doc = "Checks if the value of the field is `EXT_TRIG_ANY`"]
    #[inline(always)]
    pub fn is_ext_trig_any(&self) -> bool {
        *self == TRGMOD_A::EXT_TRIG_ANY
    }
    #[doc = "Checks if the value of the field is `PERIOD_TRIG`"]
    #[inline(always)]
    pub fn is_period_trig(&self) -> bool {
        *self == TRGMOD_A::PERIOD_TRIG
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == TRGMOD_A::CONTINUOUS
    }
}
#[doc = "Field `TRGMOD` writer - Trigger Mode"]
pub type TRGMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRGR_SPEC, u8, TRGMOD_A, 3, O>;
impl<'a, const O: u8> TRGMOD_W<'a, O> {
    #[doc = "No trigger, only software trigger can start conversions"]
    #[inline(always)]
    pub fn no_trigger(self) -> &'a mut W {
        self.variant(TRGMOD_A::NO_TRIGGER)
    }
    #[doc = "External Trigger Rising Edge"]
    #[inline(always)]
    pub fn ext_trig_rise(self) -> &'a mut W {
        self.variant(TRGMOD_A::EXT_TRIG_RISE)
    }
    #[doc = "External Trigger Falling Edge"]
    #[inline(always)]
    pub fn ext_trig_fall(self) -> &'a mut W {
        self.variant(TRGMOD_A::EXT_TRIG_FALL)
    }
    #[doc = "External Trigger Any Edge"]
    #[inline(always)]
    pub fn ext_trig_any(self) -> &'a mut W {
        self.variant(TRGMOD_A::EXT_TRIG_ANY)
    }
    #[doc = "Periodic Trigger (TRGPER shall be initiated appropriately)"]
    #[inline(always)]
    pub fn period_trig(self) -> &'a mut W {
        self.variant(TRGMOD_A::PERIOD_TRIG)
    }
    #[doc = "Continuous Mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(TRGMOD_A::CONTINUOUS)
    }
}
#[doc = "Field `TRGPER` reader - Trigger Period"]
pub type TRGPER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRGPER` writer - Trigger Period"]
pub type TRGPER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TRGR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:2 - Trigger Mode"]
    #[inline(always)]
    pub fn trgmod(&self) -> TRGMOD_R {
        TRGMOD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:31 - Trigger Period"]
    #[inline(always)]
    pub fn trgper(&self) -> TRGPER_R {
        TRGPER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - Trigger Mode"]
    #[inline(always)]
    pub fn trgmod(&mut self) -> TRGMOD_W<0> {
        TRGMOD_W::new(self)
    }
    #[doc = "Bits 16:31 - Trigger Period"]
    #[inline(always)]
    pub fn trgper(&mut self) -> TRGPER_W<16> {
        TRGPER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trgr](index.html) module"]
pub struct TRGR_SPEC;
impl crate::RegisterSpec for TRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trgr::R](R) reader structure"]
impl crate::Readable for TRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trgr::W](W) writer structure"]
impl crate::Writable for TRGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRGR to value 0"]
impl crate::Resettable for TRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
