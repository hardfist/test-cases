const {defineConfig} = require('rolldown');
const path = require('path');
console.log(path.resolve(__dirname,'./fixtures/input1.js'));
module.exports = defineConfig({
   input: {
     rolldown: path.resolve(__dirname,'./fixtures/input1.js')
   },
   
   output: {
    dir: path.resolve(__dirname,'dist'),
    minify:true,
    sourcemap:true
   }
})