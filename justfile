target := "riscv64imac-unknown-none-elf"
mode := "debug"
bin_file := "../target/" + target + "/" + mode + "/kernel.bin"

user_dir := "../target/" + target
user_build := user_dir + "/build"

out_dir := "disk"

build app_name: 
    @cargo build
    @rm -rf {{out_dir}}/{{app_name}}
    @cp {{user_build}}/{{app_name}} {{out_dir}}
    @rcore-fs-fuse --fs sfs {{out_dir}}/raw.{{app_name}}.img {{out_dir}} zip
    @qemu-img convert -f raw {{out_dir}}/raw.{{app_name}}.img -O qcow2 out_dir/qcow.{{app_name}}.img
    @qemu-img resize out_dir/qcow.{{app_name}}.img +1G
