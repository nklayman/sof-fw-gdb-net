# sof-fw-gdb-net

Exposes Zephyr's GDB stub on SOF DSP over a network port. Useful for running gdb host on development machine instead of target so source files are available.

Sample usage:

```bash
# (Start on development machine)
ssh -4 -L 4000:localhost:4000 user@ssh-target-ip # SSH into target machine and forward port
sudo ./sof-fw-gdb-net # Run app, needs sudo to access fw_gdb file


# In a separate terminal, on development machine
[toolchain-specific gdb binary] -q
# In GDB console
file [path to zephyr.elf file] # Should be in build-[platform]/zephyr/zephyr.elf
target remote :4000 # Connect to the forwarded port
# GDB debugging should work as normal from now on
```
