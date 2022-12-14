#[doc = "Register `EOICR` writer"]
pub struct W(crate::W<EOICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EOICR_SPEC>;
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
impl From<crate::W<EOICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EOICR_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "End of Interrupt Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eoicr](index.html) module"]
pub struct EOICR_SPEC;
impl crate::RegisterSpec for EOICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [eoicr::W](W) writer structure"]
impl crate::Writable for EOICR_SPEC {
    type Writer = W;
}
