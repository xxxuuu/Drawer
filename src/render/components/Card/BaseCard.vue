<template>
  <div></div>
</template>
<script>
const { clipboard, remote } = window.require('electron');
const robotjs = window.require('robotjs');

export default {
  props: [
    // 剪贴板对象数据
    'info',
  ],
  data() {
    return {
      // 描述信息
      description: null,
    };
  },
  methods: {
    copyOnCard() {
      clipboard.writeText(this.info.data);
      // 文本类的直接粘贴
      if (this.info.type === 'text' || this.info.type === 'color' || this.info.type === 'url') {
        remote.getGlobal('windows').mainWindow.hide();
        robotjs.typeString(this.info.data);
      }
      new Notification('复制成功', {
        body: `已复制到剪贴板：${this.info.data}`,
      }).show();
    },
  },
};
</script>
