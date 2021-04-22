<template>
  <div></div>
</template>
<script>
const { clipboard } = window.require('electron');

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
      new Notification('复制成功', {
        body: `已复制到剪贴板：${this.info.data}`,
      }).show();
    },
  },
};
</script>
