<template>
  <div class="rtf-card">
    <div class="content" v-html="rtfHtml" ref="rtf"></div>
    <div class="bg"></div>
  </div>
</template>

<script>
import { RTFJS, WMFJS, EMFJS } from 'rtf.js';
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
    // FIXME: 从Word复制来的RTF数据前面有乱码，Excel的RTF数据解析有问题，原生实现是直接转换成NSAttributedString渲染在TextView上的
    this.doc = new RTFJS.Document(this.stringToArrayBuffer(this.decode(this.info.data.rtf)));
    this.doc.render().then((elements) => {
      elements.forEach((e) => {
        // 渲染tab缩进
        e.style.whiteSpace = 'pre-wrap';
        this.rtfHtml += e.outerHTML;
      });
    }).catch((err) => {
      console.error('rtf render error: ' + err);
    });
  },
  methods: {
    decode(base64Code) {
      let data = atob(base64Code);
      // console.log(data);
      return data;
    },
    stringToArrayBuffer(string) {
      const buffer = new ArrayBuffer(string.length);
      const bufferView = new Uint8Array(buffer);
      for (let i = 0; i < string.length; i += 1) {
        bufferView[i] = string.charCodeAt(i);
      }
      return buffer;
    }
  },
};
</script>

<style lang="stylus" scoped>
.rtf-card
  .content
    padding 10px
    font-size-adjust 0.35
    zoom .7
  .bg
    height 80px
    position absolute
    bottom 0
    width 100%
    z-index 2
    background var(--bg-linear-gradient)
</style>
