#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MR` writer"]
pub struct W(crate::W<MR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MR_SPEC>;
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
impl From<crate::W<MR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WKMODE0` reader - Wake-up Mode 0"]
pub type WKMODE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WKMODE0` writer - Wake-up Mode 0"]
pub type WKMODE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CPTWK0` reader - Counter on Wake-up 0"]
pub type CPTWK0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPTWK0` writer - Counter on Wake-up 0"]
pub type CPTWK0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u8, u8, 4, O>;
#[doc = "Field `RTCWKEN` reader - Real-time Clock Wake-up Enable"]
pub type RTCWKEN_R = crate::BitReader<bool>;
#[doc = "Field `RTCWKEN` writer - Real-time Clock Wake-up Enable"]
pub type RTCWKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Wake-up Mode 0"]
    #[inline(always)]
    pub fn wkmode0(&self) -> WKMODE0_R {
        WKMODE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:7 - Counter on Wake-up 0"]
    #[inline(always)]
    pub fn cptwk0(&self) -> CPTWK0_R {
        CPTWK0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcwken(&self) -> RTCWKEN_R {
        RTCWKEN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wake-up Mode 0"]
    #[inline(always)]
    pub fn wkmode0(&mut self) -> WKMODE0_W<0> {
        WKMODE0_W::new(self)
    }
    #[doc = "Bits 4:7 - Counter on Wake-up 0"]
    #[inline(always)]
    pub fn cptwk0(&mut self) -> CPTWK0_W<4> {
        CPTWK0_W::new(self)
    }
    #[doc = "Bit 17 - Real-time Clock Wake-up Enable"]
    #[inline(always)]
    pub fn rtcwken(&mut self) -> RTCWKEN_W<17> {
        RTCWKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shutdown Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mr::W](W) writer structure"]
impl crate::Writable for MR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MR to value 0x03"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
