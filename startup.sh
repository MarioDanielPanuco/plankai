#!/bin/bash
cd libs/simulation-wasm
wasm-pack build
cd ../../www
npm run start