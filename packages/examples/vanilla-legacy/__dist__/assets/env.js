const n=/^\s*([\w.-]+)\s*=\s*("[^"]*"|'[^']*'|.*?)(\s+#.*)?$/,t=/\\n/g,e=/\r\n|\n|\r/;const s=function(s,l){const i=Boolean(l&&l.debug),o=Boolean(l&&l.multiline),r={},c=s.toString().split(e);for(let e=0;e<c.length;e++){let s=c[e];const l=s.match(n);if(null!=l){const n=l[1];let i=l[2]||"",g=i.length-1;const a='"'===i[0]&&'"'===i[g],u="'"===i[0]&&"'"===i[g],h='"'===i[0]&&'"'!==i[g],f="'"===i[0]&&"'"!==i[g];if(o&&(h||f)){const n=h?'"':"'";for(i=i.substring(1);e++<c.length-1;){if(s=c[e],g=s.length-1,s[g]===n){i+="\n"+s.substring(0,g);break}i+="\n"+s}}else u||a?(i=i.substring(1,g),a&&(i=i.replace(t,"\n"))):i=i.trim();r[n]=i}else if(i){const n=s.trim();n.length&&"#"!==n[0]&&log("Failed to match key and value when parsing line ${idx + 1}: ${line}")}}return r}(`VITE_NAME=vite-plugin-dotenv
`,{});var l=Object.assign(s,{BASE_URL:"/",MODE:"production",DEV:!1,PROD:!0});export{l as v};