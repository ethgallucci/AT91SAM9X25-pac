#[doc = "Register `RTR` reader"]
pub struct R(crate::R<RTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTR` writer"]
pub struct W(crate::W<RTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTR_SPEC>;
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
impl From<crate::W<RTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT` reader - DDRSDRC Refresh Timer Count"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT` writer - DDRSDRC Refresh Timer Count"]
pub type COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RTR_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - DDRSDRC Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - DDRSDRC Refresh Timer Count"]
    #[inline(always)]
    pub fn count(&mut self) -> COUNT_W<0> {
        COUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRSDRC Refresh Timer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtr](index.html) module"]
pub struct RTR_SPEC;
impl crate::RegisterSpec for RTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rtr::R](R) reader structure"]
impl crate::Readable for RTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtr::W](W) writer structure"]
impl crate::Writable for RTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTR to value 0"]
impl crate::Resettable for RTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
