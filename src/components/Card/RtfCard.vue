<template>
  <div class="rtf-card">
    <div class="content" v-html="rtfHtml" ref="rtf"></div>
    <div class="bg"></div>
  </div>
</template>

<script>
import { RTFJS } from 'rtf.js';
import BaseCard from './BaseCard.vue';

const { clipboard } = window.require('electron');

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
    this.doc = new RTFJS.Document(this.stringToArrayBuffer(this.info.data.rtf));
    this.doc.render().then((elements) => {
      elements.forEach((e) => {
        // 渲染tab缩进
        e.style.whiteSpace = 'pre';
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
    copyOnCard() {
      clipboard.write({
        text: this.info.data.text,
        rtf: this.info.data.rtf,
      });
      new Notification('复制成功', {
        body: `已复制到剪贴板：${this.$refs.rtf.textContent}`,
      }).show();
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
