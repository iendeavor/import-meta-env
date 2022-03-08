set -e

# set up
rm -rf dist

# act
yarn vue-cli-service build
yarn cross-env HELLO=import-meta-env node node_modules/.bin/import-meta-env --example .env.example

# assert
diff -r dist __fixtures__
