#[doc = "Register `DRIVER2` reader"]
pub struct R(crate::R<DRIVER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DRIVER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DRIVER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DRIVER2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DRIVER2` writer"]
pub struct W(crate::W<DRIVER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DRIVER2_SPEC>;
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
impl From<crate::W<DRIVER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DRIVER2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINE16` reader - Drive of PIO line 16"]
pub type LINE16_R = crate::FieldReader<u8, LINE16_A>;
#[doc = "Drive of PIO line 16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE16_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE16_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE16_A) -> Self {
        variant as _
    }
}
impl LINE16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE16_A> {
        match self.bits {
            0 => Some(LINE16_A::HI_DRIVE),
            1 => Some(LINE16_A::ME_DRIVE),
            2 => Some(LINE16_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE16_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE16_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE16_A::LO_DRIVE
    }
}
#[doc = "Field `LINE16` writer - Drive of PIO line 16"]
pub type LINE16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE16_A, 2, O>;
impl<'a, const O: u8> LINE16_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE16_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE16_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE16_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE17` reader - Drive of PIO line 17"]
pub type LINE17_R = crate::FieldReader<u8, LINE17_A>;
#[doc = "Drive of PIO line 17\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE17_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE17_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE17_A) -> Self {
        variant as _
    }
}
impl LINE17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE17_A> {
        match self.bits {
            0 => Some(LINE17_A::HI_DRIVE),
            1 => Some(LINE17_A::ME_DRIVE),
            2 => Some(LINE17_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE17_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE17_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE17_A::LO_DRIVE
    }
}
#[doc = "Field `LINE17` writer - Drive of PIO line 17"]
pub type LINE17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE17_A, 2, O>;
impl<'a, const O: u8> LINE17_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE17_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE17_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE17_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE18` reader - Drive of PIO line 18"]
pub type LINE18_R = crate::FieldReader<u8, LINE18_A>;
#[doc = "Drive of PIO line 18\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE18_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE18_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE18_A) -> Self {
        variant as _
    }
}
impl LINE18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE18_A> {
        match self.bits {
            0 => Some(LINE18_A::HI_DRIVE),
            1 => Some(LINE18_A::ME_DRIVE),
            2 => Some(LINE18_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE18_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE18_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE18_A::LO_DRIVE
    }
}
#[doc = "Field `LINE18` writer - Drive of PIO line 18"]
pub type LINE18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE18_A, 2, O>;
impl<'a, const O: u8> LINE18_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE18_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE18_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE18_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE19` reader - Drive of PIO line 19"]
pub type LINE19_R = crate::FieldReader<u8, LINE19_A>;
#[doc = "Drive of PIO line 19\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE19_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE19_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE19_A) -> Self {
        variant as _
    }
}
impl LINE19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE19_A> {
        match self.bits {
            0 => Some(LINE19_A::HI_DRIVE),
            1 => Some(LINE19_A::ME_DRIVE),
            2 => Some(LINE19_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE19_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE19_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE19_A::LO_DRIVE
    }
}
#[doc = "Field `LINE19` writer - Drive of PIO line 19"]
pub type LINE19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE19_A, 2, O>;
impl<'a, const O: u8> LINE19_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE19_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE19_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE19_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE20` reader - Drive of PIO line 20"]
pub type LINE20_R = crate::FieldReader<u8, LINE20_A>;
#[doc = "Drive of PIO line 20\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE20_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE20_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE20_A) -> Self {
        variant as _
    }
}
impl LINE20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE20_A> {
        match self.bits {
            0 => Some(LINE20_A::HI_DRIVE),
            1 => Some(LINE20_A::ME_DRIVE),
            2 => Some(LINE20_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE20_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE20_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE20_A::LO_DRIVE
    }
}
#[doc = "Field `LINE20` writer - Drive of PIO line 20"]
pub type LINE20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE20_A, 2, O>;
impl<'a, const O: u8> LINE20_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE20_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE20_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE20_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE21` reader - Drive of PIO line 21"]
pub type LINE21_R = crate::FieldReader<u8, LINE21_A>;
#[doc = "Drive of PIO line 21\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE21_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE21_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE21_A) -> Self {
        variant as _
    }
}
impl LINE21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE21_A> {
        match self.bits {
            0 => Some(LINE21_A::HI_DRIVE),
            1 => Some(LINE21_A::ME_DRIVE),
            2 => Some(LINE21_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE21_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE21_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE21_A::LO_DRIVE
    }
}
#[doc = "Field `LINE21` writer - Drive of PIO line 21"]
pub type LINE21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE21_A, 2, O>;
impl<'a, const O: u8> LINE21_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE21_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE21_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE21_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE22` reader - Drive of PIO line 22"]
pub type LINE22_R = crate::FieldReader<u8, LINE22_A>;
#[doc = "Drive of PIO line 22\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE22_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE22_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE22_A) -> Self {
        variant as _
    }
}
impl LINE22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE22_A> {
        match self.bits {
            0 => Some(LINE22_A::HI_DRIVE),
            1 => Some(LINE22_A::ME_DRIVE),
            2 => Some(LINE22_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE22_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE22_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE22_A::LO_DRIVE
    }
}
#[doc = "Field `LINE22` writer - Drive of PIO line 22"]
pub type LINE22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE22_A, 2, O>;
impl<'a, const O: u8> LINE22_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE22_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE22_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE22_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE23` reader - Drive of PIO line 23"]
pub type LINE23_R = crate::FieldReader<u8, LINE23_A>;
#[doc = "Drive of PIO line 23\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE23_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE23_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE23_A) -> Self {
        variant as _
    }
}
impl LINE23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE23_A> {
        match self.bits {
            0 => Some(LINE23_A::HI_DRIVE),
            1 => Some(LINE23_A::ME_DRIVE),
            2 => Some(LINE23_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE23_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE23_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE23_A::LO_DRIVE
    }
}
#[doc = "Field `LINE23` writer - Drive of PIO line 23"]
pub type LINE23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE23_A, 2, O>;
impl<'a, const O: u8> LINE23_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE23_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE23_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE23_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE24` reader - Drive of PIO line 24"]
pub type LINE24_R = crate::FieldReader<u8, LINE24_A>;
#[doc = "Drive of PIO line 24\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE24_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE24_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE24_A) -> Self {
        variant as _
    }
}
impl LINE24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE24_A> {
        match self.bits {
            0 => Some(LINE24_A::HI_DRIVE),
            1 => Some(LINE24_A::ME_DRIVE),
            2 => Some(LINE24_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE24_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE24_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE24_A::LO_DRIVE
    }
}
#[doc = "Field `LINE24` writer - Drive of PIO line 24"]
pub type LINE24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE24_A, 2, O>;
impl<'a, const O: u8> LINE24_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE24_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE24_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE24_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE25` reader - Drive of PIO line 25"]
pub type LINE25_R = crate::FieldReader<u8, LINE25_A>;
#[doc = "Drive of PIO line 25\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE25_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE25_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE25_A) -> Self {
        variant as _
    }
}
impl LINE25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE25_A> {
        match self.bits {
            0 => Some(LINE25_A::HI_DRIVE),
            1 => Some(LINE25_A::ME_DRIVE),
            2 => Some(LINE25_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE25_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE25_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE25_A::LO_DRIVE
    }
}
#[doc = "Field `LINE25` writer - Drive of PIO line 25"]
pub type LINE25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE25_A, 2, O>;
impl<'a, const O: u8> LINE25_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE25_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE25_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE25_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE26` reader - Drive of PIO line 26"]
pub type LINE26_R = crate::FieldReader<u8, LINE26_A>;
#[doc = "Drive of PIO line 26\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE26_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE26_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE26_A) -> Self {
        variant as _
    }
}
impl LINE26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE26_A> {
        match self.bits {
            0 => Some(LINE26_A::HI_DRIVE),
            1 => Some(LINE26_A::ME_DRIVE),
            2 => Some(LINE26_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE26_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE26_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE26_A::LO_DRIVE
    }
}
#[doc = "Field `LINE26` writer - Drive of PIO line 26"]
pub type LINE26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE26_A, 2, O>;
impl<'a, const O: u8> LINE26_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE26_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE26_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE26_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE27` reader - Drive of PIO line 27"]
pub type LINE27_R = crate::FieldReader<u8, LINE27_A>;
#[doc = "Drive of PIO line 27\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE27_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE27_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE27_A) -> Self {
        variant as _
    }
}
impl LINE27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE27_A> {
        match self.bits {
            0 => Some(LINE27_A::HI_DRIVE),
            1 => Some(LINE27_A::ME_DRIVE),
            2 => Some(LINE27_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE27_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE27_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE27_A::LO_DRIVE
    }
}
#[doc = "Field `LINE27` writer - Drive of PIO line 27"]
pub type LINE27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE27_A, 2, O>;
impl<'a, const O: u8> LINE27_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE27_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE27_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE27_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE28` reader - Drive of PIO line 28"]
pub type LINE28_R = crate::FieldReader<u8, LINE28_A>;
#[doc = "Drive of PIO line 28\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE28_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE28_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE28_A) -> Self {
        variant as _
    }
}
impl LINE28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE28_A> {
        match self.bits {
            0 => Some(LINE28_A::HI_DRIVE),
            1 => Some(LINE28_A::ME_DRIVE),
            2 => Some(LINE28_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE28_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE28_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE28_A::LO_DRIVE
    }
}
#[doc = "Field `LINE28` writer - Drive of PIO line 28"]
pub type LINE28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE28_A, 2, O>;
impl<'a, const O: u8> LINE28_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE28_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE28_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE28_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE29` reader - Drive of PIO line 29"]
pub type LINE29_R = crate::FieldReader<u8, LINE29_A>;
#[doc = "Drive of PIO line 29\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE29_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE29_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE29_A) -> Self {
        variant as _
    }
}
impl LINE29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE29_A> {
        match self.bits {
            0 => Some(LINE29_A::HI_DRIVE),
            1 => Some(LINE29_A::ME_DRIVE),
            2 => Some(LINE29_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE29_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE29_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE29_A::LO_DRIVE
    }
}
#[doc = "Field `LINE29` writer - Drive of PIO line 29"]
pub type LINE29_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE29_A, 2, O>;
impl<'a, const O: u8> LINE29_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE29_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE29_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE29_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE30` reader - Drive of PIO line 30"]
pub type LINE30_R = crate::FieldReader<u8, LINE30_A>;
#[doc = "Drive of PIO line 30\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE30_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE30_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE30_A) -> Self {
        variant as _
    }
}
impl LINE30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE30_A> {
        match self.bits {
            0 => Some(LINE30_A::HI_DRIVE),
            1 => Some(LINE30_A::ME_DRIVE),
            2 => Some(LINE30_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE30_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE30_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE30_A::LO_DRIVE
    }
}
#[doc = "Field `LINE30` writer - Drive of PIO line 30"]
pub type LINE30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE30_A, 2, O>;
impl<'a, const O: u8> LINE30_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE30_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE30_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE30_A::LO_DRIVE)
    }
}
#[doc = "Field `LINE31` reader - Drive of PIO line 31"]
pub type LINE31_R = crate::FieldReader<u8, LINE31_A>;
#[doc = "Drive of PIO line 31\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LINE31_A {
    #[doc = "0: High drive"]
    HI_DRIVE = 0,
    #[doc = "1: Medium drive"]
    ME_DRIVE = 1,
    #[doc = "2: Low drive"]
    LO_DRIVE = 2,
}
impl From<LINE31_A> for u8 {
    #[inline(always)]
    fn from(variant: LINE31_A) -> Self {
        variant as _
    }
}
impl LINE31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LINE31_A> {
        match self.bits {
            0 => Some(LINE31_A::HI_DRIVE),
            1 => Some(LINE31_A::ME_DRIVE),
            2 => Some(LINE31_A::LO_DRIVE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HI_DRIVE`"]
    #[inline(always)]
    pub fn is_hi_drive(&self) -> bool {
        *self == LINE31_A::HI_DRIVE
    }
    #[doc = "Checks if the value of the field is `ME_DRIVE`"]
    #[inline(always)]
    pub fn is_me_drive(&self) -> bool {
        *self == LINE31_A::ME_DRIVE
    }
    #[doc = "Checks if the value of the field is `LO_DRIVE`"]
    #[inline(always)]
    pub fn is_lo_drive(&self) -> bool {
        *self == LINE31_A::LO_DRIVE
    }
}
#[doc = "Field `LINE31` writer - Drive of PIO line 31"]
pub type LINE31_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DRIVER2_SPEC, u8, LINE31_A, 2, O>;
impl<'a, const O: u8> LINE31_W<'a, O> {
    #[doc = "High drive"]
    #[inline(always)]
    pub fn hi_drive(self) -> &'a mut W {
        self.variant(LINE31_A::HI_DRIVE)
    }
    #[doc = "Medium drive"]
    #[inline(always)]
    pub fn me_drive(self) -> &'a mut W {
        self.variant(LINE31_A::ME_DRIVE)
    }
    #[doc = "Low drive"]
    #[inline(always)]
    pub fn lo_drive(self) -> &'a mut W {
        self.variant(LINE31_A::LO_DRIVE)
    }
}
impl R {
    #[doc = "Bits 0:1 - Drive of PIO line 16"]
    #[inline(always)]
    pub fn line16(&self) -> LINE16_R {
        LINE16_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Drive of PIO line 17"]
    #[inline(always)]
    pub fn line17(&self) -> LINE17_R {
        LINE17_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Drive of PIO line 18"]
    #[inline(always)]
    pub fn line18(&self) -> LINE18_R {
        LINE18_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Drive of PIO line 19"]
    #[inline(always)]
    pub fn line19(&self) -> LINE19_R {
        LINE19_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Drive of PIO line 20"]
    #[inline(always)]
    pub fn line20(&self) -> LINE20_R {
        LINE20_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Drive of PIO line 21"]
    #[inline(always)]
    pub fn line21(&self) -> LINE21_R {
        LINE21_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Drive of PIO line 22"]
    #[inline(always)]
    pub fn line22(&self) -> LINE22_R {
        LINE22_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Drive of PIO line 23"]
    #[inline(always)]
    pub fn line23(&self) -> LINE23_R {
        LINE23_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Drive of PIO line 24"]
    #[inline(always)]
    pub fn line24(&self) -> LINE24_R {
        LINE24_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Drive of PIO line 25"]
    #[inline(always)]
    pub fn line25(&self) -> LINE25_R {
        LINE25_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Drive of PIO line 26"]
    #[inline(always)]
    pub fn line26(&self) -> LINE26_R {
        LINE26_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Drive of PIO line 27"]
    #[inline(always)]
    pub fn line27(&self) -> LINE27_R {
        LINE27_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Drive of PIO line 28"]
    #[inline(always)]
    pub fn line28(&self) -> LINE28_R {
        LINE28_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Drive of PIO line 29"]
    #[inline(always)]
    pub fn line29(&self) -> LINE29_R {
        LINE29_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Drive of PIO line 30"]
    #[inline(always)]
    pub fn line30(&self) -> LINE30_R {
        LINE30_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Drive of PIO line 31"]
    #[inline(always)]
    pub fn line31(&self) -> LINE31_R {
        LINE31_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Drive of PIO line 16"]
    #[inline(always)]
    pub fn line16(&mut self) -> LINE16_W<0> {
        LINE16_W::new(self)
    }
    #[doc = "Bits 2:3 - Drive of PIO line 17"]
    #[inline(always)]
    pub fn line17(&mut self) -> LINE17_W<2> {
        LINE17_W::new(self)
    }
    #[doc = "Bits 4:5 - Drive of PIO line 18"]
    #[inline(always)]
    pub fn line18(&mut self) -> LINE18_W<4> {
        LINE18_W::new(self)
    }
    #[doc = "Bits 6:7 - Drive of PIO line 19"]
    #[inline(always)]
    pub fn line19(&mut self) -> LINE19_W<6> {
        LINE19_W::new(self)
    }
    #[doc = "Bits 8:9 - Drive of PIO line 20"]
    #[inline(always)]
    pub fn line20(&mut self) -> LINE20_W<8> {
        LINE20_W::new(self)
    }
    #[doc = "Bits 10:11 - Drive of PIO line 21"]
    #[inline(always)]
    pub fn line21(&mut self) -> LINE21_W<10> {
        LINE21_W::new(self)
    }
    #[doc = "Bits 12:13 - Drive of PIO line 22"]
    #[inline(always)]
    pub fn line22(&mut self) -> LINE22_W<12> {
        LINE22_W::new(self)
    }
    #[doc = "Bits 14:15 - Drive of PIO line 23"]
    #[inline(always)]
    pub fn line23(&mut self) -> LINE23_W<14> {
        LINE23_W::new(self)
    }
    #[doc = "Bits 16:17 - Drive of PIO line 24"]
    #[inline(always)]
    pub fn line24(&mut self) -> LINE24_W<16> {
        LINE24_W::new(self)
    }
    #[doc = "Bits 18:19 - Drive of PIO line 25"]
    #[inline(always)]
    pub fn line25(&mut self) -> LINE25_W<18> {
        LINE25_W::new(self)
    }
    #[doc = "Bits 20:21 - Drive of PIO line 26"]
    #[inline(always)]
    pub fn line26(&mut self) -> LINE26_W<20> {
        LINE26_W::new(self)
    }
    #[doc = "Bits 22:23 - Drive of PIO line 27"]
    #[inline(always)]
    pub fn line27(&mut self) -> LINE27_W<22> {
        LINE27_W::new(self)
    }
    #[doc = "Bits 24:25 - Drive of PIO line 28"]
    #[inline(always)]
    pub fn line28(&mut self) -> LINE28_W<24> {
        LINE28_W::new(self)
    }
    #[doc = "Bits 26:27 - Drive of PIO line 29"]
    #[inline(always)]
    pub fn line29(&mut self) -> LINE29_W<26> {
        LINE29_W::new(self)
    }
    #[doc = "Bits 28:29 - Drive of PIO line 30"]
    #[inline(always)]
    pub fn line30(&mut self) -> LINE30_W<28> {
        LINE30_W::new(self)
    }
    #[doc = "Bits 30:31 - Drive of PIO line 31"]
    #[inline(always)]
    pub fn line31(&mut self) -> LINE31_W<30> {
        LINE31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Drive Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [driver2](index.html) module"]
pub struct DRIVER2_SPEC;
impl crate::RegisterSpec for DRIVER2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [driver2::R](R) reader structure"]
impl crate::Readable for DRIVER2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [driver2::W](W) writer structure"]
impl crate::Writable for DRIVER2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DRIVER2 to value 0"]
impl crate::Resettable for DRIVER2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
