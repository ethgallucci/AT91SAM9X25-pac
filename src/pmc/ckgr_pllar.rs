#[doc = "Register `CKGR_PLLAR` reader"]
pub struct R(crate::R<CKGR_PLLAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CKGR_PLLAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CKGR_PLLAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CKGR_PLLAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CKGR_PLLAR` writer"]
pub struct W(crate::W<CKGR_PLLAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CKGR_PLLAR_SPEC>;
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
impl From<crate::W<CKGR_PLLAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CKGR_PLLAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVA` reader - Divider A"]
pub type DIVA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIVA` writer - Divider A"]
pub type DIVA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_PLLAR_SPEC, u8, u8, 8, O>;
#[doc = "Field `PLLACOUNT` reader - PLLA Counter"]
pub type PLLACOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PLLACOUNT` writer - PLLA Counter"]
pub type PLLACOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_PLLAR_SPEC, u8, u8, 6, O>;
#[doc = "Field `OUTA` reader - PLLA Clock Frequency Range"]
pub type OUTA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OUTA` writer - PLLA Clock Frequency Range"]
pub type OUTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_PLLAR_SPEC, u8, u8, 2, O>;
#[doc = "Field `MULA` reader - PLLA Multiplier"]
pub type MULA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MULA` writer - PLLA Multiplier"]
pub type MULA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CKGR_PLLAR_SPEC, u16, u16, 11, O>;
#[doc = "Field `STUCKTO1` reader - "]
pub type STUCKTO1_R = crate::BitReader<bool>;
#[doc = "Field `STUCKTO1` writer - "]
pub type STUCKTO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CKGR_PLLAR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Divider A"]
    #[inline(always)]
    pub fn diva(&self) -> DIVA_R {
        DIVA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&self) -> PLLACOUNT_R {
        PLLACOUNT_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - PLLA Clock Frequency Range"]
    #[inline(always)]
    pub fn outa(&self) -> OUTA_R {
        OUTA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&self) -> MULA_R {
        MULA_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn stuckto1(&self) -> STUCKTO1_R {
        STUCKTO1_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Divider A"]
    #[inline(always)]
    pub fn diva(&mut self) -> DIVA_W<0> {
        DIVA_W::new(self)
    }
    #[doc = "Bits 8:13 - PLLA Counter"]
    #[inline(always)]
    pub fn pllacount(&mut self) -> PLLACOUNT_W<8> {
        PLLACOUNT_W::new(self)
    }
    #[doc = "Bits 14:15 - PLLA Clock Frequency Range"]
    #[inline(always)]
    pub fn outa(&mut self) -> OUTA_W<14> {
        OUTA_W::new(self)
    }
    #[doc = "Bits 16:26 - PLLA Multiplier"]
    #[inline(always)]
    pub fn mula(&mut self) -> MULA_W<16> {
        MULA_W::new(self)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn stuckto1(&mut self) -> STUCKTO1_W<29> {
        STUCKTO1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLLA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ckgr_pllar](index.html) module"]
pub struct CKGR_PLLAR_SPEC;
impl crate::RegisterSpec for CKGR_PLLAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ckgr_pllar::R](R) reader structure"]
impl crate::Readable for CKGR_PLLAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ckgr_pllar::W](W) writer structure"]
impl crate::Writable for CKGR_PLLAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CKGR_PLLAR to value 0x3f00"]
impl crate::Resettable for CKGR_PLLAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3f00
    }
}
