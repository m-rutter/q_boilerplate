const path = require("path");
const PKG = require("../package.json");

const PATHS = {
    entry: path.resolve(__dirname, "../src/index.ts"),
    output: path.resolve(__dirname, "../dist"),
    tsConfig: path.resolve(__dirname, "tsconfig.json")
};

module.exports = {
    entry: PATHS.entry,
    output: {
        path: PATHS.output,
        library: PKG.name,
        libraryTarget: "umd",
        filename: PKG.name + ".js"
    },
    resolve: {
        extensions: [".ts", ".tsx", ".js"]
    },
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                loader: "ts-loader",
                exclude: /node_modules/,
                options: {
                    configFile: PATHS.tsConfig
                }
            },
            {
                test: /\.css$/,
                use: [{ loader: "style-loader" }, { loader: "css-loader" }]
            }
        ]
    },
    externals: {
        qlik: "qlik"
    },
    performance: {
        hints: false
    }
};
