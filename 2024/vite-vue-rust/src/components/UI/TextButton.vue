<template>
    <Button class="
    bg-transparent
    hover:bg-blue-500
    focus:bg-blue-500
    active:bg-blue-700">
        <slot></slot>
    </Button>
</template>

<script setup>
import Button from './Button.vue';
</script>