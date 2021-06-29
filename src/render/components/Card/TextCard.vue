<template>
  <div class="text-card-container">
    <div class="text-card">
      <p class="text" v-html="replaceSpace"></p>
      <p class="bg">
        {{ space }}
      </p>
    </div>
    <div class="bg2"/>
  </div>
</template>

<script>
import BaseCard from './BaseCard.vue';

export default {
  mixins: [BaseCard],
  data() {
    return {
      space: '一',
    };
  },
  created() {
    for (let i = 1; i <= 500; i += 1) {
      this.space += '一';
    }
  },
  computed: {
    replaceSpace() {
      return this.info.data.replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/\n/g, '<br/>').replace(/ /g, '&nbsp;');
    },
  },
};
</script>

<style lang="stylus" scoped>
.text-card-container
  height 100%
  position relative
  .text-card
    padding 7px
    position relative
    width 100%
    height 100%
    p
      margin 0
      font-size 14px
    .text
      z-index 1
      position absolute
      margin 0
      color: black
      overflow hidden
      width 236px
    .bg
      z-index 0
      position absolute
      color white
      text-decoration underline #e2e2e2
      width 240px
  .bg2
    height 80px
    position relative
    top -80px
    z-index 2
    background linear-gradient(to top, \
      rgba(255,255,255,1), rgba(255,255,255,0.8), rgba(255,255,255,0) );
</style>
