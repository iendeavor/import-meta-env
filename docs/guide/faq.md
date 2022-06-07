# FAQ

## Why use ImportMeta?

Since `process.env` is a Node specific object, we should not use it in browser environment.

For server-side rendering, it is also more precise to use `import.meta.env` (heavily inspired by Vite) and `process.env` respectively.

## Boolean Values

Environment variables are always strings.

```bash
export DEBUG=whatever # true
export DEBUG= # false
```

The easiest way to do this is to treat `""` and `undefined` as `false`, otherwise treat them as `true`:

```js
if (import.meta.env.DEBUG) {
  console.log("DEBUG is anything but the empty string and undefined.");
} else {
  console.log("DEBUG is the empty string.");
}
```

If you need to convert it to `boolean` type:

```diff
- if (  import.meta.env.DEBUG) {
+ if (!!import.meta.env.DEBUG === true) {
  console.log("DEBUG is anything but the empty string and undefined.");
} else {
  console.log("DEBUG is the empty string.");
}
```

## Changes to environment variables is not updated

You will need to restart your dev server after changing the environment variables.

This is useful, for example:

- You want to track bugs for production, but you don't want to shut down the development environment.
- Or, you want to start multiple development environments at the same time and each one has its own environment variables.

## Can I have multiple `.env` files?

Yes. You can choose which one to be used by passing the `env` option to `import-meta-env`, for example, you can pass `.env.local` to `import-meta-env`:

```bash
./node_modules/.bin/import-meta-env \
  --env .env.local \
  --example .env.example
```

## Should I commit my `.env` file?

No. We strongly recommend against committing your `.env` file to version control.
