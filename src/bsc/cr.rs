#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT` reader - Boot media sequence"]
pub type BOOT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BOOT` writer - Boot media sequence"]
pub type BOOT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BOOTKEY` reader - "]
pub type BOOTKEY_R = crate::FieldReader<u16, BOOTKEY_A>;
#[doc = ""]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BOOTKEY_A {
    #[doc = "26243: valid key to write BSC_CR register; it needs to be written at the same time as the BOOT field."]
    BSC_KEY = 26243,
}
impl From<BOOTKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: BOOTKEY_A) -> Self {
        variant as _
    }
}
impl BOOTKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BOOTKEY_A> {
        match self.bits {
            26243 => Some(BOOTKEY_A::BSC_KEY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BSC_KEY`"]
    #[inline(always)]
    pub fn is_bsc_key(&self) -> bool {
        *self == BOOTKEY_A::BSC_KEY
    }
}
#[doc = "Field `BOOTKEY` writer - "]
pub type BOOTKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u16, BOOTKEY_A, 16, O>;
impl<'a, const O: u8> BOOTKEY_W<'a, O> {
    #[doc = "valid key to write BSC_CR register; it needs to be written at the same time as the BOOT field."]
    #[inline(always)]
    pub fn bsc_key(self) -> &'a mut W {
        self.variant(BOOTKEY_A::BSC_KEY)
    }
}
impl R {
    #[doc = "Bits 0:7 - Boot media sequence"]
    #[inline(always)]
    pub fn boot(&self) -> BOOT_R {
        BOOT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn bootkey(&self) -> BOOTKEY_R {
        BOOTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Boot media sequence"]
    #[inline(always)]
    pub fn boot(&mut self) -> BOOT_W<0> {
        BOOT_W::new(self)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn bootkey(&mut self) -> BOOTKEY_W<16> {
        BOOTKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Boot Sequence Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
