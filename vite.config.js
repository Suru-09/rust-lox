// vite.config.js
export default {
  root: './',
  build: {
      outDir: '../dist',
      commonjsOptions: {
        esmExternals: true
     },
  }
};
