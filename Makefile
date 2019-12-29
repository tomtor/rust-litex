ELF=target/riscv32imac-unknown-none-elf/debug/litex 
ELF_REL=target/riscv32imac-unknown-none-elf/release/litex 

run: litex.bin
	./rust.py
	#(cd ..; ./rust.py --trace --trace-start=1076000 --trace-end=1100000)

litex.bin: $(ELF) $(ELF_REL)
	riscv64-unknown-elf-objcopy -O binary $(ELF_REL) $@

size: litex.bin
	size $(ELF)
	size $(ELF_REL)

fmt:
	cargo fmt

$(ELF_REL): src/main.rs
	cargo build --release

$(ELF): src/main.rs
	cargo build

dis:
	riscv64-unknown-elf-objdump -D $(ELF) | less

view:
	gtkwave ../build/sim/gateware/start.gtkw # dut.vcd
