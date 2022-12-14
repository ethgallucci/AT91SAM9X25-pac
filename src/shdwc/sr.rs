#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WAKEUP0` reader - Wake-up 0 Status"]
pub type WAKEUP0_R = crate::BitReader<bool>;
#[doc = "Field `RTCWK` reader - Real-time Clock Wake-up"]
pub type RTCWK_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Wake-up 0 Status"]
    #[inline(always)]
    pub fn wakeup0(&self) -> WAKEUP0_R {
        WAKEUP0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 17 - Real-time Clock Wake-up"]
    #[inline(always)]
    pub fn rtcwk(&self) -> RTCWK_R {
        RTCWK_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Shutdown Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
