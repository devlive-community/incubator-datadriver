<template>
  <div>
    <div class="hidden flex-col md:flex">
      <LayoutHeader/>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import LayoutHeader from '@/layout/components/LayoutHeader.vue'

export default defineComponent({
  name: 'LayoutContainer',
  components: {LayoutHeader},
})
</script>
