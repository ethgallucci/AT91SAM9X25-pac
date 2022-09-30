#[doc = "Register `SPU` reader"]
pub struct R(crate::R<SPU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPU` writer"]
pub struct W(crate::W<SPU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPU_SPEC>;
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
impl From<crate::W<SPU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIVR` reader - Spurious Interrupt Vector Register"]
pub type SIVR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SIVR` writer - Spurious Interrupt Vector Register"]
pub type SIVR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Spurious Interrupt Vector Register"]
    #[inline(always)]
    pub fn sivr(&self) -> SIVR_R {
        SIVR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Spurious Interrupt Vector Register"]
    #[inline(always)]
    pub fn sivr(&mut self) -> SIVR_W<0> {
        SIVR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Spurious Interrupt Vector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spu](index.html) module"]
pub struct SPU_SPEC;
impl crate::RegisterSpec for SPU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spu::R](R) reader structure"]
impl crate::Readable for SPU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spu::W](W) writer structure"]
impl crate::Writable for SPU_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPU to value 0"]
impl crate::Resettable for SPU_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
