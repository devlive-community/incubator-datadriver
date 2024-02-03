<template>
  <div>
    <Menu mode="horizontal" theme="dark" active-name="home">
      <div class="layout-logo">
        <RouterLink to="/">
          <img src="/logo.png"/>
        </RouterLink>
      </div>
      <div>
        <MenuItem name="connector" to="/connector">
          <FontAwesomeIcon :icon="['fas', 'plug']"/>
          Connector
        </MenuItem>
      </div>
      <div class="layout-nav">
      </div>
    </Menu>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'

export default defineComponent({
  name: 'LayoutHeader'
})
</script>

<style scoped>
.layout-logo {
  width: 50px;
  height: 50px;
  border-radius: 3px;
  float: left;
  position: relative;
  top: 5px;
  left: 20px;
  margin-right: 100px;
}

.layout-logo > a > img {
  width: 100%;
  height: 100%;
}

.layout-nav {
  width: 420px;
  margin: 0 20px 0 auto;
}
</style>
