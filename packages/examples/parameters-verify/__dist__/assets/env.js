const NEWLINE = "\n";
const RE_INI_KEY_VAL = /^\s*([\w.-]+)\s*=\s*("[^"]*"|'[^']*'|.*?)(\s+#.*)?$/;
const RE_NEWLINES = /\\n/g;
const NEWLINES_MATCH = /\r\n|\n|\r/;
function parse(src, options) {
  const debug = Boolean(options && options.debug);
  const multiline = Boolean(options && options.multiline);
  const obj = {};
  const lines = src.toString().split(NEWLINES_MATCH);
  for (let idx = 0; idx < lines.length; idx++) {
    let line = lines[idx];
    const keyValueArr = line.match(RE_INI_KEY_VAL);
    if (keyValueArr != null) {
      const key = keyValueArr[1];
      let val = keyValueArr[2] || "";
      let end = val.length - 1;
      const isDoubleQuoted = val[0] === '"' && val[end] === '"';
      const isSingleQuoted = val[0] === "'" && val[end] === "'";
      const isMultilineDoubleQuoted = val[0] === '"' && val[end] !== '"';
      const isMultilineSingleQuoted = val[0] === "'" && val[end] !== "'";
      if (multiline && (isMultilineDoubleQuoted || isMultilineSingleQuoted)) {
        const quoteChar = isMultilineDoubleQuoted ? '"' : "'";
        val = val.substring(1);
        while (idx++ < lines.length - 1) {
          line = lines[idx];
          end = line.length - 1;
          if (line[end] === quoteChar) {
            val += NEWLINE + line.substring(0, end);
            break;
          }
          val += NEWLINE + line;
        }
      } else if (isSingleQuoted || isDoubleQuoted) {
        val = val.substring(1, end);
        if (isDoubleQuoted) {
          val = val.replace(RE_NEWLINES, NEWLINE);
        }
      } else {
        val = val.trim();
      }
      obj[key] = val;
    } else if (debug) {
      const trimmedLine = line.trim();
      if (trimmedLine.length && trimmedLine[0] !== "#") {
        log(`Failed to match key and value when parsing line \${idx + 1}: \${line}`);
      }
    }
  }
  return obj;
}
const e = parse(`VITE_NAME=vite-plugin-dotenv
`, {});
function verify(actual) {
  const expected = { "VITE_NAME": "" };
  const importMetaEnv = "import.meta.env";
  const missingKeys = [];
  Object.keys(expected).forEach((key) => {
    if (Object.hasOwnProperty.call(actual, key) === false) {
      missingKeys.push(JSON.stringify(key));
    }
  });
  if (missingKeys.length) {
    throw new Error(`[vite-plugin-dotenv]: The following variables were defined in ${importMetaEnv} but are not present in the environment: ` + missingKeys.join(", "));
  }
  const notExistsKeys = [];
  Object.keys(actual).forEach((key) => {
    if (Object.hasOwnProperty.call(expected, key) === false) {
      notExistsKeys.push(JSON.stringify(key));
    }
  });
  if (notExistsKeys.length) {
    throw new Error(`[vite-plugin-dotenv]: The following variables were NOT defined in ${importMetaEnv} but are present in the environment: ` + notExistsKeys.join(", "));
  }
}
verify(e);
var vite_plugin_dotenv_unique_id_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx = Object.assign(e, { "BASE_URL": "/", "MODE": "production", "DEV": false, "PROD": true });
export { vite_plugin_dotenv_unique_id_xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx as v };