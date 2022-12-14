#[doc = "Register `REM9_4` reader"]
pub struct R(crate::R<REM9_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REM9_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REM9_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REM9_4_SPEC>) -> Self {
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
#[doc = "PMECC REM 9 Register (sec_num = 4)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rem9_4](index.html) module"]
pub struct REM9_4_SPEC;
impl crate::RegisterSpec for REM9_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rem9_4::R](R) reader structure"]
impl crate::Readable for REM9_4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets REM9_4 to value 0"]
impl crate::Resettable for REM9_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
