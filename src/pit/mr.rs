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
#[doc = "Field `PIV` reader - Periodic Interval Value"]
pub type PIV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PIV` writer - Periodic Interval Value"]
pub type PIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MR_SPEC, u32, u32, 20, O>;
#[doc = "Field `PITEN` reader - Period Interval Timer Enabled"]
pub type PITEN_R = crate::BitReader<bool>;
#[doc = "Field `PITEN` writer - Period Interval Timer Enabled"]
pub type PITEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
#[doc = "Field `PITIEN` reader - Periodic Interval Timer Interrupt Enable"]
pub type PITIEN_R = crate::BitReader<bool>;
#[doc = "Field `PITIEN` writer - Periodic Interval Timer Interrupt Enable"]
pub type PITIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:19 - Periodic Interval Value"]
    #[inline(always)]
    pub fn piv(&self) -> PIV_R {
        PIV_R::new((self.bits & 0x000f_ffff) as u32)
    }
    #[doc = "Bit 24 - Period Interval Timer Enabled"]
    #[inline(always)]
    pub fn piten(&self) -> PITEN_R {
        PITEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Periodic Interval Timer Interrupt Enable"]
    #[inline(always)]
    pub fn pitien(&self) -> PITIEN_R {
        PITIEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:19 - Periodic Interval Value"]
    #[inline(always)]
    pub fn piv(&mut self) -> PIV_W<0> {
        PIV_W::new(self)
    }
    #[doc = "Bit 24 - Period Interval Timer Enabled"]
    #[inline(always)]
    pub fn piten(&mut self) -> PITEN_W<24> {
        PITEN_W::new(self)
    }
    #[doc = "Bit 25 - Periodic Interval Timer Interrupt Enable"]
    #[inline(always)]
    pub fn pitien(&mut self) -> PITIEN_W<25> {
        PITIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
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
#[doc = "`reset()` method sets MR to value 0x000f_ffff"]
impl crate::Resettable for MR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_ffff
    }
}
