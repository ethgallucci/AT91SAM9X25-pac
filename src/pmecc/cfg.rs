#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCH_ERR` reader - Error Correct Capability"]
pub type BCH_ERR_R = crate::FieldReader<u8, BCH_ERR_A>;
#[doc = "Error Correct Capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BCH_ERR_A {
    #[doc = "0: 2 errors"]
    BCH_ERR2 = 0,
    #[doc = "1: 4 errors"]
    BCH_ERR4 = 1,
    #[doc = "2: 8 errors"]
    BCH_ERR8 = 2,
    #[doc = "3: 12 errors"]
    BCH_ERR12 = 3,
    #[doc = "4: 24 errors"]
    BCH_ERR24 = 4,
}
impl From<BCH_ERR_A> for u8 {
    #[inline(always)]
    fn from(variant: BCH_ERR_A) -> Self {
        variant as _
    }
}
impl BCH_ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BCH_ERR_A> {
        match self.bits {
            0 => Some(BCH_ERR_A::BCH_ERR2),
            1 => Some(BCH_ERR_A::BCH_ERR4),
            2 => Some(BCH_ERR_A::BCH_ERR8),
            3 => Some(BCH_ERR_A::BCH_ERR12),
            4 => Some(BCH_ERR_A::BCH_ERR24),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BCH_ERR2`"]
    #[inline(always)]
    pub fn is_bch_err2(&self) -> bool {
        *self == BCH_ERR_A::BCH_ERR2
    }
    #[doc = "Checks if the value of the field is `BCH_ERR4`"]
    #[inline(always)]
    pub fn is_bch_err4(&self) -> bool {
        *self == BCH_ERR_A::BCH_ERR4
    }
    #[doc = "Checks if the value of the field is `BCH_ERR8`"]
    #[inline(always)]
    pub fn is_bch_err8(&self) -> bool {
        *self == BCH_ERR_A::BCH_ERR8
    }
    #[doc = "Checks if the value of the field is `BCH_ERR12`"]
    #[inline(always)]
    pub fn is_bch_err12(&self) -> bool {
        *self == BCH_ERR_A::BCH_ERR12
    }
    #[doc = "Checks if the value of the field is `BCH_ERR24`"]
    #[inline(always)]
    pub fn is_bch_err24(&self) -> bool {
        *self == BCH_ERR_A::BCH_ERR24
    }
}
#[doc = "Field `BCH_ERR` writer - Error Correct Capability"]
pub type BCH_ERR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CFG_SPEC, u8, BCH_ERR_A, 3, O>;
impl<'a, const O: u8> BCH_ERR_W<'a, O> {
    #[doc = "2 errors"]
    #[inline(always)]
    pub fn bch_err2(self) -> &'a mut W {
        self.variant(BCH_ERR_A::BCH_ERR2)
    }
    #[doc = "4 errors"]
    #[inline(always)]
    pub fn bch_err4(self) -> &'a mut W {
        self.variant(BCH_ERR_A::BCH_ERR4)
    }
    #[doc = "8 errors"]
    #[inline(always)]
    pub fn bch_err8(self) -> &'a mut W {
        self.variant(BCH_ERR_A::BCH_ERR8)
    }
    #[doc = "12 errors"]
    #[inline(always)]
    pub fn bch_err12(self) -> &'a mut W {
        self.variant(BCH_ERR_A::BCH_ERR12)
    }
    #[doc = "24 errors"]
    #[inline(always)]
    pub fn bch_err24(self) -> &'a mut W {
        self.variant(BCH_ERR_A::BCH_ERR24)
    }
}
#[doc = "Field `SECTORSZ` reader - Sector Size"]
pub type SECTORSZ_R = crate::BitReader<bool>;
#[doc = "Field `SECTORSZ` writer - Sector Size"]
pub type SECTORSZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `PAGESIZE` reader - Number of Sectors in the Page"]
pub type PAGESIZE_R = crate::FieldReader<u8, PAGESIZE_A>;
#[doc = "Number of Sectors in the Page\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PAGESIZE_A {
    #[doc = "0: 1 sector for main area (512 or 1024 bytes)"]
    PAGESIZE_1SEC = 0,
    #[doc = "1: 2 sectors for main area (1024 or 2048 bytes)"]
    PAGESIZE_2SEC = 1,
    #[doc = "2: 4 sectors for main area (2048 or 4096 bytes)"]
    PAGESIZE_4SEC = 2,
    #[doc = "3: 8 errors for main area (4096 or 8192 bytes)"]
    PAGESIZE_8SEC = 3,
}
impl From<PAGESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PAGESIZE_A) -> Self {
        variant as _
    }
}
impl PAGESIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PAGESIZE_A {
        match self.bits {
            0 => PAGESIZE_A::PAGESIZE_1SEC,
            1 => PAGESIZE_A::PAGESIZE_2SEC,
            2 => PAGESIZE_A::PAGESIZE_4SEC,
            3 => PAGESIZE_A::PAGESIZE_8SEC,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PAGESIZE_1SEC`"]
    #[inline(always)]
    pub fn is_pagesize_1sec(&self) -> bool {
        *self == PAGESIZE_A::PAGESIZE_1SEC
    }
    #[doc = "Checks if the value of the field is `PAGESIZE_2SEC`"]
    #[inline(always)]
    pub fn is_pagesize_2sec(&self) -> bool {
        *self == PAGESIZE_A::PAGESIZE_2SEC
    }
    #[doc = "Checks if the value of the field is `PAGESIZE_4SEC`"]
    #[inline(always)]
    pub fn is_pagesize_4sec(&self) -> bool {
        *self == PAGESIZE_A::PAGESIZE_4SEC
    }
    #[doc = "Checks if the value of the field is `PAGESIZE_8SEC`"]
    #[inline(always)]
    pub fn is_pagesize_8sec(&self) -> bool {
        *self == PAGESIZE_A::PAGESIZE_8SEC
    }
}
#[doc = "Field `PAGESIZE` writer - Number of Sectors in the Page"]
pub type PAGESIZE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CFG_SPEC, u8, PAGESIZE_A, 2, O>;
impl<'a, const O: u8> PAGESIZE_W<'a, O> {
    #[doc = "1 sector for main area (512 or 1024 bytes)"]
    #[inline(always)]
    pub fn pagesize_1sec(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PAGESIZE_1SEC)
    }
    #[doc = "2 sectors for main area (1024 or 2048 bytes)"]
    #[inline(always)]
    pub fn pagesize_2sec(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PAGESIZE_2SEC)
    }
    #[doc = "4 sectors for main area (2048 or 4096 bytes)"]
    #[inline(always)]
    pub fn pagesize_4sec(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PAGESIZE_4SEC)
    }
    #[doc = "8 errors for main area (4096 or 8192 bytes)"]
    #[inline(always)]
    pub fn pagesize_8sec(self) -> &'a mut W {
        self.variant(PAGESIZE_A::PAGESIZE_8SEC)
    }
}
#[doc = "Field `NANDWR` reader - NAND Write Access"]
pub type NANDWR_R = crate::BitReader<bool>;
#[doc = "Field `NANDWR` writer - NAND Write Access"]
pub type NANDWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `SPAREEN` reader - Spare Enable"]
pub type SPAREEN_R = crate::BitReader<bool>;
#[doc = "Field `SPAREEN` writer - Spare Enable"]
pub type SPAREEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
#[doc = "Field `AUTO` reader - Automatic Mode Enable"]
pub type AUTO_R = crate::BitReader<bool>;
#[doc = "Field `AUTO` writer - Automatic Mode Enable"]
pub type AUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Error Correct Capability"]
    #[inline(always)]
    pub fn bch_err(&self) -> BCH_ERR_R {
        BCH_ERR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Sector Size"]
    #[inline(always)]
    pub fn sectorsz(&self) -> SECTORSZ_R {
        SECTORSZ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Number of Sectors in the Page"]
    #[inline(always)]
    pub fn pagesize(&self) -> PAGESIZE_R {
        PAGESIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - NAND Write Access"]
    #[inline(always)]
    pub fn nandwr(&self) -> NANDWR_R {
        NANDWR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Spare Enable"]
    #[inline(always)]
    pub fn spareen(&self) -> SPAREEN_R {
        SPAREEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Automatic Mode Enable"]
    #[inline(always)]
    pub fn auto(&self) -> AUTO_R {
        AUTO_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Error Correct Capability"]
    #[inline(always)]
    pub fn bch_err(&mut self) -> BCH_ERR_W<0> {
        BCH_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Sector Size"]
    #[inline(always)]
    pub fn sectorsz(&mut self) -> SECTORSZ_W<4> {
        SECTORSZ_W::new(self)
    }
    #[doc = "Bits 8:9 - Number of Sectors in the Page"]
    #[inline(always)]
    pub fn pagesize(&mut self) -> PAGESIZE_W<8> {
        PAGESIZE_W::new(self)
    }
    #[doc = "Bit 12 - NAND Write Access"]
    #[inline(always)]
    pub fn nandwr(&mut self) -> NANDWR_W<12> {
        NANDWR_W::new(self)
    }
    #[doc = "Bit 16 - Spare Enable"]
    #[inline(always)]
    pub fn spareen(&mut self) -> SPAREEN_W<16> {
        SPAREEN_W::new(self)
    }
    #[doc = "Bit 20 - Automatic Mode Enable"]
    #[inline(always)]
    pub fn auto(&mut self) -> AUTO_W<20> {
        AUTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMECC Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
