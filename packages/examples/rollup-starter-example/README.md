# Setup

1. Install package:

   ```sh
   $ pnpm i -D @import-meta-env/cli
   $ pnpm i -D @import-meta-env/unplugin
   ```

1. Register `import-meta-env` plugin:

   ```js
   // rollup.config.js
   import importMetaEnv from "@import-meta-env/unplugin";

   const dev = {
     plugins: [
       // ...,
       importMetaEnv.rollup({ example: ".env.example.public" }),
     ],
   };

   const prod = {
     plugins: [
       // ...,
       importMetaEnv.rollup({ example: ".env.example.public" }),
     ],
   };

   // ...
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
   $ pnpm run dev
   ```

1. Build production:

   ```sh
   $ pnpm run build
   ```

1. Preview production:

   ```sh
   $ pnpm run preview
   ```
