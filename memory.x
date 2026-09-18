/* Provides information about the memory layout of the device */
MEMORY {
  /* ITCM - note that External Flash won't boot? */
  FLASH (rx): ORIGIN = 0x00000000, LENGTH = 0x20000
  /* FLEXRAM - note that DTCM won't work for RTT */
  RAM (rwx) : ORIGIN = 0x24000000, LENGTH = 0x200000
}
