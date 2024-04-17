<template>
    <RowCenter class="gap-2">
        <TextField @text="bind_text"></TextField>
        <OutlinedButton @click="go" class="px-10">GO</OutlinedButton>
    </RowCenter>
</template>

<script setup>
import { ref } from 'vue';
import OutlinedButton from './UI/OutlinedButton.vue';
import RowCenter from './layout/RowCenter.vue';
import TextField from './UI/TextField.vue';
// 
let text = ref('');
const bind_text = (new_text) => {
    text.value = new_text;
}
const go = () => {
    console.log(text.value);
}
</script>