<template>
  <div class="img-card">
    <img class="img" :src="info.data" alt=""/>
  </div>
</template>

<script>
import BaseCard from './BaseCard.vue';

const { clipboard, nativeImage } = window.require('electron');

export default {
  mixins: [BaseCard],
  methods: {
    copyOnCard() {
      clipboard.writeImage(nativeImage.createFromDataURL(this.info.data));
      new Notification('复制成功', {
        body: '已复制到剪贴板：「图片」',
      }).show();
    },
  },
};
</script>

<style lang="stylus" scoped>
.img-card
  display flex
  align-items center
  justify-content center
  width 100%
  height 100%
  overflow hidden
  background-image \
    linear-gradient(45deg,rgba(0,0,0,0.15) 25%,transparent 0,transparent 75%,rgba(0,0,0,0.15) 0),\
    linear-gradient(45deg,rgba(0,0,0,0.15) 25%,transparent 0,transparent 75%,rgba(0,0,0,0.15) 0)
  background-size 20px 20px
  background-position 0 0, 10px 10px
  .img
    height 100%
    width 100%
    object-fit contain
</style>
