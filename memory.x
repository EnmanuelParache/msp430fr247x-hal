MEMORY
{
  RAM : ORIGIN = 0x2000, LENGTH = 0x1FFF
  ROM : ORIGIN = 0x8000, LENGTH = 0xFFFF
  VECTORS : ORIGIN = 0xFF80, LENGTH = 0x80
}
