import { defineConfig } from 'tsdown'

export default defineConfig({
  entry: ['./npm'],
  target: 'esnext',
  format: ['esm'],
  clean: true,
  minify: false,
  outDir: 'dist',
  treeshake: true
})
