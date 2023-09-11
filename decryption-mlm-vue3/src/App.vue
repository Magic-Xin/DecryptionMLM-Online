<script setup>
import HelloWorld from './components/HelloWorld.vue'
import DragUpload from "./components/DragUpload.vue";
</script>

<template>
  <header>
      <HelloWorld msg="MLM 触发器解密" />
  </header>

  <main>
    <DragUpload />
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}

@media (min-width: 1024px) {

  header {
    position: fixed;
    align-items: center;
    justify-content: center;
    top: 100px;
  }

  main {
    display: flex;
    align-items: center;
    justify-content: center;
  }
}
</style>
