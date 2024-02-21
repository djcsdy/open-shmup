"use strict";
const config = require("@softwareventures/webpack-config");
module.exports = config(mode => ({
    title: "open-shmup",
    customize: configuration => ({
        ...configuration,
        experiments: {
            asyncWebAssembly: true
        },
        module: {
            ...configuration.module,
            rules: [
                ...configuration.module.rules,
                {
                    test: /\.openshmup$/iu,
                    use: {
                        loader: require.resolve("file-loader"),
                        options: {
                            esModule: false,
                            name:
                                mode === "development"
                                    ? "[path][name]-[sha256:contenthash:base62:8].[ext]"
                                    : "[sha256:contenthash:base62:8].[ext]"
                        }
                    },
                    type: "javascript/auto"
                }
            ]
        }
    })
}));
