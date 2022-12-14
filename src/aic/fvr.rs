#[doc = "Register `FVR` reader"]
pub struct R(crate::R<FVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FIQV` reader - FIQ Vector Register"]
pub type FIQV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - FIQ Vector Register"]
    #[inline(always)]
    pub fn fiqv(&self) -> FIQV_R {
        FIQV_R::new(self.bits)
    }
}
#[doc = "FIQ Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fvr](index.html) module"]
pub struct FVR_SPEC;
impl crate::RegisterSpec for FVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fvr::R](R) reader structure"]
impl crate::Readable for FVR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FVR to value 0"]
impl crate::Resettable for FVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
