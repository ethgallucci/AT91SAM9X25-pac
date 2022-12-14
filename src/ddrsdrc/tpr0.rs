#[doc = "Register `TPR0` reader"]
pub struct R(crate::R<TPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TPR0` writer"]
pub struct W(crate::W<TPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TPR0_SPEC>;
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
impl From<crate::W<TPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRAS` reader - Active to Precharge Delay"]
pub type TRAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRAS` writer - Active to Precharge Delay"]
pub type TRAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRCD` reader - Row to Column Delay"]
pub type TRCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRCD` writer - Row to Column Delay"]
pub type TRCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TWR` reader - Write Recovery Delay"]
pub type TWR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWR` writer - Write Recovery Delay"]
pub type TWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRC` reader - Row Cycle Delay"]
pub type TRC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRC` writer - Row Cycle Delay"]
pub type TRC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRP` reader - Row Precharge Delay"]
pub type TRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRP` writer - Row Precharge Delay"]
pub type TRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TRRD` reader - Active bankA to Active bankB"]
pub type TRRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRRD` writer - Active bankA to Active bankB"]
pub type TRRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR0_SPEC, u8, u8, 4, O>;
#[doc = "Field `TWTR` reader - Internal Write to Read Delay"]
pub type TWTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWTR` writer - Internal Write to Read Delay"]
pub type TWTR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR0_SPEC, u8, u8, 3, O>;
#[doc = "Field `REDUCE_WRRD` reader - Reduce Write to Read Delay"]
pub type REDUCE_WRRD_R = crate::BitReader<bool>;
#[doc = "Field `REDUCE_WRRD` writer - Reduce Write to Read Delay"]
pub type REDUCE_WRRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TPR0_SPEC, bool, O>;
#[doc = "Field `TMRD` reader - Load Mode Register Command to Active or Refresh Command"]
pub type TMRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TMRD` writer - Load Mode Register Command to Active or Refresh Command"]
pub type TMRD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TPR0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&self) -> TRAS_R {
        TRAS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&self) -> TRCD_R {
        TRCD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&self) -> TWR_R {
        TWR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay"]
    #[inline(always)]
    pub fn trc(&self) -> TRC_R {
        TRC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Active bankA to Active bankB"]
    #[inline(always)]
    pub fn trrd(&self) -> TRRD_R {
        TRRD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Internal Write to Read Delay"]
    #[inline(always)]
    pub fn twtr(&self) -> TWTR_R {
        TWTR_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Reduce Write to Read Delay"]
    #[inline(always)]
    pub fn reduce_wrrd(&self) -> REDUCE_WRRD_R {
        REDUCE_WRRD_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&self) -> TMRD_R {
        TMRD_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active to Precharge Delay"]
    #[inline(always)]
    pub fn tras(&mut self) -> TRAS_W<0> {
        TRAS_W::new(self)
    }
    #[doc = "Bits 4:7 - Row to Column Delay"]
    #[inline(always)]
    pub fn trcd(&mut self) -> TRCD_W<4> {
        TRCD_W::new(self)
    }
    #[doc = "Bits 8:11 - Write Recovery Delay"]
    #[inline(always)]
    pub fn twr(&mut self) -> TWR_W<8> {
        TWR_W::new(self)
    }
    #[doc = "Bits 12:15 - Row Cycle Delay"]
    #[inline(always)]
    pub fn trc(&mut self) -> TRC_W<12> {
        TRC_W::new(self)
    }
    #[doc = "Bits 16:19 - Row Precharge Delay"]
    #[inline(always)]
    pub fn trp(&mut self) -> TRP_W<16> {
        TRP_W::new(self)
    }
    #[doc = "Bits 20:23 - Active bankA to Active bankB"]
    #[inline(always)]
    pub fn trrd(&mut self) -> TRRD_W<20> {
        TRRD_W::new(self)
    }
    #[doc = "Bits 24:26 - Internal Write to Read Delay"]
    #[inline(always)]
    pub fn twtr(&mut self) -> TWTR_W<24> {
        TWTR_W::new(self)
    }
    #[doc = "Bit 27 - Reduce Write to Read Delay"]
    #[inline(always)]
    pub fn reduce_wrrd(&mut self) -> REDUCE_WRRD_W<27> {
        REDUCE_WRRD_W::new(self)
    }
    #[doc = "Bits 28:31 - Load Mode Register Command to Active or Refresh Command"]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TMRD_W<28> {
        TMRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRSDRC Timing Parameter 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tpr0](index.html) module"]
pub struct TPR0_SPEC;
impl crate::RegisterSpec for TPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tpr0::R](R) reader structure"]
impl crate::Readable for TPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tpr0::W](W) writer structure"]
impl crate::Writable for TPR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TPR0 to value 0x2022_7225"]
impl crate::Resettable for TPR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x2022_7225
    }
}
