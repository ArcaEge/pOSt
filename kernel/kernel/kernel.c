#include <stdio.h>
#include <stdlib.h>

#include <kernel/tty.h>

void kernel_main(void) {
	terminal_initialize();
	printf("Hello, kernel World!\n");
	abort();
}
