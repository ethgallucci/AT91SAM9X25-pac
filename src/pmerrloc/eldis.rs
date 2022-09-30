#[doc = "Register `ELDIS` reader"]
pub struct R(crate::R<ELDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELDIS` writer"]
pub struct W(crate::W<ELDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELDIS_SPEC>;
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
impl From<crate::W<ELDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS` reader - Disable Error Location Engine"]
pub type DIS_R = crate::BitReader<bool>;
#[doc = "Field `DIS` writer - Disable Error Location Engine"]
pub type DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELDIS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Disable Error Location Engine"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Error Location Engine"]
    #[inline(always)]
    pub fn dis(&mut self) -> DIS_W<0> {
        DIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Location Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eldis](index.html) module"]
pub struct ELDIS_SPEC;
impl crate::RegisterSpec for ELDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eldis::R](R) reader structure"]
impl crate::Readable for ELDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eldis::W](W) writer structure"]
impl crate::Writable for ELDIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ELDIS to value 0"]
impl crate::Resettable for ELDIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
