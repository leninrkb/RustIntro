<template>
    scope
    here the component SCOPE should render both homeview and resultview
    holding their states
    <hr>
    <TextButton @click="cambiar">intercambiar</TextButton>
    <hr>
    <KeepAlive>
        <component :is="current_view"></component>
    </KeepAlive>
</template>

<script setup>
import TextButton from '../components/UI/TextButton.vue';
import { ref, shallowRef } from 'vue'; 
import HomeView from './HomeView.vue';
import ResultView from './ResultView.vue';
// 
let current_view = shallowRef(HomeView);
const cambiar = () => {
    current_view.value = current_view.value == HomeView ? ResultView : HomeView;
}
</script>