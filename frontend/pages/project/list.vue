<script setup lang="ts">
const route = useRoute()
const {data: projects} = await useFetch('http://127.0.0.1:8000/project/all')
const sortIcon = ref("arrow-up");
const sortIconSize = ref("small");
</script>

<template>
  <div>
    <h1>Project List</h1>
    <p class="underline">Current route: {{ route.path }}</p>
    <o-table    
            :data="projects"
            :bordered=true
            :striped=true
            :hoverable=true
            :paginated=true
            :per-page=3
            :pagination-simple=false
            :sort-icon="sortIcon"
            :sort-icon-size="sortIconSize"
        >
        <o-table-column 
                v-slot="projects"
                field="id"
                label="ID"
                width="50">
                {{ projects.row._id.$oid }}
        </o-table-column>
        <o-table-column 
                v-slot="projects"
                field="index"
                label="Index"
                sortable
                width="50">
                {{ projects.row.index }}
        </o-table-column>
        <o-table-column 
                v-slot="projects"
                field="name"
                label="Name"
                width="50">
                {{ projects.row.name }}
        </o-table-column>
    </o-table>
    <pre>{{ projects }}</pre>

  </div>
</template>

