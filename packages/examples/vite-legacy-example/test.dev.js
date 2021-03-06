const runTest = require("../run-test");

const commands = [];
const longRunningCommands = [
  "pnpm exec cross-env HELLO=foo vite dev --port 3002",
];
const expected = ["Hello: foo", "Is legacy? false"].join("\n");
const url = "http://localhost:3002";
const waitMs = 1000;

module.exports = () =>
  runTest({
    commands,
    longRunningCommands,
    expected,
    url,
    waitMs,
    noExit: true,
  });
