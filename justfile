target := "riscv64imac-unknown-none-elf"
mode := "debug"
user_build := "../target/" + target + "/" + mode

build_dir := "../build"
out_dir := "../build/disk"

build app_name: 
    @cargo build
    @rm -rf {{out_dir}}
    @mkdir -p {{out_dir}}
    @cp {{user_build}}/{{app_name}} {{out_dir}}
    @rcore-fs-fuse --fs sfs {{build_dir}}/raw.{{app_name}}.img {{out_dir}} zip
    @qemu-img convert -f raw {{build_dir}}/raw.{{app_name}}.img \
        -O qcow2 {{build_dir}}/qcow.disk.img
    @qemu-img resize {{build_dir}}/qcow.disk.img +1G
