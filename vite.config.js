// vite.config.js
export default {
  root: '/rust-lox/',
  build: {
      outDir: '../dist',
      commonjsOptions: {
        esmExternals: true
     },
  }
};
