#!/bin/sh
cargo build --release
./osx_vst_bundler.sh OSCBridge target/release/libplugin.dylib
rm -r ~/Library/Audio/Plug-Ins/VST/OSCBridge.vst
cp -r OSCBridge.vst ~/Library/Audio/Plug-Ins/VST
