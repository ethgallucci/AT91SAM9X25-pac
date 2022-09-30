#[doc = "Register `PRBS2` reader"]
pub struct R(crate::R<PRBS2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRBS2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRBS2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRBS2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRBS2` writer"]
pub struct W(crate::W<PRBS2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRBS2_SPEC>;
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
impl From<crate::W<PRBS2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRBS2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M8PR` reader - Master 8 Priority"]
pub type M8PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M8PR` writer - Master 8 Priority"]
pub type M8PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRBS2_SPEC, u8, u8, 2, O>;
#[doc = "Field `M9PR` reader - Master 9 Priority"]
pub type M9PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M9PR` writer - Master 9 Priority"]
pub type M9PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRBS2_SPEC, u8, u8, 2, O>;
#[doc = "Field `M10PR` reader - Master 10 Priority"]
pub type M10PR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M10PR` writer - Master 10 Priority"]
pub type M10PR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRBS2_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&self) -> M8PR_R {
        M8PR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&self) -> M9PR_R {
        M9PR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&self) -> M10PR_R {
        M10PR_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 8 Priority"]
    #[inline(always)]
    pub fn m8pr(&mut self) -> M8PR_W<0> {
        M8PR_W::new(self)
    }
    #[doc = "Bits 4:5 - Master 9 Priority"]
    #[inline(always)]
    pub fn m9pr(&mut self) -> M9PR_W<4> {
        M9PR_W::new(self)
    }
    #[doc = "Bits 8:9 - Master 10 Priority"]
    #[inline(always)]
    pub fn m10pr(&mut self) -> M10PR_W<8> {
        M10PR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Register B for Slave 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prbs2](index.html) module"]
pub struct PRBS2_SPEC;
impl crate::RegisterSpec for PRBS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prbs2::R](R) reader structure"]
impl crate::Readable for PRBS2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prbs2::W](W) writer structure"]
impl crate::Writable for PRBS2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRBS2 to value 0"]
impl crate::Resettable for PRBS2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
