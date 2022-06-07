# Installation

## Install Transformation Plugin

- If you're already using Babel, you can install [babel plugin](#install-babel-plugin).
- If you're already using Rollup, Vite, or Webpack, you can install [unplugin](#install-unplugin).

### Install Babel Plugin

[![NPM version](https://img.shields.io/npm/v/@import-meta-env/babel.svg)](https://www.npmjs.com/package/@import-meta-env/babel)

::: warning
This plugin is not compatible with the Vite, you should use the [Unplugin](#install-unplugin) instead.
:::

Install it with your favorite package manager:

```bash
npm install @import-meta-env/babel --save-dev
yarn add @import-meta-env/babel --dev
pnpm add -D @import-meta-env/babel
```

Register the plugin:

```json
// babel.config.json
{
  "plugins": [["module:@import-meta-env/babel", { "example": ".env.example" }]]
}
```

Related examples: [babel](https://github.com/iendeavor/import-meta-env/blob/main/packages/examples/babel-starter-example), [babel-loader](https://github.com/iendeavor/import-meta-env/blob/main/packages/examples/webpack-babel-loader-example), [jest](https://github.com/iendeavor/import-meta-env/blob/main/packages/examples/jest-example), [rollup-plugin-babel](https://github.com/iendeavor/import-meta-env/blob/main/packages/examples/rollup-plugin-babel-example)

### Install Unplugin

[![NPM version](https://img.shields.io/npm/v/@import-meta-env/unplugin.svg)](https://www.npmjs.com/package/@import-meta-env/unplugin)

Install it with your favorite package manager:

```bash
npm install @import-meta-env/unplugin --save-dev
yarn add @import-meta-env/unplugin --dev
pnpm add -D @import-meta-env/unplugin
```

Register the plugin:

Rollup:

```js
// rollup.config.js
import ImportMetaEnvPlugin from "@import-meta-env/unplugin";

export default {
  plugins: [
    ImportMetaEnvPlugin.rollup({
      example: ".env.example",
    }),
  ],
};
```

Vite:

```ts
// vite.config.ts
import ImportMetaEnvPlugin from "@import-meta-env/unplugin";

export default {
  plugins: [
    ImportMetaEnvPlugin.vite({
      example: ".env.example",
    }),
  ],
};
```

Webpack:

```js
// webpack.config.js
module.exports = {
  plugins: [
    require("@import-meta-env/unplugin").webpack({
      example: ".env.example",
    }),
  ],
};
```

Related examples: [rollup](https://github.com/iendeavor/import-meta-env/blob/main/packages/examples/rollup-starter-example), [vite](https://github.com/iendeavor/import-meta-env/blob/main/packages/examples/vite-starter-example), [webpack](https://github.com/iendeavor/import-meta-env/blob/main/packages/examples/webpack-starter-example)

## Install CLI

[![NPM version](https://img.shields.io/npm/v/@import-meta-env/cli.svg)](https://www.npmjs.com/package/@import-meta-env/cli)

Install it with your favorite package manager:

```bash
npm install @import-meta-env/cli --save-dev
yarn add @import-meta-env/cli --dev
pnpm add -D @import-meta-env/cli
```
