const path = require('path');

function resolve(dir) {
  return path.join(__dirname, dir);
}

module.exports = {
  chainWebpack: (config) => {
    config.resolve.alias
      .set('@', resolve('src'));
  },
  pages: {
    index: {
      entry: 'src/render/main.js',
      template: 'public/index.html',
      filename: 'index.html',
      title: 'Drawer',
    },
    worker: {
      entry: 'src/worker/worker.js',
      template: 'public/worker.html',
      filename: 'worker.html',
      title: 'Worker',
    },
  },
  pluginOptions: {
    electronBuilder: {
      builderOptions: {
        productName: 'Drawer',
        asar: false,
      },
    },
  },
};
