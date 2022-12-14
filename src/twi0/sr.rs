#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCOMP` reader - Transmission Completed (automatically set / reset)"]
pub type TXCOMP_R = crate::BitReader<bool>;
#[doc = "Field `RXRDY` reader - Receive Holding Register Ready (automatically set / reset)"]
pub type RXRDY_R = crate::BitReader<bool>;
#[doc = "Field `TXRDY` reader - Transmit Holding Register Ready (automatically set / reset)"]
pub type TXRDY_R = crate::BitReader<bool>;
#[doc = "Field `SVREAD` reader - Slave Read (automatically set / reset)"]
pub type SVREAD_R = crate::BitReader<bool>;
#[doc = "Field `SVACC` reader - Slave Access (automatically set / reset)"]
pub type SVACC_R = crate::BitReader<bool>;
#[doc = "Field `GACC` reader - General Call Access (clear on read)"]
pub type GACC_R = crate::BitReader<bool>;
#[doc = "Field `OVRE` reader - Overrun Error (clear on read)"]
pub type OVRE_R = crate::BitReader<bool>;
#[doc = "Field `NACK` reader - Not Acknowledged (clear on read)"]
pub type NACK_R = crate::BitReader<bool>;
#[doc = "Field `ARBLST` reader - Arbitration Lost (clear on read)"]
pub type ARBLST_R = crate::BitReader<bool>;
#[doc = "Field `SCLWS` reader - Clock Wait State (automatically set / reset)"]
pub type SCLWS_R = crate::BitReader<bool>;
#[doc = "Field `EOSACC` reader - End Of Slave Access (clear on read)"]
pub type EOSACC_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Transmission Completed (automatically set / reset)"]
    #[inline(always)]
    pub fn txcomp(&self) -> TXCOMP_R {
        TXCOMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Holding Register Ready (automatically set / reset)"]
    #[inline(always)]
    pub fn rxrdy(&self) -> RXRDY_R {
        RXRDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Holding Register Ready (automatically set / reset)"]
    #[inline(always)]
    pub fn txrdy(&self) -> TXRDY_R {
        TXRDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Slave Read (automatically set / reset)"]
    #[inline(always)]
    pub fn svread(&self) -> SVREAD_R {
        SVREAD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Access (automatically set / reset)"]
    #[inline(always)]
    pub fn svacc(&self) -> SVACC_R {
        SVACC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - General Call Access (clear on read)"]
    #[inline(always)]
    pub fn gacc(&self) -> GACC_R {
        GACC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun Error (clear on read)"]
    #[inline(always)]
    pub fn ovre(&self) -> OVRE_R {
        OVRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Not Acknowledged (clear on read)"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration Lost (clear on read)"]
    #[inline(always)]
    pub fn arblst(&self) -> ARBLST_R {
        ARBLST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock Wait State (automatically set / reset)"]
    #[inline(always)]
    pub fn sclws(&self) -> SCLWS_R {
        SCLWS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - End Of Slave Access (clear on read)"]
    #[inline(always)]
    pub fn eosacc(&self) -> EOSACC_R {
        EOSACC_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0xf009"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf009
    }
}
