run: 
  cd antimono &&  cargo osdk run

build:
  cd antimono && cargo osdk build

gdb-server: 
	cd antimono && cargo osdk run -G --vsc --gdb-server-addr :1234

gdb-client: 
	cd antimono && cargo osdk debug --remote :1234

asm:
  echo "todo"
  