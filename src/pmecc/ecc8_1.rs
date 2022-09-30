#[doc = "Register `ECC8_1` reader"]
pub struct R(crate::R<ECC8_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC8_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC8_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC8_1_SPEC>) -> Self {
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
#[doc = "PMECC ECC 8 Register (sec_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc8_1](index.html) module"]
pub struct ECC8_1_SPEC;
impl crate::RegisterSpec for ECC8_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc8_1::R](R) reader structure"]
impl crate::Readable for ECC8_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC8_1 to value 0"]
impl crate::Resettable for ECC8_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
