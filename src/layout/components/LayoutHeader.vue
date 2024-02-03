<template>
  <div>
    <div class="border-b">
      <div class="flex h-16 items-center px-4">
        <!-- Top menu -->
        <NavigationMenu>
          <NavigationMenuList>
            <NavigationMenuItem>
              <NavigationMenuLink href="/">
                <FontAwesomeIcon :icon="['fas', 'laptop']"/>
                Query
              </NavigationMenuLink>
            </NavigationMenuItem>
          </NavigationMenuList>
        </NavigationMenu>
        <!-- System settings -->
        <div class="ml-auto flex items-center space-x-4">
          <LayoutHeaderRight/>
        </div>
      </div>
    </div>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import {
  NavigationMenu,
  NavigationMenuContent,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  NavigationMenuTrigger,
  navigationMenuTriggerStyle,
} from '@/components/ui/navigation-menu'
import LayoutHeaderRight from "@/layout/components/LayoutHeaderRight.vue";

export default defineComponent({
  name: 'LayoutHeader',
  components: {
    LayoutHeaderRight,
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink, NavigationMenuList, NavigationMenuTrigger, navigationMenuTriggerStyle
  },
})
</script>
