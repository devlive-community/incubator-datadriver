<template>
  <div>
    <DropdownMenu>
      <DropdownMenuTrigger as-child>
        <Button variant="ghost" class="relative h-8 w-8 rounded-full">
          <Avatar class="h-8 w-8">
            <AvatarImage src="/logo.png" alt="@datadriver"/>
            <AvatarFallback>DataDriver</AvatarFallback>
          </Avatar>
        </Button>
      </DropdownMenuTrigger>
      <DropdownMenuContent class="w-56" align="end">
        <DropdownMenuLabel class="font-normal flex">
          <div class="flex flex-col space-y-1">
            <p class="text-sm font-medium leading-none">
              datadriver
            </p>
            <p class="text-xs leading-none text-muted-foreground">
              datadriver@devlive.org
            </p>
          </div>
        </DropdownMenuLabel>
        <DropdownMenuSeparator/>
        <DropdownMenuGroup>
          <DropdownMenuItem>
            Profile
            <DropdownMenuShortcut>⇧⌘P</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem>
            Billing
            <DropdownMenuShortcut>⌘B</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem>
            Settings
            <DropdownMenuShortcut>⌘S</DropdownMenuShortcut>
          </DropdownMenuItem>
          <DropdownMenuItem>New Team</DropdownMenuItem>
        </DropdownMenuGroup>
        <DropdownMenuSeparator/>
        <DropdownMenuItem>
          Log out
          <DropdownMenuShortcut>⇧⌘Q</DropdownMenuShortcut>
        </DropdownMenuItem>
      </DropdownMenuContent>
    </DropdownMenu>
  </div>
</template>

<script lang="ts">
import {defineComponent} from 'vue'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuGroup,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuShortcut,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu";
import {Avatar, AvatarFallback, AvatarImage} from "@/components/ui/avatar";

export default defineComponent({
  name: 'LayoutHeaderRight',
  components: {
    AvatarFallback,
    AvatarImage,
    Avatar, DropdownMenuShortcut, DropdownMenuGroup, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuLabel, DropdownMenuContent, DropdownMenuTrigger, DropdownMenu
  },
})
</script>
