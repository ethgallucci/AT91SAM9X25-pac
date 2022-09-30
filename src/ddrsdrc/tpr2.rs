#[doc = "Register `TPR2` reader"]
pub struct R(crate::R<TPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPR2` writer"]
pub struct W(crate::W<TPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPR2_SPEC>;
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
impl From<crate::W<TPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXARD` reader - Exit Active Power Down Delay to Read Command in Mode \"Fast Exit\"."]
pub type TXARD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXARD` writer - Exit Active Power Down Delay to Read Command in Mode \"Fast Exit\"."]
pub type TXARD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXARDS` reader - Exit Active Power Down Delay to Read Command in Mode \"Slow Exit\"."]
pub type TXARDS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXARDS` writer - Exit Active Power Down Delay to Read Command in Mode \"Slow Exit\"."]
pub type TXARDS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRPA` reader - Row Precharge All Delay"]
pub type TRPA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRPA` writer - Row Precharge All Delay"]
pub type TRPA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRTP` reader - Read to Precharge"]
pub type TRTP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRTP` writer - Read to Precharge"]
pub type TRTP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `TFAW` reader - Four Active window"]
pub type TFAW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TFAW` writer - Four Active window"]
pub type TFAW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Exit Active Power Down Delay to Read Command in Mode \"Fast Exit\"."]
    #[inline(always)]
    pub fn txard(&self) -> TXARD_R {
        TXARD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Exit Active Power Down Delay to Read Command in Mode \"Slow Exit\"."]
    #[inline(always)]
    pub fn txards(&self) -> TXARDS_R {
        TXARDS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Row Precharge All Delay"]
    #[inline(always)]
    pub fn trpa(&self) -> TRPA_R {
        TRPA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Read to Precharge"]
    #[inline(always)]
    pub fn trtp(&self) -> TRTP_R {
        TRTP_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Four Active window"]
    #[inline(always)]
    pub fn tfaw(&self) -> TFAW_R {
        TFAW_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Exit Active Power Down Delay to Read Command in Mode \"Fast Exit\"."]
    #[inline(always)]
    pub fn txard(&mut self) -> TXARD_W<0> {
        TXARD_W::new(self)
    }
    #[doc = "Bits 4:7 - Exit Active Power Down Delay to Read Command in Mode \"Slow Exit\"."]
    #[inline(always)]
    pub fn txards(&mut self) -> TXARDS_W<4> {
        TXARDS_W::new(self)
    }
    #[doc = "Bits 8:11 - Row Precharge All Delay"]
    #[inline(always)]
    pub fn trpa(&mut self) -> TRPA_W<8> {
        TRPA_W::new(self)
    }
    #[doc = "Bits 12:14 - Read to Precharge"]
    #[inline(always)]
    pub fn trtp(&mut self) -> TRTP_W<12> {
        TRTP_W::new(self)
    }
    #[doc = "Bits 16:19 - Four Active window"]
    #[inline(always)]
    pub fn tfaw(&mut self) -> TFAW_W<16> {
        TFAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRSDRC Timing Parameter 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpr2](index.html) module"]
pub struct TPR2_SPEC;
impl crate::RegisterSpec for TPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpr2::R](R) reader structure"]
impl crate::Readable for TPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpr2::W](W) writer structure"]
impl crate::Writable for TPR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPR2 to value 0x2062"]
impl crate::Resettable for TPR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2062
    }
}
