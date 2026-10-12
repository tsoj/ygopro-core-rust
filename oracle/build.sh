#!/usr/bin/env bash
# Builds the differential oracle, target/oracle/libocgcore.so: upstream ygopro-core
# (oracle/ygopro-core, a submodule: `git submodule update --init --recursive oracle/ygopro-core`)
# plus oracle/ocgcore.patch and the extension sources oracle/ocgext.{h,cpp}. ygo-diff loads it.
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TARGET="$HERE/../target/oracle"
SRC="$TARGET/src"
OUT="$TARGET/build"
BUILDTYPE="${BUILDTYPE:-release}"

if [ ! -f "$HERE/ygopro-core/lua/src/lua.h" ]; then
	echo "the core is not checked out, run: git submodule update --init --recursive oracle/ygopro-core" >&2
	exit 1
fi

rm -rf "$SRC"
mkdir -p "$SRC"
# copy without the .git metadata
(cd "$HERE/ygopro-core" && tar --exclude=.git -cf - .) | (cd "$SRC" && tar -xf -)
(cd "$SRC" && patch -p1 --quiet < "$HERE/ocgcore.patch")
cp "$HERE/ocgext.h" "$HERE/ocgext.cpp" "$SRC/"

if [ ! -f "$OUT/build.ninja" ]; then
	meson setup "$OUT" "$SRC" --buildtype="$BUILDTYPE" -Ddefault_library=shared -Db_ndebug=if-release >/dev/null
fi
ninja -C "$OUT" >"$OUT/build.log" 2>&1 || { grep -E "error|Error" "$OUT/build.log" >&2; exit 1; }
# replace atomically: running processes keep their mapping of the old library
cp "$OUT/libocgcore.so" "$TARGET/libocgcore.so.new"
mv -f "$TARGET/libocgcore.so.new" "$TARGET/libocgcore.so"
echo "built $TARGET/libocgcore.so"
