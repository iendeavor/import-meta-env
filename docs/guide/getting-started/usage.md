# Usage

Suppose you have the following environment variables:

```bash
export S3_BUCKET=YOUR_S3_BUCKET
```

## Accessing Environment Variables

You can access the environment variables in code like:

```js
console.log(import.meta.env.S3_BUCKET);
```

## Development

In development, `import.meta.env` will simply be replaced with environment variables.

```js
console.log("YOUR_S3_BUCKET");
```

## Production

During production, `import.meta.env` will be temporarily replaced with a placeholder string.

```js
console.log('__import_meta_env_placeholder__'.S3_BUCKET));
```

Therefore, before serving your production build, you need to run the CLI to populate the environment variables:

```bash
./node_modules/.bin/import-meta-env --example .env.example
```

... and it will output:

```js
console.log("YOUR_S3_BUCKET");
```

Since your application may be deployed on a system that doesn't have Node.js installed, you can use [pkg](https://github.com/vercel/pkg) to package the `import-meta-env` script into a standalone binary.

For example, you may host your application on the [Alpine Linux nginx image](https://hub.docker.com/_/nginx)):

```bash
npx pkg ./node_modules/@import-meta-env/cli/bin/import-meta-env.js \
  --target node16-alpine \
  --output import-meta-env-alpine
```

and use it like above to populate environment variables:

```bash
./import-meta-env-alpine --example .env.example
```

Related examples: [docker](https://github.com/iendeavor/import-meta-env/blob/main/packages/examples/docker-starter-example)

::: warning
**Import-meta-env** will also replace all `import.meta.env` appearing in JavaScript strings, so you may see errors like:

```js
console.log("import.meta.env.S3_BUCKET is:", import.meta.env.S3_BUCKET);
```

will be transformed into:

```js
console.log(
  ""__import_meta_env_placeholder__".S3_BUCKET is:",
// ^ SyntaxError: missing ) after argument list
  "__import_meta_env_placeholder__".S3_BUCKET
);
```

To avoid this, you can break the string up with a unicode zero-width space, e.g.:

```js
console.log("import.meta\u200b.env.S3_BUCKET is:", import.meta.env.S3_BUCKET);
```

output:

```js
console.log(
  "import.meta\u200b.env.S3_BUCKET is:",
  "__import_meta_env_placeholder__".S3_BUCKET
);
```

:::

::: info
By default, **import-meta-env** will automatically determine the mode (development or production) and replace `import.meta.env` with environment variables or placeholders, respectively.

You can override this by setting the `shouldInlineEnv` option for the [babel plugin](/guide/getting-started/installation.html#install-babel-plugin) and the [unplugin](/guide/getting-started/installation.html#install-unplugin).

For more information, see [API](/api).
:::
