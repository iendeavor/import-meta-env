set -e

# set up
rm -rf dist

# act
pnpm exec webpack
pnpm exec cross-env HELLO=import-meta-env import-meta-env --disposable

# assert
diff -r dist __fixtures__