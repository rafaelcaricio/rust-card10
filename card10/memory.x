MEMORY {
    SPIX (rx)  : ORIGIN = 0x08000000, LENGTH = 128M
    FLASH (rx) : ORIGIN = 0x10010000, LENGTH = 960k
    RAM (rwx)  : ORIGIN = 0x20000000, LENGTH = 512k
    SPID (r)   : ORIGIN = 0x80000000, LENGTH = 512M
}

SECTIONS
{
  /DISCARD/ :
  {
    /* cortex-m-rt's link.x drops only `.ARM.exidx.*` */
    *(.ARM.exidx);
  }
}
