<script>
import {saveAs} from 'file-saver';
import {decryption_mlm} from "../../pkg";
import initSync from "../../pkg/decryption_mlm_wasm";

export default {
  data() {
    return {
      files: [],
      dropzoneActive: false
    }
  },
  methods: {
    /**
     * 处理拖拽区域的拖入事件
     * @param {Event} event - 拖入事件对象
     */
    dragEnter(event) {
      event.preventDefault();
      this.dropzoneActive = true;
    },
    /**
     * 处理拖拽区域的拖离事件
     * @param {Event} event - 拖离事件对象
     */
    dragLeave(event) {
      event.preventDefault();
      this.dropzoneActive = false;
    },
    /**
     * 处理拖拽区域的拖放事件
     * @param {Event} event - 拖放事件对象
     */
    dragOver(event) {
      event.preventDefault();
    },
    /**
     * 处理文件的拖放上传
     * @param {Event} event - 拖放事件对象
     */
    dropFile(event) {
      event.preventDefault();
      this.dropzoneActive = false;
      const files = event.dataTransfer.files;
      this.handleFiles(files);
      console.log(files);
    },
    /**
     * 处理上传的文件列表
     * @param {FileList} files - 文件列表
     */
    handleFiles(files) {
      for (let i = 0; i < files.length; i++) {
        const file = files[i];
        this.files.push(file);
        // 在这里可以执行文件上传的相关操作

      }
    },
    /**
     * 删除文件
     * @param file
     */
    removeFile(file){
      const i = this.files.indexOf(file);
      if (i > -1) {
        this.files.splice(i, 1);
      }
    },
    /**
     * 下载所有文件
     */
    async downloadFile() {
      for (let i = 0; i < this.files.length; i++) {
        await this.decryptFile(this.files[i]);
      }
    },
    /**
     * 解密文件并下载
     * @param {File} file - 文件对象
     */
    async decryptFile(file) {
      if (file.name.includes(".xml")) {
        const reader = new FileReader();
        reader.readAsText(file);
        await new Promise(resolve => reader.onload = () => resolve());
        let convertedString = reader.result;
        if (typeof (convertedString) === 'string') {
          await initSync();
          let blob = new Blob([decryption_mlm(convertedString)], {type: "text/xml;charset=utf-8"});
          await saveAs(blob, file.name + "-解密.xml");
        }
      }
    }
  }
}

</script>
<template>
  <div id="app">
    <div class="dropzone" @dragenter="dragEnter" @dragleave="dragLeave" @dragover="dragOver" @drop="dropFile">
      将文件拖放到此处
    </div>
    <transition-group tag="ul" class="container">
      <div v-for="file in files" :key="file.name" class="file">
        {{ file.name }}
        <button @click="removeFile(file)" class="b2">×</button>
      </div>
    </transition-group>
    <button @click="downloadFile" class="b1">解密全部</button>
  </div>
</template>

<style>
#app {
  display: flex;
  flex-wrap: wrap;
  flex-direction: column;
  align-items: center;
  justify-content: center;
}

.dropzone {
  width: 500px;
  height: 400px;
  border: dashed #ccc;
  text-align: center;
  font-size: 36px;
  color: #999;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  left: 5px;
}

.file {
  width: 110%;
  height: 30px;
  text-align: center;
  background-color: #f3f3f3;
  border: 1px solid #666;
  box-sizing: border-box;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  left: -20px;
  top: 10px;
}

.b1 {
  font-size: 20px;
  position: relative;
  top: 20px;
}

.b2 {
  font-size: 14px;
  position: relative;
  left: 5px;
}
</style>