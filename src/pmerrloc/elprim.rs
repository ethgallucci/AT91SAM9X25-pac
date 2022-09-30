#[doc = "Register `ELPRIM` reader"]
pub struct R(crate::R<ELPRIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELPRIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELPRIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELPRIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRIMITIV` reader - Primitive Polynomial"]
pub type PRIMITIV_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Primitive Polynomial"]
    #[inline(always)]
    pub fn primitiv(&self) -> PRIMITIV_R {
        PRIMITIV_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Error Location Primitive Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elprim](index.html) module"]
pub struct ELPRIM_SPEC;
impl crate::RegisterSpec for ELPRIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elprim::R](R) reader structure"]
impl crate::Readable for ELPRIM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ELPRIM to value 0"]
impl crate::Resettable for ELPRIM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
