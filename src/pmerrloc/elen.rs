#[doc = "Register `ELEN` reader"]
pub struct R(crate::R<ELEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELEN` writer"]
pub struct W(crate::W<ELEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELEN_SPEC>;
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
impl From<crate::W<ELEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENINIT` reader - Initial Number of Bits in the Codeword"]
pub type ENINIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ENINIT` writer - Initial Number of Bits in the Codeword"]
pub type ENINIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ELEN_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Initial Number of Bits in the Codeword"]
    #[inline(always)]
    pub fn eninit(&self) -> ENINIT_R {
        ENINIT_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Initial Number of Bits in the Codeword"]
    #[inline(always)]
    pub fn eninit(&mut self) -> ENINIT_W<0> {
        ENINIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Location Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elen](index.html) module"]
pub struct ELEN_SPEC;
impl crate::RegisterSpec for ELEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elen::R](R) reader structure"]
impl crate::Readable for ELEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elen::W](W) writer structure"]
impl crate::Writable for ELEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ELEN to value 0"]
impl crate::Resettable for ELEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
