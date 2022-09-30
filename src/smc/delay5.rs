#[doc = "Register `DELAY5` reader"]
pub struct R(crate::R<DELAY5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAY5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAY5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAY5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DELAY5` writer"]
pub struct W(crate::W<DELAY5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAY5_SPEC>;
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
impl From<crate::W<DELAY5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAY5_SPEC>) -> Self {
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
#[doc = "SMC Delay on I/O\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delay5](index.html) module"]
pub struct DELAY5_SPEC;
impl crate::RegisterSpec for DELAY5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delay5::R](R) reader structure"]
impl crate::Readable for DELAY5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delay5::W](W) writer structure"]
impl crate::Writable for DELAY5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DELAY5 to value 0"]
impl crate::Resettable for DELAY5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
