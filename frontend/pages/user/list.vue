<script setup lang="ts">
const route = useRoute()
const {data: users} = await useFetch('http://127.0.0.1:8000/user/all')

</script>

<template>
  <div>
    <h1>Users List</h1>
    <p>Current route: {{ route.path }}</p>
    <pre>{{ users }}</pre>
  </div>
</template>
