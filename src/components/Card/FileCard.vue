<template>
  <div class="file-card" @click="previewFile">
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
    this.setImage(`data:image/png;base64,${this.info.thumbnail}`, this.$refs.img);
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
