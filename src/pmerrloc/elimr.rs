#[doc = "Register `ELIMR` reader"]
pub struct R(crate::R<ELIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE` reader - Computation Terminated Interrupt Mask"]
pub type DONE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Computation Terminated Interrupt Mask"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
}
#[doc = "Error Location Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elimr](index.html) module"]
pub struct ELIMR_SPEC;
impl crate::RegisterSpec for ELIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elimr::R](R) reader structure"]
impl crate::Readable for ELIMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ELIMR to value 0"]
impl crate::Resettable for ELIMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
