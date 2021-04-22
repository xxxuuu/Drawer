<template>
  <div class="rtf-card">
    <div class="content" v-html="rtfHtml"></div>
    <div class="bg"></div>
  </div>
</template>

<script>
import { RTFJS } from 'rtf.js';
import BaseCard from './BaseCard.vue';

export default {
  mixins: [BaseCard],
  data() {
    return {
      doc: '',
      rtfHtml: '',
    };
  },
  created() {
    RTFJS.loggingEnabled(false);
    this.doc = new RTFJS.Document(this.stringToArrayBuffer(this.info.data));
    this.doc.render().then((elements) => {
      // console.log(elements);
      elements.forEach((e) => {
        // TODO: 缩进
        this.rtfHtml += e.outerHTML;
      });
    }).catch((err) => {
      console.error(err);
    });
  },
  methods: {
    stringToArrayBuffer(string) {
      const buffer = new ArrayBuffer(string.length);
      const bufferView = new Uint8Array(buffer);
      for (let i = 0; i < string.length; i += 1) {
        bufferView[i] = string.charCodeAt(i);
      }
      return buffer;
    },
  },
};
</script>

<style lang="stylus" scoped>
.rtf-card
  .content
    padding 10px
    font-size-adjust 0.35
  .bg
    height 80px
    position absolute
    bottom 0
    width 100%
    z-index 2
    background linear-gradient(to top, \
      rgba(255,255,255,1), rgba(255,255,255,0.8), rgba(255,255,255,0) );
</style>
