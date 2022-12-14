#[doc = "Register `BLKR` reader"]
pub struct R(crate::R<BLKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLKR` writer"]
pub struct W(crate::W<BLKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLKR_SPEC>;
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
impl From<crate::W<BLKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCNT` reader - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BCNT_R = crate::FieldReader<u16, BCNT_A>;
#[doc = "MMC/SDIO Block Count - SDIO Byte Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u16)]
pub enum BCNT_A {
    #[doc = "0: MMC/SD CARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    MULTIPLE = 0,
    #[doc = "4: SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    BYTE = 4,
    #[doc = "5: SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    BLOCK = 5,
}
impl From<BCNT_A> for u16 {
    #[inline(always)]
    fn from(variant: BCNT_A) -> Self {
        variant as _
    }
}
impl BCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BCNT_A> {
        match self.bits {
            0 => Some(BCNT_A::MULTIPLE),
            4 => Some(BCNT_A::BYTE),
            5 => Some(BCNT_A::BLOCK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MULTIPLE`"]
    #[inline(always)]
    pub fn is_multiple(&self) -> bool {
        *self == BCNT_A::MULTIPLE
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == BCNT_A::BYTE
    }
    #[doc = "Checks if the value of the field is `BLOCK`"]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == BCNT_A::BLOCK
    }
}
#[doc = "Field `BCNT` writer - MMC/SDIO Block Count - SDIO Byte Count"]
pub type BCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLKR_SPEC, u16, BCNT_A, 16, O>;
impl<'a, const O: u8> BCNT_W<'a, O> {
    #[doc = "MMC/SD CARD Multiple BlockFrom 1 to 65635: Value 0 corresponds to an infinite block transfer."]
    #[inline(always)]
    pub fn multiple(self) -> &'a mut W {
        self.variant(BCNT_A::MULTIPLE)
    }
    #[doc = "SDIO ByteFrom 1 to 512 bytes: Value 0 corresponds to a 512-byte transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(BCNT_A::BYTE)
    }
    #[doc = "SDIO BlockFrom 1 to 511 blocks: Value 0 corresponds to an infinite block transfer.Values from 0x200 to 0xFFFF are forbidden."]
    #[inline(always)]
    pub fn block(self) -> &'a mut W {
        self.variant(BCNT_A::BLOCK)
    }
}
#[doc = "Field `BLKLEN` reader - Data Block Length"]
pub type BLKLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BLKLEN` writer - Data Block Length"]
pub type BLKLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BLKR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BCNT_R {
        BCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    pub fn blklen(&self) -> BLKLEN_R {
        BLKLEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MMC/SDIO Block Count - SDIO Byte Count"]
    #[inline(always)]
    pub fn bcnt(&mut self) -> BCNT_W<0> {
        BCNT_W::new(self)
    }
    #[doc = "Bits 16:31 - Data Block Length"]
    #[inline(always)]
    pub fn blklen(&mut self) -> BLKLEN_W<16> {
        BLKLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blkr](index.html) module"]
pub struct BLKR_SPEC;
impl crate::RegisterSpec for BLKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blkr::R](R) reader structure"]
impl crate::Readable for BLKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blkr::W](W) writer structure"]
impl crate::Writable for BLKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BLKR to value 0"]
impl crate::Resettable for BLKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
