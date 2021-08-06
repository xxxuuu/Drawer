<template>
  <div class="root" @wheel="handleWheel" >
    <div class="header">
      <!-- <div class="search">🔍</div> -->
      <tags class="tags" :tags="tags" @add-tag="addTag" @switch-tag="switchTag" />
    </div>
    <div class="content" ref="content">
      <transition-group class="card-list" name="card-list">
        <card
          @contextmenu.native="cardContextMenu(i)"
          v-for="(i) in activeList"
          :key="i.time"
          :info="i"
        />
      </transition-group>
    </div>
  </div>
</template>

<script>
import event from '@/utils/event-topic';
import card from '@/render/components/Card/Card.vue';
import tags from '@/render/components/Tags.vue';

const { ipcRenderer, remote } = window.require('electron');

export default {
  components: {
    card,
    tags,
  },
  data() {
    return {
      clipboardList: [],
      tagClipboardList: [],
      tags: [{
        id: 0,
        name: '📝 剪贴板历史',
      }],
      nowTagIdx: 0,
    };
  },
  methods: {
    /** 卡片右键菜单 */
    cardContextMenu(cardData) {
      const submenu = this.tags.slice(1).map((item) => {
        const newItem = item;
        newItem.label = item.name;
        newItem.click = () => {
          // 钉到标签过去
          ipcRenderer.sendTo(
            remote.getGlobal('winId').worker,
            event.STORE_CLIPBOARD_TO_TAG,
            cardData,
            item.id,
          );
        };
        return newItem;
      });

      const menuTemplate = [{
        label: '钉起来',
        submenu,
      }, {
        label: '删除',
        enabled: this.nowTagIdx !== 0,
        click: () => {
          ipcRenderer.sendTo(
            remote.getGlobal('winId').worker,
            event.DEL_CLIPBOARD_TAG,
            cardData.id,
          );
        },
      }];

      remote.Menu.buildFromTemplate(menuTemplate).popup();
    },
    /** 新增tag */
    addTag(name) {
      ipcRenderer.sendTo(remote.getGlobal('winId').worker, event.ADD_TAG, name);
    },
    /** 获取所有剪贴板 */
    getAllClipboard() {
      ipcRenderer.sendTo(remote.getGlobal('winId').worker, event.INIT);
    },
    /** 更新tags */
    updateTags() {
      this.tags = this.tags.slice(0, 1);
      ipcRenderer.sendTo(remote.getGlobal('winId').worker, event.GET_ALL_TAG);
    },
    /** 更新当前所在标签的剪贴板列表 */
    updateTagClipboardList() {
      ipcRenderer.sendTo(
        remote.getGlobal('winId').worker,
        event.GET_CLIPBOARD_BY_TAG,
        this.tags[this.nowTagIdx].id,
      );
    },
    /** 切换tag */
    switchTag(index) {
      this.nowTagIdx = index;
      if (index === 0) {
        this.tagClipboardList = [];
        return;
      }
      // 切换后查询对应tag的数据
      this.updateTagClipboardList();
    },
    /** 鼠标滚动事件 */
    handleWheel(e) {
      if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) {
        return;
      }
      this.$refs.content.scrollLeft += e.deltaY;
    },
    /** 初始化事件监听 */
    initEvent() {
      ipcRenderer.on(event.INIT_RESP, (e, data) => {
        this.clipboardList = data.reverse();
      });
      ipcRenderer.on(event.APPEND, (e, data) => {
        this.clipboardList.unshift(data);
      });
      ipcRenderer.on(event.DELETE_OLD, (e, len) => {
        this.clipboardList = this.clipboardList.slice(0, this.clipboardList.length - len);
      });
      ipcRenderer.on(event.ADD_TAG_RESP, (e, tagData) => {
        this.tags.push(tagData);
      });
      ipcRenderer.on(event.GET_ALL_TAG_RESP, (e, tagData) => {
        this.tags.push(...tagData);
      });
      ipcRenderer.on(event.GET_CLIPBOARD_BY_TAG_RESP, (e, clipboards) => {
        this.tagClipboardList = clipboards;
      });
      ipcRenderer.on(event.DEL_TAG_RESP, this.updateTags);
      ipcRenderer.on(event.DEL_CLIPBOARD_TAG_RESP, this.updateTagClipboardList);
    },
  },
  computed: {
    activeList() {
      if (this.nowTagIdx === 0) {
        return this.clipboardList;
      }
      return this.tagClipboardList;
    },
  },
  created() {
    this.initEvent();
    this.getAllClipboard();
    this.updateTags();
  },
};
</script>

<style lang="stylus" scoped>
.root
  display flex
  flex-direction column
  height 100%
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
      & > *
        transition all 0.4s
      .card-list-enter, .card-list-leave-to
        opacity 0
        transform translateY(30px)
      .card-list-leave-active
        position absolute
</style>
