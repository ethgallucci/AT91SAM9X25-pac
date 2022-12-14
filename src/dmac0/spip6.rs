#[doc = "Register `SPIP6` reader"]
pub struct R(crate::R<SPIP6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPIP6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPIP6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPIP6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPIP6` writer"]
pub struct W(crate::W<SPIP6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPIP6_SPEC>;
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
impl From<crate::W<SPIP6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPIP6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPIP_HOLE` reader - Source Picture-in-Picture Hole"]
pub type SPIP_HOLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPIP_HOLE` writer - Source Picture-in-Picture Hole"]
pub type SPIP_HOLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPIP6_SPEC, u16, u16, 16, O>;
#[doc = "Field `SPIP_BOUNDARY` reader - Source Picture-in-Picture Boundary"]
pub type SPIP_BOUNDARY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPIP_BOUNDARY` writer - Source Picture-in-Picture Boundary"]
pub type SPIP_BOUNDARY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPIP6_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:15 - Source Picture-in-Picture Hole"]
    #[inline(always)]
    pub fn spip_hole(&self) -> SPIP_HOLE_R {
        SPIP_HOLE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:25 - Source Picture-in-Picture Boundary"]
    #[inline(always)]
    pub fn spip_boundary(&self) -> SPIP_BOUNDARY_R {
        SPIP_BOUNDARY_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Source Picture-in-Picture Hole"]
    #[inline(always)]
    pub fn spip_hole(&mut self) -> SPIP_HOLE_W<0> {
        SPIP_HOLE_W::new(self)
    }
    #[doc = "Bits 16:25 - Source Picture-in-Picture Boundary"]
    #[inline(always)]
    pub fn spip_boundary(&mut self) -> SPIP_BOUNDARY_W<16> {
        SPIP_BOUNDARY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Source Picture-in-Picture Configuration Register (ch_num = 6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spip6](index.html) module"]
pub struct SPIP6_SPEC;
impl crate::RegisterSpec for SPIP6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spip6::R](R) reader structure"]
impl crate::Readable for SPIP6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spip6::W](W) writer structure"]
impl crate::Writable for SPIP6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPIP6 to value 0"]
impl crate::Resettable for SPIP6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
