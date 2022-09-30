#[doc = "Register `CTRLB7` reader"]
pub struct R(crate::R<CTRLB7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB7` writer"]
pub struct W(crate::W<CTRLB7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB7_SPEC>;
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
impl From<crate::W<CTRLB7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIF` reader - Source Interface Selection Field"]
pub type SIF_R = crate::FieldReader<u8, SIF_A>;
#[doc = "Source Interface Selection Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SIF_A {
    #[doc = "0: The source transfer is done via AHB-Lite Interface 0"]
    AHB_IF0 = 0,
    #[doc = "1: The source transfer is done via AHB-Lite Interface 1"]
    AHB_IF1 = 1,
}
impl From<SIF_A> for u8 {
    #[inline(always)]
    fn from(variant: SIF_A) -> Self {
        variant as _
    }
}
impl SIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SIF_A> {
        match self.bits {
            0 => Some(SIF_A::AHB_IF0),
            1 => Some(SIF_A::AHB_IF1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == SIF_A::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == SIF_A::AHB_IF1
    }
}
#[doc = "Field `SIF` writer - Source Interface Selection Field"]
pub type SIF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLB7_SPEC, u8, SIF_A, 2, O>;
impl<'a, const O: u8> SIF_W<'a, O> {
    #[doc = "The source transfer is done via AHB-Lite Interface 0"]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(SIF_A::AHB_IF0)
    }
    #[doc = "The source transfer is done via AHB-Lite Interface 1"]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(SIF_A::AHB_IF1)
    }
}
#[doc = "Field `DIF` reader - Destination Interface Selection Field"]
pub type DIF_R = crate::FieldReader<u8, DIF_A>;
#[doc = "Destination Interface Selection Field\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIF_A {
    #[doc = "0: The destination transfer is done via AHB-Lite Interface 0"]
    AHB_IF0 = 0,
    #[doc = "1: The destination transfer is done via AHB-Lite Interface 1"]
    AHB_IF1 = 1,
}
impl From<DIF_A> for u8 {
    #[inline(always)]
    fn from(variant: DIF_A) -> Self {
        variant as _
    }
}
impl DIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DIF_A> {
        match self.bits {
            0 => Some(DIF_A::AHB_IF0),
            1 => Some(DIF_A::AHB_IF1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AHB_IF0`"]
    #[inline(always)]
    pub fn is_ahb_if0(&self) -> bool {
        *self == DIF_A::AHB_IF0
    }
    #[doc = "Checks if the value of the field is `AHB_IF1`"]
    #[inline(always)]
    pub fn is_ahb_if1(&self) -> bool {
        *self == DIF_A::AHB_IF1
    }
}
#[doc = "Field `DIF` writer - Destination Interface Selection Field"]
pub type DIF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLB7_SPEC, u8, DIF_A, 2, O>;
impl<'a, const O: u8> DIF_W<'a, O> {
    #[doc = "The destination transfer is done via AHB-Lite Interface 0"]
    #[inline(always)]
    pub fn ahb_if0(self) -> &'a mut W {
        self.variant(DIF_A::AHB_IF0)
    }
    #[doc = "The destination transfer is done via AHB-Lite Interface 1"]
    #[inline(always)]
    pub fn ahb_if1(self) -> &'a mut W {
        self.variant(DIF_A::AHB_IF1)
    }
}
#[doc = "Field `SRC_PIP` reader - Source Picture-in-Picture Mode"]
pub type SRC_PIP_R = crate::BitReader<SRC_PIP_A>;
#[doc = "Source Picture-in-Picture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_PIP_A {
    #[doc = "0: Picture-in-Picture mode is disabled. The source data area is contiguous."]
    DISABLE = 0,
    #[doc = "1: Picture-in-Picture mode is enabled. When the source PIP counter reaches the programmable boundary, the address is automatically incremented by a user defined amount."]
    ENABLE = 1,
}
impl From<SRC_PIP_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_PIP_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_PIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_PIP_A {
        match self.bits {
            false => SRC_PIP_A::DISABLE,
            true => SRC_PIP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SRC_PIP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SRC_PIP_A::ENABLE
    }
}
#[doc = "Field `SRC_PIP` writer - Source Picture-in-Picture Mode"]
pub type SRC_PIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB7_SPEC, SRC_PIP_A, O>;
impl<'a, const O: u8> SRC_PIP_W<'a, O> {
    #[doc = "Picture-in-Picture mode is disabled. The source data area is contiguous."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SRC_PIP_A::DISABLE)
    }
    #[doc = "Picture-in-Picture mode is enabled. When the source PIP counter reaches the programmable boundary, the address is automatically incremented by a user defined amount."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SRC_PIP_A::ENABLE)
    }
}
#[doc = "Field `DST_PIP` reader - Destination Picture-in-Picture Mode"]
pub type DST_PIP_R = crate::BitReader<DST_PIP_A>;
#[doc = "Destination Picture-in-Picture Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_PIP_A {
    #[doc = "0: Picture-in-Picture mode is disabled. The Destination data area is contiguous."]
    DISABLE = 0,
    #[doc = "1: Picture-in-Picture mode is enabled. When the Destination PIP counter reaches the programmable boundary the address is automatically incremented by a user-defined amount."]
    ENABLE = 1,
}
impl From<DST_PIP_A> for bool {
    #[inline(always)]
    fn from(variant: DST_PIP_A) -> Self {
        variant as u8 != 0
    }
}
impl DST_PIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_PIP_A {
        match self.bits {
            false => DST_PIP_A::DISABLE,
            true => DST_PIP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DST_PIP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DST_PIP_A::ENABLE
    }
}
#[doc = "Field `DST_PIP` writer - Destination Picture-in-Picture Mode"]
pub type DST_PIP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB7_SPEC, DST_PIP_A, O>;
impl<'a, const O: u8> DST_PIP_W<'a, O> {
    #[doc = "Picture-in-Picture mode is disabled. The Destination data area is contiguous."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DST_PIP_A::DISABLE)
    }
    #[doc = "Picture-in-Picture mode is enabled. When the Destination PIP counter reaches the programmable boundary the address is automatically incremented by a user-defined amount."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DST_PIP_A::ENABLE)
    }
}
#[doc = "Field `SRC_DSCR` reader - Source Address Descriptor"]
pub type SRC_DSCR_R = crate::BitReader<SRC_DSCR_A>;
#[doc = "Source Address Descriptor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_DSCR_A {
    #[doc = "0: Source address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM = 0,
    #[doc = "1: Buffer Descriptor Fetch operation is disabled for the source."]
    FETCH_DISABLE = 1,
}
impl From<SRC_DSCR_A> for bool {
    #[inline(always)]
    fn from(variant: SRC_DSCR_A) -> Self {
        variant as u8 != 0
    }
}
impl SRC_DSCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRC_DSCR_A {
        match self.bits {
            false => SRC_DSCR_A::FETCH_FROM_MEM,
            true => SRC_DSCR_A::FETCH_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FETCH_FROM_MEM`"]
    #[inline(always)]
    pub fn is_fetch_from_mem(&self) -> bool {
        *self == SRC_DSCR_A::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        *self == SRC_DSCR_A::FETCH_DISABLE
    }
}
#[doc = "Field `SRC_DSCR` writer - Source Address Descriptor"]
pub type SRC_DSCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB7_SPEC, SRC_DSCR_A, O>;
impl<'a, const O: u8> SRC_DSCR_W<'a, O> {
    #[doc = "Source address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn fetch_from_mem(self) -> &'a mut W {
        self.variant(SRC_DSCR_A::FETCH_FROM_MEM)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the source."]
    #[inline(always)]
    pub fn fetch_disable(self) -> &'a mut W {
        self.variant(SRC_DSCR_A::FETCH_DISABLE)
    }
}
#[doc = "Field `DST_DSCR` reader - Destination Address Descriptor"]
pub type DST_DSCR_R = crate::BitReader<DST_DSCR_A>;
#[doc = "Destination Address Descriptor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DST_DSCR_A {
    #[doc = "0: Destination address is updated when the descriptor is fetched from the memory."]
    FETCH_FROM_MEM = 0,
    #[doc = "1: Buffer Descriptor Fetch operation is disabled for the destination."]
    FETCH_DISABLE = 1,
}
impl From<DST_DSCR_A> for bool {
    #[inline(always)]
    fn from(variant: DST_DSCR_A) -> Self {
        variant as u8 != 0
    }
}
impl DST_DSCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DST_DSCR_A {
        match self.bits {
            false => DST_DSCR_A::FETCH_FROM_MEM,
            true => DST_DSCR_A::FETCH_DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `FETCH_FROM_MEM`"]
    #[inline(always)]
    pub fn is_fetch_from_mem(&self) -> bool {
        *self == DST_DSCR_A::FETCH_FROM_MEM
    }
    #[doc = "Checks if the value of the field is `FETCH_DISABLE`"]
    #[inline(always)]
    pub fn is_fetch_disable(&self) -> bool {
        *self == DST_DSCR_A::FETCH_DISABLE
    }
}
#[doc = "Field `DST_DSCR` writer - Destination Address Descriptor"]
pub type DST_DSCR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB7_SPEC, DST_DSCR_A, O>;
impl<'a, const O: u8> DST_DSCR_W<'a, O> {
    #[doc = "Destination address is updated when the descriptor is fetched from the memory."]
    #[inline(always)]
    pub fn fetch_from_mem(self) -> &'a mut W {
        self.variant(DST_DSCR_A::FETCH_FROM_MEM)
    }
    #[doc = "Buffer Descriptor Fetch operation is disabled for the destination."]
    #[inline(always)]
    pub fn fetch_disable(self) -> &'a mut W {
        self.variant(DST_DSCR_A::FETCH_DISABLE)
    }
}
#[doc = "Field `FC` reader - Flow Control"]
pub type FC_R = crate::FieldReader<u8, FC_A>;
#[doc = "Flow Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FC_A {
    #[doc = "0: Memory-to-Memory Transfer DMAC is flow controller"]
    MEM2MEM_DMA_FC = 0,
    #[doc = "1: Memory-to-Peripheral Transfer DMAC is flow controller"]
    MEM2PER_DMA_FC = 1,
    #[doc = "2: Peripheral-to-Memory Transfer DMAC is flow controller"]
    PER2MEM_DMA_FC = 2,
    #[doc = "3: Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    PER2PER_DMA_FC = 3,
}
impl From<FC_A> for u8 {
    #[inline(always)]
    fn from(variant: FC_A) -> Self {
        variant as _
    }
}
impl FC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FC_A> {
        match self.bits {
            0 => Some(FC_A::MEM2MEM_DMA_FC),
            1 => Some(FC_A::MEM2PER_DMA_FC),
            2 => Some(FC_A::PER2MEM_DMA_FC),
            3 => Some(FC_A::PER2PER_DMA_FC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MEM2MEM_DMA_FC`"]
    #[inline(always)]
    pub fn is_mem2mem_dma_fc(&self) -> bool {
        *self == FC_A::MEM2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `MEM2PER_DMA_FC`"]
    #[inline(always)]
    pub fn is_mem2per_dma_fc(&self) -> bool {
        *self == FC_A::MEM2PER_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2MEM_DMA_FC`"]
    #[inline(always)]
    pub fn is_per2mem_dma_fc(&self) -> bool {
        *self == FC_A::PER2MEM_DMA_FC
    }
    #[doc = "Checks if the value of the field is `PER2PER_DMA_FC`"]
    #[inline(always)]
    pub fn is_per2per_dma_fc(&self) -> bool {
        *self == FC_A::PER2PER_DMA_FC
    }
}
#[doc = "Field `FC` writer - Flow Control"]
pub type FC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRLB7_SPEC, u8, FC_A, 3, O>;
impl<'a, const O: u8> FC_W<'a, O> {
    #[doc = "Memory-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn mem2mem_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::MEM2MEM_DMA_FC)
    }
    #[doc = "Memory-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn mem2per_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::MEM2PER_DMA_FC)
    }
    #[doc = "Peripheral-to-Memory Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn per2mem_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::PER2MEM_DMA_FC)
    }
    #[doc = "Peripheral-to-Peripheral Transfer DMAC is flow controller"]
    #[inline(always)]
    pub fn per2per_dma_fc(self) -> &'a mut W {
        self.variant(FC_A::PER2PER_DMA_FC)
    }
}
#[doc = "Field `SRC_INCR` reader - Incrementing, Decrementing or Fixed Address for the Source"]
pub type SRC_INCR_R = crate::FieldReader<u8, SRC_INCR_A>;
#[doc = "Incrementing, Decrementing or Fixed Address for the Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SRC_INCR_A {
    #[doc = "0: The source address is incremented"]
    INCREMENTING = 0,
    #[doc = "1: The source address is decremented"]
    DECREMENTING = 1,
    #[doc = "2: The source address remains unchanged"]
    FIXED = 2,
}
impl From<SRC_INCR_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_INCR_A) -> Self {
        variant as _
    }
}
impl SRC_INCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SRC_INCR_A> {
        match self.bits {
            0 => Some(SRC_INCR_A::INCREMENTING),
            1 => Some(SRC_INCR_A::DECREMENTING),
            2 => Some(SRC_INCR_A::FIXED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENTING`"]
    #[inline(always)]
    pub fn is_incrementing(&self) -> bool {
        *self == SRC_INCR_A::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        *self == SRC_INCR_A::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == SRC_INCR_A::FIXED
    }
}
#[doc = "Field `SRC_INCR` writer - Incrementing, Decrementing or Fixed Address for the Source"]
pub type SRC_INCR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLB7_SPEC, u8, SRC_INCR_A, 2, O>;
impl<'a, const O: u8> SRC_INCR_W<'a, O> {
    #[doc = "The source address is incremented"]
    #[inline(always)]
    pub fn incrementing(self) -> &'a mut W {
        self.variant(SRC_INCR_A::INCREMENTING)
    }
    #[doc = "The source address is decremented"]
    #[inline(always)]
    pub fn decrementing(self) -> &'a mut W {
        self.variant(SRC_INCR_A::DECREMENTING)
    }
    #[doc = "The source address remains unchanged"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(SRC_INCR_A::FIXED)
    }
}
#[doc = "Field `DST_INCR` reader - Incrementing, Decrementing or Fixed Address for the Destination"]
pub type DST_INCR_R = crate::FieldReader<u8, DST_INCR_A>;
#[doc = "Incrementing, Decrementing or Fixed Address for the Destination\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DST_INCR_A {
    #[doc = "0: The destination address is incremented"]
    INCREMENTING = 0,
    #[doc = "1: The destination address is decremented"]
    DECREMENTING = 1,
    #[doc = "2: The destination address remains unchanged"]
    FIXED = 2,
}
impl From<DST_INCR_A> for u8 {
    #[inline(always)]
    fn from(variant: DST_INCR_A) -> Self {
        variant as _
    }
}
impl DST_INCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DST_INCR_A> {
        match self.bits {
            0 => Some(DST_INCR_A::INCREMENTING),
            1 => Some(DST_INCR_A::DECREMENTING),
            2 => Some(DST_INCR_A::FIXED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INCREMENTING`"]
    #[inline(always)]
    pub fn is_incrementing(&self) -> bool {
        *self == DST_INCR_A::INCREMENTING
    }
    #[doc = "Checks if the value of the field is `DECREMENTING`"]
    #[inline(always)]
    pub fn is_decrementing(&self) -> bool {
        *self == DST_INCR_A::DECREMENTING
    }
    #[doc = "Checks if the value of the field is `FIXED`"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == DST_INCR_A::FIXED
    }
}
#[doc = "Field `DST_INCR` writer - Incrementing, Decrementing or Fixed Address for the Destination"]
pub type DST_INCR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRLB7_SPEC, u8, DST_INCR_A, 2, O>;
impl<'a, const O: u8> DST_INCR_W<'a, O> {
    #[doc = "The destination address is incremented"]
    #[inline(always)]
    pub fn incrementing(self) -> &'a mut W {
        self.variant(DST_INCR_A::INCREMENTING)
    }
    #[doc = "The destination address is decremented"]
    #[inline(always)]
    pub fn decrementing(self) -> &'a mut W {
        self.variant(DST_INCR_A::DECREMENTING)
    }
    #[doc = "The destination address remains unchanged"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut W {
        self.variant(DST_INCR_A::FIXED)
    }
}
#[doc = "Field `IEN` reader - "]
pub type IEN_R = crate::BitReader<bool>;
#[doc = "Field `IEN` writer - "]
pub type IEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB7_SPEC, bool, O>;
#[doc = "Field `AUTO` reader - Automatic Multiple Buffer Transfer"]
pub type AUTO_R = crate::BitReader<AUTO_A>;
#[doc = "Automatic Multiple Buffer Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AUTO_A {
    #[doc = "0: Automatic multiple buffer transfer is disabled."]
    DISABLE = 0,
    #[doc = "1: Automatic multiple buffer transfer is enabled. This bit enables replay mode or contiguous mode when several buffers are transferred."]
    ENABLE = 1,
}
impl From<AUTO_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_A {
        match self.bits {
            false => AUTO_A::DISABLE,
            true => AUTO_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == AUTO_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == AUTO_A::ENABLE
    }
}
#[doc = "Field `AUTO` writer - Automatic Multiple Buffer Transfer"]
pub type AUTO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRLB7_SPEC, AUTO_A, O>;
impl<'a, const O: u8> AUTO_W<'a, O> {
    #[doc = "Automatic multiple buffer transfer is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(AUTO_A::DISABLE)
    }
    #[doc = "Automatic multiple buffer transfer is enabled. This bit enables replay mode or contiguous mode when several buffers are transferred."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(AUTO_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Source Interface Selection Field"]
    #[inline(always)]
    pub fn sif(&self) -> SIF_R {
        SIF_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Destination Interface Selection Field"]
    #[inline(always)]
    pub fn dif(&self) -> DIF_R {
        DIF_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Source Picture-in-Picture Mode"]
    #[inline(always)]
    pub fn src_pip(&self) -> SRC_PIP_R {
        SRC_PIP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Destination Picture-in-Picture Mode"]
    #[inline(always)]
    pub fn dst_pip(&self) -> DST_PIP_R {
        DST_PIP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    pub fn src_dscr(&self) -> SRC_DSCR_R {
        SRC_DSCR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    pub fn dst_dscr(&self) -> DST_DSCR_R {
        DST_DSCR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Flow Control"]
    #[inline(always)]
    pub fn fc(&self) -> FC_R {
        FC_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    pub fn src_incr(&self) -> SRC_INCR_R {
        SRC_INCR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    pub fn dst_incr(&self) -> DST_INCR_R {
        DST_INCR_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ien(&self) -> IEN_R {
        IEN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Automatic Multiple Buffer Transfer"]
    #[inline(always)]
    pub fn auto(&self) -> AUTO_R {
        AUTO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Source Interface Selection Field"]
    #[inline(always)]
    pub fn sif(&mut self) -> SIF_W<0> {
        SIF_W::new(self)
    }
    #[doc = "Bits 4:5 - Destination Interface Selection Field"]
    #[inline(always)]
    pub fn dif(&mut self) -> DIF_W<4> {
        DIF_W::new(self)
    }
    #[doc = "Bit 8 - Source Picture-in-Picture Mode"]
    #[inline(always)]
    pub fn src_pip(&mut self) -> SRC_PIP_W<8> {
        SRC_PIP_W::new(self)
    }
    #[doc = "Bit 12 - Destination Picture-in-Picture Mode"]
    #[inline(always)]
    pub fn dst_pip(&mut self) -> DST_PIP_W<12> {
        DST_PIP_W::new(self)
    }
    #[doc = "Bit 16 - Source Address Descriptor"]
    #[inline(always)]
    pub fn src_dscr(&mut self) -> SRC_DSCR_W<16> {
        SRC_DSCR_W::new(self)
    }
    #[doc = "Bit 20 - Destination Address Descriptor"]
    #[inline(always)]
    pub fn dst_dscr(&mut self) -> DST_DSCR_W<20> {
        DST_DSCR_W::new(self)
    }
    #[doc = "Bits 21:23 - Flow Control"]
    #[inline(always)]
    pub fn fc(&mut self) -> FC_W<21> {
        FC_W::new(self)
    }
    #[doc = "Bits 24:25 - Incrementing, Decrementing or Fixed Address for the Source"]
    #[inline(always)]
    pub fn src_incr(&mut self) -> SRC_INCR_W<24> {
        SRC_INCR_W::new(self)
    }
    #[doc = "Bits 28:29 - Incrementing, Decrementing or Fixed Address for the Destination"]
    #[inline(always)]
    pub fn dst_incr(&mut self) -> DST_INCR_W<28> {
        DST_INCR_W::new(self)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn ien(&mut self) -> IEN_W<30> {
        IEN_W::new(self)
    }
    #[doc = "Bit 31 - Automatic Multiple Buffer Transfer"]
    #[inline(always)]
    pub fn auto(&mut self) -> AUTO_W<31> {
        AUTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAC Channel Control B Register (ch_num = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb7](index.html) module"]
pub struct CTRLB7_SPEC;
impl crate::RegisterSpec for CTRLB7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrlb7::R](R) reader structure"]
impl crate::Readable for CTRLB7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb7::W](W) writer structure"]
impl crate::Writable for CTRLB7_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRLB7 to value 0"]
impl crate::Resettable for CTRLB7_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
