#!/bin/bash

# Rust examples - simple build script for Switch formats
# Customizable options (those you should only edit) are those starting by '*'.
# Most attributes will be commented (disabled), uncomment to set specific attributes

# * Output type -> { elf, nro, nso }
output_type="nro"

# * NACP attributes (optional, NRO only) -> NRO name, author and version (try using the same ones that are in Cargo.toml)
nacp_name="snake-rs"
nacp_author="kirjavascript"
nacp_version="1.0.0"

# * Icon (optional, NRO only) -> NRO's icon (256x256 JPEG), using libnx's one by default
icon=icon.jpg

# * RomFs directory (optional, NRO only) -> directory name to be embedded within the NRO as a RomFs filesystem.
romfs_dir=romfs

# ---
# ---
# ! Build implementation, better not to edit this part
# ---
# ---

# Toolchain, must be the JSON's name (better not to change this or the JSON itself, unless you want to add extra libraries)
target=aarch64-horizon-elf

# Build the project
CARGO_INCREMENTAL=0 RUST_TARGET_PATH=$PWD RUST_BACKTRACE=1 xargo build --release --target $target

# Locate ELF output file
for elf in "$PWD/target/$target/release"/*.elf; do
    outelf=$elf
    break 1
done

# Output file name (for generating other Switch formats)
outelfname="$(basename -- "$PWD/$outelf")"
outname="${outelfname%.*}"

# Default value for nacp_name if isn't set
if [ -z "$nacp_name" ]; then
    nacp_name=Rust application
fi

# Default value for nacp_author if isn't set
if [ -z "$nacp_author" ]; then
    nacp_name=Unknown author
fi

# Default value for nacp_version if isn't set
if [ -z "$nacp_version" ]; then
    nacp_name=Unknown version
fi

# Default icon if isn't specified
if [ -z "$icon" ]; then
    icon=/opt/devkitpro/libnx/default_icon.jpg
fi

if [ "$output_type" = "elf" ]; then
    # If just plain ELF, copy output one and that's it
    newelf=$PWD/$outname
    cp -r "$elf" "$newelf"
elif [ "$output_type" = "nro" ]; then
    outnro="$PWD/$outname.nro"

    # Generate NACP
    outnacp="$PWD/target/$target/release/$outname.nacp"
    /opt/devkitpro/tools/bin/nacptool --create "$nacp_name" "$nacp_author" "$nacp_version" "$outnacp"

    # Generate NRO
    if [ -n "$romfs_dir" ]; then
        romfsdir="--romfsdir=$PWD/$romfs_dir"
    fi
    /opt/devkitpro/tools/bin/elf2nro "$elf" "$outnro" --nacp="$outnacp" --icon="$icon" "$romfsdir"

    echo "Generated NRO: $outnro"
elif [ "$output_type" = "nso" ]; then
    outnso="$PWD/$outname".nso

    # Command for NSO generation
    /opt/devkitpro/tools/bin/elf2nso "$elf" "$outnso"
    echo "Generated NSO: $outnso"
fi
