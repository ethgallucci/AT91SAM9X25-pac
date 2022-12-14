#[doc = "Register `TPR1` reader"]
pub struct R(crate::R<TPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPR1` writer"]
pub struct W(crate::W<TPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPR1_SPEC>;
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
impl From<crate::W<TPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRFC` reader - Row Cycle Delay"]
pub type TRFC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRFC` writer - Row Cycle Delay"]
pub type TRFC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR1_SPEC, u8, u8, 5, O>;
#[doc = "Field `TXSNR` reader - Exit Self Refresh Delay to Non-read Command"]
pub type TXSNR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSNR` writer - Exit Self Refresh Delay to Non-read Command"]
pub type TXSNR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXSRD` reader - ExiT Self Refresh Delay to Read Command"]
pub type TXSRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSRD` writer - ExiT Self Refresh Delay to Read Command"]
pub type TXSRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR1_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXP` reader - Exit Power-down Delay to First Command"]
pub type TXP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXP` writer - Exit Power-down Delay to First Command"]
pub type TXP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:4 - Row Cycle Delay"]
    #[inline(always)]
    pub fn trfc(&self) -> TRFC_R {
        TRFC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Exit Self Refresh Delay to Non-read Command"]
    #[inline(always)]
    pub fn txsnr(&self) -> TXSNR_R {
        TXSNR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - ExiT Self Refresh Delay to Read Command"]
    #[inline(always)]
    pub fn txsrd(&self) -> TXSRD_R {
        TXSRD_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Exit Power-down Delay to First Command"]
    #[inline(always)]
    pub fn txp(&self) -> TXP_R {
        TXP_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Row Cycle Delay"]
    #[inline(always)]
    pub fn trfc(&mut self) -> TRFC_W<0> {
        TRFC_W::new(self)
    }
    #[doc = "Bits 8:15 - Exit Self Refresh Delay to Non-read Command"]
    #[inline(always)]
    pub fn txsnr(&mut self) -> TXSNR_W<8> {
        TXSNR_W::new(self)
    }
    #[doc = "Bits 16:23 - ExiT Self Refresh Delay to Read Command"]
    #[inline(always)]
    pub fn txsrd(&mut self) -> TXSRD_W<16> {
        TXSRD_W::new(self)
    }
    #[doc = "Bits 24:27 - Exit Power-down Delay to First Command"]
    #[inline(always)]
    pub fn txp(&mut self) -> TXP_W<24> {
        TXP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRSDRC Timing Parameter 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpr1](index.html) module"]
pub struct TPR1_SPEC;
impl crate::RegisterSpec for TPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpr1::R](R) reader structure"]
impl crate::Readable for TPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpr1::W](W) writer structure"]
impl crate::Writable for TPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPR1 to value 0x03c8_0808"]
impl crate::Resettable for TPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03c8_0808
    }
}
