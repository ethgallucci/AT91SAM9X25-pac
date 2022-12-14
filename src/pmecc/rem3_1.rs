#[doc = "Register `REM3_1` reader"]
pub struct R(crate::R<REM3_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REM3_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REM3_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REM3_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REM2NP1` reader - BCH Remainder 2 * N + 1"]
pub type REM2NP1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `REM2NP3` reader - BCH Remainder 2 * N + 3"]
pub type REM2NP3_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:13 - BCH Remainder 2 * N + 1"]
    #[inline(always)]
    pub fn rem2np1(&self) -> REM2NP1_R {
        REM2NP1_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - BCH Remainder 2 * N + 3"]
    #[inline(always)]
    pub fn rem2np3(&self) -> REM2NP3_R {
        REM2NP3_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "PMECC REM 3 Register (sec_num = 1)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rem3_1](index.html) module"]
pub struct REM3_1_SPEC;
impl crate::RegisterSpec for REM3_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rem3_1::R](R) reader structure"]
impl crate::Readable for REM3_1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REM3_1 to value 0"]
impl crate::Resettable for REM3_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
