---
home: true
heroText: Import-meta-env
tagline: Populates your environment variables at run-time rather than build-time.
actionText: Get Started
actionLink: /guide
features:
  - title: Strict
    details: Code has no access to arbitrary environment variables by default.
  - title: Cross-platform
    details: The CLI is developed using Rust. You can populate environment variables anywhere, such as the Alpine Linux nginx image.
  - title: Learn Once, Write Anywhere
    details: Vite, Webpack and Rollup, CSR, SSR and SSG, Jest and Mocha, etc. are all supported.
footer: MIT Licensed | Copyright © 2021-present Ernest
---

<p style="text-align: center;"><b>Write Your Code:</b></p>

```js
// src/index.js
console.log(import.meta.env.HELLO);
```

<p style="text-align: center;"><b>Bundle It:</b></p>

```js
// dist/index.js
console.log("__import_meta_env_placeholder__".HELLO);
```

<p style="text-align: center;"><b>Run the <code>import-meta-env</code> command to populate it</b>:</p>

```js
// dist/index.js
console.log({ HELLO: "there" }.HELLO);
```
