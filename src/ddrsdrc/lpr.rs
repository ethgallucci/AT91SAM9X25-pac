#[doc = "Register `LPR` reader"]
pub struct R(crate::R<LPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPR` writer"]
pub struct W(crate::W<LPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPR_SPEC>;
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
impl From<crate::W<LPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPCB` reader - Low-power Command Bit"]
pub type LPCB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPCB` writer - Low-power Command Bit"]
pub type LPCB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CLK_FR` reader - Clock Frozen Command Bit"]
pub type CLK_FR_R = crate::BitReader<bool>;
#[doc = "Field `CLK_FR` writer - Clock Frozen Command Bit"]
pub type CLK_FR_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPR_SPEC, bool, O>;
#[doc = "Field `PASR` reader - Partial Array Self Refresh"]
pub type PASR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PASR` writer - Partial Array Self Refresh"]
pub type PASR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DS` reader - Drive Strength"]
pub type DS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DS` writer - Drive Strength"]
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPR_SPEC, u8, u8, 3, O>;
#[doc = "Field `TIMEOUT` reader - Low Power Mode"]
pub type TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMEOUT` writer - Low Power Mode"]
pub type TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPR_SPEC, u8, u8, 2, O>;
#[doc = "Field `APDE` reader - Active Power Down Exit Time"]
pub type APDE_R = crate::BitReader<bool>;
#[doc = "Field `APDE` writer - Active Power Down Exit Time"]
pub type APDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, LPR_SPEC, bool, O>;
#[doc = "Field `UPD_MR` reader - Update Load Mode Register and Extended Mode Register"]
pub type UPD_MR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UPD_MR` writer - Update Load Mode Register and Extended Mode Register"]
pub type UPD_MR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Low-power Command Bit"]
    #[inline(always)]
    pub fn lpcb(&self) -> LPCB_R {
        LPCB_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Clock Frozen Command Bit"]
    #[inline(always)]
    pub fn clk_fr(&self) -> CLK_FR_R {
        CLK_FR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Partial Array Self Refresh"]
    #[inline(always)]
    pub fn pasr(&self) -> PASR_R {
        PASR_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Drive Strength"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Low Power Mode"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - Active Power Down Exit Time"]
    #[inline(always)]
    pub fn apde(&self) -> APDE_R {
        APDE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Update Load Mode Register and Extended Mode Register"]
    #[inline(always)]
    pub fn upd_mr(&self) -> UPD_MR_R {
        UPD_MR_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-power Command Bit"]
    #[inline(always)]
    pub fn lpcb(&mut self) -> LPCB_W<0> {
        LPCB_W::new(self)
    }
    #[doc = "Bit 2 - Clock Frozen Command Bit"]
    #[inline(always)]
    pub fn clk_fr(&mut self) -> CLK_FR_W<2> {
        CLK_FR_W::new(self)
    }
    #[doc = "Bits 4:6 - Partial Array Self Refresh"]
    #[inline(always)]
    pub fn pasr(&mut self) -> PASR_W<4> {
        PASR_W::new(self)
    }
    #[doc = "Bits 8:10 - Drive Strength"]
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W<8> {
        DS_W::new(self)
    }
    #[doc = "Bits 12:13 - Low Power Mode"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<12> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Bit 16 - Active Power Down Exit Time"]
    #[inline(always)]
    pub fn apde(&mut self) -> APDE_W<16> {
        APDE_W::new(self)
    }
    #[doc = "Bits 20:21 - Update Load Mode Register and Extended Mode Register"]
    #[inline(always)]
    pub fn upd_mr(&mut self) -> UPD_MR_W<20> {
        UPD_MR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRSDRC Low-power Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpr](index.html) module"]
pub struct LPR_SPEC;
impl crate::RegisterSpec for LPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpr::R](R) reader structure"]
impl crate::Readable for LPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpr::W](W) writer structure"]
impl crate::Writable for LPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LPR to value 0x0001_0000"]
impl crate::Resettable for LPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0000
    }
}
