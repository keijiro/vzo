#!/bin/sh

VST_DIR=${HOME}"/Library/Audio/Plug-Ins/VST"

cargo build --release
./osx_vst_bundler.sh vzo target/release/libplugin.dylib

[ -e ${VST_DIR}/vzo.vst ] && rm -r ${VST_DIR}/vzo.vst
cp -r vzo.vst ${VST_DIR}
