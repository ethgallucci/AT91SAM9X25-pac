#[doc = "Register `MD` reader"]
pub struct R(crate::R<MD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MD` writer"]
pub struct W(crate::W<MD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MD_SPEC>;
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
impl From<crate::W<MD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MD` reader - Memory Device"]
pub type MD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MD` writer - Memory Device"]
pub type MD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MD_SPEC, u8, u8, 3, O>;
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DBW_R = crate::BitReader<bool>;
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DBW_W<'a, const O: u8> = crate::BitWriter<'a, u32, MD_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Memory Device"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Memory Device"]
    #[inline(always)]
    pub fn md(&mut self) -> MD_W<0> {
        MD_W::new(self)
    }
    #[doc = "Bit 4 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DBW_W<4> {
        DBW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRSDRC Memory Device Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [md](index.html) module"]
pub struct MD_SPEC;
impl crate::RegisterSpec for MD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [md::R](R) reader structure"]
impl crate::Readable for MD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [md::W](W) writer structure"]
impl crate::Writable for MD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MD to value 0x10"]
impl crate::Resettable for MD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
