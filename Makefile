ELF=target/riscv32imac-unknown-none-elf/debug/litex 
ELF_REL=target/riscv32imac-unknown-none-elf/release/litex 

run: litex.bin
	./rust.py

trace: litex.bin
	./rust.py --trace --trace-start=1000000 --trace-end=1100000

view:
	gtkwave ./build/sim/gateware/dut.vcd

litex.bin: $(ELF) $(ELF_REL)
	riscv64-unknown-elf-objcopy -O binary $(ELF_REL) $@

size: litex.bin
	size $(ELF)
	size $(ELF_REL)

fmt:
	cargo fmt

$(ELF_REL): src/main.rs Cargo.toml
	cargo build --release

$(ELF): src/main.rs Cargo.toml
	cargo build

dis:
	riscv64-unknown-elf-objdump -D $(ELF_REL) | less
