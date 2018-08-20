'use strict';

const dotEnv = require('dotenv');
const parsedEnv = dotEnv.config().parsed;

module.exports = function () {
    // Let's stringify our variable
    for (let key in parsedEnv) {
        if (typeof parsedEnv[key] !== 'string') {
            parsedEnv[key] = JSON.stringify(parsedEnv[key])
        }
    }
    return parsedEnv
}