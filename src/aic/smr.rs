#[doc = "Register `SMR[%s]` reader"]
pub struct R(crate::R<SMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMR[%s]` writer"]
pub struct W(crate::W<SMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMR_SPEC>;
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
impl From<crate::W<SMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIOR` reader - Priority Level"]
pub type PRIOR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIOR` writer - Priority Level"]
pub type PRIOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMR_SPEC, u8, u8, 3, O>;
#[doc = "Field `SRCTYPE` reader - Interrupt Source Type"]
pub type SRCTYPE_R = crate::FieldReader<u8, SRCTYPE_A>;
#[doc = "Interrupt Source Type"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRCTYPE_A {
    #[doc = "0: High level Sensitive for internal sourceLow level Sensitive for external source"]
    INT_LEVEL_SENSITIVE = 0,
    #[doc = "1: Positive edge triggered for internal sourceNegative edge triggered for external source"]
    INT_EDGE_TRIGGERED = 1,
    #[doc = "2: High level Sensitive for internal sourceHigh level Sensitive for external source"]
    EXT_HIGH_LEVEL = 2,
    #[doc = "3: Positive edge triggered for internal sourcePositive edge triggered for external source"]
    EXT_POSITIVE_EDGE = 3,
}
impl From<SRCTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: SRCTYPE_A) -> Self {
        variant as _
    }
}
impl SRCTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRCTYPE_A {
        match self.bits {
            0 => SRCTYPE_A::INT_LEVEL_SENSITIVE,
            1 => SRCTYPE_A::INT_EDGE_TRIGGERED,
            2 => SRCTYPE_A::EXT_HIGH_LEVEL,
            3 => SRCTYPE_A::EXT_POSITIVE_EDGE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `INT_LEVEL_SENSITIVE`"]
    #[inline(always)]
    pub fn is_int_level_sensitive(&self) -> bool {
        *self == SRCTYPE_A::INT_LEVEL_SENSITIVE
    }
    #[doc = "Checks if the value of the field is `INT_EDGE_TRIGGERED`"]
    #[inline(always)]
    pub fn is_int_edge_triggered(&self) -> bool {
        *self == SRCTYPE_A::INT_EDGE_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `EXT_HIGH_LEVEL`"]
    #[inline(always)]
    pub fn is_ext_high_level(&self) -> bool {
        *self == SRCTYPE_A::EXT_HIGH_LEVEL
    }
    #[doc = "Checks if the value of the field is `EXT_POSITIVE_EDGE`"]
    #[inline(always)]
    pub fn is_ext_positive_edge(&self) -> bool {
        *self == SRCTYPE_A::EXT_POSITIVE_EDGE
    }
}
#[doc = "Field `SRCTYPE` writer - Interrupt Source Type"]
pub type SRCTYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SMR_SPEC, u8, SRCTYPE_A, 2, O>;
impl<'a, const O: u8> SRCTYPE_W<'a, O> {
    #[doc = "High level Sensitive for internal sourceLow level Sensitive for external source"]
    #[inline(always)]
    pub fn int_level_sensitive(self) -> &'a mut W {
        self.variant(SRCTYPE_A::INT_LEVEL_SENSITIVE)
    }
    #[doc = "Positive edge triggered for internal sourceNegative edge triggered for external source"]
    #[inline(always)]
    pub fn int_edge_triggered(self) -> &'a mut W {
        self.variant(SRCTYPE_A::INT_EDGE_TRIGGERED)
    }
    #[doc = "High level Sensitive for internal sourceHigh level Sensitive for external source"]
    #[inline(always)]
    pub fn ext_high_level(self) -> &'a mut W {
        self.variant(SRCTYPE_A::EXT_HIGH_LEVEL)
    }
    #[doc = "Positive edge triggered for internal sourcePositive edge triggered for external source"]
    #[inline(always)]
    pub fn ext_positive_edge(self) -> &'a mut W {
        self.variant(SRCTYPE_A::EXT_POSITIVE_EDGE)
    }
}
impl R {
    #[doc = "Bits 0:2 - Priority Level"]
    #[inline(always)]
    pub fn prior(&self) -> PRIOR_R {
        PRIOR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 5:6 - Interrupt Source Type"]
    #[inline(always)]
    pub fn srctype(&self) -> SRCTYPE_R {
        SRCTYPE_R::new(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Priority Level"]
    #[inline(always)]
    pub fn prior(&mut self) -> PRIOR_W<0> {
        PRIOR_W::new(self)
    }
    #[doc = "Bits 5:6 - Interrupt Source Type"]
    #[inline(always)]
    pub fn srctype(&mut self) -> SRCTYPE_W<5> {
        SRCTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smr](index.html) module"]
pub struct SMR_SPEC;
impl crate::RegisterSpec for SMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smr::R](R) reader structure"]
impl crate::Readable for SMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smr::W](W) writer structure"]
impl crate::Writable for SMR_SPEC {
    type Writer = W;
}
