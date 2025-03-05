const path = require('path');
module.exports = {
    entry: {
        main: './fixtures/input1.js',
    },
    output: {
        path: path.resolve(__dirname, 'dist/rspack'),
    }
}