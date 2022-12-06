<template>
  <div class="card" @dblclick="paste" :id="`card-${info.id}`">
    <div class="header">
      <span>{{ this.title }}</span>
      <span class="time">{{ dateFormat(info.time) }}</span>
    </div>
    <component ref="cardInstance" :is="switchCard" :info="info"/>
    <div class="footer">
      <div class="decription">{{ info.description }}</div>
    </div>
  </div>
</template>

<script>
import dayjs from 'dayjs';
import ImgCard from './ImgCard.vue';
import TextCard from './TextCard.vue';
import ColorCard from './ColorCard.vue';
import FileCard from './FileCard.vue';
import RtfCard from './RtfCard.vue';
import RightMenu from '@right-menu/core';

import { invoke } from '@tauri-apps/api/tauri';
import { nextTick } from 'vue';

export default {
  props: [
    'info',
    'rightmenu',
  ],
  components: {
    'img-card': ImgCard,
    'text-card': TextCard,
    'color-card': ColorCard,
    'file-card': FileCard,
    'rtf-card': RtfCard,
  },
  methods: {
    /** 粘贴卡片内容 */
    paste() {
      invoke("paste", { id: this.info.id }).catch(err => {
        console.log(err);
        alert(err);
      })
    },
    /** 日期格式化 */
    dateFormat(timestamp) {
      return dayjs(Number(timestamp)).format('MM/DD HH:mm');
    },
    /** 初始化右键菜单 */
    initContextMenu() {
      let id = this.info.id;
      new RightMenu(`#card-${id}`, async () => this.rightmenu(id));
    }
  },
  mounted() {
    nextTick(this.initContextMenu);
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
        case 'rtf':
          return '📃 文本';
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

<style lang="stylus">
@media (prefers-color-scheme: light)
  :root
    --card-background white
    --card-header-background #eaeaea
    --card-header-text #333
@media (prefers-color-scheme: dark)
  :root
    --card-background #1e1e1e
    --card-header-background #282828
    --card-header-text white
</style>
<style lang="stylus" scoped>
.card
  display flex
  flex-direction column
  position relative
  background var(--card-background)
  height 100%
  min-width 250px
  width 250px
  height 275px
  word-break break-all
  box-shadow 2px 5px 5px rgba(0, 0, 0, 0.1)
  border-radius 5px
  margin 0px 15px 0 15px
  overflow hidden
  cursor default
  user-select none
  &:hover
    box-shadow 0px 10px 20px var(--tag-active-background)
    transform translateY(-8px)
  .header
    display flex
    align-items center
    position relative
    background var(--card-header-background)
    padding 5px 10px
    font-size 20px
    font-weight bold
    color var(--card-header-text)
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
    .decription
      margin 0 15px
</style>
