#[doc = "Register `SIGMA[%s]` reader"]
pub struct R(crate::R<SIGMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIGMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIGMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIGMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIGMA[%s]` writer"]
pub struct W(crate::W<SIGMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIGMA_SPEC>;
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
impl From<crate::W<SIGMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIGMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGMAN` reader - "]
pub type SIGMAN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SIGMAN` writer - "]
pub type SIGMAN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SIGMA_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sigman(&self) -> SIGMAN_R {
        SIGMAN_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13"]
    #[inline(always)]
    pub fn sigman(&mut self) -> SIGMAN_W<0> {
        SIGMAN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMECC SIGMA 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sigma](index.html) module"]
pub struct SIGMA_SPEC;
impl crate::RegisterSpec for SIGMA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sigma::R](R) reader structure"]
impl crate::Readable for SIGMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sigma::W](W) writer structure"]
impl crate::Writable for SIGMA_SPEC {
    type Writer = W;
}
