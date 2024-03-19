<script setup lang="ts">
const route = useRoute()
</script>

<template>
    <div>
        <h1>Users List</h1>
        <p>Current route: {{ route.path }}</p>
    </div>
</template>
