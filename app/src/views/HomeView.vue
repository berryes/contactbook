<!-- Ennel szebb kodot soha senki nem fog irni-->
<template>
  <div class="noDB">
    <!--- Displays if user doesn't have DB cookie -->
    <div v-if="noDB">
      <h3>Upload database or create a new one</h3>
      <input type="file">
      <button v-on:click="createDB">Create a new one</button>
    </div>

    <!-- Displays if user HAS db cookie-->
    <div v-else>
      <br>
      <!--- Search-->
      <input type="text" placeholder="Search" v-on:change="filterPeople">

      <!--- Adding a contact-->
      <button v-on:click="addingPerson=true">Add person</button>

      <button v-on:click="downloadDB">Download database</button>
      
      <!-- Listing people (table) -->
      <a-table :dataSource="book.data.people" :columns="columns"  v-if="showTable"/>

      <!--- Form drawer (used for adding a person)-->
      <a-drawer
      title="Add new person"
      :width="720"
      :visible="addingPerson"
      :body-style="{ paddingBottom: '80px' }"
      :footer-style="{ textAlign: 'right' }"
      @close ="addingPerson=false"
      :row-selection="{ selectedRowKeys: selectedRowKeys, onChange: onSelectChange }"
      >

    <!--- Start of  | Add person form-->
    <a-form
      layout="inline"
      :model="formData"
      @finish="handleFinish"
      @finishFailed="handleFinishFailed"
      >

      <a-form-item  label="First name">
          <a-input v-model:value="formData.firstName" />
        </a-form-item>

      <a-form-item  label="Last name">
          <a-input v-model:value="formData.lastName" />
        </a-form-item>

      <a-form-item  label="Birthday">
        <!--- Show years first instead of days-->
        <a-date-picker v-model:value="formData.birth" />
        </a-form-item>
        <br>

        <br>
      <a-space
        v-for="(data, index) in formData.custom"
        :key="data.id"
        style=" margin-bottom: 8px"
        align="baseline"
        >
        <br>
      <!-- Custom fields-->
      <!---Key of custom field -->
      <a-form-item>
        
                                          <!--  ˇˇˇˇ changes value and key of the array item by index -->
          <a-input v-model:value="formData.custom[index].key" placeholder="Name of custom field" />
        </a-form-item>
        <a-form-item
          :name="['users', index, 'last']"
        >
    
        <!--- Value of custom field -->
          <a-input v-model:value="formData.custom[index].value" placeholder="Value" />
        </a-form-item>
        <MinusCircleOutlined @click="removeNewCustomObject(index)" />
       </a-space>


      <a-form-item>
        <a-button type="dashed" block @click="addNewCustomObject">
          <PlusOutlined />
          Add new custom field
        </a-button>
      </a-form-item>


    <!---End of  | Add person form-->
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
// fuck linters 
// i write MY code how I WANT!

import { MinusCircleOutlined } from '@ant-design/icons-vue'; // minus icon
import { Book } from '@/assets/Controllers/book' // book controller
import VueCookies from 'vue-cookies' // cookie manager
import {  reactive } from 'vue' // reactive for making functions well... ractive?
export default {
  name: 'IndexPage',

  components: {
    MinusCircleOutlined, // just an icon
  },

  // RAM, or just data that can be accesed through-out the program
  data(){
    return {
      book: null, // the contact book by its self, becomes an object with data inside it
      noDB: true, // if a cookie is found DB this will become true
      addingPerson: false, // toggles the drawer
      showTable: true, // toggles the table
      selectedRow: [], // id of selected rows in the table 
      
      /// i fucking hate this formatting but this is the most readable way ˇˇˇˇˇˇ
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

  // setup, this is necesarry cus of the life-cycle of the Antdesign vue components (table & drawer)
  setup(){
    const tableState = reactive({
      selectedRowKeys: [],
    })
    const onSelectChange = selectedRowKeys => {
      tableState.selectedRowKeys = selectedRowKeys;
    };
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
      formData,
      tableState,
      onSelectChange
    } 
  },
  methods:{
    cacheData(){VueCookies.set("ctb_cache_data",JSON.stringify(this.book)) }, // caching stuff in cookies so when user closes data remains
    createDB(){ // creating "database"
      this.noDB = false
      this.book = new Book({version: 1, created: Date.now(),hostdata: "asd"})
      this.cacheData() // caching to cookie
    },
    downloadDB(){
/*       
      var blob1 = new Blob(JSON.stringify(this.book), { type: "text/plain;charset=utf-8" });
      const url = window.URL || window.webkitURL;
            const link = url.createObjectURL(blob1);
            var a = document.createElement("a");
            a.download = "Customers.txt";
            a.href = link;
            document.body.appendChild(a);
            a.click();
            document.body.removeChild(a); */
      
            /// wtf is this piece of shit
    },
    addPerson(){
      this.showTable = false // disabling table from DOM
      this.book.addPerson(this.formData) // adding person
      this.cacheData() // caching to cookie
      this.showTable = true // enablib table, (a retarded but only functionin way to rerender the component)
    },
    filterPeople(){
      return
    },

    addNewCustomObject(){ // Adding a custom value field when addig a person | (basically just adds an object to a list)
      this.formData.custom.push({
        key: '',
        value: '',
        id: Date.now()
      })
    },

    removeNewCustomObject(index){ // same shit as above ^^^ jsut the other way around
      this.formData.custom.splice(index,1)
    }
  },

  beforeMount(){ // before the app is mounted / view rendered

    // in case user has data DB in the cookie
    let cookie = VueCookies.get("ctb_cache_data")
    if(!cookie) return;  
    // if not then just create one
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