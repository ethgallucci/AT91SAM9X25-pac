#[doc = "Register `PIIR` reader"]
pub struct R(crate::R<PIIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIIR_SPEC>) -> Self {
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
#[doc = "Periodic Interval Image Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [piir](index.html) module"]
pub struct PIIR_SPEC;
impl crate::RegisterSpec for PIIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [piir::R](R) reader structure"]
impl crate::Readable for PIIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIIR to value 0"]
impl crate::Resettable for PIIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
