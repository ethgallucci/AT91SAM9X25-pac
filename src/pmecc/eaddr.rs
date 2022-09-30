#[doc = "Register `EADDR` reader"]
pub struct R(crate::R<EADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EADDR` writer"]
pub struct W(crate::W<EADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EADDR_SPEC>;
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
impl From<crate::W<EADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDADDR` reader - ECC Area End Address (byte oriented address)"]
pub type ENDADDR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ENDADDR` writer - ECC Area End Address (byte oriented address)"]
pub type ENDADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EADDR_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - ECC Area End Address (byte oriented address)"]
    #[inline(always)]
    pub fn endaddr(&self) -> ENDADDR_R {
        ENDADDR_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - ECC Area End Address (byte oriented address)"]
    #[inline(always)]
    pub fn endaddr(&mut self) -> ENDADDR_W<0> {
        ENDADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMECC End Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eaddr](index.html) module"]
pub struct EADDR_SPEC;
impl crate::RegisterSpec for EADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eaddr::R](R) reader structure"]
impl crate::Readable for EADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eaddr::W](W) writer structure"]
impl crate::Writable for EADDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EADDR to value 0"]
impl crate::Resettable for EADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
