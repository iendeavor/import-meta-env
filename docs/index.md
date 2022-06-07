# Import-meta-env

**Note: You can also follow [iendeavor/runtime-environment-variables-example](https://github.com/iendeavor/runtime-environment-variables-example) to implement a runtime environment variable solution without using this library.**

Populates your environment variables after build-time.

[![License - MIT](https://img.shields.io/github/license/iendeavor/import-meta-env?color=blue&label=License)](https://github.com/iendeavor/import-meta-env/blob/main/LICENSE)

[![SemVer version](https://img.shields.io/badge/Sem%20Ver-2.0.0-black.svg)](https://semver.org/)

[![CI](https://github.com/iendeavor/import-meta-env/actions/workflows/ci.yml/badge.svg)](https://github.com/iendeavor/import-meta-env/actions/workflows/ci.yml)

## How it Works

This plugin exposes environment variables on a special `import.meta.env`[<sup>?</sup>](/guide/faq.html#why-use-importmeta) object:

```js
// src/index.js
console.log(import.meta.env.API_BASE_URL);
```

During bundle step (for example, running Webpack in Github Actions), the code will be temporarily replaced with a placeholder:

```js
// dist/index.js
console.log("__import_meta_env_placeholder__".API_BASE_URL);
```

You can then run the [CLI](/guide/getting-started/installation.html#install-cli) anywhere to populating the bundle files with environment variables _without rebuilding your application_.

For example, [`docker run --env API_BASE_URL=https://httpbin.org ...`](https://docs.docker.com/engine/reference/commandline/run/#set-environment-variables--e---env---env-file):

```js
// dist/index.js
console.log("https://httpbin.org");
```
