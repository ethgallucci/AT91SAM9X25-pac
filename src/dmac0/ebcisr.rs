#[doc = "Register `EBCISR` reader"]
pub struct R(crate::R<EBCISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EBCISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EBCISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EBCISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BTC0` reader - Buffer Transfer Completed \\[7:0\\]"]
pub type BTC0_R = crate::BitReader<bool>;
#[doc = "Field `BTC1` reader - Buffer Transfer Completed \\[7:0\\]"]
pub type BTC1_R = crate::BitReader<bool>;
#[doc = "Field `BTC2` reader - Buffer Transfer Completed \\[7:0\\]"]
pub type BTC2_R = crate::BitReader<bool>;
#[doc = "Field `BTC3` reader - Buffer Transfer Completed \\[7:0\\]"]
pub type BTC3_R = crate::BitReader<bool>;
#[doc = "Field `BTC4` reader - Buffer Transfer Completed \\[7:0\\]"]
pub type BTC4_R = crate::BitReader<bool>;
#[doc = "Field `BTC5` reader - Buffer Transfer Completed \\[7:0\\]"]
pub type BTC5_R = crate::BitReader<bool>;
#[doc = "Field `BTC6` reader - Buffer Transfer Completed \\[7:0\\]"]
pub type BTC6_R = crate::BitReader<bool>;
#[doc = "Field `BTC7` reader - Buffer Transfer Completed \\[7:0\\]"]
pub type BTC7_R = crate::BitReader<bool>;
#[doc = "Field `CBTC0` reader - Chained Buffer Transfer Completed \\[7:0\\]"]
pub type CBTC0_R = crate::BitReader<bool>;
#[doc = "Field `CBTC1` reader - Chained Buffer Transfer Completed \\[7:0\\]"]
pub type CBTC1_R = crate::BitReader<bool>;
#[doc = "Field `CBTC2` reader - Chained Buffer Transfer Completed \\[7:0\\]"]
pub type CBTC2_R = crate::BitReader<bool>;
#[doc = "Field `CBTC3` reader - Chained Buffer Transfer Completed \\[7:0\\]"]
pub type CBTC3_R = crate::BitReader<bool>;
#[doc = "Field `CBTC4` reader - Chained Buffer Transfer Completed \\[7:0\\]"]
pub type CBTC4_R = crate::BitReader<bool>;
#[doc = "Field `CBTC5` reader - Chained Buffer Transfer Completed \\[7:0\\]"]
pub type CBTC5_R = crate::BitReader<bool>;
#[doc = "Field `CBTC6` reader - Chained Buffer Transfer Completed \\[7:0\\]"]
pub type CBTC6_R = crate::BitReader<bool>;
#[doc = "Field `CBTC7` reader - Chained Buffer Transfer Completed \\[7:0\\]"]
pub type CBTC7_R = crate::BitReader<bool>;
#[doc = "Field `ERR0` reader - Access Error \\[7:0\\]"]
pub type ERR0_R = crate::BitReader<bool>;
#[doc = "Field `ERR1` reader - Access Error \\[7:0\\]"]
pub type ERR1_R = crate::BitReader<bool>;
#[doc = "Field `ERR2` reader - Access Error \\[7:0\\]"]
pub type ERR2_R = crate::BitReader<bool>;
#[doc = "Field `ERR3` reader - Access Error \\[7:0\\]"]
pub type ERR3_R = crate::BitReader<bool>;
#[doc = "Field `ERR4` reader - Access Error \\[7:0\\]"]
pub type ERR4_R = crate::BitReader<bool>;
#[doc = "Field `ERR5` reader - Access Error \\[7:0\\]"]
pub type ERR5_R = crate::BitReader<bool>;
#[doc = "Field `ERR6` reader - Access Error \\[7:0\\]"]
pub type ERR6_R = crate::BitReader<bool>;
#[doc = "Field `ERR7` reader - Access Error \\[7:0\\]"]
pub type ERR7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn btc0(&self) -> BTC0_R {
        BTC0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn btc1(&self) -> BTC1_R {
        BTC1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn btc2(&self) -> BTC2_R {
        BTC2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn btc3(&self) -> BTC3_R {
        BTC3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn btc4(&self) -> BTC4_R {
        BTC4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn btc5(&self) -> BTC5_R {
        BTC5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn btc6(&self) -> BTC6_R {
        BTC6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn btc7(&self) -> BTC7_R {
        BTC7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Chained Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn cbtc0(&self) -> CBTC0_R {
        CBTC0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Chained Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn cbtc1(&self) -> CBTC1_R {
        CBTC1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Chained Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn cbtc2(&self) -> CBTC2_R {
        CBTC2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Chained Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn cbtc3(&self) -> CBTC3_R {
        CBTC3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Chained Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn cbtc4(&self) -> CBTC4_R {
        CBTC4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Chained Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn cbtc5(&self) -> CBTC5_R {
        CBTC5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Chained Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn cbtc6(&self) -> CBTC6_R {
        CBTC6_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Chained Buffer Transfer Completed \\[7:0\\]"]
    #[inline(always)]
    pub fn cbtc7(&self) -> CBTC7_R {
        CBTC7_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Access Error \\[7:0\\]"]
    #[inline(always)]
    pub fn err0(&self) -> ERR0_R {
        ERR0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Access Error \\[7:0\\]"]
    #[inline(always)]
    pub fn err1(&self) -> ERR1_R {
        ERR1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Access Error \\[7:0\\]"]
    #[inline(always)]
    pub fn err2(&self) -> ERR2_R {
        ERR2_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Access Error \\[7:0\\]"]
    #[inline(always)]
    pub fn err3(&self) -> ERR3_R {
        ERR3_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Access Error \\[7:0\\]"]
    #[inline(always)]
    pub fn err4(&self) -> ERR4_R {
        ERR4_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Access Error \\[7:0\\]"]
    #[inline(always)]
    pub fn err5(&self) -> ERR5_R {
        ERR5_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Access Error \\[7:0\\]"]
    #[inline(always)]
    pub fn err6(&self) -> ERR6_R {
        ERR6_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Access Error \\[7:0\\]"]
    #[inline(always)]
    pub fn err7(&self) -> ERR7_R {
        ERR7_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "DMAC Error, Chained Buffer Transfer Completed Interrupt and Buffer transfer completed Status Register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ebcisr](index.html) module"]
pub struct EBCISR_SPEC;
impl crate::RegisterSpec for EBCISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ebcisr::R](R) reader structure"]
impl crate::Readable for EBCISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EBCISR to value 0"]
impl crate::Resettable for EBCISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
