#[doc = "Register `BRGR` reader"]
pub struct R(crate::R<BRGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BRGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BRGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BRGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BRGR` writer"]
pub struct W(crate::W<BRGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRGR_SPEC>;
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
impl From<crate::W<BRGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CD` reader - Clock Divisor"]
pub type CD_R = crate::FieldReader<u16, CD_A>;
#[doc = "Clock Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum CD_A {
    #[doc = "0: DBGU Disabled"]
    DISABLED = 0,
    #[doc = "1: MCK"]
    MCK = 1,
}
impl From<CD_A> for u16 {
    #[inline(always)]
    fn from(variant: CD_A) -> Self {
        variant as _
    }
}
impl CD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CD_A> {
        match self.bits {
            0 => Some(CD_A::DISABLED),
            1 => Some(CD_A::MCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CD_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `MCK`"]
    #[inline(always)]
    pub fn is_mck(&self) -> bool {
        *self == CD_A::MCK
    }
}
#[doc = "Field `CD` writer - Clock Divisor"]
pub type CD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BRGR_SPEC, u16, CD_A, 16, O>;
impl<'a, const O: u8> CD_W<'a, O> {
    #[doc = "DBGU Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CD_A::DISABLED)
    }
    #[doc = "MCK"]
    #[inline(always)]
    pub fn mck(self) -> &'a mut W {
        self.variant(CD_A::MCK)
    }
}
impl R {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&self) -> CD_R {
        CD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Clock Divisor"]
    #[inline(always)]
    pub fn cd(&mut self) -> CD_W<0> {
        CD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Baud Rate Generator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brgr](index.html) module"]
pub struct BRGR_SPEC;
impl crate::RegisterSpec for BRGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [brgr::R](R) reader structure"]
impl crate::Readable for BRGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [brgr::W](W) writer structure"]
impl crate::Writable for BRGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BRGR to value 0"]
impl crate::Resettable for BRGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
