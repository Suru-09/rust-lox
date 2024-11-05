// vite.config.js
export default {
  root: './',
  build: {
      outDir: 'dist',
      assetsDir: 'assets',
      commonjsOptions: {
        esmExternals: true
     },
  }
};
