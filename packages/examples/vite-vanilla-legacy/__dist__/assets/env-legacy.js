System.register([],(function(e){"use strict";return{execute:function(){var r=/(?:^|^)\s*(?:export\s+)?([\w.-]+)(?:\s*=\s*?|:\s+?)(\s*'(?:\\'|[^'])*'|\s*"(?:\\"|[^"])*"|[^#\r\n]+)?\s*(?:#.*)?(?:$|$)/gm,n=function(e){var n,t={},s=e.toString();for(s=s.replace(/\r\n?/gm,"\n");null!=(n=r.exec(s));){var c=n[1],i=n[2]||"",u=(i=i.trim())[0];i=i.replace(/^(['"])([\s\S]+)\1$/gm,"$2"),'"'===u&&(i=(i=i.replace(/\\n/g,"\n")).replace(/\\r/g,"\r")),t[c]=i}return t}(`VITE_NAME=vite-plugin-dotenv
`);e("e",Object.assign(n,{BASE_URL:"/",MODE:"production",DEV:!1,PROD:!0}))}}}));