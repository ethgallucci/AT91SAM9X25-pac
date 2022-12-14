#[doc = "Register `FNR` reader"]
pub struct R(crate::R<FNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FNR` writer"]
pub struct W(crate::W<FNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FNR_SPEC>;
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
impl From<crate::W<FNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FNTRST` reader - Force NTRST"]
pub type FNTRST_R = crate::BitReader<bool>;
#[doc = "Field `FNTRST` writer - Force NTRST"]
pub type FNTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, FNR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Force NTRST"]
    #[inline(always)]
    pub fn fntrst(&self) -> FNTRST_R {
        FNTRST_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Force NTRST"]
    #[inline(always)]
    pub fn fntrst(&mut self) -> FNTRST_W<0> {
        FNTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force NTRST Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fnr](index.html) module"]
pub struct FNR_SPEC;
impl crate::RegisterSpec for FNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fnr::R](R) reader structure"]
impl crate::Readable for FNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fnr::W](W) writer structure"]
impl crate::Writable for FNR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FNR to value 0"]
impl crate::Resettable for FNR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
