const path = require('path');
module.exports = {
    entry: {
        rspack: './fixtures/input1.js',
    },
    output: {
        path: path.resolve(__dirname, 'dist'),
    }
}