const fs = require("fs");
const path = require("path");
const webpack = require("webpack");
const archiver = require("archiver");
const PKG = require("../package.json");

const compilerConifg = require("../config/webpack.config");

compilerConifg.mode = "production";

const compiler = webpack(compilerConifg);

var archive = archiver("zip", {
    zlib: { level: 9 }
});

const outputPath = path.resolve(__dirname, "../deploy");
const outputFile = path.resolve(outputPath, PKG.name + ".zip");
const projectDirectory = path.resolve(__dirname, "../");

if (!fs.existsSync(outputPath)) {
    fs.mkdirSync(outputPath);
}

if (fs.existsSync(outputFile)) {
    fs.unlinkSync(outputFile);
}

const compilePromise = new Promise((resolve, reject) => {
    compiler.run((err, stats) => {
        if (err) {
            reject(err);
        } else {
            resolve(stats);
        }
    });
});

compilePromise.then(stats => {
    console.log(stats.toString({ colors: true }));

    const output = fs.createWriteStream(outputFile);

    archive.pipe(output);

    archive.glob("**/*", {
        cwd: projectDirectory,
        ignore: [
            "node_modules/**/*",
            "coverage/**/*",
            "deploy/**/*",
            "node_modules",
            "deploy",
            "coverage"
        ]
    });
    console.log("Packing extension for deployment");

    archive.finalize();

    console.log(`Extension has been built and can be found at: ${outputFile}`);
});
