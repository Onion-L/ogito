import { defineConfig } from 'tsdown'

export default defineConfig({
  entry: ['./package/index.ts'],
  target: 'esnext',
  format: ['esm'],
  clean: true,
  minify: false,
  outDir: 'dist',
  treeshake: true
})
