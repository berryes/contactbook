<template>
  <div class="home">
    <h3>Upload database or create a new one</h3>
    <input type="file">
    <button v-on:click="createDB">Create a new one</button>
  </div>
</template>

<script>
// @ is an alias to /src
import * as sqlite3 from 'sqlite3';

export default {
  name: 'HomeView',
  data(){
    return {
      data: "asd",
    }
  },
  methods:{
    createDB(){
      const db = new sqlite3.default.Database(':memory:');
      console.log(db)
    }
  }
}
</script>
<style scoped>
.home { 
  margin-top: 20em;
}
</style>