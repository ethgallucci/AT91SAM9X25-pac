#[doc = "Register `DLL` reader"]
pub struct R(crate::R<DLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MDINC` reader - DLL Master Delay Increment"]
pub type MDINC_R = crate::BitReader<bool>;
#[doc = "Field `MDDEC` reader - DLL Master Delay Decrement"]
pub type MDDEC_R = crate::BitReader<bool>;
#[doc = "Field `MDOVF` reader - DLL Master Delay Overflow Flag"]
pub type MDOVF_R = crate::BitReader<bool>;
#[doc = "Field `MDVAL` reader - DLL Master Delay Value"]
pub type MDVAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - DLL Master Delay Increment"]
    #[inline(always)]
    pub fn mdinc(&self) -> MDINC_R {
        MDINC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL Master Delay Decrement"]
    #[inline(always)]
    pub fn mddec(&self) -> MDDEC_R {
        MDDEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DLL Master Delay Overflow Flag"]
    #[inline(always)]
    pub fn mdovf(&self) -> MDOVF_R {
        MDOVF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - DLL Master Delay Value"]
    #[inline(always)]
    pub fn mdval(&self) -> MDVAL_R {
        MDVAL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "DDRSDRC DLL Information Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dll](index.html) module"]
pub struct DLL_SPEC;
impl crate::RegisterSpec for DLL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dll::R](R) reader structure"]
impl crate::Readable for DLL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DLL to value 0x01"]
impl crate::Resettable for DLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
