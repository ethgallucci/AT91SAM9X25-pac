#[doc = "Register `IVR` reader"]
pub struct R(crate::R<IVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IRQV` reader - Interrupt Vector Register"]
pub type IRQV_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Interrupt Vector Register"]
    #[inline(always)]
    pub fn irqv(&self) -> IRQV_R {
        IRQV_R::new(self.bits)
    }
}
#[doc = "Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ivr](index.html) module"]
pub struct IVR_SPEC;
impl crate::RegisterSpec for IVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ivr::R](R) reader structure"]
impl crate::Readable for IVR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IVR to value 0"]
impl crate::Resettable for IVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
