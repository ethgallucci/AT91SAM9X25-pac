# [doc = "Register `ECC7_5` reader"] pub struct R (crate :: R < ECC7_5_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < ECC7_5_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < ECC7_5_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < ECC7_5_SPEC >) -> Self { R (reader) } } # [doc = "Field `ECC` reader - BCH Redundancy"] pub type ECC_R = crate :: FieldReader < u32 , u32 > ; impl R { # [doc = "Bits 0:31 - BCH Redundancy"] # [inline (always)] pub fn ecc (& self) -> ECC_R { ECC_R :: new (self . bits) } } # [doc = "PMECC ECC 7 Register (sec_num = 5)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecc7_5](index.html) module"] pub struct ECC7_5_SPEC ; impl crate :: RegisterSpec for ECC7_5_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [ecc7_5::R](R) reader structure"] impl crate :: Readable for ECC7_5_SPEC { type Reader = R ; } # [doc = "`reset()` method sets ECC7_5 to value 0"] impl crate :: Resettable for ECC7_5_SPEC { # [inline (always)] fn reset_value () -> Self :: Ux { 0 } }