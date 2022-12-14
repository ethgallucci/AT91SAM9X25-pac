#[doc = "Register `FFSR` reader"]
pub struct R(crate::R<FFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SYS` reader - Fast Forcing Status"]
pub type SYS_R = crate::BitReader<bool>;
#[doc = "Field `PID2` reader - Fast Forcing Status"]
pub type PID2_R = crate::BitReader<bool>;
#[doc = "Field `PID3` reader - Fast Forcing Status"]
pub type PID3_R = crate::BitReader<bool>;
#[doc = "Field `PID4` reader - Fast Forcing Status"]
pub type PID4_R = crate::BitReader<bool>;
#[doc = "Field `PID5` reader - Fast Forcing Status"]
pub type PID5_R = crate::BitReader<bool>;
#[doc = "Field `PID6` reader - Fast Forcing Status"]
pub type PID6_R = crate::BitReader<bool>;
#[doc = "Field `PID7` reader - Fast Forcing Status"]
pub type PID7_R = crate::BitReader<bool>;
#[doc = "Field `PID8` reader - Fast Forcing Status"]
pub type PID8_R = crate::BitReader<bool>;
#[doc = "Field `PID9` reader - Fast Forcing Status"]
pub type PID9_R = crate::BitReader<bool>;
#[doc = "Field `PID10` reader - Fast Forcing Status"]
pub type PID10_R = crate::BitReader<bool>;
#[doc = "Field `PID11` reader - Fast Forcing Status"]
pub type PID11_R = crate::BitReader<bool>;
#[doc = "Field `PID12` reader - Fast Forcing Status"]
pub type PID12_R = crate::BitReader<bool>;
#[doc = "Field `PID13` reader - Fast Forcing Status"]
pub type PID13_R = crate::BitReader<bool>;
#[doc = "Field `PID14` reader - Fast Forcing Status"]
pub type PID14_R = crate::BitReader<bool>;
#[doc = "Field `PID15` reader - Fast Forcing Status"]
pub type PID15_R = crate::BitReader<bool>;
#[doc = "Field `PID16` reader - Fast Forcing Status"]
pub type PID16_R = crate::BitReader<bool>;
#[doc = "Field `PID17` reader - Fast Forcing Status"]
pub type PID17_R = crate::BitReader<bool>;
#[doc = "Field `PID18` reader - Fast Forcing Status"]
pub type PID18_R = crate::BitReader<bool>;
#[doc = "Field `PID19` reader - Fast Forcing Status"]
pub type PID19_R = crate::BitReader<bool>;
#[doc = "Field `PID20` reader - Fast Forcing Status"]
pub type PID20_R = crate::BitReader<bool>;
#[doc = "Field `PID21` reader - Fast Forcing Status"]
pub type PID21_R = crate::BitReader<bool>;
#[doc = "Field `PID22` reader - Fast Forcing Status"]
pub type PID22_R = crate::BitReader<bool>;
#[doc = "Field `PID23` reader - Fast Forcing Status"]
pub type PID23_R = crate::BitReader<bool>;
#[doc = "Field `PID24` reader - Fast Forcing Status"]
pub type PID24_R = crate::BitReader<bool>;
#[doc = "Field `PID25` reader - Fast Forcing Status"]
pub type PID25_R = crate::BitReader<bool>;
#[doc = "Field `PID26` reader - Fast Forcing Status"]
pub type PID26_R = crate::BitReader<bool>;
#[doc = "Field `PID27` reader - Fast Forcing Status"]
pub type PID27_R = crate::BitReader<bool>;
#[doc = "Field `PID28` reader - Fast Forcing Status"]
pub type PID28_R = crate::BitReader<bool>;
#[doc = "Field `PID29` reader - Fast Forcing Status"]
pub type PID29_R = crate::BitReader<bool>;
#[doc = "Field `PID30` reader - Fast Forcing Status"]
pub type PID30_R = crate::BitReader<bool>;
#[doc = "Field `PID31` reader - Fast Forcing Status"]
pub type PID31_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 1 - Fast Forcing Status"]
    #[inline(always)]
    pub fn sys(&self) -> SYS_R {
        SYS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid2(&self) -> PID2_R {
        PID2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid3(&self) -> PID3_R {
        PID3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid4(&self) -> PID4_R {
        PID4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid5(&self) -> PID5_R {
        PID5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid6(&self) -> PID6_R {
        PID6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid7(&self) -> PID7_R {
        PID7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid8(&self) -> PID8_R {
        PID8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid9(&self) -> PID9_R {
        PID9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid10(&self) -> PID10_R {
        PID10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid11(&self) -> PID11_R {
        PID11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid12(&self) -> PID12_R {
        PID12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid13(&self) -> PID13_R {
        PID13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid14(&self) -> PID14_R {
        PID14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid15(&self) -> PID15_R {
        PID15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid16(&self) -> PID16_R {
        PID16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid17(&self) -> PID17_R {
        PID17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid18(&self) -> PID18_R {
        PID18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid19(&self) -> PID19_R {
        PID19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid20(&self) -> PID20_R {
        PID20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid21(&self) -> PID21_R {
        PID21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid22(&self) -> PID22_R {
        PID22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid23(&self) -> PID23_R {
        PID23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid24(&self) -> PID24_R {
        PID24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid25(&self) -> PID25_R {
        PID25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid26(&self) -> PID26_R {
        PID26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid27(&self) -> PID27_R {
        PID27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid28(&self) -> PID28_R {
        PID28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid29(&self) -> PID29_R {
        PID29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid30(&self) -> PID30_R {
        PID30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Fast Forcing Status"]
    #[inline(always)]
    pub fn pid31(&self) -> PID31_R {
        PID31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Fast Forcing Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffsr](index.html) module"]
pub struct FFSR_SPEC;
impl crate::RegisterSpec for FFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ffsr::R](R) reader structure"]
impl crate::Readable for FFSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FFSR to value 0"]
impl crate::Resettable for FFSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
