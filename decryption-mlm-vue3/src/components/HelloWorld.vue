<script setup>
defineProps({
  msg: {
    type: String,
    required: true
  }
})
</script>

<template>
  <div class="greetings">
    <h1 class="green">{{ msg }}</h1>
    <h3>
      如果觉得好用，可以给
      <a href="https://github.com/Magic-Xin/DecryptionMLM-Online" target="_blank" rel="noopener">这个 Gihub 项目</a>
      点一个 Star
    </h3>
  </div>
</template>

<style scoped>
h1 {
  font-weight: 500;
  font-size: 2.6rem;
}

h3 {
  font-weight: bold;
  font-size: 1.2rem;
}

.greetings h1,
.greetings h3 {
  text-align: center;
}

@media (min-width: 512px) {
  .greetings h1,
  .greetings h3 {
    text-align: center;
  }
}
</style>
