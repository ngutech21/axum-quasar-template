<template>
  <q-page>
    <q-toolbar>
      <q-toolbar-title>Movie Database</q-toolbar-title>
      <!-- add button thats calls a java script funtion-->

      <q-btn flat label="Delete Movies" @click="delete_movies" />
      <q-btn flat label="Import Movies" @click="import_movies" />
    </q-toolbar>
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

function import_movies(evt: Event) {
  if (evt) {
    api.get('/import_movies').then((_) => {
      alert('Movies imported');
    });
  }
}

function delete_movies(evt: Event) {
  if (evt) {
    api.delete('/movies').then((_response) => {
      alert('Movies deleted');
    });
  }
}

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
  {
    name: 'genres',
    align: 'left',
    label: 'Genres',
    field: 'genres',
    sortable: true,

    format: (val, row) => val.join(', '),
  },
] as QTableColumn[];

const $q = useQuasar();
api
  .get('/movies')
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
