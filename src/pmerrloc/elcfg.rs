#[doc = "Register `ELCFG` reader"]
pub struct R(crate::R<ELCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ELCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ELCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ELCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ELCFG` writer"]
pub struct W(crate::W<ELCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ELCFG_SPEC>;
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
impl From<crate::W<ELCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ELCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECTORSZ` reader - Sector Size"]
pub type SECTORSZ_R = crate::BitReader<bool>;
#[doc = "Field `SECTORSZ` writer - Sector Size"]
pub type SECTORSZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ELCFG_SPEC, bool, O>;
#[doc = "Field `ERRNUM` reader - Number of Errors"]
pub type ERRNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ERRNUM` writer - Number of Errors"]
pub type ERRNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ELCFG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Sector Size"]
    #[inline(always)]
    pub fn sectorsz(&self) -> SECTORSZ_R {
        SECTORSZ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:20 - Number of Errors"]
    #[inline(always)]
    pub fn errnum(&self) -> ERRNUM_R {
        ERRNUM_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Sector Size"]
    #[inline(always)]
    pub fn sectorsz(&mut self) -> SECTORSZ_W<0> {
        SECTORSZ_W::new(self)
    }
    #[doc = "Bits 16:20 - Number of Errors"]
    #[inline(always)]
    pub fn errnum(&mut self) -> ERRNUM_W<16> {
        ERRNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Location Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [elcfg](index.html) module"]
pub struct ELCFG_SPEC;
impl crate::RegisterSpec for ELCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [elcfg::R](R) reader structure"]
impl crate::Readable for ELCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [elcfg::W](W) writer structure"]
impl crate::Writable for ELCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ELCFG to value 0"]
impl crate::Resettable for ELCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
