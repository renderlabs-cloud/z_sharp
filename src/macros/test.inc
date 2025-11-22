#include "assembly.inc"
#include "test_arch/arm64.inc"

ARCH_USE(ARM64)

BEGIN()


.global main
main:
	ret x1
