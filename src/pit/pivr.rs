#[doc = "Register `PIVR` reader"]
pub struct R(crate::R<PIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CPIV` reader - Current Periodic Interval Value"]
pub type CPIV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PICNT` reader - Periodic Interval Counter"]
pub type PICNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:19 - Current Periodic Interval Value"]
    #[inline(always)]
    pub fn cpiv(&self) -> CPIV_R {
        CPIV_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bits 20:31 - Periodic Interval Counter"]
    #[inline(always)]
    pub fn picnt(&self) -> PICNT_R {
        PICNT_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "Periodic Interval Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pivr](index.html) module"]
pub struct PIVR_SPEC;
impl crate::RegisterSpec for PIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pivr::R](R) reader structure"]
impl crate::Readable for PIVR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIVR to value 0"]
impl crate::Resettable for PIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
