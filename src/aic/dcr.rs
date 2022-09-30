#[doc = "Register `DCR` reader"]
pub struct R(crate::R<DCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCR` writer"]
pub struct W(crate::W<DCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCR_SPEC>;
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
impl From<crate::W<DCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT` reader - Protection Mode"]
pub type PROT_R = crate::BitReader<bool>;
#[doc = "Field `PROT` writer - Protection Mode"]
pub type PROT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR_SPEC, bool, O>;
#[doc = "Field `GMSK` reader - General Mask"]
pub type GMSK_R = crate::BitReader<bool>;
#[doc = "Field `GMSK` writer - General Mask"]
pub type GMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Protection Mode"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - General Mask"]
    #[inline(always)]
    pub fn gmsk(&self) -> GMSK_R {
        GMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Protection Mode"]
    #[inline(always)]
    pub fn prot(&mut self) -> PROT_W<0> {
        PROT_W::new(self)
    }
    #[doc = "Bit 1 - General Mask"]
    #[inline(always)]
    pub fn gmsk(&mut self) -> GMSK_W<1> {
        GMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcr](index.html) module"]
pub struct DCR_SPEC;
impl crate::RegisterSpec for DCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcr::R](R) reader structure"]
impl crate::Readable for DCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcr::W](W) writer structure"]
impl crate::Writable for DCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCR to value 0"]
impl crate::Resettable for DCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
