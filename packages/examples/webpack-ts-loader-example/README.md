# Setup

1. Install package:

   ```sh
   $ pnpm i -D @import-meta-env/unplugin
   $ pnpm i -D @import-meta-env/cli
   ```

1. Register `import-meta-env` plugin:

   ```js
   // webpack.config.js

   const importMetaEnv = require("@import-meta-env/unplugin");

   module.exports = {
     // ...
     plugins: [
       // ...
       importMetaEnv.webpack({ example: ".env.example.public" }),
     ],
   };
   ```

1. List public environment variables under `.env.example.public`.

   ```
   # .env.example.public
   HELLO=
   ```

1. Set environment variables:

   ```sh
   $ export HELLO=import-meta-env
   ```

1. Start dev server:

   ```sh
   $ pnpm exec webpack --watch
   ```

   ```sh
   $ serve dist -p 3000
   ```

1. Build production:

   ```sh
   $ pnpm exec webpack
   ```

1. Serve production:

   ```sh
   $ pnpm exec import-meta-env --example .env.example.public
   $ pnpm exec serve dist
   ```