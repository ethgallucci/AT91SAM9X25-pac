#[doc = "Register `CKGR_MOR` reader"]
pub struct R(crate::R<CKGR_MOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_MOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_MOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_MOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGR_MOR` writer"]
pub struct W(crate::W<CKGR_MOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGR_MOR_SPEC>;
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
impl From<crate::W<CKGR_MOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGR_MOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOSCXTEN` reader - Main Crystal Oscillator Enable"]
pub type MOSCXTEN_R = crate::BitReader<bool>;
#[doc = "Field `MOSCXTEN` writer - Main Crystal Oscillator Enable"]
pub type MOSCXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
#[doc = "Field `MOSCXTBY` reader - Main Crystal Oscillator Bypass"]
pub type MOSCXTBY_R = crate::BitReader<bool>;
#[doc = "Field `MOSCXTBY` writer - Main Crystal Oscillator Bypass"]
pub type MOSCXTBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
#[doc = "Field `MOSCRCEN` reader - Main On-Chip RC Oscillator Enable"]
pub type MOSCRCEN_R = crate::BitReader<bool>;
#[doc = "Field `MOSCRCEN` writer - Main On-Chip RC Oscillator Enable"]
pub type MOSCRCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
#[doc = "Field `MOSCXTST` reader - Main Crystal Oscillator Start-up Time"]
pub type MOSCXTST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MOSCXTST` writer - Main Crystal Oscillator Start-up Time"]
pub type MOSCXTST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_MOR_SPEC, u8, u8, 8, O>;
#[doc = "Field `KEY` reader - Password"]
pub type KEY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY` writer - Password"]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_MOR_SPEC, u8, u8, 8, O>;
#[doc = "Field `MOSCSEL` reader - Main Oscillator Selection"]
pub type MOSCSEL_R = crate::BitReader<bool>;
#[doc = "Field `MOSCSEL` writer - Main Oscillator Selection"]
pub type MOSCSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
#[doc = "Field `CFDEN` reader - Clock Failure Detector Enable"]
pub type CFDEN_R = crate::BitReader<bool>;
#[doc = "Field `CFDEN` writer - Clock Failure Detector Enable"]
pub type CFDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_MOR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&self) -> MOSCXTEN_R {
        MOSCXTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&self) -> MOSCXTBY_R {
        MOSCXTBY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&self) -> MOSCRCEN_R {
        MOSCRCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    pub fn moscxtst(&self) -> MOSCXTST_R {
        MOSCXTST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Password"]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&self) -> MOSCSEL_R {
        MOSCSEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&self) -> CFDEN_R {
        CFDEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Main Crystal Oscillator Enable"]
    #[inline(always)]
    pub fn moscxten(&mut self) -> MOSCXTEN_W<0> {
        MOSCXTEN_W::new(self)
    }
    #[doc = "Bit 1 - Main Crystal Oscillator Bypass"]
    #[inline(always)]
    pub fn moscxtby(&mut self) -> MOSCXTBY_W<1> {
        MOSCXTBY_W::new(self)
    }
    #[doc = "Bit 3 - Main On-Chip RC Oscillator Enable"]
    #[inline(always)]
    pub fn moscrcen(&mut self) -> MOSCRCEN_W<3> {
        MOSCRCEN_W::new(self)
    }
    #[doc = "Bits 8:15 - Main Crystal Oscillator Start-up Time"]
    #[inline(always)]
    pub fn moscxtst(&mut self) -> MOSCXTST_W<8> {
        MOSCXTST_W::new(self)
    }
    #[doc = "Bits 16:23 - Password"]
    #[inline(always)]
    pub fn key(&mut self) -> KEY_W<16> {
        KEY_W::new(self)
    }
    #[doc = "Bit 24 - Main Oscillator Selection"]
    #[inline(always)]
    pub fn moscsel(&mut self) -> MOSCSEL_W<24> {
        MOSCSEL_W::new(self)
    }
    #[doc = "Bit 25 - Clock Failure Detector Enable"]
    #[inline(always)]
    pub fn cfden(&mut self) -> CFDEN_W<25> {
        CFDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Main Oscillator Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_mor](index.html) module"]
pub struct CKGR_MOR_SPEC;
impl crate::RegisterSpec for CKGR_MOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_mor::R](R) reader structure"]
impl crate::Readable for CKGR_MOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_mor::W](W) writer structure"]
impl crate::Writable for CKGR_MOR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKGR_MOR to value 0x0100_0008"]
impl crate::Resettable for CKGR_MOR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0008
    }
}
