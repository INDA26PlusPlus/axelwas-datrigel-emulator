.global _start
_start:
    addi x1, x0, 0
    addi x2, x0, 1
    addi x3, x0, 20
    addi x31, x0, 1
    ebreak
    add x4, x1, x2
    addi x1, x2, 0
    addi x2, x4, 0
    addi x3, x3, -1
    blt x3, x31, -20
    beq x0, x0, 0