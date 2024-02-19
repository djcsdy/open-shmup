"use strict";
const config = require("@softwareventures/webpack-config");
module.exports = config({
    title: "open-shmup",
    customize: configuration => ({
        ...configuration,
        experiments: {
            asyncWebAssembly: true
        }
    })
});
