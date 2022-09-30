#[doc = "Register `ELIDR` reader"]
pub struct R(crate::R<ELIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE` reader - Computation Terminated Interrupt Disable"]
pub type DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Computation Terminated Interrupt Disable"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Error Location Interrupt Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elidr](index.html) module"]
pub struct ELIDR_SPEC;
impl crate::RegisterSpec for ELIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elidr::R](R) reader structure"]
impl crate::Readable for ELIDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ELIDR to value 0"]
impl crate::Resettable for ELIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
