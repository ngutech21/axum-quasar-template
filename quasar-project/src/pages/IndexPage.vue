<template>
  <q-page>
    <div class="q-pa-md">
      <q-table title="Movies" :rows="movies" :columns="columns" row-key="id" />
    </div>
  </q-page>
</template>

<script lang="ts">
import { Movie } from 'components/models';
import { defineComponent, ref } from 'vue';
import { api } from 'boot/axios';
import { QTableColumn } from 'quasar';
import { useQuasar } from 'quasar';

export default defineComponent({
  name: 'IndexPage',
  setup() {
    const movies = ref([] as Movie[]);
    const $q = useQuasar();

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

    return { movies, columns };
  },
});
</script>
