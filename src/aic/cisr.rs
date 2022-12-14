#[doc = "Register `CISR` reader"]
pub struct R(crate::R<CISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NFIQ` reader - NFIQ Status"]
pub type NFIQ_R = crate::BitReader<bool>;
#[doc = "Field `NIRQ` reader - NIRQ Status"]
pub type NIRQ_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - NFIQ Status"]
    #[inline(always)]
    pub fn nfiq(&self) -> NFIQ_R {
        NFIQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NIRQ Status"]
    #[inline(always)]
    pub fn nirq(&self) -> NIRQ_R {
        NIRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Core Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cisr](index.html) module"]
pub struct CISR_SPEC;
impl crate::RegisterSpec for CISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cisr::R](R) reader structure"]
impl crate::Readable for CISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CISR to value 0"]
impl crate::Resettable for CISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
