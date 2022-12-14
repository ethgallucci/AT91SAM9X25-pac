#[doc = "Register `CREQ` reader"]
pub struct R(crate::R<CREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CREQ` writer"]
pub struct W(crate::W<CREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CREQ_SPEC>;
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
impl From<crate::W<CREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SCREQ0` reader - Source Chunk Request"]
pub type SCREQ0_R = crate::BitReader<bool>;
#[doc = "Field `SCREQ0` writer - Source Chunk Request"]
pub type SCREQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `DCREQ0` reader - Destination Chunk Request"]
pub type DCREQ0_R = crate::BitReader<bool>;
#[doc = "Field `DCREQ0` writer - Destination Chunk Request"]
pub type DCREQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `SCREQ1` reader - Source Chunk Request"]
pub type SCREQ1_R = crate::BitReader<bool>;
#[doc = "Field `SCREQ1` writer - Source Chunk Request"]
pub type SCREQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `DCREQ1` reader - Destination Chunk Request"]
pub type DCREQ1_R = crate::BitReader<bool>;
#[doc = "Field `DCREQ1` writer - Destination Chunk Request"]
pub type DCREQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `SCREQ2` reader - Source Chunk Request"]
pub type SCREQ2_R = crate::BitReader<bool>;
#[doc = "Field `SCREQ2` writer - Source Chunk Request"]
pub type SCREQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `DCREQ2` reader - Destination Chunk Request"]
pub type DCREQ2_R = crate::BitReader<bool>;
#[doc = "Field `DCREQ2` writer - Destination Chunk Request"]
pub type DCREQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `SCREQ3` reader - Source Chunk Request"]
pub type SCREQ3_R = crate::BitReader<bool>;
#[doc = "Field `SCREQ3` writer - Source Chunk Request"]
pub type SCREQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `DCREQ3` reader - Destination Chunk Request"]
pub type DCREQ3_R = crate::BitReader<bool>;
#[doc = "Field `DCREQ3` writer - Destination Chunk Request"]
pub type DCREQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `SCREQ4` reader - Source Chunk Request"]
pub type SCREQ4_R = crate::BitReader<bool>;
#[doc = "Field `SCREQ4` writer - Source Chunk Request"]
pub type SCREQ4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `DCREQ4` reader - Destination Chunk Request"]
pub type DCREQ4_R = crate::BitReader<bool>;
#[doc = "Field `DCREQ4` writer - Destination Chunk Request"]
pub type DCREQ4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `SCREQ5` reader - Source Chunk Request"]
pub type SCREQ5_R = crate::BitReader<bool>;
#[doc = "Field `SCREQ5` writer - Source Chunk Request"]
pub type SCREQ5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `DCREQ5` reader - Destination Chunk Request"]
pub type DCREQ5_R = crate::BitReader<bool>;
#[doc = "Field `DCREQ5` writer - Destination Chunk Request"]
pub type DCREQ5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `SCREQ6` reader - Source Chunk Request"]
pub type SCREQ6_R = crate::BitReader<bool>;
#[doc = "Field `SCREQ6` writer - Source Chunk Request"]
pub type SCREQ6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `DCREQ6` reader - Destination Chunk Request"]
pub type DCREQ6_R = crate::BitReader<bool>;
#[doc = "Field `DCREQ6` writer - Destination Chunk Request"]
pub type DCREQ6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `SCREQ7` reader - Source Chunk Request"]
pub type SCREQ7_R = crate::BitReader<bool>;
#[doc = "Field `SCREQ7` writer - Source Chunk Request"]
pub type SCREQ7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
#[doc = "Field `DCREQ7` reader - Destination Chunk Request"]
pub type DCREQ7_R = crate::BitReader<bool>;
#[doc = "Field `DCREQ7` writer - Destination Chunk Request"]
pub type DCREQ7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CREQ_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq0(&self) -> SCREQ0_R {
        SCREQ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq0(&self) -> DCREQ0_R {
        DCREQ0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq1(&self) -> SCREQ1_R {
        SCREQ1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq1(&self) -> DCREQ1_R {
        DCREQ1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq2(&self) -> SCREQ2_R {
        SCREQ2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq2(&self) -> DCREQ2_R {
        DCREQ2_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq3(&self) -> SCREQ3_R {
        SCREQ3_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq3(&self) -> DCREQ3_R {
        DCREQ3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq4(&self) -> SCREQ4_R {
        SCREQ4_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq4(&self) -> DCREQ4_R {
        DCREQ4_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq5(&self) -> SCREQ5_R {
        SCREQ5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq5(&self) -> DCREQ5_R {
        DCREQ5_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq6(&self) -> SCREQ6_R {
        SCREQ6_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq6(&self) -> DCREQ6_R {
        DCREQ6_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq7(&self) -> SCREQ7_R {
        SCREQ7_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq7(&self) -> DCREQ7_R {
        DCREQ7_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq0(&mut self) -> SCREQ0_W<0> {
        SCREQ0_W::new(self)
    }
    #[doc = "Bit 1 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq0(&mut self) -> DCREQ0_W<1> {
        DCREQ0_W::new(self)
    }
    #[doc = "Bit 2 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq1(&mut self) -> SCREQ1_W<2> {
        SCREQ1_W::new(self)
    }
    #[doc = "Bit 3 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq1(&mut self) -> DCREQ1_W<3> {
        DCREQ1_W::new(self)
    }
    #[doc = "Bit 4 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq2(&mut self) -> SCREQ2_W<4> {
        SCREQ2_W::new(self)
    }
    #[doc = "Bit 5 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq2(&mut self) -> DCREQ2_W<5> {
        DCREQ2_W::new(self)
    }
    #[doc = "Bit 6 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq3(&mut self) -> SCREQ3_W<6> {
        SCREQ3_W::new(self)
    }
    #[doc = "Bit 7 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq3(&mut self) -> DCREQ3_W<7> {
        DCREQ3_W::new(self)
    }
    #[doc = "Bit 8 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq4(&mut self) -> SCREQ4_W<8> {
        SCREQ4_W::new(self)
    }
    #[doc = "Bit 9 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq4(&mut self) -> DCREQ4_W<9> {
        DCREQ4_W::new(self)
    }
    #[doc = "Bit 10 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq5(&mut self) -> SCREQ5_W<10> {
        SCREQ5_W::new(self)
    }
    #[doc = "Bit 11 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq5(&mut self) -> DCREQ5_W<11> {
        DCREQ5_W::new(self)
    }
    #[doc = "Bit 12 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq6(&mut self) -> SCREQ6_W<12> {
        SCREQ6_W::new(self)
    }
    #[doc = "Bit 13 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq6(&mut self) -> DCREQ6_W<13> {
        DCREQ6_W::new(self)
    }
    #[doc = "Bit 14 - Source Chunk Request"]
    #[inline(always)]
    pub fn screq7(&mut self) -> SCREQ7_W<14> {
        SCREQ7_W::new(self)
    }
    #[doc = "Bit 15 - Destination Chunk Request"]
    #[inline(always)]
    pub fn dcreq7(&mut self) -> DCREQ7_W<15> {
        DCREQ7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Software Chunk Transfer Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [creq](index.html) module"]
pub struct CREQ_SPEC;
impl crate::RegisterSpec for CREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [creq::R](R) reader structure"]
impl crate::Readable for CREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [creq::W](W) writer structure"]
impl crate::Writable for CREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CREQ to value 0"]
impl crate::Resettable for CREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
