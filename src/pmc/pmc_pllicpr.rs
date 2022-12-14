#[doc = "Register `PMC_PLLICPR` writer"]
pub struct W(crate::W<PMC_PLLICPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_PLLICPR_SPEC>;
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
impl From<crate::W<PMC_PLLICPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_PLLICPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICPLLA` writer - Charge Pump Current"]
pub type ICPLLA_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_PLLICPR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Charge Pump Current"]
    #[inline(always)]
    pub fn icplla(&mut self) -> ICPLLA_W<0> {
        ICPLLA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Charge Pump Current Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_pllicpr](index.html) module"]
pub struct PMC_PLLICPR_SPEC;
impl crate::RegisterSpec for PMC_PLLICPR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_pllicpr::W](W) writer structure"]
impl crate::Writable for PMC_PLLICPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_PLLICPR to value 0x0100_0100"]
impl crate::Resettable for PMC_PLLICPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0100_0100
    }
}
