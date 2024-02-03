<template>
  <div>
    <Card dis-hover>
      <template #title>
        Connector
      </template>
      <template #extra>
        <Tooltip transfer placement="left" content="Add Connector">
          <Button icon="md-add" shape="circle" size="small" type="primary">
          </Button>
        </Tooltip>
      </template>
      <Table :columns="columns" :data="items">
      </Table>
    </Card>
  </div>
</template>

<script lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {createHeaders} from "@/views/connector/Utils.ts";

export default {
  name: 'ConnectorHomeView',
  data()
  {
    return {
      columns: createHeaders(),
      items: []
    }
  },
  created()
  {
    this.handlerInitialize()
  },
  methods: {
    handlerInitialize()
    {
      invoke('get_connector').then((value) => {
        console.log(value)
      })
    }
  }
}
</script>
