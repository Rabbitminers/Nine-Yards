<template>
  <main class="conent-wrapper">
    <navigation-bar class="header"/>
    <article class="content-body">
      <side-bar/>
      <router-view class="conent-body-router"/>
    </article>
  </main>
</template>

<script lang="ts">
import NavigationBar from './components/navigation/NavigationBar.vue';
import SideBar from './components/navigation/SideBar.vue';

export default {
  name: "App",
  components: {
    "navigation-bar": NavigationBar,
    "side-bar": SideBar
}
}
</script>

<style lang="scss">
.content {
  &-wrapper {
    display: flex;
    flex-direction: column;
  }

  &-body {
    flex-grow: 1;
    display: flex;
    flex-direction: row;
  }
}

</style>