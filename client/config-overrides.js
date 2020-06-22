const path = require('path');

module.exports = function override(config, env) {
  const wasmExtensionRegExp = /\.wasm$/;

  // This doesn't work with "wasm-pack build --target web" 
  //https://github.com/webpack/webpack/issues/8656
  // config.module.rules.push( {
  //   test: /\.js$/,
  //   exclude: /(node_modules|bower_components)/,
  //   use: {
  //     loader: 'babel-loader',
  //     options: {
  //       presets: ['@babel/preset-env'],
  //       plugins: ["@babel/plugin-syntax-import-meta"]
  //     }
  //   }
  // })


  config.resolve.extensions.push('.wasm');

  config.module.rules.forEach(rule => {
    (rule.oneOf || []).forEach(oneOf => {
      if (oneOf.loader && oneOf.loader.indexOf('file-loader') >= 0) {
        // Make file-loader ignore WASM files
        oneOf.exclude.push(wasmExtensionRegExp);
      }
    });
  });

  // Add a dedicated loader for WASM
  // https://github.com/ballercat/wasm-loader/issues/3
  // https://github.com/L-E-G/simulator/blob/e04e6dd855b7b8c30ad82847deba5a6e851c4b4a/gui/config-overrides.js
  config.module.rules.push({
    test: wasmExtensionRegExp,
    include: path.resolve(__dirname, 'src'),
    use: [{ loader: require.resolve('wasm-loader'), options: {} }]
  });


  return config;
};

