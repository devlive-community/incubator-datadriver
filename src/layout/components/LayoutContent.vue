<template>
  <div class="container">
    <RouterView/>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'

export default defineComponent({
  name: 'LayoutContent',
})
</script>
