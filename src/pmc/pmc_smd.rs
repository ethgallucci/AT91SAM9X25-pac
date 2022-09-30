#[doc = "Register `PMC_SMD` reader"]
pub struct R(crate::R<PMC_SMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMC_SMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMC_SMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMC_SMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMC_SMD` writer"]
pub struct W(crate::W<PMC_SMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMC_SMD_SPEC>;
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
impl From<crate::W<PMC_SMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMC_SMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMDS` reader - SMD input clock selection"]
pub type SMDS_R = crate::BitReader<bool>;
#[doc = "Field `SMDS` writer - SMD input clock selection"]
pub type SMDS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PMC_SMD_SPEC, bool, O>;
#[doc = "Field `SMDDIV` reader - Divider for SMD Clock."]
pub type SMDDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SMDDIV` writer - Divider for SMD Clock."]
pub type SMDDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PMC_SMD_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - SMD input clock selection"]
    #[inline(always)]
    pub fn smds(&self) -> SMDS_R {
        SMDS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:12 - Divider for SMD Clock."]
    #[inline(always)]
    pub fn smddiv(&self) -> SMDDIV_R {
        SMDDIV_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SMD input clock selection"]
    #[inline(always)]
    pub fn smds(&mut self) -> SMDS_W<0> {
        SMDS_W::new(self)
    }
    #[doc = "Bits 8:12 - Divider for SMD Clock."]
    #[inline(always)]
    pub fn smddiv(&mut self) -> SMDDIV_W<8> {
        SMDDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Soft Modem Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmc_smd](index.html) module"]
pub struct PMC_SMD_SPEC;
impl crate::RegisterSpec for PMC_SMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmc_smd::R](R) reader structure"]
impl crate::Readable for PMC_SMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmc_smd::W](W) writer structure"]
impl crate::Writable for PMC_SMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMC_SMD to value 0"]
impl crate::Resettable for PMC_SMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
