#!/bin/sh
cargo build --release
./osx_vst_bundler.sh OSCBridge target/release/libplugin.dylib
cp -r OSCBridge.vst ~/Library/Audio/Plug-Ins/VST
