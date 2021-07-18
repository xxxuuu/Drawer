<template>
  <div class="file-card" @click="previewFile">
    <img class="img" :src="info.preview" alt="">
  </div>
</template>

<script>
import BaseCard from './BaseCard.vue';

const { clipboard, remote } = window.require('electron');

export default {
  mixins: [BaseCard],
  methods: {
    copyOnCard() {
      // TODO: 多文件&文件夹
      // 复制文件到剪贴板
      // https://github.com/electron/electron/issues/9035
      clipboard.writeBuffer(
        'NSFilenamesPboardType',
        Buffer.from(`
          <?xml version="1.0" encoding="UTF-8"?>
          <!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
          <plist version="1.0">
            <array>
              <string>${this.info.data}</string>
            </array>
          </plist>
        `),
      );
      new Notification('复制成功', {
        body: `已复制到剪贴板：「文件 ${this.info.data}」`,
      }).show();
    },
    previewFile() {
      const mainWindow = remote.getCurrentWindow();
      mainWindow.previewFile(this.info.data);
    },
  },
};
</script>

<style lang="stylus" scoped>
.file-card
  display flex
  align-items center
  justify-content center
  width 100%
  height 100%
  overflow hidden
  .img
    height 70%
    width 70%
    object-fit contain
    filter drop-shadow(2px 5px 5px rgba(0,0,0,0.2))
</style>
