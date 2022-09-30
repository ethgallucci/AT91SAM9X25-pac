#[doc = "Register `ELIER` reader"]
pub struct R(crate::R<ELIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE` reader - Computation Terminated Interrupt Enable"]
pub type DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Computation Terminated Interrupt Enable"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Error Location Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elier](index.html) module"]
pub struct ELIER_SPEC;
impl crate::RegisterSpec for ELIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elier::R](R) reader structure"]
impl crate::Readable for ELIER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ELIER to value 0"]
impl crate::Resettable for ELIER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
