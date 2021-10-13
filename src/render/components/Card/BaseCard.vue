<template>
  <div></div>
</template>
<script>
const { clipboard } = window.require('electron');
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
      // 让到下一个event loop再粘贴 不然太快的话窗口还来不及隐藏 导致前面几个文字没能输出到
      setTimeout(() => {
        robotjs.typeString(this.info.data);
      }, 0);
      new Notification('复制成功', {
        body: `已复制到剪贴板：${this.info.data}`,
      }).show();
    },
  },
};
</script>
