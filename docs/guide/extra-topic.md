# Extra Topics

## Local Development

In local development, for convenience, you can create a `.env.defaults` file in the project instead of manipulating environment variables in the system:

```ini
# Import-meta-env will only load `S3_BUCKET`'s value if you only defined it in the `.env.example` file.
S3_BUCKET="YOUR_S3_BUCKET"
SECRET_KEY="YOUR_SECRET_KEY_GOES_HERE"
```

## Sensitive Environment Variables

You may want to define all necessary environment variables in `.env.example` (i.e. including credentials), in this case, you can creating two example files, and pass the `.env.example.public` file to `import-meta-env` options:

```ini
# .env.example
S3_BUCKET=
SECRET_KEY=
```

```ini
# .env.example.public
S3_BUCKET=
```

If you need to access the sensitive environment variables, you should use `process.env` instead of `import.meta.env`, this is because sensitive environment variables should only be accessible on the server side, it can also help you identify the sensitive environment variables:

```js
const S3_BUCKET = import.meta.env.S3_BUCKET;
const SECRET_KEY = process.env.SECRET_KEY;
```

To populate sensitive environment variables, you can use [webpack.EnvironmentPlugin](https://webpack.js.org/plugins/environment-plugin/) or similar:

```js
new webpack.EnvironmentPlugin(["SECRET_KEY"]);
```

```js
console.log(process.env.SECRET_KEY); // "YOUR_SECRET_KEY_GOES_HERE"
```

If you need to populate the sensitive environment variables at run-time, you need to find out another way to do it, for example:

1. For [NEXT.js](https://nextjs.org/), you can use [serverRuntimeConfig](https://nextjs.org/docs/api-reference/next.config.js/runtime-configuration).
2. For [NuxtJS](https://nuxtjs.org/), you can use [privateRuntimeConfig](https://nuxtjs.org/docs/configuration-glossary/configuration-runtime-config).

## IntelliSense for TypeScript

You may want to get TypeScript IntelliSense for user-defined environment variables.

To achieve, you can create an `env.d.ts`, then define `ImportMeta` like this:

```ts
// src/env.d.ts
interface ImportMeta {
  readonly env: {
    readonly S3_BUCKET: string;
  };
}
```
