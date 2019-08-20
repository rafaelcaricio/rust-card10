ENTRY(__isr_vector);

/*
 * Segment in the output l0dable.
 *
 * They are mostly standard, but we define them explicitely so that we can
 * target them in sections.
 */
PHDRS
{
    header PT_PHDR PHDRS ;
    interp PT_INTERP ;
    text PT_LOAD FILEHDR PHDRS ;
    data PT_LOAD ;
}

/*
 * ELF sections.
 */
SECTIONS {
    . = SIZEOF_HEADERS;

    /*
     * Customer card10-l0dable INTERP/intepreter path.
     *
     * We nuke the original one (.interp) provided by gcc/ld, and inject out
     * own. This section is populated in l0dable.ld.
     */
    .cinterp :
    {
        *(.cinterp);
    } :interp :text

    .text :
    {
        /* The vector table needs 128 byte alignment */
        . = ALIGN(128);
        KEEP(*(.text.isr_vector))
        *(.text*)
        *(.rodata*)
    } :text

    .data :
    {
        . = ALIGN(4);
        *(.data*)
    } :data

    .bss :
    {
        . = ALIGN(4);
        __bss_start = .;
        *(.bss*)
        *(COMMON)
        . = ALIGN(4);
        __bss_end = .;
    } :data

    /* Used by hardware.c as start of heap. */
    __heap_start = .;

    /* Limit based on current limitations of l0dable setup - only uses core1 RAM. */
    ASSERT(. < 0x40000, "Exceeded available RAM")

    /DISCARD/ :
    {
        /* Compiler version - nuke. */
        *(.comment)
        /* ARM attributes - nuke. */
        *(.ARM.attributes)
        /* Original interpreter path from gcc/ld - nuke. */
        *(.interp)
        /* Dynamic linking section - nuke, we're not a .so and nothing is going to link against us. */
        *(.dynamic)

        *(.debug*)
        *(.init .fini .ARM.exidx.text.*)
    }
}
