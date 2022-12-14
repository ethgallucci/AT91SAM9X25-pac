#[doc = "Register `CHER` writer"]
pub struct W(crate::W<CHER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHER_SPEC>;
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
impl From<crate::W<CHER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENA0` writer - Enable \\[7:0\\]"]
pub type ENA0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `ENA1` writer - Enable \\[7:0\\]"]
pub type ENA1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `ENA2` writer - Enable \\[7:0\\]"]
pub type ENA2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `ENA3` writer - Enable \\[7:0\\]"]
pub type ENA3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `ENA4` writer - Enable \\[7:0\\]"]
pub type ENA4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `ENA5` writer - Enable \\[7:0\\]"]
pub type ENA5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `ENA6` writer - Enable \\[7:0\\]"]
pub type ENA6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `ENA7` writer - Enable \\[7:0\\]"]
pub type ENA7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `SUSP0` writer - Suspend \\[7:0\\]"]
pub type SUSP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `SUSP1` writer - Suspend \\[7:0\\]"]
pub type SUSP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `SUSP2` writer - Suspend \\[7:0\\]"]
pub type SUSP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `SUSP3` writer - Suspend \\[7:0\\]"]
pub type SUSP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `SUSP4` writer - Suspend \\[7:0\\]"]
pub type SUSP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `SUSP5` writer - Suspend \\[7:0\\]"]
pub type SUSP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `SUSP6` writer - Suspend \\[7:0\\]"]
pub type SUSP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `SUSP7` writer - Suspend \\[7:0\\]"]
pub type SUSP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `KEEP0` writer - Keep on \\[7:0\\]"]
pub type KEEP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `KEEP1` writer - Keep on \\[7:0\\]"]
pub type KEEP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `KEEP2` writer - Keep on \\[7:0\\]"]
pub type KEEP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `KEEP3` writer - Keep on \\[7:0\\]"]
pub type KEEP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `KEEP4` writer - Keep on \\[7:0\\]"]
pub type KEEP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `KEEP5` writer - Keep on \\[7:0\\]"]
pub type KEEP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `KEEP6` writer - Keep on \\[7:0\\]"]
pub type KEEP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
#[doc = "Field `KEEP7` writer - Keep on \\[7:0\\]"]
pub type KEEP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHER_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Enable \\[7:0\\]"]
    #[inline(always)]
    pub fn ena0(&mut self) -> ENA0_W<0> {
        ENA0_W::new(self)
    }
    #[doc = "Bit 1 - Enable \\[7:0\\]"]
    #[inline(always)]
    pub fn ena1(&mut self) -> ENA1_W<1> {
        ENA1_W::new(self)
    }
    #[doc = "Bit 2 - Enable \\[7:0\\]"]
    #[inline(always)]
    pub fn ena2(&mut self) -> ENA2_W<2> {
        ENA2_W::new(self)
    }
    #[doc = "Bit 3 - Enable \\[7:0\\]"]
    #[inline(always)]
    pub fn ena3(&mut self) -> ENA3_W<3> {
        ENA3_W::new(self)
    }
    #[doc = "Bit 4 - Enable \\[7:0\\]"]
    #[inline(always)]
    pub fn ena4(&mut self) -> ENA4_W<4> {
        ENA4_W::new(self)
    }
    #[doc = "Bit 5 - Enable \\[7:0\\]"]
    #[inline(always)]
    pub fn ena5(&mut self) -> ENA5_W<5> {
        ENA5_W::new(self)
    }
    #[doc = "Bit 6 - Enable \\[7:0\\]"]
    #[inline(always)]
    pub fn ena6(&mut self) -> ENA6_W<6> {
        ENA6_W::new(self)
    }
    #[doc = "Bit 7 - Enable \\[7:0\\]"]
    #[inline(always)]
    pub fn ena7(&mut self) -> ENA7_W<7> {
        ENA7_W::new(self)
    }
    #[doc = "Bit 8 - Suspend \\[7:0\\]"]
    #[inline(always)]
    pub fn susp0(&mut self) -> SUSP0_W<8> {
        SUSP0_W::new(self)
    }
    #[doc = "Bit 9 - Suspend \\[7:0\\]"]
    #[inline(always)]
    pub fn susp1(&mut self) -> SUSP1_W<9> {
        SUSP1_W::new(self)
    }
    #[doc = "Bit 10 - Suspend \\[7:0\\]"]
    #[inline(always)]
    pub fn susp2(&mut self) -> SUSP2_W<10> {
        SUSP2_W::new(self)
    }
    #[doc = "Bit 11 - Suspend \\[7:0\\]"]
    #[inline(always)]
    pub fn susp3(&mut self) -> SUSP3_W<11> {
        SUSP3_W::new(self)
    }
    #[doc = "Bit 12 - Suspend \\[7:0\\]"]
    #[inline(always)]
    pub fn susp4(&mut self) -> SUSP4_W<12> {
        SUSP4_W::new(self)
    }
    #[doc = "Bit 13 - Suspend \\[7:0\\]"]
    #[inline(always)]
    pub fn susp5(&mut self) -> SUSP5_W<13> {
        SUSP5_W::new(self)
    }
    #[doc = "Bit 14 - Suspend \\[7:0\\]"]
    #[inline(always)]
    pub fn susp6(&mut self) -> SUSP6_W<14> {
        SUSP6_W::new(self)
    }
    #[doc = "Bit 15 - Suspend \\[7:0\\]"]
    #[inline(always)]
    pub fn susp7(&mut self) -> SUSP7_W<15> {
        SUSP7_W::new(self)
    }
    #[doc = "Bit 24 - Keep on \\[7:0\\]"]
    #[inline(always)]
    pub fn keep0(&mut self) -> KEEP0_W<24> {
        KEEP0_W::new(self)
    }
    #[doc = "Bit 25 - Keep on \\[7:0\\]"]
    #[inline(always)]
    pub fn keep1(&mut self) -> KEEP1_W<25> {
        KEEP1_W::new(self)
    }
    #[doc = "Bit 26 - Keep on \\[7:0\\]"]
    #[inline(always)]
    pub fn keep2(&mut self) -> KEEP2_W<26> {
        KEEP2_W::new(self)
    }
    #[doc = "Bit 27 - Keep on \\[7:0\\]"]
    #[inline(always)]
    pub fn keep3(&mut self) -> KEEP3_W<27> {
        KEEP3_W::new(self)
    }
    #[doc = "Bit 28 - Keep on \\[7:0\\]"]
    #[inline(always)]
    pub fn keep4(&mut self) -> KEEP4_W<28> {
        KEEP4_W::new(self)
    }
    #[doc = "Bit 29 - Keep on \\[7:0\\]"]
    #[inline(always)]
    pub fn keep5(&mut self) -> KEEP5_W<29> {
        KEEP5_W::new(self)
    }
    #[doc = "Bit 30 - Keep on \\[7:0\\]"]
    #[inline(always)]
    pub fn keep6(&mut self) -> KEEP6_W<30> {
        KEEP6_W::new(self)
    }
    #[doc = "Bit 31 - Keep on \\[7:0\\]"]
    #[inline(always)]
    pub fn keep7(&mut self) -> KEEP7_W<31> {
        KEEP7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Handler Enable Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cher](index.html) module"]
pub struct CHER_SPEC;
impl crate::RegisterSpec for CHER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cher::W](W) writer structure"]
impl crate::Writable for CHER_SPEC {
    type Writer = W;
}
