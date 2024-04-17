<template>
    <input class="px-3 rounded-md 
    w-full outline-none
    transition ease-in-out
    border border-transparent
    hover:border-blue-400
    focus:border-blue-400" type="text"
    v-model="text"
    @keyup="send">
</template>

<script setup>
import { ref, defineEmits } from 'vue';
// 
const emit = defineEmits(['text']);
let text = ref('');
const send = () => {
    emit('text', text.value);
}
</script>

