<template>
  <div class="tags-list">
    <div
      v-for="(i, index) in tags" :key="index"
      :class="{ 'active': index===activeIndex, 'tags': true }"
      @click="switchTag(index)"
      @contextmenu="tagContextMenu(i, index)"
    >
      {{ i.name }}
    </div>
    <input
      v-if="isAdding"
      v-model="addInputText"
      @blur="cancelInput"
      @keydown.enter="isAdding = false"
      ref="addInput"
      class="add-input"
    />
    <div
      v-if="addable"
      @click="addTag"
      class="add-tag"
    >
      ➕
    </div>
  </div>
</template>

<script>
import Vue from 'vue';
import event from '@/utils/event-topic';

const { ipcRenderer, remote } = window.require('electron');

const ADD_TAG_EVENT = 'add-tag';
const SWITCH_TAG_EVENT = 'switch-tag';

export default {
  props: {
    tags: Array,
    addable: {
      type: Boolean,
      default: true,
    },
  },
  data() {
    return {
      activeIndex: 0,
      isAdding: false,
      addInputText: '',
    };
  },
  methods: {
    tagContextMenu(tagData, idx) {
      remote.Menu.buildFromTemplate([{
        label: '删除标签',
        enabled: tagData.id !== 0, // 剪贴板历史不能删
        click: () => {
          // 删除的是当前的或前面的 就切换到前一个
          if (tagData.id === this.tags[this.activeIndex].id || idx <= this.activeIndex) {
            this.switchTag(this.activeIndex - 1);
          }
          ipcRenderer.sendTo(remote.getGlobal('winId').worker, event.DEL_TAG, tagData.id);
        },
      }]).popup();
    },
    addTag() {
      this.addInputText = '';
      this.isAdding = true;
      Vue.nextTick(() => this.$refs.addInput.focus());
    },
    cancelInput() {
      if (this.addInputText.trim() !== '') {
        this.$emit(ADD_TAG_EVENT, this.addInputText.trim());
      }
      this.isAdding = false;
    },
    switchTag(index) {
      this.activeIndex = index;
      this.$emit(SWITCH_TAG_EVENT, index);
    },
  },
};
</script>

<style lang="stylus" scoped>
.tags-list
  display flex
  flex-direction row
  align-items center
  justify-content center
  .tags
    font-size 14px
    border-radius 4px
    padding 6px 8px
  .active
    background #bbb
  .tags:not(:first-child)
    margin-left 10px
  .add-tag
    height 22px
    width 22px
    margin-left 15px
    border-radius 5px
    border 1px dashed gray
    line-height 22px
    text-align center
  .add-input
    display inline-block
    background #bbb
    margin-left 15px
    padding 6px 8px
    height 22px
    border none
    border-radius 4px
    &:focus
      outline none
</style>
