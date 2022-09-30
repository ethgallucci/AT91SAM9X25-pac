#[doc = "Register `DSCR3` reader"]
pub struct R(crate::R<DSCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSCR3` writer"]
pub struct W(crate::W<DSCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSCR3_SPEC>;
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
impl From<crate::W<DSCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSCR_IF` reader - "]
pub type DSCR_IF_R = crate::FieldReader<u8, DSCR_IF_A>;
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DSCR_IF_A {
    #[doc = "0: The buffer transfer descriptor is fetched via AHB-Lite Interface 0"]
    AHB_IF0 = 0,
    #[doc = "1: The buffer transfer descriptor is fetched via AHB-Lite Interface 1"]
    AHB_IF1 = 1,
}
impl From<DSCR_IF_A> for u8 {
    #[inline(always)]
    fn from(variant: DSCR_IF_A) -> Self {
        variant as _
    }
}
impl DSCR_IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DSCR_IF_A> {
        match self.bits {
            0 => Some(DSCR_IF_A::AHB_IF0),
            1 => Some(DSCR_IF_A::AHB_IF1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == DSCR_IF_A::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == DSCR_IF_A::AHB_IF1
    }
}
#[doc = "Field `DSCR_IF` writer - "]
pub type DSCR_IF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSCR3_SPEC, u8, DSCR_IF_A, 2, O>;
impl<'a, const O: u8> DSCR_IF_W<'a, O> {
    #[doc = "The buffer transfer descriptor is fetched via AHB-Lite Interface 0"]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(DSCR_IF_A::AHB_IF0)
    }
    #[doc = "The buffer transfer descriptor is fetched via AHB-Lite Interface 1"]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(DSCR_IF_A::AHB_IF1)
    }
}
#[doc = "Field `DSCR` reader - Buffer Transfer Descriptor Address"]
pub type DSCR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSCR` writer - Buffer Transfer Descriptor Address"]
pub type DSCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSCR3_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dscr_if(&self) -> DSCR_IF_R {
        DSCR_IF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    pub fn dscr(&self) -> DSCR_R {
        DSCR_R::new(((self.bits >> 2) & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dscr_if(&mut self) -> DSCR_IF_W<0> {
        DSCR_IF_W::new(self)
    }
    #[doc = "Bits 2:31 - Buffer Transfer Descriptor Address"]
    #[inline(always)]
    pub fn dscr(&mut self) -> DSCR_W<2> {
        DSCR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Descriptor Address Register (ch_num = 3)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dscr3](index.html) module"]
pub struct DSCR3_SPEC;
impl crate::RegisterSpec for DSCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dscr3::R](R) reader structure"]
impl crate::Readable for DSCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dscr3::W](W) writer structure"]
impl crate::Writable for DSCR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DSCR3 to value 0"]
impl crate::Resettable for DSCR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
