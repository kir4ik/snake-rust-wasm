import { defineConfig } from 'vite';
import wasmPack from 'vite-plugin-wasm-pack';

/** @type {import('vite').UserConfig} */
export default defineConfig({
  plugins: [wasmPack('./rust_lib')]
});
