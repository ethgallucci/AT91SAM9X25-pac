# [doc = "Register `CR_SPI_MODE` writer"] pub struct W (crate :: W < SPI_MODE_CR_SPI_MODE_SPEC >) ; impl core :: ops :: Deref for W { type Target = crate :: W < SPI_MODE_CR_SPI_MODE_SPEC > ; # [inline (always)] fn deref (& self) -> & Self :: Target { & self . 0 } } impl core :: ops :: DerefMut for W { # [inline (always)] fn deref_mut (& mut self) -> & mut Self :: Target { & mut self . 0 } } impl From < crate :: W < SPI_MODE_CR_SPI_MODE_SPEC > > for W { # [inline (always)] fn from (writer : crate :: W < SPI_MODE_CR_SPI_MODE_SPEC >) -> Self { W (writer) } } # [doc = "Field `RSTRX` writer - Reset Receiver"] pub type RSTRX_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SPI_MODE_CR_SPI_MODE_SPEC , bool , O > ; # [doc = "Field `RSTTX` writer - Reset Transmitter"] pub type RSTTX_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SPI_MODE_CR_SPI_MODE_SPEC , bool , O > ; # [doc = "Field `RXEN` writer - Receiver Enable"] pub type RXEN_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SPI_MODE_CR_SPI_MODE_SPEC , bool , O > ; # [doc = "Field `RXDIS` writer - Receiver Disable"] pub type RXDIS_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SPI_MODE_CR_SPI_MODE_SPEC , bool , O > ; # [doc = "Field `TXEN` writer - Transmitter Enable"] pub type TXEN_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SPI_MODE_CR_SPI_MODE_SPEC , bool , O > ; # [doc = "Field `TXDIS` writer - Transmitter Disable"] pub type TXDIS_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SPI_MODE_CR_SPI_MODE_SPEC , bool , O > ; # [doc = "Field `RSTSTA` writer - Reset Status Bits"] pub type RSTSTA_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SPI_MODE_CR_SPI_MODE_SPEC , bool , O > ; # [doc = "Field `FCS` writer - Force SPI Chip Select"] pub type FCS_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SPI_MODE_CR_SPI_MODE_SPEC , bool , O > ; # [doc = "Field `RCS` writer - Release SPI Chip Select"] pub type RCS_W < 'a , const O : u8 > = crate :: BitWriter < 'a , u32 , SPI_MODE_CR_SPI_MODE_SPEC , bool , O > ; impl W { # [doc = "Bit 2 - Reset Receiver"] # [inline (always)] pub fn rstrx (& mut self) -> RSTRX_W < 2 > { RSTRX_W :: new (self) } # [doc = "Bit 3 - Reset Transmitter"] # [inline (always)] pub fn rsttx (& mut self) -> RSTTX_W < 3 > { RSTTX_W :: new (self) } # [doc = "Bit 4 - Receiver Enable"] # [inline (always)] pub fn rxen (& mut self) -> RXEN_W < 4 > { RXEN_W :: new (self) } # [doc = "Bit 5 - Receiver Disable"] # [inline (always)] pub fn rxdis (& mut self) -> RXDIS_W < 5 > { RXDIS_W :: new (self) } # [doc = "Bit 6 - Transmitter Enable"] # [inline (always)] pub fn txen (& mut self) -> TXEN_W < 6 > { TXEN_W :: new (self) } # [doc = "Bit 7 - Transmitter Disable"] # [inline (always)] pub fn txdis (& mut self) -> TXDIS_W < 7 > { TXDIS_W :: new (self) } # [doc = "Bit 8 - Reset Status Bits"] # [inline (always)] pub fn rststa (& mut self) -> RSTSTA_W < 8 > { RSTSTA_W :: new (self) } # [doc = "Bit 18 - Force SPI Chip Select"] # [inline (always)] pub fn fcs (& mut self) -> FCS_W < 18 > { FCS_W :: new (self) } # [doc = "Bit 19 - Release SPI Chip Select"] # [inline (always)] pub fn rcs (& mut self) -> RCS_W < 19 > { RCS_W :: new (self) } # [doc = "Writes raw bits to the register."] # [inline (always)] pub unsafe fn bits (& mut self , bits : u32) -> & mut Self { self . 0 . bits (bits) ; self } } # [doc = "Control Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mode_cr_spi_mode](index.html) module"] pub struct SPI_MODE_CR_SPI_MODE_SPEC ; impl crate :: RegisterSpec for SPI_MODE_CR_SPI_MODE_SPEC { type Ux = u32 ; } # [doc = "`write(|w| ..)` method takes [spi_mode_cr_spi_mode::W](W) writer structure"] impl crate :: Writable for SPI_MODE_CR_SPI_MODE_SPEC { type Writer = W ; }