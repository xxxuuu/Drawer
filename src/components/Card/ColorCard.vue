<template>
  <div class="color-card"
      :style="{ background: info.data }">
    <div class="color-code">{{ info.data.toUpperCase() }}</div>
  </div>
</template>

<script>
import BaseCard from './BaseCard.vue';

export default {
  mixins: [BaseCard],
};
</script>

<style lang="stylus" scoped>
.color-card
  display flex
  height 100%
  width 100%
  justify-content center
  align-items center
  .color-code
    font-size 26px
    font-weight bold
    color rgba(0,0,0,0.5)
</style>
