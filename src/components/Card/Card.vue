<template>
  <div class="card" @dblclick="copyOnCard">
    <div class="header">
      {{ this.title }}
      <span class="time">{{ info.time | dateFormat }}</span>
    </div>
    <component :is="switchCard" :data="info.data"/>
    <div class="footer">{{ info.description }}</div>
  </div>
</template>

<script>
import ImgCard from './ImgCard.vue';
import TextCard from './TextCard.vue';
import ColorCard from './ColorCard.vue';
import FileCard from './FileCard.vue';
import RtfCard from './RtfCard.vue';

const { clipboard, nativeImage } = window.require('electron');

export default {
  props: [
    'info',
  ],
  components: {
    'img-card': ImgCard,
    'text-card': TextCard,
    'color-card': ColorCard,
    'file-card': FileCard,
    'rtf-card': RtfCard,
  },
  data() {
    return {
    };
  },
  methods: {
    // 从卡片中复制
    copyOnCard() {
      // TODO: 不同类型复制信息的优化
      if (this.info.type === 'image') {
        clipboard.writeImage(nativeImage.createFromDataURL(this.info.data));
        new Notification('复制成功', {
          body: '已复制到剪贴板：「图片」',
        }).show();
      } else {
        clipboard.writeText(this.info.data);
        new Notification('复制成功', {
          body: `已复制到剪贴板：${this.info.data}`,
        }).show();
      }
    },
  },
  filters: {
    dateFormat(timestamp) {
      const date = new Date(timestamp);
      return `${date.getMonth() + 1}/${date.getDate()} ${date.getHours()}:${date.getMinutes()}`;
    },
  },
  computed: {
    title() {
      switch (this.info.type) {
        case 'image':
          return '🖼 图片';
        case 'file':
          return '🗂 文件';
        case 'color':
          return '🎨 颜色';
        case 'url':
          return '🔗 链接';
        default:
          return '📃 文本';
      }
    },
    switchCard() {
      switch (this.info.type) {
        case 'image':
          return ImgCard;
        case 'color':
          return ColorCard;
        case 'file':
          return FileCard;
        case 'rtf':
          return RtfCard;
        default:
          return TextCard;
      }
    },
  },
};
</script>

<style lang="stylus" scoped>
.card
  display flex
  flex-direction column
  position relative
  background white
  height 100%
  min-width 250px
  width 250px
  word-break break-all
  box-shadow 2px 5px 5px rgba(0, 0, 0, 0.1)
  border-radius 5px
  margin 0 15px
  overflow hidden
  cursor default
  user-select none
  .header
    display flex
    align-items center
    position relative
    height 40px
    background #eaeaea
    padding 5px 10px
    font-size 20px
    font-weight bold
    color #333
    .time
      font-size 12px
      text-align right
      position absolute
      font-weight normal
      right 10px
      bottom 5px
      color gray
  .footer
    z-index 5
    position absolute
    font-size 12px
    color gray
    left 0
    bottom 2px
    width 100%
    display flex
    justify-content center
</style>
