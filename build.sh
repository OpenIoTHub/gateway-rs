~/export-esp.sh
cargo build  --features pio
espflash /dev/tty.usbserial-1420 target/[xtensa-esp32-espidf|xtensa-esp32s2-espidf|xtensa-esp32s3-espidf|riscv32imc-esp-espidf]/debug/<your-project-name>
espflash monitor /dev/ttyUSB0