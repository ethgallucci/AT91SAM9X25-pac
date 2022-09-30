#[doc = "Register `ECC2_0` reader"]
pub struct R(crate::R<ECC2_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC2_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC2_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC2_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ECC` reader - BCH Redundancy"]
pub type ECC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - BCH Redundancy"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(self.bits)
    }
}
#[doc = "PMECC ECC 2 Register (sec_num = 0)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc2_0](index.html) module"]
pub struct ECC2_0_SPEC;
impl crate::RegisterSpec for ECC2_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc2_0::R](R) reader structure"]
impl crate::Readable for ECC2_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC2_0 to value 0"]
impl crate::Resettable for ECC2_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
