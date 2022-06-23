# sof-fw-gdb-net

Exposes Zephyr's GDB stub on SOF DSP over a network port. Useful for running gdb host on development machine instead of target so source files are available.

Sample usage:

```bash
# (Start on development machine)
ssh -4 -L 4000:localhost:4000 user@ssh-target-ip # SSH into target machine and forward port
wget https://github.com/nklayman/sof-fw-gdb-net/releases/latest/download/sof-fw-gdb-net && chmod +x ./sof-fw-gdb-net # Download program, only do this once
sudo ./sof-fw-gdb-net # Run program (needs sudo to access fw_gdb file)


# In a separate terminal, on development machine
[toolchain-specific gdb binary] -q
# In GDB console
file [path to zephyr.elf file] # Should be in build-[platform]/zephyr/zephyr.elf
target remote :4000 # Connect to the forwarded port
# GDB debugging should work as normal from now on
```
