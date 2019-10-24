MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 1024K
  RAM   : ORIGIN = 0x20000000, LENGTH =  192K
  STACK : ORIGIN = 0x1FFF0000, LENGTH =   64K
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
_stack_start = ORIGIN(STACK) + LENGTH(STACK);
