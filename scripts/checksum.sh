#!/bin/bash
VERSION=1.2.0

print_txt () {
	echo  "md5sum:"
	md5sum ./target/release/minix | echo "`awk '{print $1}'` minix-$VERSION-ubuntu-20.04-x86_64"
	md5sum ./target/release/wbuild/minix-runtime/minix_runtime.wasm | echo "`awk '{print $1}'` minix_wasm"
	md5sum ./target/release/wbuild/minix-runtime/minix_runtime.compact.wasm | echo "`awk '{print $1}'` minix_compact_wasm"
	md5sum ./target/release/wbuild/minix-runtime/minix_runtime.compact.compressed.wasm | echo "`awk '{print $1}'` minix_compressed_wasm"

	echo  "sha256:"
	sha256sum ./target/release/minix | echo "`awk '{print $1}'` minix-$VERSION-ubuntu-20.04-x86_64"
	sha256sum ./target/release/wbuild/minix-runtime/minix_runtime.wasm | echo "`awk '{print $1}'` minix_wasm"
	sha256sum ./target/release/wbuild/minix-runtime/minix_runtime.compact.wasm | echo "`awk '{print $1}'` minix_compact_wasm"
	sha256sum ./target/release/wbuild/minix-runtime/minix_runtime.compact.compressed.wasm | echo "`awk '{print $1}'` minix_compressed_wasm"
}

print_markdown () {
	md5sum_minix=$(md5sum ./target/release/minix | awk '{print $1}')
	md5sum_wasm=$(md5sum ./target/release/wbuild/minix-runtime/minix_runtime.wasm | awk '{print $1}')
	md5sum_compact=$(md5sum ./target/release/wbuild/minix-runtime/minix_runtime.compact.wasm | awk '{print $1}')
	md5sum_compressed=$(md5sum ./target/release/wbuild/minix-runtime/minix_runtime.compact.compressed.wasm | awk '{print $1}')

	sha256sum_minix=$(sha256sum ./target/release/minix | awk '{print $1}')
	sha256sum_wasm=$(sha256sum ./target/release/wbuild/minix-runtime/minix_runtime.wasm | awk '{print $1}')
	sha256sum_compact=$(sha256sum ./target/release/wbuild/minix-runtime/minix_runtime.compact.wasm | awk '{print $1}')
	sha256sum_compressed=$(sha256sum ./target/release/wbuild/minix-runtime/minix_runtime.compact.compressed.wasm | awk '{print $1}')

	echo "| md5sum | sha256 | name |
| :---: | :-----: | :-----: |
|$md5sum_minix|$sha256sum_minix|minix-$VERSION-ubuntu-20.04-x86_64|
|$md5sum_wasm|$sha256sum_wasm|minix_wasm|
|$md5sum_compact|$sha256sum_compact|minix_compact_wasm|
|$md5sum_compressed|$sha256sum_compressed|minix_compressed_wasm|
"
}


if [ "$1" = md ]; then
    print_markdown
else
    print_txt
fi
