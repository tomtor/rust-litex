# Using Rust on Litex VexRiscv (Linux) #

Get started with https://github.com/litex-hub/linux-on-litex-vexriscv
and experiment with it until you are comfortable with using it.

## Enabling the C Riscv extension ##

Because the standard Riscv32 Rust compiler target is: `riscv32imac-unknown-none-elf`
and the default setup for VexRiscv has no `C` extension we need to change that.

See https://github.com/SpinalHDL/VexRiscv/issues/93 for a discussion on this subject.

To add the `C` extension you have to build a new VexRiscv variant:

https://github.com/tomtor/linux-on-litex-vexriscv#generating-the-vexriscv-linux-variant-optional

Apply the following diff before building with `sbt`.

```
diff --git a/src/main/scala/vexriscv/GenCoreDefault.scala b/src/main/scala/vexriscv/GenCoreDefault.scala
index a052205..9066e9e 100644
--- a/src/main/scala/vexriscv/GenCoreDefault.scala
+++ b/src/main/scala/vexriscv/GenCoreDefault.scala
@@ -92,6 +92,7 @@ object GenCoreDefault{
             resetVector = argConfig.resetVector,
             relaxedPcCalculation = argConfig.relaxedPcCalculation,
             prediction = argConfig.prediction,
+            compressedGen = true,
             memoryTranslatorPortConfig = if(linux) MmuPortConfig(portTlbSize = 4),
             config = InstructionCacheConfig(
               cacheSize = argConfig.iCacheSize,
@@ -149,7 +150,7 @@ object GenCoreDefault{
           catchIllegalInstruction = true
         ),
         new RegFilePlugin(
-          regFileReadyKind = plugin.SYNC,
+          regFileReadyKind = plugin.ASYNC,
           zeroBoot = false
         ),
         new IntAluPlugin,
@@ -268,4 +269,4 @@ class ForceRamBlockPhase() extends spinal.core.internals.Phase{
     }
   }
   override def hasNetlistImpact: Boolean = false

```

and copy the result `VexRiscv.v` to the `VexRiscv_Linux.v` of your `litex` tree (I assume `..`):

```
sudo cp VexRiscv.v ../litex/litex/soc/cores/cpu/vexriscv/verilog/VexRiscv_Linux.v
```

Verify that you can still boot the simulation in your `linux-on-litex-vexriscv` directory:
```
./sim.py
```

## Building a new root with the C Riscv extension (Optional) ##

Clone `https://github.com/tomtor/linux-on-litex-vexriscv` and checkout branch `rv32imac`

This applies the following changes:

```
CONFIG_PACKET=y            # in buildroot/board/litex_vexriscv/linux.config for a working DHCP
BR2_RISCV_ISA_CUSTOM_RVC=y # in buildroot/configs/litex_vexriscv_defconfig Enable C compression
riscv,isa = "rv32imac";    # in json2dts.py for a correct /proc/cpuinfo
```

Build the new root: https://github.com/tomtor/linux-on-litex-vexriscv#generating-the-linux-binaries-optional

Copy the result images (`Image` and `rootfs.cpio` to your `linux-on-litex-vexriscv/buildroot` directory
and test again with `./sim.py`).

## Using Rust in Linux

Currently there is no Rust `std` support for Riscv32. A workaround is to build a `nostd` Rust library and link this with a simple `C` main program. See the `rustlib` directory in `https://github.com/tomtor/linux-on-litex-vexriscv` for a minimal demo.

## Boot a Rust embedded program instead of the Linux image

This crate creates a simple demo program which you can boot as a replacement for the standard Linux image.

Run `./rust.py` to experiment with it in a simulation.

```
--============= Liftoff! ===============--
VexRiscv Machine Mode software built Dec 28 2019 19:48:43
--========== Booting Linux =============--
Hello World
i: 1 1/i: 1
533379
i: 2 1/i: 0.5
540759
i: 3 1/i: 0.33333334
549040
i: 4 1/i: 0.25
556395
i: 5 1/i: 0.2
563675
i: 6 1/i: 0.16666667
571637
i: 7 1/i: 0.14285715
579679
i: 8 1/i: 0.125
587093
i: 9 1/i: 0.11111111
595013
100
200
300
400
500
600
700
800
900
[-450, -400, -350, -300, -250, -200, -150, -100, -50, 0, 50, 100, 150, 200, 250, 300, 350, 400, 450] in 4054

panic!
panicked at 'End of main()', src/main.rs:63:5
```
