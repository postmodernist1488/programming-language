    pop rbx
    pop rax
    push rbx
    xor rcx, rcx
    mov ebx, 10            ; divisor (base 10)

.numloop:

    xor edx, edx           ; dividend high half = 0.
    div ebx                ; Divides 1234 by 10.
                           ; EDX =   4 = 1234 % 10 remainder
                           ; EAX = 123 = 1234 / 10 quotient

    add edx, '0'           ; add up to an ascii code
    push rdx               ; push digit
    inc rcx                ; increment counter

    cmp eax, 0             ; while not zero
    jne .numloop

    mov rdx, rcx
    xor rcx, rcx
.string:
    cmp rcx, rdx
    jge .out
    pop rax
    mov BYTE [__stringspace + rcx], al
    inc rcx
    jmp .string

.out:

    ; write syscall
    mov rax, 1
    mov rdi, 1
    mov rsi, __stringspace
    ; mov rdx, rcx - already done
    syscall
