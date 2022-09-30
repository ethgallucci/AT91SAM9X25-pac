# [doc = "Register `DPIP7` reader"] pub struct R (crate :: R < DPIP7_SPEC >) ; impl core :: ops :: Deref for R { type Target = crate :: R < DPIP7_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl From < crate :: R < DPIP7_SPEC > > for R { # [inline (always)] fn from (reader : crate :: R < DPIP7_SPEC >) -> Self { R (reader) } } # [doc = "Register `DPIP7` writer"] pub struct W (crate :: W < DPIP7_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < DPIP7_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < DPIP7_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < DPIP7_SPEC >) -> Self { W (writer) } } # [doc = "Field `DPIP_HOLE` reader - Destination Picture-in-Picture Hole"] pub type DPIP_HOLE_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `DPIP_HOLE` writer - Destination Picture-in-Picture Hole"] pub type DPIP_HOLE_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , DPIP7_SPEC , u16 , u16 , 16 , O > ; # [doc = "Field `DPIP_BOUNDARY` reader - Destination Picture-in-Picture Boundary"] pub type DPIP_BOUNDARY_R = crate :: FieldReader < u16 , u16 > ; # [doc = "Field `DPIP_BOUNDARY` writer - Destination Picture-in-Picture Boundary"] pub type DPIP_BOUNDARY_W < 'a , const O : u8 > = crate :: FieldWriter < 'a , u32 , DPIP7_SPEC , u16 , u16 , 10 , O > ; impl R { # [doc = "Bits 0:15 - Destination Picture-in-Picture Hole"] # [inline (always)] pub fn dpip_hole (& self) -> DPIP_HOLE_R { DPIP_HOLE_R :: new ((self . bits & 0xffff) as u16) } # [doc = "Bits 16:25 - Destination Picture-in-Picture Boundary"] # [inline (always)] pub fn dpip_boundary (& self) -> DPIP_BOUNDARY_R { DPIP_BOUNDARY_R :: new (((self . bits >> 16) & 0x03ff) as u16) } } impl W { # [doc = "Bits 0:15 - Destination Picture-in-Picture Hole"] # [inline (always)] pub fn dpip_hole (& mut self) -> DPIP_HOLE_W < 0 > { DPIP_HOLE_W :: new (self) } # [doc = "Bits 16:25 - Destination Picture-in-Picture Boundary"] # [inline (always)] pub fn dpip_boundary (& mut self) -> DPIP_BOUNDARY_W < 16 > { DPIP_BOUNDARY_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "DMAC Channel Destination Picture-in-Picture Configuration Register (ch_num = 7)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpip7](index.html) module"] pub struct DPIP7_SPEC ; impl crate :: RegisterSpec for DPIP7_SPEC { type Ux = u32 ; } # [doc = "`read()` method returns [dpip7::R](R) reader structure"] impl crate :: Readable for DPIP7_SPEC { type Reader = R ; } # [doc = "`write(|w| ..)` method takes [dpip7::W](W) writer structure"] impl crate :: Writable for DPIP7_SPEC { type Writer = W ; } # [doc = "`reset()` method sets DPIP7 to value 0"] impl crate :: Resettable for DPIP7_SPEC { # [inline (always)] fn reset_value () -> Self :: Ux { 0 } }