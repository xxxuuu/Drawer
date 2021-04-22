<template>
  <div class="tags-list">
    <div v-for="(i, index) in tags" :key="index"
      :class="{ 'active': index===activeIndex, 'tags': true }">
      {{ i }}
    </div>
  </div>
</template>

<script>
export default {
  props: {
    tags: Array,
  },
  data() {
    return {
      activeIndex: 0,
    };
  },
};
</script>

<style lang="stylus" scoped>
.tags-list
  display flex
  flex-direction row
  .tags
    font-size 14px
    border-radius 4px
    padding 6px 8px
  .active
    background #bbb
  .tags:not(:first-child)
    margin-left 10px
</style>
