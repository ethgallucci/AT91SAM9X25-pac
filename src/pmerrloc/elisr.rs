#[doc = "Register `ELISR` reader"]
pub struct R(crate::R<ELISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DONE` reader - Computation Terminated Interrupt Status"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `ERR_CNT` reader - Error Counter value"]
pub type ERR_CNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Computation Terminated Interrupt Status"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12 - Error Counter value"]
    #[inline(always)]
    pub fn err_cnt(&self) -> ERR_CNT_R {
        ERR_CNT_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
#[doc = "Error Location Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elisr](index.html) module"]
pub struct ELISR_SPEC;
impl crate::RegisterSpec for ELISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elisr::R](R) reader structure"]
impl crate::Readable for ELISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ELISR to value 0"]
impl crate::Resettable for ELISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
