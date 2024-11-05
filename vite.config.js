// vite.config.js
export default {
  root: '/rust-lox/rlox-web/',
  build: {
      outDir: '../dist',
      commonjsOptions: {
        esmExternals: true
     },
  }
};
