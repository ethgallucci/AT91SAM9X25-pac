#[doc = "Register `HS` reader"]
pub struct R(crate::R<HS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HS` writer"]
pub struct W(crate::W<HS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HS_SPEC>;
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
impl From<crate::W<HS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS_ANTICIP_READ` reader - Anticip Read Access"]
pub type DIS_ANTICIP_READ_R = crate::BitReader<bool>;
#[doc = "Field `DIS_ANTICIP_READ` writer - Anticip Read Access"]
pub type DIS_ANTICIP_READ_W<'a, const O: u8> = crate::BitWriter<'a, u32, HS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - Anticip Read Access"]
    #[inline(always)]
    pub fn dis_anticip_read(&self) -> DIS_ANTICIP_READ_R {
        DIS_ANTICIP_READ_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Anticip Read Access"]
    #[inline(always)]
    pub fn dis_anticip_read(&mut self) -> DIS_ANTICIP_READ_W<2> {
        DIS_ANTICIP_READ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRSDRC High Speed Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs](index.html) module"]
pub struct HS_SPEC;
impl crate::RegisterSpec for HS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hs::R](R) reader structure"]
impl crate::Readable for HS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hs::W](W) writer structure"]
impl crate::Writable for HS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HS to value 0"]
impl crate::Resettable for HS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
