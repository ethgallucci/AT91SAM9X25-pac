#[doc = "Register `DELAYR` reader"]
pub struct R(crate::R<DELAYR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DELAYR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DELAYR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DELAYR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DELAYR` writer"]
pub struct W(crate::W<DELAYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DELAYR_SPEC>;
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
impl From<crate::W<DELAYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DELAYR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Delay0` reader - "]
pub type DELAY0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Delay0` writer - "]
pub type DELAY0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAYR_SPEC, u8, u8, 4, O>;
#[doc = "Field `Delay1` reader - "]
pub type DELAY1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Delay1` writer - "]
pub type DELAY1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAYR_SPEC, u8, u8, 4, O>;
#[doc = "Field `Delay2` reader - "]
pub type DELAY2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Delay2` writer - "]
pub type DELAY2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAYR_SPEC, u8, u8, 4, O>;
#[doc = "Field `Delay3` reader - "]
pub type DELAY3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Delay3` writer - "]
pub type DELAY3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAYR_SPEC, u8, u8, 4, O>;
#[doc = "Field `Delay4` reader - "]
pub type DELAY4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Delay4` writer - "]
pub type DELAY4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAYR_SPEC, u8, u8, 4, O>;
#[doc = "Field `Delay5` reader - "]
pub type DELAY5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Delay5` writer - "]
pub type DELAY5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAYR_SPEC, u8, u8, 4, O>;
#[doc = "Field `Delay6` reader - "]
pub type DELAY6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Delay6` writer - "]
pub type DELAY6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAYR_SPEC, u8, u8, 4, O>;
#[doc = "Field `Delay7` reader - "]
pub type DELAY7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Delay7` writer - "]
pub type DELAY7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DELAYR_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn delay0(&self) -> DELAY0_R {
        DELAY0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn delay1(&self) -> DELAY1_R {
        DELAY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn delay2(&self) -> DELAY2_R {
        DELAY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn delay3(&self) -> DELAY3_R {
        DELAY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn delay4(&self) -> DELAY4_R {
        DELAY4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn delay5(&self) -> DELAY5_R {
        DELAY5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn delay6(&self) -> DELAY6_R {
        DELAY6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn delay7(&self) -> DELAY7_R {
        DELAY7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn delay0(&mut self) -> DELAY0_W<0> {
        DELAY0_W::new(self)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn delay1(&mut self) -> DELAY1_W<4> {
        DELAY1_W::new(self)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn delay2(&mut self) -> DELAY2_W<8> {
        DELAY2_W::new(self)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn delay3(&mut self) -> DELAY3_W<12> {
        DELAY3_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn delay4(&mut self) -> DELAY4_W<16> {
        DELAY4_W::new(self)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn delay5(&mut self) -> DELAY5_W<20> {
        DELAY5_W::new(self)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn delay6(&mut self) -> DELAY6_W<24> {
        DELAY6_W::new(self)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn delay7(&mut self) -> DELAY7_W<28> {
        DELAY7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO Delay Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [delayr](index.html) module"]
pub struct DELAYR_SPEC;
impl crate::RegisterSpec for DELAYR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [delayr::R](R) reader structure"]
impl crate::Readable for DELAYR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [delayr::W](W) writer structure"]
impl crate::Writable for DELAYR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DELAYR to value 0"]
impl crate::Resettable for DELAYR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
