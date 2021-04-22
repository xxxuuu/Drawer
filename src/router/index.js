import Vue from 'vue';
import VueRouter from 'vue-router';
import Drawer from '../views/Drawer.vue';

Vue.use(VueRouter);

const routes = [
  {
    path: '/',
    name: 'Drawer',
    component: Drawer,
  },
];

const router = new VueRouter({
  routes,
});

export default router;
