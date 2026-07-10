/*
 * BTI fingerprint to make satisfy the compiler requirement
 */
.section .note.gnu.property, "a"
.p2align 3
.word 4
.word 16
.word 5
.asciz "GNU"
.word 0xc0000000
.word 4
.word 3            /* BTI and PAC flags */
.p2align 3
