# ksymhunter

Simple tool to search for kernel symbol addresses.  


Original one is at [HERE](https://github.com/jonoberheide/ksymhunter) by `Jon Oberheide`.

This repository rewrites it with Rust and makes a little modification.


# Install

```install.sh
git clone https://github.com/smallkirby/ksymhunter && cd ./ksymhunber
cargo build --release
```

# Usage

Binary is linked statically. So you can just copy it in a target filesystem.

```usage.sh
$ sudo ksymhunber kmem_cache_alloc_trace
0xFFFFFFFFB7148270
$ sudo ksymhunter "not_existing_symbol"
```


# Original README

```original.txt
ksymhunter.c

Jon Oberheide <jon@oberheide.org>
http://jon.oberheide.org

Routines for hunting down kernel symbols from from kallsyms,
System.map, vmlinux, vmlinuz, and remote symbol servers.

Example:

$ ./ksymhunter prepare_kernel_cred
[+] trying to resolve prepare_kernel_cred...
[+] resolved prepare_kernel_cred using /boot/System.map-2.6.38-gentoo
[+] resolved prepare_kernel_cred to 0xffffffff81061060

$ ./ksymhunter commit_creds
[+] trying to resolve commit_creds...
[+] resolved commit_creds using /boot/System.map-2.6.38-gentoo
[+] resolved commit_creds to 0xffffffff81060dc0
```