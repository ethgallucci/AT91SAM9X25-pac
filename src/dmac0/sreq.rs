#[doc = "Register `SREQ` reader"]
pub struct R(crate::R<SREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SREQ` writer"]
pub struct W(crate::W<SREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSREQ0` reader - Source Request"]
pub type SSREQ0_R = crate::BitReader<bool>;
#[doc = "Field `SSREQ0` writer - Source Request"]
pub type SSREQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `DSREQ0` reader - Destination Request"]
pub type DSREQ0_R = crate::BitReader<bool>;
#[doc = "Field `DSREQ0` writer - Destination Request"]
pub type DSREQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `SSREQ1` reader - Source Request"]
pub type SSREQ1_R = crate::BitReader<bool>;
#[doc = "Field `SSREQ1` writer - Source Request"]
pub type SSREQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `DSREQ1` reader - Destination Request"]
pub type DSREQ1_R = crate::BitReader<bool>;
#[doc = "Field `DSREQ1` writer - Destination Request"]
pub type DSREQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `SSREQ2` reader - Source Request"]
pub type SSREQ2_R = crate::BitReader<bool>;
#[doc = "Field `SSREQ2` writer - Source Request"]
pub type SSREQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `DSREQ2` reader - Destination Request"]
pub type DSREQ2_R = crate::BitReader<bool>;
#[doc = "Field `DSREQ2` writer - Destination Request"]
pub type DSREQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `SSREQ3` reader - Source Request"]
pub type SSREQ3_R = crate::BitReader<bool>;
#[doc = "Field `SSREQ3` writer - Source Request"]
pub type SSREQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `DSREQ3` reader - Destination Request"]
pub type DSREQ3_R = crate::BitReader<bool>;
#[doc = "Field `DSREQ3` writer - Destination Request"]
pub type DSREQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `SSREQ4` reader - Source Request"]
pub type SSREQ4_R = crate::BitReader<bool>;
#[doc = "Field `SSREQ4` writer - Source Request"]
pub type SSREQ4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `DSREQ4` reader - Destination Request"]
pub type DSREQ4_R = crate::BitReader<bool>;
#[doc = "Field `DSREQ4` writer - Destination Request"]
pub type DSREQ4_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `SSREQ5` reader - Source Request"]
pub type SSREQ5_R = crate::BitReader<bool>;
#[doc = "Field `SSREQ5` writer - Source Request"]
pub type SSREQ5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `DSREQ5` reader - Destination Request"]
pub type DSREQ5_R = crate::BitReader<bool>;
#[doc = "Field `DSREQ5` writer - Destination Request"]
pub type DSREQ5_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `SSREQ6` reader - Source Request"]
pub type SSREQ6_R = crate::BitReader<bool>;
#[doc = "Field `SSREQ6` writer - Source Request"]
pub type SSREQ6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `DSREQ6` reader - Destination Request"]
pub type DSREQ6_R = crate::BitReader<bool>;
#[doc = "Field `DSREQ6` writer - Destination Request"]
pub type DSREQ6_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `SSREQ7` reader - Source Request"]
pub type SSREQ7_R = crate::BitReader<bool>;
#[doc = "Field `SSREQ7` writer - Source Request"]
pub type SSREQ7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
#[doc = "Field `DSREQ7` reader - Destination Request"]
pub type DSREQ7_R = crate::BitReader<bool>;
#[doc = "Field `DSREQ7` writer - Destination Request"]
pub type DSREQ7_W<'a, const O: u8> = crate::BitWriter<'a, u32, SREQ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    pub fn ssreq0(&self) -> SSREQ0_R {
        SSREQ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    pub fn dsreq0(&self) -> DSREQ0_R {
        DSREQ0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    pub fn ssreq1(&self) -> SSREQ1_R {
        SSREQ1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    pub fn dsreq1(&self) -> DSREQ1_R {
        DSREQ1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    pub fn ssreq2(&self) -> SSREQ2_R {
        SSREQ2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    pub fn dsreq2(&self) -> DSREQ2_R {
        DSREQ2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    pub fn ssreq3(&self) -> SSREQ3_R {
        SSREQ3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    pub fn dsreq3(&self) -> DSREQ3_R {
        DSREQ3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    pub fn ssreq4(&self) -> SSREQ4_R {
        SSREQ4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    pub fn dsreq4(&self) -> DSREQ4_R {
        DSREQ4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    pub fn ssreq5(&self) -> SSREQ5_R {
        SSREQ5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    pub fn dsreq5(&self) -> DSREQ5_R {
        DSREQ5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Source Request"]
    #[inline(always)]
    pub fn ssreq6(&self) -> SSREQ6_R {
        SSREQ6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Destination Request"]
    #[inline(always)]
    pub fn dsreq6(&self) -> DSREQ6_R {
        DSREQ6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Source Request"]
    #[inline(always)]
    pub fn ssreq7(&self) -> SSREQ7_R {
        SSREQ7_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Destination Request"]
    #[inline(always)]
    pub fn dsreq7(&self) -> DSREQ7_R {
        DSREQ7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Request"]
    #[inline(always)]
    pub fn ssreq0(&mut self) -> SSREQ0_W<0> {
        SSREQ0_W::new(self)
    }
    #[doc = "Bit 1 - Destination Request"]
    #[inline(always)]
    pub fn dsreq0(&mut self) -> DSREQ0_W<1> {
        DSREQ0_W::new(self)
    }
    #[doc = "Bit 2 - Source Request"]
    #[inline(always)]
    pub fn ssreq1(&mut self) -> SSREQ1_W<2> {
        SSREQ1_W::new(self)
    }
    #[doc = "Bit 3 - Destination Request"]
    #[inline(always)]
    pub fn dsreq1(&mut self) -> DSREQ1_W<3> {
        DSREQ1_W::new(self)
    }
    #[doc = "Bit 4 - Source Request"]
    #[inline(always)]
    pub fn ssreq2(&mut self) -> SSREQ2_W<4> {
        SSREQ2_W::new(self)
    }
    #[doc = "Bit 5 - Destination Request"]
    #[inline(always)]
    pub fn dsreq2(&mut self) -> DSREQ2_W<5> {
        DSREQ2_W::new(self)
    }
    #[doc = "Bit 6 - Source Request"]
    #[inline(always)]
    pub fn ssreq3(&mut self) -> SSREQ3_W<6> {
        SSREQ3_W::new(self)
    }
    #[doc = "Bit 7 - Destination Request"]
    #[inline(always)]
    pub fn dsreq3(&mut self) -> DSREQ3_W<7> {
        DSREQ3_W::new(self)
    }
    #[doc = "Bit 8 - Source Request"]
    #[inline(always)]
    pub fn ssreq4(&mut self) -> SSREQ4_W<8> {
        SSREQ4_W::new(self)
    }
    #[doc = "Bit 9 - Destination Request"]
    #[inline(always)]
    pub fn dsreq4(&mut self) -> DSREQ4_W<9> {
        DSREQ4_W::new(self)
    }
    #[doc = "Bit 10 - Source Request"]
    #[inline(always)]
    pub fn ssreq5(&mut self) -> SSREQ5_W<10> {
        SSREQ5_W::new(self)
    }
    #[doc = "Bit 11 - Destination Request"]
    #[inline(always)]
    pub fn dsreq5(&mut self) -> DSREQ5_W<11> {
        DSREQ5_W::new(self)
    }
    #[doc = "Bit 12 - Source Request"]
    #[inline(always)]
    pub fn ssreq6(&mut self) -> SSREQ6_W<12> {
        SSREQ6_W::new(self)
    }
    #[doc = "Bit 13 - Destination Request"]
    #[inline(always)]
    pub fn dsreq6(&mut self) -> DSREQ6_W<13> {
        DSREQ6_W::new(self)
    }
    #[doc = "Bit 14 - Source Request"]
    #[inline(always)]
    pub fn ssreq7(&mut self) -> SSREQ7_W<14> {
        SSREQ7_W::new(self)
    }
    #[doc = "Bit 15 - Destination Request"]
    #[inline(always)]
    pub fn dsreq7(&mut self) -> DSREQ7_W<15> {
        DSREQ7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Software Single Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sreq](index.html) module"]
pub struct SREQ_SPEC;
impl crate::RegisterSpec for SREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sreq::R](R) reader structure"]
impl crate::Readable for SREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sreq::W](W) writer structure"]
impl crate::Writable for SREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SREQ to value 0"]
impl crate::Resettable for SREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
