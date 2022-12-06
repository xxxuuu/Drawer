<template>
  <div class="root" @wheel="handleWheel" @contextmenu.native="e => e.preventDefault()">
    <div class="header">
      <!-- <div class="search">🔍</div> -->
      <tags class="tags" :tags="tags" @change-tag="refreshTags" @add-tag="addTag" @switch-tag="switchTag" />
    </div>
    <div class="content">
      <!-- 
        尝试了下vue-virtual-scroller优化下这里的长列表性能，坑太多了
        不兼容Vue的Transition，监听wheel设置deltaX来支持横向滚动的方法也不能用了
        复用DOM导致RightMenu没法移除事件从而内存泄漏
        （虽然这是RightMenu的锅，需要通过闭包来传递参数，变化时只能不停新增事件，但之前DOM销毁可以正常释放也就无所谓了）
        后面有空得自己实现一个才能满足需求 
      -->
      <div class="card-list" ref="list">
        <transition-group name="card-list">
          <card
            v-for="(i) in clipboardList"
            :key="i.id"
            :info="i"
            :rightmenu="this.cardRightMenu"
          />
        </transition-group>
      </div>
    </div>
  </div>
</template>

<script>
import Card from './components/Card/Card.vue';
import Tags from './components/Tags.vue';
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'

export default {
  components: {
    'card': Card,
    'tags': Tags
  },
  data() {
    return {
      clipboardList: [],
      tags: [],
      nowTagIdx: 0,
    };
  },
  methods: {
    /** 生成右键菜单选项，传递给Card用 */
    cardRightMenu(id) {
      return [
        {
          type: 'ul',
          text: 'Pin',
          children: this.tags.filter(item => item.id !== 0 && item.id !== this.nowTagIdx).map((item) => {
            return {
              type: "li",
              text: item.name,
              callback: () => invoke('pin_record', { recordId: id, tagId: item.id }).catch(err => {
                console.log(err);
                alert(err);
              })
            }
          })
        },
        {
          type: 'li',
          text: '删除',
          callback: () => invoke('delete_record', { id })
        }
      ];
    },
    /** 新增tag */
    addTag(name) {
      invoke("create_tag", { name }).then(msg => {
        this.tags.push(msg);
      }).catch(err => {
        console.log(err);
        alert(err);
      })
    },
    /** 刷新tag */
    refreshTags() {
      invoke("get_all_tags").then(msg => {
        this.tags = msg
      }).catch(err => {
        console.log(err);
        alert(err);
      });
    },  
    /** 初始化标签栏 */
    initTags() {
      this.refreshTags();
    },
    /** 初始化剪贴板 */
    initClipboard() {
      this.refreshClipboard();
    },
    /** 更新当前所在标签的剪贴板列表 */
    refreshClipboard() {
      let tagId = this.tags[this.nowTagIdx]?.id ?? 0;
      invoke("get_all_record", { tagId }).then(msg => {
        this.clipboardList = msg.reverse();
      }).catch(err => { console.log(err); alert(err) });
    },
    /** 切换tag */
    switchTag(index) {
      if(index === this.nowTagIdx) return;
      this.nowTagIdx = index;
      // 切换后查询对应tag的数据
      this.refreshClipboard();
    },
    /** 鼠标滚动事件 */
    handleWheel(e) {
      if (Math.abs(e.deltaX) > Math.abs(e.deltaY)) {
        return;
      }
      this.$refs.list.scrollLeft += e.deltaY;
    },
    /** 初始化事件监听 */
    initEvent() {
      listen('CLIPBOARD_UPDATE', (event) => {
        if (this.tags[this.nowTagIdx].id !== 0) return;
        let item = event.payload;
        this.clipboardList.unshift(item);
      })
      listen("CLIPBOARD_DELETE", (event) => {
        let delete_ids = event.payload;
        this.clipboardList = this.clipboardList.filter(item => !delete_ids.includes(item.id));
      })
    },
  },
  created() {
    this.initTags();
    this.initClipboard();
    this.initEvent();
  }
};
</script>

<style lang="stylus">
html, body, #app
  margin 0
  height 100%
  background rgba(255,255,255,0)
  font-family: "PingFang SC",Arial,"Microsoft YaHei"
img
  -webkit-user-drag none
::-webkit-scrollbar
  display none
</style>

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
    overflow-x scroll
    .card-list
      display flex
      flex-direction row
      align-items center
      overflow-x scroll
      padding-right 15px
      &>*
        transition all 0.3s
      .card-list-enter-from, .card-list-leave-to
        opacity 0
        transform translateY(30px)
      .card-list-leave-active
        position absolute
</style>
