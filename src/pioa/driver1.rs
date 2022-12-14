#[doc = "Register `DRIVER1` reader"]
pub struct R(crate::R<DRIVER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRIVER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRIVER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRIVER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRIVER1` writer"]
pub struct W(crate::W<DRIVER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRIVER1_SPEC>;
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
impl From<crate::W<DRIVER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRIVER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE0` reader - Drive of PIO Line 0"]
pub type LINE0_R = crate::FieldReader<u8, LINE0_A>;
#[doc = "Drive of PIO Line 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE0_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE0_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE0_A) -> Self {
        variant as _
    }
}
impl LINE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE0_A> {
        match self.bits {
            0 => Some(LINE0_A::HI_DRIVE),
            1 => Some(LINE0_A::ME_DRIVE),
            2 => Some(LINE0_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE0_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE0_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE0_A::LO_DRIVE
    }
}
#[doc = "Field `LINE0` writer - Drive of PIO Line 0"]
pub type LINE0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE0_A, 2, O>;
impl<'a, const O: u8> LINE0_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE0_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE0_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE0_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE1` reader - Drive of PIO Line 1"]
pub type LINE1_R = crate::FieldReader<u8, LINE1_A>;
#[doc = "Drive of PIO Line 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE1_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE1_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE1_A) -> Self {
        variant as _
    }
}
impl LINE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE1_A> {
        match self.bits {
            0 => Some(LINE1_A::HI_DRIVE),
            1 => Some(LINE1_A::ME_DRIVE),
            2 => Some(LINE1_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE1_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE1_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE1_A::LO_DRIVE
    }
}
#[doc = "Field `LINE1` writer - Drive of PIO Line 1"]
pub type LINE1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE1_A, 2, O>;
impl<'a, const O: u8> LINE1_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE1_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE1_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE1_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE2` reader - Drive of PIO Line 2"]
pub type LINE2_R = crate::FieldReader<u8, LINE2_A>;
#[doc = "Drive of PIO Line 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE2_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE2_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE2_A) -> Self {
        variant as _
    }
}
impl LINE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE2_A> {
        match self.bits {
            0 => Some(LINE2_A::HI_DRIVE),
            1 => Some(LINE2_A::ME_DRIVE),
            2 => Some(LINE2_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE2_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE2_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE2_A::LO_DRIVE
    }
}
#[doc = "Field `LINE2` writer - Drive of PIO Line 2"]
pub type LINE2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE2_A, 2, O>;
impl<'a, const O: u8> LINE2_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE2_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE2_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE2_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE3` reader - Drive of PIO Line 3"]
pub type LINE3_R = crate::FieldReader<u8, LINE3_A>;
#[doc = "Drive of PIO Line 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE3_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE3_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE3_A) -> Self {
        variant as _
    }
}
impl LINE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE3_A> {
        match self.bits {
            0 => Some(LINE3_A::HI_DRIVE),
            1 => Some(LINE3_A::ME_DRIVE),
            2 => Some(LINE3_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE3_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE3_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE3_A::LO_DRIVE
    }
}
#[doc = "Field `LINE3` writer - Drive of PIO Line 3"]
pub type LINE3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE3_A, 2, O>;
impl<'a, const O: u8> LINE3_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE3_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE3_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE3_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE4` reader - Drive of PIO Line 4"]
pub type LINE4_R = crate::FieldReader<u8, LINE4_A>;
#[doc = "Drive of PIO Line 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE4_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE4_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE4_A) -> Self {
        variant as _
    }
}
impl LINE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE4_A> {
        match self.bits {
            0 => Some(LINE4_A::HI_DRIVE),
            1 => Some(LINE4_A::ME_DRIVE),
            2 => Some(LINE4_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE4_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE4_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE4_A::LO_DRIVE
    }
}
#[doc = "Field `LINE4` writer - Drive of PIO Line 4"]
pub type LINE4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE4_A, 2, O>;
impl<'a, const O: u8> LINE4_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE4_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE4_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE4_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE5` reader - Drive of PIO Line 5"]
pub type LINE5_R = crate::FieldReader<u8, LINE5_A>;
#[doc = "Drive of PIO Line 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE5_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE5_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE5_A) -> Self {
        variant as _
    }
}
impl LINE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE5_A> {
        match self.bits {
            0 => Some(LINE5_A::HI_DRIVE),
            1 => Some(LINE5_A::ME_DRIVE),
            2 => Some(LINE5_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE5_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE5_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE5_A::LO_DRIVE
    }
}
#[doc = "Field `LINE5` writer - Drive of PIO Line 5"]
pub type LINE5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE5_A, 2, O>;
impl<'a, const O: u8> LINE5_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE5_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE5_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE5_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE6` reader - Drive of PIO Line 6"]
pub type LINE6_R = crate::FieldReader<u8, LINE6_A>;
#[doc = "Drive of PIO Line 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE6_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE6_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE6_A) -> Self {
        variant as _
    }
}
impl LINE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE6_A> {
        match self.bits {
            0 => Some(LINE6_A::HI_DRIVE),
            1 => Some(LINE6_A::ME_DRIVE),
            2 => Some(LINE6_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE6_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE6_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE6_A::LO_DRIVE
    }
}
#[doc = "Field `LINE6` writer - Drive of PIO Line 6"]
pub type LINE6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE6_A, 2, O>;
impl<'a, const O: u8> LINE6_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE6_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE6_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE6_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE7` reader - Drive of PIO Line 7"]
pub type LINE7_R = crate::FieldReader<u8, LINE7_A>;
#[doc = "Drive of PIO Line 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE7_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE7_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE7_A) -> Self {
        variant as _
    }
}
impl LINE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE7_A> {
        match self.bits {
            0 => Some(LINE7_A::HI_DRIVE),
            1 => Some(LINE7_A::ME_DRIVE),
            2 => Some(LINE7_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE7_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE7_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE7_A::LO_DRIVE
    }
}
#[doc = "Field `LINE7` writer - Drive of PIO Line 7"]
pub type LINE7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE7_A, 2, O>;
impl<'a, const O: u8> LINE7_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE7_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE7_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE7_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE8` reader - Drive of PIO Line 8"]
pub type LINE8_R = crate::FieldReader<u8, LINE8_A>;
#[doc = "Drive of PIO Line 8\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE8_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE8_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE8_A) -> Self {
        variant as _
    }
}
impl LINE8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE8_A> {
        match self.bits {
            0 => Some(LINE8_A::HI_DRIVE),
            1 => Some(LINE8_A::ME_DRIVE),
            2 => Some(LINE8_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE8_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE8_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE8_A::LO_DRIVE
    }
}
#[doc = "Field `LINE8` writer - Drive of PIO Line 8"]
pub type LINE8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE8_A, 2, O>;
impl<'a, const O: u8> LINE8_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE8_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE8_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE8_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE9` reader - Drive of PIO Line 9"]
pub type LINE9_R = crate::FieldReader<u8, LINE9_A>;
#[doc = "Drive of PIO Line 9\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE9_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE9_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE9_A) -> Self {
        variant as _
    }
}
impl LINE9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE9_A> {
        match self.bits {
            0 => Some(LINE9_A::HI_DRIVE),
            1 => Some(LINE9_A::ME_DRIVE),
            2 => Some(LINE9_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE9_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE9_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE9_A::LO_DRIVE
    }
}
#[doc = "Field `LINE9` writer - Drive of PIO Line 9"]
pub type LINE9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE9_A, 2, O>;
impl<'a, const O: u8> LINE9_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE9_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE9_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE9_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE10` reader - Drive of PIO Line 10"]
pub type LINE10_R = crate::FieldReader<u8, LINE10_A>;
#[doc = "Drive of PIO Line 10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE10_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE10_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE10_A) -> Self {
        variant as _
    }
}
impl LINE10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE10_A> {
        match self.bits {
            0 => Some(LINE10_A::HI_DRIVE),
            1 => Some(LINE10_A::ME_DRIVE),
            2 => Some(LINE10_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE10_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE10_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE10_A::LO_DRIVE
    }
}
#[doc = "Field `LINE10` writer - Drive of PIO Line 10"]
pub type LINE10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE10_A, 2, O>;
impl<'a, const O: u8> LINE10_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE10_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE10_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE10_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE11` reader - Drive of PIO Line 11"]
pub type LINE11_R = crate::FieldReader<u8, LINE11_A>;
#[doc = "Drive of PIO Line 11\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE11_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE11_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE11_A) -> Self {
        variant as _
    }
}
impl LINE11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE11_A> {
        match self.bits {
            0 => Some(LINE11_A::HI_DRIVE),
            1 => Some(LINE11_A::ME_DRIVE),
            2 => Some(LINE11_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE11_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE11_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE11_A::LO_DRIVE
    }
}
#[doc = "Field `LINE11` writer - Drive of PIO Line 11"]
pub type LINE11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE11_A, 2, O>;
impl<'a, const O: u8> LINE11_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE11_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE11_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE11_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE12` reader - Drive of PIO Line 12"]
pub type LINE12_R = crate::FieldReader<u8, LINE12_A>;
#[doc = "Drive of PIO Line 12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE12_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE12_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE12_A) -> Self {
        variant as _
    }
}
impl LINE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE12_A> {
        match self.bits {
            0 => Some(LINE12_A::HI_DRIVE),
            1 => Some(LINE12_A::ME_DRIVE),
            2 => Some(LINE12_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE12_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE12_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE12_A::LO_DRIVE
    }
}
#[doc = "Field `LINE12` writer - Drive of PIO Line 12"]
pub type LINE12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE12_A, 2, O>;
impl<'a, const O: u8> LINE12_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE12_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE12_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE12_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE13` reader - Drive of PIO Line 13"]
pub type LINE13_R = crate::FieldReader<u8, LINE13_A>;
#[doc = "Drive of PIO Line 13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE13_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE13_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE13_A) -> Self {
        variant as _
    }
}
impl LINE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE13_A> {
        match self.bits {
            0 => Some(LINE13_A::HI_DRIVE),
            1 => Some(LINE13_A::ME_DRIVE),
            2 => Some(LINE13_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE13_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE13_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE13_A::LO_DRIVE
    }
}
#[doc = "Field `LINE13` writer - Drive of PIO Line 13"]
pub type LINE13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE13_A, 2, O>;
impl<'a, const O: u8> LINE13_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE13_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE13_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE13_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE14` reader - Drive of PIO Line 14"]
pub type LINE14_R = crate::FieldReader<u8, LINE14_A>;
#[doc = "Drive of PIO Line 14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE14_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE14_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE14_A) -> Self {
        variant as _
    }
}
impl LINE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE14_A> {
        match self.bits {
            0 => Some(LINE14_A::HI_DRIVE),
            1 => Some(LINE14_A::ME_DRIVE),
            2 => Some(LINE14_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE14_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE14_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE14_A::LO_DRIVE
    }
}
#[doc = "Field `LINE14` writer - Drive of PIO Line 14"]
pub type LINE14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE14_A, 2, O>;
impl<'a, const O: u8> LINE14_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE14_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE14_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE14_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE15` reader - Drive of PIO Line 15"]
pub type LINE15_R = crate::FieldReader<u8, LINE15_A>;
#[doc = "Drive of PIO Line 15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE15_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE15_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE15_A) -> Self {
        variant as _
    }
}
impl LINE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE15_A> {
        match self.bits {
            0 => Some(LINE15_A::HI_DRIVE),
            1 => Some(LINE15_A::ME_DRIVE),
            2 => Some(LINE15_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE15_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE15_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE15_A::LO_DRIVE
    }
}
#[doc = "Field `LINE15` writer - Drive of PIO Line 15"]
pub type LINE15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER1_SPEC, u8, LINE15_A, 2, O>;
impl<'a, const O: u8> LINE15_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE15_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE15_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE15_A::LO_DRIVE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&self) -> LINE0_R {
        LINE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&self) -> LINE1_R {
        LINE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&self) -> LINE2_R {
        LINE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&self) -> LINE3_R {
        LINE3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&self) -> LINE4_R {
        LINE4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&self) -> LINE5_R {
        LINE5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&self) -> LINE6_R {
        LINE6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&self) -> LINE7_R {
        LINE7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&self) -> LINE8_R {
        LINE8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&self) -> LINE9_R {
        LINE9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&self) -> LINE10_R {
        LINE10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&self) -> LINE11_R {
        LINE11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&self) -> LINE12_R {
        LINE12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&self) -> LINE13_R {
        LINE13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&self) -> LINE14_R {
        LINE14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&self) -> LINE15_R {
        LINE15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Drive of PIO Line 0"]
    #[inline(always)]
    pub fn line0(&mut self) -> LINE0_W<0> {
        LINE0_W::new(self)
    }
    #[doc = "Bits 2:3 - Drive of PIO Line 1"]
    #[inline(always)]
    pub fn line1(&mut self) -> LINE1_W<2> {
        LINE1_W::new(self)
    }
    #[doc = "Bits 4:5 - Drive of PIO Line 2"]
    #[inline(always)]
    pub fn line2(&mut self) -> LINE2_W<4> {
        LINE2_W::new(self)
    }
    #[doc = "Bits 6:7 - Drive of PIO Line 3"]
    #[inline(always)]
    pub fn line3(&mut self) -> LINE3_W<6> {
        LINE3_W::new(self)
    }
    #[doc = "Bits 8:9 - Drive of PIO Line 4"]
    #[inline(always)]
    pub fn line4(&mut self) -> LINE4_W<8> {
        LINE4_W::new(self)
    }
    #[doc = "Bits 10:11 - Drive of PIO Line 5"]
    #[inline(always)]
    pub fn line5(&mut self) -> LINE5_W<10> {
        LINE5_W::new(self)
    }
    #[doc = "Bits 12:13 - Drive of PIO Line 6"]
    #[inline(always)]
    pub fn line6(&mut self) -> LINE6_W<12> {
        LINE6_W::new(self)
    }
    #[doc = "Bits 14:15 - Drive of PIO Line 7"]
    #[inline(always)]
    pub fn line7(&mut self) -> LINE7_W<14> {
        LINE7_W::new(self)
    }
    #[doc = "Bits 16:17 - Drive of PIO Line 8"]
    #[inline(always)]
    pub fn line8(&mut self) -> LINE8_W<16> {
        LINE8_W::new(self)
    }
    #[doc = "Bits 18:19 - Drive of PIO Line 9"]
    #[inline(always)]
    pub fn line9(&mut self) -> LINE9_W<18> {
        LINE9_W::new(self)
    }
    #[doc = "Bits 20:21 - Drive of PIO Line 10"]
    #[inline(always)]
    pub fn line10(&mut self) -> LINE10_W<20> {
        LINE10_W::new(self)
    }
    #[doc = "Bits 22:23 - Drive of PIO Line 11"]
    #[inline(always)]
    pub fn line11(&mut self) -> LINE11_W<22> {
        LINE11_W::new(self)
    }
    #[doc = "Bits 24:25 - Drive of PIO Line 12"]
    #[inline(always)]
    pub fn line12(&mut self) -> LINE12_W<24> {
        LINE12_W::new(self)
    }
    #[doc = "Bits 26:27 - Drive of PIO Line 13"]
    #[inline(always)]
    pub fn line13(&mut self) -> LINE13_W<26> {
        LINE13_W::new(self)
    }
    #[doc = "Bits 28:29 - Drive of PIO Line 14"]
    #[inline(always)]
    pub fn line14(&mut self) -> LINE14_W<28> {
        LINE14_W::new(self)
    }
    #[doc = "Bits 30:31 - Drive of PIO Line 15"]
    #[inline(always)]
    pub fn line15(&mut self) -> LINE15_W<30> {
        LINE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Drive Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [driver1](index.html) module"]
pub struct DRIVER1_SPEC;
impl crate::RegisterSpec for DRIVER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [driver1::R](R) reader structure"]
impl crate::Readable for DRIVER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [driver1::W](W) writer structure"]
impl crate::Writable for DRIVER1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRIVER1 to value 0"]
impl crate::Resettable for DRIVER1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
