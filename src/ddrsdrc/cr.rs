#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NC` reader - Number of Column Bits"]
pub type NC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NC` writer - Number of Column Bits"]
pub type NC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `NR` reader - Number of Row Bits"]
pub type NR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NR` writer - Number of Row Bits"]
pub type NR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
#[doc = "Field `CAS` reader - CAS Latency"]
pub type CAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CAS` writer - CAS Latency"]
pub type CAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DLL` reader - Reset DLL"]
pub type DLL_R = crate::BitReader<bool>;
#[doc = "Field `DLL` writer - Reset DLL"]
pub type DLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DIC` reader - Output Driver Impedance Control"]
pub type DIC_R = crate::BitReader<bool>;
#[doc = "Field `DIC` writer - Output Driver Impedance Control"]
pub type DIC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DIS_DLL` reader - Disable DLL"]
pub type DIS_DLL_R = crate::BitReader<bool>;
#[doc = "Field `DIS_DLL` writer - Disable DLL"]
pub type DIS_DLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `OCD` reader - Off-chip Driver"]
pub type OCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OCD` writer - Off-chip Driver"]
pub type OCD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
#[doc = "Field `EBISHARE` reader - External Bus Interface is Shared"]
pub type EBISHARE_R = crate::BitReader<bool>;
#[doc = "Field `EBISHARE` writer - External Bus Interface is Shared"]
pub type EBISHARE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `ACTBST` reader - ACTIVE Bank X to Burst Stop Read Access Bank Y"]
pub type ACTBST_R = crate::BitReader<bool>;
#[doc = "Field `ACTBST` writer - ACTIVE Bank X to Burst Stop Read Access Bank Y"]
pub type ACTBST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `NB` reader - Number of Banks"]
pub type NB_R = crate::BitReader<bool>;
#[doc = "Field `NB` writer - Number of Banks"]
pub type NB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
#[doc = "Field `DECOD` reader - Type of Decoding"]
pub type DECOD_R = crate::BitReader<bool>;
#[doc = "Field `DECOD` writer - Type of Decoding"]
pub type DECOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&self) -> NC_R {
        NC_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&self) -> NR_R {
        NR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&self) -> CAS_R {
        CAS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Reset DLL"]
    #[inline(always)]
    pub fn dll(&self) -> DLL_R {
        DLL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Output Driver Impedance Control"]
    #[inline(always)]
    pub fn dic(&self) -> DIC_R {
        DIC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable DLL"]
    #[inline(always)]
    pub fn dis_dll(&self) -> DIS_DLL_R {
        DIS_DLL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Off-chip Driver"]
    #[inline(always)]
    pub fn ocd(&self) -> OCD_R {
        OCD_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - External Bus Interface is Shared"]
    #[inline(always)]
    pub fn ebishare(&self) -> EBISHARE_R {
        EBISHARE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - ACTIVE Bank X to Burst Stop Read Access Bank Y"]
    #[inline(always)]
    pub fn actbst(&self) -> ACTBST_R {
        ACTBST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Type of Decoding"]
    #[inline(always)]
    pub fn decod(&self) -> DECOD_R {
        DECOD_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Number of Column Bits"]
    #[inline(always)]
    pub fn nc(&mut self) -> NC_W<0> {
        NC_W::new(self)
    }
    #[doc = "Bits 2:3 - Number of Row Bits"]
    #[inline(always)]
    pub fn nr(&mut self) -> NR_W<2> {
        NR_W::new(self)
    }
    #[doc = "Bits 4:6 - CAS Latency"]
    #[inline(always)]
    pub fn cas(&mut self) -> CAS_W<4> {
        CAS_W::new(self)
    }
    #[doc = "Bit 7 - Reset DLL"]
    #[inline(always)]
    pub fn dll(&mut self) -> DLL_W<7> {
        DLL_W::new(self)
    }
    #[doc = "Bit 8 - Output Driver Impedance Control"]
    #[inline(always)]
    pub fn dic(&mut self) -> DIC_W<8> {
        DIC_W::new(self)
    }
    #[doc = "Bit 9 - Disable DLL"]
    #[inline(always)]
    pub fn dis_dll(&mut self) -> DIS_DLL_W<9> {
        DIS_DLL_W::new(self)
    }
    #[doc = "Bits 12:14 - Off-chip Driver"]
    #[inline(always)]
    pub fn ocd(&mut self) -> OCD_W<12> {
        OCD_W::new(self)
    }
    #[doc = "Bit 16 - External Bus Interface is Shared"]
    #[inline(always)]
    pub fn ebishare(&mut self) -> EBISHARE_W<16> {
        EBISHARE_W::new(self)
    }
    #[doc = "Bit 18 - ACTIVE Bank X to Burst Stop Read Access Bank Y"]
    #[inline(always)]
    pub fn actbst(&mut self) -> ACTBST_W<18> {
        ACTBST_W::new(self)
    }
    #[doc = "Bit 20 - Number of Banks"]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W<20> {
        NB_W::new(self)
    }
    #[doc = "Bit 22 - Type of Decoding"]
    #[inline(always)]
    pub fn decod(&mut self) -> DECOD_W<22> {
        DECOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRSDRC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0x7024"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7024
    }
}
