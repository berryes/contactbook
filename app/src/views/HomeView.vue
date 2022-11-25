<template>
  <div class="noDB">
    <div v-if="noDB">
      <h3>Upload database or create a new one</h3>
      <input type="file">
      <button v-on:click="createDB">Create a new one</button>
    </div>
    <div v-else>
      db xd
      <br>
      
      <input type="text" placeholder="Search" v-on:change="filterPeople">
      <button v-on:click="addingPerson=true">Add person</button>
      
      <!-- Listing people -->
      <a-table :dataSource="book.data.people" :columns="columns"  v-if="showTable"/>


      <a-drawer
    title="Add new person"
    :width="720"
    :visible="addingPerson"
    :body-style="{ paddingBottom: '80px' }"
    :footer-style="{ textAlign: 'right' }"
    @close ="addingPerson=false"
  >
  <a-form
    layout="inline"
    :model="formData"
    @finish="handleFinish"
    @finishFailed="handleFinishFailed"
  >

  <a-form-item  label="First name" :rules="[{ required: true }]">
      <a-input v-model:value="formData.firstName" />
    </a-form-item>

  <a-form-item  label="Last name" :rules="[{ required: true }]">
      <a-input v-model:value="formData.lastName" />
    </a-form-item>

  <a-form-item  label="Birthday">
    <!--- Show years first instead of days-->
    <a-date-picker v-model:value="formData.birth" />
    </a-form-item>
    <br>

    <a-space
      v-for="(data, index) in formData.custom"
      :key="data.id"
      style=" margin-bottom: 8px"
      align="baseline"
    >
    <a-form-item>
        <a-input v-model:value="formData.custom[index].key" placeholder="Name of custoom field" />
      </a-form-item>
      <a-form-item
        :name="['users', index, 'last']"
      >
        <a-input v-model:value="formData.custom[index].value" placeholder="Data" />
      </a-form-item>
      <MinusCircleOutlined @click="removeNewCustomObject(index)" />
    </a-space>

    <a-form-item>
      <a-button type="dashed" block @click="addNewCustomObject">
        <PlusOutlined />
        Add new custom field
      </a-button>
    </a-form-item>


    <!---End of fomr-->
  </a-form>
  

  <template #extra>
      <a-space>
        <a-button @click="onClose">Cancel</a-button>
        <a-button type="primary" @click="addPerson">Submit</a-button>
      </a-space>
    </template>
      </a-drawer>
    </div>
  </div>
</template>

<script>
import { MinusCircleOutlined } from '@ant-design/icons-vue';
import { Book } from '@/assets/Controllers/book'
import VueCookies from 'vue-cookies'
import { reactive } from 'vue'
export default {
  name: 'HomeView',
  components: {
    MinusCircleOutlined,
  },
  data(){
    return {
      progver: 1,
      book: null,
      noDB: true,
      addingPerson: false,
      showTable: true,
      columns: [
          {
            title: 'First name',
            dataIndex: 'firstName',
            key: 'firstName',
          },
          {
            title: 'Last name',
            dataIndex: 'lastName',
            key: 'lastName',
          },
          {
            title: 'Nickname',
            dataIndex: 'nickname',
            key: 'nickname',
          },
          {
            title: 'Birth Day',
            dataIndex: 'birth',
            key: 'birth',
          },
          {
            title: 'Email',
            dataIndex: 'email',
            key: 'email',
          },
          {
            title: 'Address',
            dataIndex: 'address',
            key: 'address',
          },
          {
            title: 'Website',
            dataIndex: 'website',
            key: 'website',
          },
        ],
    }
  },
  setup(){
    const formData = reactive({
      firstName: '',
      lastName: '',
      cellPhone: null,
      birth: '',
      homePhone: null,
      email: null,
      notes: null,
      adress: null,
      website: null,
      custom: [],
    })
    return{
      formData
    }
  },
  methods:{
    cacheData(){VueCookies.set("ctb_cache_data",JSON.stringify(this.book)) }, // caching stuff in cookies so when user closes data remains
    createDB(){ // creating "database"
      this.noDB = false
      this.book = new Book({version: 1, created: Date.now(),hostdata: "asd"})
      this.cacheData() // caching to cookie
    },
    addPerson(){
      this.showTable = false
      this.book.addPerson(this.formData)
      this.cacheData() // caching to cookie
      this.showTable = true
    },
    filterPeople(){
      return
    },

    addNewCustomObject(){
      this.formData.custom.push({
        key: '',
        value: '',
        id: Date.now()
      })
    },
    removeNewCustomObject(index){
      this.formData.custom.splice(index,1)
    }
  },

  beforeMount(){
    // in case data window was closed
    let cookie = VueCookies.get("ctb_cache_data")
    if(!cookie) return; 
    this.noDB = false;
    this.book = new Book(cookie)
  }
}
</script>
<style scoped>
.home { 
  margin-top: 20em;
}
</style>