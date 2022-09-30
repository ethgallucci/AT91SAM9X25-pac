#[doc = "Register `PMC_SCSR` reader"]
pub struct R(crate::R<PMC_SCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PCK` reader - Processor Clock Status"]
pub type PCK_R = crate::BitReader<bool>;
#[doc = "Field `DDRCK` reader - DDR Clock Status"]
pub type DDRCK_R = crate::BitReader<bool>;
#[doc = "Field `SMDCK` reader - SMD Clock Status"]
pub type SMDCK_R = crate::BitReader<bool>;
#[doc = "Field `UHP` reader - USB Host Port Clock Status"]
pub type UHP_R = crate::BitReader<bool>;
#[doc = "Field `UDP` reader - USB Device Port Clock Status"]
pub type UDP_R = crate::BitReader<bool>;
#[doc = "Field `PCK0` reader - Programmable Clock 0 Output Status"]
pub type PCK0_R = crate::BitReader<bool>;
#[doc = "Field `PCK1` reader - Programmable Clock 1 Output Status"]
pub type PCK1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Processor Clock Status"]
    #[inline(always)]
    pub fn pck(&self) -> PCK_R {
        PCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - DDR Clock Status"]
    #[inline(always)]
    pub fn ddrck(&self) -> DDRCK_R {
        DDRCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - SMD Clock Status"]
    #[inline(always)]
    pub fn smdck(&self) -> SMDCK_R {
        SMDCK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - USB Host Port Clock Status"]
    #[inline(always)]
    pub fn uhp(&self) -> UHP_R {
        UHP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Device Port Clock Status"]
    #[inline(always)]
    pub fn udp(&self) -> UDP_R {
        UDP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Programmable Clock 0 Output Status"]
    #[inline(always)]
    pub fn pck0(&self) -> PCK0_R {
        PCK0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Programmable Clock 1 Output Status"]
    #[inline(always)]
    pub fn pck1(&self) -> PCK1_R {
        PCK1_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "System Clock Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_scsr](index.html) module"]
pub struct PMC_SCSR_SPEC;
impl crate::RegisterSpec for PMC_SCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_scsr::R](R) reader structure"]
impl crate::Readable for PMC_SCSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMC_SCSR to value 0x05"]
impl crate::Resettable for PMC_SCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x05
    }
}
