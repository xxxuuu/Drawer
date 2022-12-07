<template>
  <div class="img-card">
    <img class="img" alt="" ref="img" />
  </div>
</template>

<script>
import BaseCard from './BaseCard.vue';

export default {
  mixins: [BaseCard],
  methods: {
    setImage(b64str, target) {
      return fetch(b64str).then(data => data.blob()).then((b) => {
        const url = URL.createObjectURL(b);
        target.src = url;
      })
    }
  },
  mounted() {
    this.setImage(`data:image/png;base64,${this.info.data}`, this.$refs.img);
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
