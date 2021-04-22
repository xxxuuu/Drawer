<template>
  <div class="img-card">
    <img class="img" :src="data" alt=""/>
  </div>
</template>

<script>
import BaseCard from './BaseCard.vue';

export default {
  mixins: [BaseCard],
};
</script>

<style lang="stylus" scoped>
.img-card
  display flex
  align-items center
  justify-content center
  width 100%
  height 100%
  background-image linear-gradient(45deg,rgba(0,0,0,0.15) 25%,transparent 0,transparent 75%,\
    rgba(0,0,0,0.15) 0),\
    linear-gradient(45deg,rgba(0,0,0,0.15) 25%,transparent 0,transparent 75%,rgba(0,0,0,0.15) 0)
  background-size 16px 16px
  background-position 0 0, 8px 8px
  .img
    height 100%
    width 100%
    object-fit contain
</style>
