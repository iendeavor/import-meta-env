# vite-plugin-dotenv

[![NPM version](https://img.shields.io/npm/v/vite-plugin-dotenv.svg)](https://www.npmjs.com/package/vite-plugin-dotenv)
[![PRs Welcome](https://img.shields.io/badge/PRs-Welcome-brightgreen.svg?style=flat-square)](http://makeapullrequest.com)

Inject your environment variables from the .env file at runtime instead of build time.

In development, this package just injects the vite environment variables as-is.

In production, this package will generate some files in your dist assets directory (see below) that allow us to inject the environment variables _after building the package_.

This project use [SemVer](https://semver.org/) for versioning. For the versions available, see the tags on this repository.

⚠️ **DO NOT** add secret environment to `<package-root>/dist/assets/.env`, the [shell script](https://github.com/iendeavor/vite-plugin-dotenv/tree/main/packages/vite-plugin-dotenv#:~:text=%3Cpackage%2Droot%3E/dist/assets/dotenv.sh%20is%20a%20shell%20script%20that%20injects%20%3Cpackage%2Droot%3E/dist/assets/.env%20into%20%3Cpackage%2Droot%3E/dist/assets/.env.js.) will inject everything from it into `<package-root>/dist/assets/.env.js`.

## 🚀 Quick Start

Install and register the plugin:

```sh
pnpm i vite-plugin-dotenv
```

```ts
import { defineConfig } from "vite";
import dotenv from "vite-plugin-dotenv";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [dotenv()],
});
```

Say you want to say hi on your website, you can do this:

`.env`:

```
VITE_NAME=World

```

`main.ts`:

```ts
// @ts-ignore: If you are using typescript ignore next line to prevent ts(2307) error.
import env from ".env"; // This is a virtual file automatically generated by this plugin

console.log(`Hollo ${env.VITE_NAME}`); // Hello World
```

After building the package, you will see something like the following in the terminal:

```diff
$ pnpm exec vite build

vite v2.7.12 building for production...
✓ 5 modules transformed.
dist/assets/index-legacy.87eadd25.js       0.47 KiB / gzip: 0.35 KiB
dist/assets/.env-legacy.js                 0.16 KiB / gzip: 0.16 KiB
dist/assets/polyfills-legacy.03bd3439.js   41.71 KiB / gzip: 17.48 KiB
dist/assets/favicon.17e50649.svg   1.49 KiB
dist/index.html                    1.57 KiB
dist/assets/index.a5010ad2.js      0.86 KiB / gzip: 0.50 KiB
dist/assets/.env.js                0.10 KiB / gzip: 0.12 KiB
dist/assets/index.06d14ce2.css     0.17 KiB / gzip: 0.14 KiB

+ ✓ [vite-plugin-dotenv] is generated.
+ Before deploying the project, replace __DOTENV__ with your environment object in the following files:
+ dist/assets/.env-legacy.js
+ dist/assets/.env.js
+ dist/assets/dotenv.sh
+ dist/assets/.env
```

- `<package-root>/dist/assets/.env` is cloned from `<package-root>/.env` as is.

- `<package-root>/dist/assets/.env.js` contains a placeholder: `__DOTENV__`, which allows us to inject environment variables.

- `<package-root>/dist/assets/dotenv.sh` is a shell script that injects `<package-root>/dist/assets/.env` into `<package-root>/dist/assets/.env.js`.

Before serving your website, you need to inject environment variables, you can do this:

```sh
$ ./dist/assets/dotenv.sh && pnpm exec vite build
```

If you run into problems, see [example](../examples/vite-vanilla-ts) or create an issue from github.

## 🤝 Contributing

Please read [CONTRIBUTING.md](./CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull
requests to us.

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details