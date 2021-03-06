const withWorkers = require('@zeit/next-workers');
module.exports = withWorkers();

const fs = require('fs');
const config_file = process.env["CONFIG_FILE"];
const config = JSON.parse(fs.readFileSync(config_file, 'utf-8'));

module.exports = {
    publicRuntimeConfig: {
        serverEndpoint: config.serverEndpoint,
        browserEndpoint: config.browserEndpoint,
        imageBaseUrl: config.imageBaseUrl
    }
};