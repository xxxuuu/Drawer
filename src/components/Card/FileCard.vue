<template>
  <div>
    <img :src="data" alt="">
  </div>
</template>

<script>
import BaseCard from './BaseCard.vue';

export default {
  mixins: [BaseCard],
};
</script>
