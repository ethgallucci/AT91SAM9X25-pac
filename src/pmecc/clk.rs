#[doc = "Register `CLK` reader"]
pub struct R(crate::R<CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK` writer"]
pub struct W(crate::W<CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_SPEC>;
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
impl From<crate::W<CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKCTRL` reader - Clock Control Register"]
pub type CLKCTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKCTRL` writer - Clock Control Register"]
pub type CLKCTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLK_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Clock Control Register"]
    #[inline(always)]
    pub fn clkctrl(&self) -> CLKCTRL_R {
        CLKCTRL_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Clock Control Register"]
    #[inline(always)]
    pub fn clkctrl(&mut self) -> CLKCTRL_W<0> {
        CLKCTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMECC Clock Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk](index.html) module"]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk::R](R) reader structure"]
impl crate::Readable for CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk::W](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK to value 0"]
impl crate::Resettable for CLK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
