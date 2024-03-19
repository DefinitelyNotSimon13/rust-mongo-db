<script setup lang="ts">
const route = useRoute()
console.log(route)
</script>

<template>
  <div>
    <h1>Nuxt Routing set up successfully!</h1>
    <p>Current route: {{ route.path }}</p>
        <ul>
            <li><nuxt-link to="/">Home</nuxt-link></li>
            <li><nuxt-link to="/about">About</nuxt-link></li>
            <li><nuxt-link to="/user/list">User list</nuxt-link></li>
        </ul>
  </div>
</template>
