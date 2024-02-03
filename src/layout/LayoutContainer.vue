<template>
  <div>
    <Layout>
      <Header>
        <LayoutHeader/>
      </Header>
      <Content style="padding: 10px 10px 0 10px; background-color: white;">
        <RouterView/>
      </Content>
      <Footer class="layout-footer">Footer</Footer>
    </Layout>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import LayoutHeader from '@/layout/components/LayoutHeader.vue'
import LayoutContent from '@/layout/components/LayoutContent.vue'

export default defineComponent({
  name: 'LayoutContainer',
  components: {
    LayoutContent, LayoutHeader
  }
})
</script>
