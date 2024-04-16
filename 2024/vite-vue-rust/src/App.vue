<template>
  <RowCenter>
    <h1 class="text-2xl text-teal-500">Vite + Vue + Rust</h1>
  </RowCenter>
  <RowCenter>
    <Button class="w-fit" @click="confirmar">
      {{btn_texto}}
    </Button>
  </RowCenter>
  <RowCenter>
    <p v-if="toggle" class="text-sm">
      Confirmen un smite en la noche!
    </p>
  </RowCenter>
</template>


<script setup>
import RowCenter from './components/layout/RowCenter.vue';
import Button from './components/UI/Button.vue';
import { ref, onMounted } from 'vue';

let btn_texto = ref('');
let toggle = ref(true);

let confirmar = () => {
  btn_texto.value = toggle.value ? 'avisar' : 'confirmar';
  toggle.value = !toggle.value;
}

onMounted(() => {
  confirmar();
});

</script>


