#[doc = "Register `NCFGR` reader"]
pub struct R(crate::R<NCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NCFGR` writer"]
pub struct W(crate::W<NCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NCFGR_SPEC>;
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
impl From<crate::W<NCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPD` reader - Speed"]
pub type SPD_R = crate::BitReader<bool>;
#[doc = "Field `SPD` writer - Speed"]
pub type SPD_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `FD` reader - Full Duplex"]
pub type FD_R = crate::BitReader<bool>;
#[doc = "Field `FD` writer - Full Duplex"]
pub type FD_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `JFRAME` reader - Jumbo Frames"]
pub type JFRAME_R = crate::BitReader<bool>;
#[doc = "Field `JFRAME` writer - Jumbo Frames"]
pub type JFRAME_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `CAF` reader - Copy All Frames"]
pub type CAF_R = crate::BitReader<bool>;
#[doc = "Field `CAF` writer - Copy All Frames"]
pub type CAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `NBC` reader - No Broadcast"]
pub type NBC_R = crate::BitReader<bool>;
#[doc = "Field `NBC` writer - No Broadcast"]
pub type NBC_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `MTI` reader - Multicast Hash Enable"]
pub type MTI_R = crate::BitReader<bool>;
#[doc = "Field `MTI` writer - Multicast Hash Enable"]
pub type MTI_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `UNI` reader - Unicast Hash Enable"]
pub type UNI_R = crate::BitReader<bool>;
#[doc = "Field `UNI` writer - Unicast Hash Enable"]
pub type UNI_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `BIG` reader - Receive 1536 bytes frames"]
pub type BIG_R = crate::BitReader<bool>;
#[doc = "Field `BIG` writer - Receive 1536 bytes frames"]
pub type BIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `CLK` reader - MDC clock divider"]
pub type CLK_R = crate::FieldReader<u8, CLK_A>;
#[doc = "MDC clock divider\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK_A {
    #[doc = "0: MCK divided by 8 (MCK up to 20 MHz)."]
    MCK_8 = 0,
    #[doc = "1: MCK divided by 16 (MCK up to 40 MHz)."]
    MCK_16 = 1,
    #[doc = "2: MCK divided by 32 (MCK up to 80 MHz)."]
    MCK_32 = 2,
    #[doc = "3: MCK divided by 64 (MCK up to 160 MHz)."]
    MCK_64 = 3,
}
impl From<CLK_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_A) -> Self {
        variant as _
    }
}
impl CLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLK_A {
        match self.bits {
            0 => CLK_A::MCK_8,
            1 => CLK_A::MCK_16,
            2 => CLK_A::MCK_32,
            3 => CLK_A::MCK_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MCK_8`"]
    #[inline(always)]
    pub fn is_mck_8(&self) -> bool {
        *self == CLK_A::MCK_8
    }
    #[doc = "Checks if the value of the field is `MCK_16`"]
    #[inline(always)]
    pub fn is_mck_16(&self) -> bool {
        *self == CLK_A::MCK_16
    }
    #[doc = "Checks if the value of the field is `MCK_32`"]
    #[inline(always)]
    pub fn is_mck_32(&self) -> bool {
        *self == CLK_A::MCK_32
    }
    #[doc = "Checks if the value of the field is `MCK_64`"]
    #[inline(always)]
    pub fn is_mck_64(&self) -> bool {
        *self == CLK_A::MCK_64
    }
}
#[doc = "Field `CLK` writer - MDC clock divider"]
pub type CLK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, NCFGR_SPEC, u8, CLK_A, 2, O>;
impl<'a, const O: u8> CLK_W<'a, O> {
    #[doc = "MCK divided by 8 (MCK up to 20 MHz)."]
    #[inline(always)]
    pub fn mck_8(self) -> &'a mut W {
        self.variant(CLK_A::MCK_8)
    }
    #[doc = "MCK divided by 16 (MCK up to 40 MHz)."]
    #[inline(always)]
    pub fn mck_16(self) -> &'a mut W {
        self.variant(CLK_A::MCK_16)
    }
    #[doc = "MCK divided by 32 (MCK up to 80 MHz)."]
    #[inline(always)]
    pub fn mck_32(self) -> &'a mut W {
        self.variant(CLK_A::MCK_32)
    }
    #[doc = "MCK divided by 64 (MCK up to 160 MHz)."]
    #[inline(always)]
    pub fn mck_64(self) -> &'a mut W {
        self.variant(CLK_A::MCK_64)
    }
}
#[doc = "Field `RTY` reader - Retry test"]
pub type RTY_R = crate::BitReader<bool>;
#[doc = "Field `RTY` writer - Retry test"]
pub type RTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `PAE` reader - Pause Enable"]
pub type PAE_R = crate::BitReader<bool>;
#[doc = "Field `PAE` writer - Pause Enable"]
pub type PAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `RBOF` reader - Receive Buffer Offset"]
pub type RBOF_R = crate::FieldReader<u8, RBOF_A>;
#[doc = "Receive Buffer Offset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RBOF_A {
    #[doc = "0: No offset from start of receive buffer."]
    OFFSET_0 = 0,
    #[doc = "1: One-byte offset from start of receive buffer."]
    OFFSET_1 = 1,
    #[doc = "2: Two-byte offset from start of receive buffer."]
    OFFSET_2 = 2,
    #[doc = "3: Three-byte offset from start of receive buffer."]
    OFFSET_3 = 3,
}
impl From<RBOF_A> for u8 {
    #[inline(always)]
    fn from(variant: RBOF_A) -> Self {
        variant as _
    }
}
impl RBOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RBOF_A {
        match self.bits {
            0 => RBOF_A::OFFSET_0,
            1 => RBOF_A::OFFSET_1,
            2 => RBOF_A::OFFSET_2,
            3 => RBOF_A::OFFSET_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFFSET_0`"]
    #[inline(always)]
    pub fn is_offset_0(&self) -> bool {
        *self == RBOF_A::OFFSET_0
    }
    #[doc = "Checks if the value of the field is `OFFSET_1`"]
    #[inline(always)]
    pub fn is_offset_1(&self) -> bool {
        *self == RBOF_A::OFFSET_1
    }
    #[doc = "Checks if the value of the field is `OFFSET_2`"]
    #[inline(always)]
    pub fn is_offset_2(&self) -> bool {
        *self == RBOF_A::OFFSET_2
    }
    #[doc = "Checks if the value of the field is `OFFSET_3`"]
    #[inline(always)]
    pub fn is_offset_3(&self) -> bool {
        *self == RBOF_A::OFFSET_3
    }
}
#[doc = "Field `RBOF` writer - Receive Buffer Offset"]
pub type RBOF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, NCFGR_SPEC, u8, RBOF_A, 2, O>;
impl<'a, const O: u8> RBOF_W<'a, O> {
    #[doc = "No offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_0(self) -> &'a mut W {
        self.variant(RBOF_A::OFFSET_0)
    }
    #[doc = "One-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_1(self) -> &'a mut W {
        self.variant(RBOF_A::OFFSET_1)
    }
    #[doc = "Two-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_2(self) -> &'a mut W {
        self.variant(RBOF_A::OFFSET_2)
    }
    #[doc = "Three-byte offset from start of receive buffer."]
    #[inline(always)]
    pub fn offset_3(self) -> &'a mut W {
        self.variant(RBOF_A::OFFSET_3)
    }
}
#[doc = "Field `RLCE` reader - Receive Length field Checking Enable"]
pub type RLCE_R = crate::BitReader<bool>;
#[doc = "Field `RLCE` writer - Receive Length field Checking Enable"]
pub type RLCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `DRFCS` reader - Discard Receive FCS"]
pub type DRFCS_R = crate::BitReader<bool>;
#[doc = "Field `DRFCS` writer - Discard Receive FCS"]
pub type DRFCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `EFRHD` reader - "]
pub type EFRHD_R = crate::BitReader<bool>;
#[doc = "Field `EFRHD` writer - "]
pub type EFRHD_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
#[doc = "Field `IRXFCS` reader - Ignore RX FCS"]
pub type IRXFCS_R = crate::BitReader<bool>;
#[doc = "Field `IRXFCS` writer - Ignore RX FCS"]
pub type IRXFCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, NCFGR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&self) -> SPD_R {
        SPD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&self) -> FD_R {
        FD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline(always)]
    pub fn jframe(&self) -> JFRAME_R {
        JFRAME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&self) -> CAF_R {
        CAF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&self) -> NBC_R {
        NBC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mti(&self) -> MTI_R {
        MTI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn uni(&self) -> UNI_R {
        UNI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline(always)]
    pub fn big(&self) -> BIG_R {
        BIG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn rty(&self) -> RTY_R {
        RTY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pae(&self) -> PAE_R {
        PAE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rbof(&self) -> RBOF_R {
        RBOF_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline(always)]
    pub fn rlce(&self) -> RLCE_R {
        RLCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline(always)]
    pub fn drfcs(&self) -> DRFCS_R {
        DRFCS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn efrhd(&self) -> EFRHD_R {
        EFRHD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&self) -> IRXFCS_R {
        IRXFCS_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Speed"]
    #[inline(always)]
    pub fn spd(&mut self) -> SPD_W<0> {
        SPD_W::new(self)
    }
    #[doc = "Bit 1 - Full Duplex"]
    #[inline(always)]
    pub fn fd(&mut self) -> FD_W<1> {
        FD_W::new(self)
    }
    #[doc = "Bit 3 - Jumbo Frames"]
    #[inline(always)]
    pub fn jframe(&mut self) -> JFRAME_W<3> {
        JFRAME_W::new(self)
    }
    #[doc = "Bit 4 - Copy All Frames"]
    #[inline(always)]
    pub fn caf(&mut self) -> CAF_W<4> {
        CAF_W::new(self)
    }
    #[doc = "Bit 5 - No Broadcast"]
    #[inline(always)]
    pub fn nbc(&mut self) -> NBC_W<5> {
        NBC_W::new(self)
    }
    #[doc = "Bit 6 - Multicast Hash Enable"]
    #[inline(always)]
    pub fn mti(&mut self) -> MTI_W<6> {
        MTI_W::new(self)
    }
    #[doc = "Bit 7 - Unicast Hash Enable"]
    #[inline(always)]
    pub fn uni(&mut self) -> UNI_W<7> {
        UNI_W::new(self)
    }
    #[doc = "Bit 8 - Receive 1536 bytes frames"]
    #[inline(always)]
    pub fn big(&mut self) -> BIG_W<8> {
        BIG_W::new(self)
    }
    #[doc = "Bits 10:11 - MDC clock divider"]
    #[inline(always)]
    pub fn clk(&mut self) -> CLK_W<10> {
        CLK_W::new(self)
    }
    #[doc = "Bit 12 - Retry test"]
    #[inline(always)]
    pub fn rty(&mut self) -> RTY_W<12> {
        RTY_W::new(self)
    }
    #[doc = "Bit 13 - Pause Enable"]
    #[inline(always)]
    pub fn pae(&mut self) -> PAE_W<13> {
        PAE_W::new(self)
    }
    #[doc = "Bits 14:15 - Receive Buffer Offset"]
    #[inline(always)]
    pub fn rbof(&mut self) -> RBOF_W<14> {
        RBOF_W::new(self)
    }
    #[doc = "Bit 16 - Receive Length field Checking Enable"]
    #[inline(always)]
    pub fn rlce(&mut self) -> RLCE_W<16> {
        RLCE_W::new(self)
    }
    #[doc = "Bit 17 - Discard Receive FCS"]
    #[inline(always)]
    pub fn drfcs(&mut self) -> DRFCS_W<17> {
        DRFCS_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn efrhd(&mut self) -> EFRHD_W<18> {
        EFRHD_W::new(self)
    }
    #[doc = "Bit 19 - Ignore RX FCS"]
    #[inline(always)]
    pub fn irxfcs(&mut self) -> IRXFCS_W<19> {
        IRXFCS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Network Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ncfgr](index.html) module"]
pub struct NCFGR_SPEC;
impl crate::RegisterSpec for NCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ncfgr::R](R) reader structure"]
impl crate::Readable for NCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ncfgr::W](W) writer structure"]
impl crate::Writable for NCFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NCFGR to value 0x0800"]
impl crate::Resettable for NCFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800
    }
}
