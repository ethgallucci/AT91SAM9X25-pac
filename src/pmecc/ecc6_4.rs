#[doc = "Register `ECC6_4` reader"]
pub struct R(crate::R<ECC6_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECC6_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECC6_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECC6_4_SPEC>) -> Self {
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
#[doc = "PMECC ECC 6 Register (sec_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc6_4](index.html) module"]
pub struct ECC6_4_SPEC;
impl crate::RegisterSpec for ECC6_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecc6_4::R](R) reader structure"]
impl crate::Readable for ECC6_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ECC6_4 to value 0"]
impl crate::Resettable for ECC6_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
