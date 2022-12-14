#[doc = "Register `IDCR` writer"]
pub struct W(crate::W<IDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDCR_SPEC>;
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
impl From<crate::W<IDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FIQ` writer - Interrupt Disable"]
pub type FIQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `SYS` writer - Interrupt Disable"]
pub type SYS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID2` writer - Interrupt Disable"]
pub type PID2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID3` writer - Interrupt Disable"]
pub type PID3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID4` writer - Interrupt Disable"]
pub type PID4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID5` writer - Interrupt Disable"]
pub type PID5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID6` writer - Interrupt Disable"]
pub type PID6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID7` writer - Interrupt Disable"]
pub type PID7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID8` writer - Interrupt Disable"]
pub type PID8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID9` writer - Interrupt Disable"]
pub type PID9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID10` writer - Interrupt Disable"]
pub type PID10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID11` writer - Interrupt Disable"]
pub type PID11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID12` writer - Interrupt Disable"]
pub type PID12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID13` writer - Interrupt Disable"]
pub type PID13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID14` writer - Interrupt Disable"]
pub type PID14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID15` writer - Interrupt Disable"]
pub type PID15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID16` writer - Interrupt Disable"]
pub type PID16_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID17` writer - Interrupt Disable"]
pub type PID17_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID18` writer - Interrupt Disable"]
pub type PID18_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID19` writer - Interrupt Disable"]
pub type PID19_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID20` writer - Interrupt Disable"]
pub type PID20_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID21` writer - Interrupt Disable"]
pub type PID21_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID22` writer - Interrupt Disable"]
pub type PID22_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID23` writer - Interrupt Disable"]
pub type PID23_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID24` writer - Interrupt Disable"]
pub type PID24_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID25` writer - Interrupt Disable"]
pub type PID25_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID26` writer - Interrupt Disable"]
pub type PID26_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID27` writer - Interrupt Disable"]
pub type PID27_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID28` writer - Interrupt Disable"]
pub type PID28_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID29` writer - Interrupt Disable"]
pub type PID29_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID30` writer - Interrupt Disable"]
pub type PID30_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
#[doc = "Field `PID31` writer - Interrupt Disable"]
pub type PID31_W<'a, const O: u8> = crate::BitWriter<'a, u32, IDCR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Interrupt Disable"]
    #[inline(always)]
    pub fn fiq(&mut self) -> FIQ_W<0> {
        FIQ_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Disable"]
    #[inline(always)]
    pub fn sys(&mut self) -> SYS_W<1> {
        SYS_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid2(&mut self) -> PID2_W<2> {
        PID2_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid3(&mut self) -> PID3_W<3> {
        PID3_W::new(self)
    }
    #[doc = "Bit 4 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid4(&mut self) -> PID4_W<4> {
        PID4_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid5(&mut self) -> PID5_W<5> {
        PID5_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid6(&mut self) -> PID6_W<6> {
        PID6_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid7(&mut self) -> PID7_W<7> {
        PID7_W::new(self)
    }
    #[doc = "Bit 8 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid8(&mut self) -> PID8_W<8> {
        PID8_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid9(&mut self) -> PID9_W<9> {
        PID9_W::new(self)
    }
    #[doc = "Bit 10 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid10(&mut self) -> PID10_W<10> {
        PID10_W::new(self)
    }
    #[doc = "Bit 11 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid11(&mut self) -> PID11_W<11> {
        PID11_W::new(self)
    }
    #[doc = "Bit 12 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid12(&mut self) -> PID12_W<12> {
        PID12_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid13(&mut self) -> PID13_W<13> {
        PID13_W::new(self)
    }
    #[doc = "Bit 14 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid14(&mut self) -> PID14_W<14> {
        PID14_W::new(self)
    }
    #[doc = "Bit 15 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid15(&mut self) -> PID15_W<15> {
        PID15_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid16(&mut self) -> PID16_W<16> {
        PID16_W::new(self)
    }
    #[doc = "Bit 17 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid17(&mut self) -> PID17_W<17> {
        PID17_W::new(self)
    }
    #[doc = "Bit 18 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid18(&mut self) -> PID18_W<18> {
        PID18_W::new(self)
    }
    #[doc = "Bit 19 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid19(&mut self) -> PID19_W<19> {
        PID19_W::new(self)
    }
    #[doc = "Bit 20 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid20(&mut self) -> PID20_W<20> {
        PID20_W::new(self)
    }
    #[doc = "Bit 21 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid21(&mut self) -> PID21_W<21> {
        PID21_W::new(self)
    }
    #[doc = "Bit 22 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid22(&mut self) -> PID22_W<22> {
        PID22_W::new(self)
    }
    #[doc = "Bit 23 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid23(&mut self) -> PID23_W<23> {
        PID23_W::new(self)
    }
    #[doc = "Bit 24 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid24(&mut self) -> PID24_W<24> {
        PID24_W::new(self)
    }
    #[doc = "Bit 25 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid25(&mut self) -> PID25_W<25> {
        PID25_W::new(self)
    }
    #[doc = "Bit 26 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid26(&mut self) -> PID26_W<26> {
        PID26_W::new(self)
    }
    #[doc = "Bit 27 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid27(&mut self) -> PID27_W<27> {
        PID27_W::new(self)
    }
    #[doc = "Bit 28 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid28(&mut self) -> PID28_W<28> {
        PID28_W::new(self)
    }
    #[doc = "Bit 29 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid29(&mut self) -> PID29_W<29> {
        PID29_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid30(&mut self) -> PID30_W<30> {
        PID30_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt Disable"]
    #[inline(always)]
    pub fn pid31(&mut self) -> PID31_W<31> {
        PID31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Disable Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcr](index.html) module"]
pub struct IDCR_SPEC;
impl crate::RegisterSpec for IDCR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [idcr::W](W) writer structure"]
impl crate::Writable for IDCR_SPEC {
    type Writer = W;
}
