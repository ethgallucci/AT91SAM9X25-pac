#[doc = "Register `PMC_SCER` writer"]
pub struct W(crate::W<PMC_SCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SCER_SPEC>;
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
impl From<crate::W<PMC_SCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DDRCK` writer - DDR Clock Enable"]
pub type DDRCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCER_SPEC, bool, O>;
#[doc = "Field `SMDCK` writer - SMD Clock Enable"]
pub type SMDCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCER_SPEC, bool, O>;
#[doc = "Field `UHP` writer - USB Host OHCI Clocks Enable"]
pub type UHP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCER_SPEC, bool, O>;
#[doc = "Field `UDP` writer - USB Device Clock Enable"]
pub type UDP_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCER_SPEC, bool, O>;
#[doc = "Field `PCK0` writer - Programmable Clock 0 Output Enable"]
pub type PCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCER_SPEC, bool, O>;
#[doc = "Field `PCK1` writer - Programmable Clock 1 Output Enable"]
pub type PCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SCER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 2 - DDR Clock Enable"]
    #[inline(always)]
    pub fn ddrck(&mut self) -> DDRCK_W<2> {
        DDRCK_W::new(self)
    }
    #[doc = "Bit 4 - SMD Clock Enable"]
    #[inline(always)]
    pub fn smdck(&mut self) -> SMDCK_W<4> {
        SMDCK_W::new(self)
    }
    #[doc = "Bit 6 - USB Host OHCI Clocks Enable"]
    #[inline(always)]
    pub fn uhp(&mut self) -> UHP_W<6> {
        UHP_W::new(self)
    }
    #[doc = "Bit 7 - USB Device Clock Enable"]
    #[inline(always)]
    pub fn udp(&mut self) -> UDP_W<7> {
        UDP_W::new(self)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Enable"]
    #[inline(always)]
    pub fn pck0(&mut self) -> PCK0_W<8> {
        PCK0_W::new(self)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Enable"]
    #[inline(always)]
    pub fn pck1(&mut self) -> PCK1_W<9> {
        PCK1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scer](index.html) module"]
pub struct PMC_SCER_SPEC;
impl crate::RegisterSpec for PMC_SCER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pmc_scer::W](W) writer structure"]
impl crate::Writable for PMC_SCER_SPEC {
    type Writer = W;
}
