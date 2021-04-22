<template>
  <div class="root">
    <div class="header">
      <!-- <div class="search">🔍</div> -->
      <tags class="tags" :tags="['📝 剪贴板历史']" />
      <!-- <div class="add-tag">➕</div> -->
    </div>
    <div class="content">
      <div class="card-list">
        <card v-for="(i) in clipboardList" :key="i.time" :info="i"/>
      </div>
    </div>
  </div>
</template>

<script>
import event from '@/event-topic';
import card from '@/components/Card/Card.vue';
import tags from '@/components/Tags.vue';

const { ipcRenderer } = window.require('electron');

export default {
  components: {
    card,
    tags,
  },
  data() {
    return {
      clipboardList: [],
    };
  },
  created() {
    ipcRenderer.on(event.INIT, (e, data) => {
      this.clipboardList = data.reverse();
    });
    ipcRenderer.on(event.APPEND, (e, data) => {
      this.clipboardList.unshift(data);
    });
  },
};
</script>

<style lang="stylus" scoped>
.root
  display flex
  flex-direction column
  height 100%
  background-color #dbe2e4
  .header
    width 100%
    display flex
    flex-direction row
    align-items center
    justify-content center
    padding-top 10px
    user-select none
    .search
      margin-right 15px
    .add-tag
      height 22px
      width 22px
      margin-left 15px
      border-radius 5px
      border 1px dashed gray
      line-height 22px
      text-align center
  .content
    height 100%
    display flex
    flex-grow 1
    padding 15px 15px 15px 0
    overflow-x scroll
    .card-list
      display flex
      flex-direction row
      align-items center
      background-color #dbe2e4
</style>
