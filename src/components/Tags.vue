<template>
  <div class="tags-list">
    <TransitionGroup name="tags">
      <div
        v-for="(i, index) in tags" :key="i.id" :id="`tag-${i.id}`"
        :class="{ 'active': index===activeIndex, 'tags': true }"
        @click="switchTag(index)"
      >
        {{ i.name }}
      </div>
      <input
        v-if="isAdding"
        v-model="addInputText"
        @blur="cancelInput"
        @keydown.enter="cancelInput"
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
    </TransitionGroup>
  </div>
</template>

<script>
import { nextTick } from 'vue';
import RightMenu from '@right-menu/core'
import { invoke } from '@tauri-apps/api/tauri'

const ADD_TAG_EVENT = 'add-tag';
const CHANGE_TAG_EVENT = 'change-tag';
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
    /** 为每一个Tag初始化菜单 */
    initTagContextMenu() {
      this.tags.forEach(tag => {
        if(tag.id == 0) return;
        let id = tag.id;
        new RightMenu(`#tag-${id}`, [
          {
            type: 'li',
            text: '删除',
            callback: () => {
              invoke('delete_tag', { id }).then(() => {
                // 通知上层更新tag列表
                this.$emit(CHANGE_TAG_EVENT);
                // 删除的是当前的或前面的 就切换到前一个(当前tag的下标变了)
                let idx = 0;
                this.tags.forEach((item, i) => { if(item.id == id) idx = i });
                console.log(idx);
                if (id === this.tags[this.activeIndex].id || idx <= this.activeIndex) {
                  this.switchTag(this.activeIndex - 1);
                }
              }).catch(err => {
                console.log(err);
                alert(err);
              });
            }
          }
        ])
      })
    },
    addTag() {
      this.addInputText = '';
      this.isAdding = true;
      nextTick(() => this.$refs.addInput.focus());
    },
    cancelInput() {
      if (this.addInputText.trim() !== '') {
        this.$emit(ADD_TAG_EVENT, this.addInputText.trim());
      }
      this.isAdding = false
    },
    switchTag(index) {
      this.activeIndex = index;
      this.$emit(SWITCH_TAG_EVENT, index);
    }
  },
  watch: {
    tags: {
      handler() {
        // 菜单创建依赖DOM，所以等DOM创建了出来才执行
        nextTick(this.initTagContextMenu)
      },
      deep: true,
    }
  },
  mounted() {
    this.initTagContextMenu();
  }
};
</script>

<style lang="stylus">
@media (prefers-color-scheme: light)
  :root
    --tag-active-background rgba(0,0,0,0.15)
    --tag-text-color #252525
    --tag-text-active-color black
@media (prefers-color-scheme: dark)
  :root
    --tag-active-background rgba(255,255,255,0.25)
    --tag-text-color #e2e2e2
    --tag-text-active-color white
</style>
<style lang="stylus" scoped>
.tags-list
  display flex
  flex-direction row
  align-items center
  justify-content center
  .tags
    font-size 14px
    color var(--tag-text-color)
    border-radius 4px
    padding 6px 8px
    transition all 0.5s
    cursor pointer
  .tags-move, .tags-enter-active, .tags-leave-active
    transition all 0.3s
  .tags-leave-active
    position absolute
  .tags-leave-to, .tags-enter-from
    opacity 0
  .tags-leave-from, .tags-enter-to
    opacity 1
    transform translate(0, 0)
  .tags-enter-from
    transform translateY(10px)
  .tags-leave-from
    transform translateY(0px)
  .tags-leave-to
    transform translateY(-10px)

  .active
    color var(--tag-text-active-color)
    background var(--tag-active-background)
  .tags:not(:first-child)
    margin-left 10px
  .add-tag
    height 22px
    width 22px
    margin-left 15px
    border-radius 5px
    border 1px dashed var(--tag-text-color)
    line-height 22px
    text-align center
    cursor pointer
    transition all 0.3s
    &:hover
      box-shadow 0px 5px 10px var(--tag-active-background)
      transform translateY(-3px)
  .add-input
    display inline-block
    color var(--tag-text-color)
    background var(--tag-active-background)
    margin-left 15px
    padding 6px 8px
    height 22px
    border none
    border-radius 4px
    transition all 0.5s
    &:focus
      outline none
</style>
