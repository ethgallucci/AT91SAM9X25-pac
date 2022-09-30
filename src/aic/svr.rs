#[doc = "Register `SVR[%s]` reader"]
pub struct R(crate::R<SVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SVR[%s]` writer"]
pub struct W(crate::W<SVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SVR_SPEC>;
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
impl From<crate::W<SVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTOR` reader - Source Vector"]
pub type VECTOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `VECTOR` writer - Source Vector"]
pub type VECTOR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SVR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Source Vector"]
    #[inline(always)]
    pub fn vector(&self) -> VECTOR_R {
        VECTOR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Source Vector"]
    #[inline(always)]
    pub fn vector(&mut self) -> VECTOR_W<0> {
        VECTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Source Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [svr](index.html) module"]
pub struct SVR_SPEC;
impl crate::RegisterSpec for SVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [svr::R](R) reader structure"]
impl crate::Readable for SVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [svr::W](W) writer structure"]
impl crate::Writable for SVR_SPEC {
    type Writer = W;
}
