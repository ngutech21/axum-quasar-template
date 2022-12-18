<template>
  <q-page>
    <div class="q-pa-md">
      <q-table title="Movies" :rows="movies" :columns="columns" row-key="id" />
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { Movie } from 'components/models';
import { api } from 'boot/axios';
import { QTableColumn } from 'quasar';
import { useQuasar } from 'quasar';
import { ref } from 'vue';

const movies = ref([] as Movie[]);

const columns = [
  { name: 'id', align: 'left', label: 'Id', field: 'id', sortable: true },
  {
    name: 'title',
    align: 'left',
    label: 'Title',
    field: 'title',
    sortable: true,
  },
] as QTableColumn[];

const $q = useQuasar();
api
  .get('/api/v1/movies')
  .then((response) => {
    movies.value = response.data;
  })
  .catch(() => {
    $q.notify({
      color: 'negative',
      position: 'top',
      message: 'Loading movies failed',
      icon: 'report_problem',
    });
    console.log('Loading failed');
  });
</script>
