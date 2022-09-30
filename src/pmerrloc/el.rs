#[doc = "Register `EL[%s]` reader"]
pub struct R(crate::R<EL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERRLOCN` reader - Error Position within the set {sector area, spare area}."]
pub type ERRLOCN_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - Error Position within the set {sector area, spare area}."]
    #[inline(always)]
    pub fn errlocn(&self) -> ERRLOCN_R {
        ERRLOCN_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "PMECC Error Location 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [el](index.html) module"]
pub struct EL_SPEC;
impl crate::RegisterSpec for EL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [el::R](R) reader structure"]
impl crate::Readable for EL_SPEC {
    type Reader = R;
}
