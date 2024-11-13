// vite.config.js
export default {
  base: './',
  build: {
      outDir: 'dist',
      assetsDir: './',
      commonjsOptions: {
        esmExternals: true
     },
      esbuild: {
        drop: ['console', 'debugger']
      }
  }
};
