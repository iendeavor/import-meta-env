import resolve from "@rollup/plugin-node-resolve";
import { babel } from "@rollup/plugin-babel";

const dev = {
  input: "src/main.js",
  output: {
    dir: "public/assets",
    format: "esm",
  },
  plugins: [
    resolve(),
    babel({
      babelHelpers: "bundled",
      plugins: [
        ["module:@import-meta-env/babel", { example: ".env.example.public" }],
      ],
    }),
  ],
};

const prod = {
  input: "src/main.js",
  output: {
    dir: "dist/assets",
    format: "esm",

    // Make output files easier to diff.
    chunkFileNames: `[name].js`,
    entryFileNames: `[name].js`,
  },
  plugins: [
    resolve(),
    babel({
      babelHelpers: "bundled",
      plugins: [
        [
          "module:@import-meta-env/babel",
          { example: ".env.example.public", shouldInlineEnv: false },
        ],
      ],
    }),
  ],
};

export default process.env.ROLLUP_WATCH === "true" ? dev : prod;
