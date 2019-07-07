target remote :3333

define reset
    mon mww 0x40000004 0x80000000
end

# print demangled symbols by default
set print asm-demangle on

monitor arm semihosting enable

load
