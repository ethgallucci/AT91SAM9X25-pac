#[doc = "Register `SADDR` reader"]
pub struct R(crate::R<SADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDR` writer"]
pub struct W(crate::W<SADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDR_SPEC>;
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
impl From<crate::W<SADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STARTADDR` reader - ECC Area Start Address (byte oriented address)"]
pub type STARTADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STARTADDR` writer - ECC Area Start Address (byte oriented address)"]
pub type STARTADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SADDR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - ECC Area Start Address (byte oriented address)"]
    #[inline(always)]
    pub fn startaddr(&self) -> STARTADDR_R {
        STARTADDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - ECC Area Start Address (byte oriented address)"]
    #[inline(always)]
    pub fn startaddr(&mut self) -> STARTADDR_W<0> {
        STARTADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMECC Start Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddr](index.html) module"]
pub struct SADDR_SPEC;
impl crate::RegisterSpec for SADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saddr::R](R) reader structure"]
impl crate::Readable for SADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddr::W](W) writer structure"]
impl crate::Writable for SADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SADDR to value 0"]
impl crate::Resettable for SADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
