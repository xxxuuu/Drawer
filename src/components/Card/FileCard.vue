<template>
  <div class="file-card" @click="previewFile">
    <img class="img" :src="`data:image/png;base64,${info.thumbnail}`" alt="">
  </div>
</template>

<script>
import BaseCard from './BaseCard.vue';

export default {
  mixins: [BaseCard],
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
