#[doc = "Register `MODE5` reader"]
pub struct R(crate::R<MODE5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE5` writer"]
pub struct W(crate::W<MODE5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE5_SPEC>;
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
impl From<crate::W<MODE5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `READ_MODE` reader - "]
pub type READ_MODE_R = crate::BitReader<bool>;
#[doc = "Field `READ_MODE` writer - "]
pub type READ_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE5_SPEC, bool, O>;
#[doc = "Field `WRITE_MODE` reader - "]
pub type WRITE_MODE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE_MODE` writer - "]
pub type WRITE_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE5_SPEC, bool, O>;
#[doc = "Field `EXNW_MODE` reader - NWAIT Mode"]
pub type EXNW_MODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EXNW_MODE` writer - NWAIT Mode"]
pub type EXNW_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE5_SPEC, u8, u8, 2, O>;
#[doc = "Field `BAT` reader - Byte Access Type"]
pub type BAT_R = crate::BitReader<bool>;
#[doc = "Field `BAT` writer - Byte Access Type"]
pub type BAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE5_SPEC, bool, O>;
#[doc = "Field `DBW` reader - Data Bus Width"]
pub type DBW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DBW` writer - Data Bus Width"]
pub type DBW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE5_SPEC, u8, u8, 2, O>;
#[doc = "Field `TDF_CYCLES` reader - Data Float Time"]
pub type TDF_CYCLES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TDF_CYCLES` writer - Data Float Time"]
pub type TDF_CYCLES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE5_SPEC, u8, u8, 4, O>;
#[doc = "Field `TDF_MODE` reader - TDF Optimization"]
pub type TDF_MODE_R = crate::BitReader<bool>;
#[doc = "Field `TDF_MODE` writer - TDF Optimization"]
pub type TDF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE5_SPEC, bool, O>;
#[doc = "Field `PMEN` reader - Page Mode Enabled"]
pub type PMEN_R = crate::BitReader<bool>;
#[doc = "Field `PMEN` writer - Page Mode Enabled"]
pub type PMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE5_SPEC, bool, O>;
#[doc = "Field `PS` reader - Page Size"]
pub type PS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PS` writer - Page Size"]
pub type PS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MODE5_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_mode(&self) -> READ_MODE_R {
        READ_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn write_mode(&self) -> WRITE_MODE_R {
        WRITE_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&self) -> EXNW_MODE_R {
        EXNW_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&self) -> BAT_R {
        BAT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DBW_R {
        DBW_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&self) -> TDF_CYCLES_R {
        TDF_CYCLES_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&self) -> TDF_MODE_R {
        TDF_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&self) -> PMEN_R {
        PMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn read_mode(&mut self) -> READ_MODE_W<0> {
        READ_MODE_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn write_mode(&mut self) -> WRITE_MODE_W<1> {
        WRITE_MODE_W::new(self)
    }
    #[doc = "Bits 4:5 - NWAIT Mode"]
    #[inline(always)]
    pub fn exnw_mode(&mut self) -> EXNW_MODE_W<4> {
        EXNW_MODE_W::new(self)
    }
    #[doc = "Bit 8 - Byte Access Type"]
    #[inline(always)]
    pub fn bat(&mut self) -> BAT_W<8> {
        BAT_W::new(self)
    }
    #[doc = "Bits 12:13 - Data Bus Width"]
    #[inline(always)]
    pub fn dbw(&mut self) -> DBW_W<12> {
        DBW_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Float Time"]
    #[inline(always)]
    pub fn tdf_cycles(&mut self) -> TDF_CYCLES_W<16> {
        TDF_CYCLES_W::new(self)
    }
    #[doc = "Bit 20 - TDF Optimization"]
    #[inline(always)]
    pub fn tdf_mode(&mut self) -> TDF_MODE_W<20> {
        TDF_MODE_W::new(self)
    }
    #[doc = "Bit 24 - Page Mode Enabled"]
    #[inline(always)]
    pub fn pmen(&mut self) -> PMEN_W<24> {
        PMEN_W::new(self)
    }
    #[doc = "Bits 28:29 - Page Size"]
    #[inline(always)]
    pub fn ps(&mut self) -> PS_W<28> {
        PS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SMC Mode Register (CS_number = 5)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode5](index.html) module"]
pub struct MODE5_SPEC;
impl crate::RegisterSpec for MODE5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode5::R](R) reader structure"]
impl crate::Readable for MODE5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode5::W](W) writer structure"]
impl crate::Writable for MODE5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE5 to value 0x1000_1000"]
impl crate::Resettable for MODE5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1000_1000
    }
}
