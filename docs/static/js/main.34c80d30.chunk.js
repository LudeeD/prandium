(this.webpackJsonpprandium=this.webpackJsonpprandium||[]).push([[0],{121:function(e,t,n){},128:function(e,t){},130:function(e,t){},140:function(e,t){},142:function(e,t){},169:function(e,t){},171:function(e,t){},172:function(e,t){},177:function(e,t){},179:function(e,t){},185:function(e,t){},187:function(e,t){},206:function(e,t){},218:function(e,t){},221:function(e,t){},225:function(e,t,n){"use strict";n.r(t);var c=n(9),r=n.n(c),a=n(112),u=n.n(a),i=n(57),s=n.n(i),o=n(113),j=n(22),l=(n(121),n(114)),b=n.n(l),f=n(115),d=n(2);function O(){var e=Object(c.useState)(null),t=Object(j.a)(e,2),n=t[0],r=t[1],a=Object(c.useState)(null),u=Object(j.a)(a,2),i=u[0],l=u[1];return Object(c.useEffect)(Object(o.a)(s.a.mark((function e(){var t,n,c,a,u,i,o;return s.a.wrap((function(e){for(;;)switch(e.prev=e.next){case 0:return e.prev=0,t=b()({locateFile:function(){return f.a}}),n=fetch("/index.db").then((function(e){return e.arrayBuffer()})),e.next=5,Promise.all([t,n]);case 5:c=e.sent,a=Object(j.a)(c,2),u=a[0],i=a[1],o=new u.Database(new Uint8Array(i)),r(o),e.next=16;break;case 13:e.prev=13,e.t0=e.catch(0),l(e.t0);case 16:case"end":return e.stop()}}),e,null,[[0,13]])}))),[]),i?Object(d.jsx)("pre",{children:i.toString()}):n?Object(d.jsx)(h,{db:n}):Object(d.jsx)("pre",{children:"Loading..."})}function h(e){var t=e.db,n=Object(c.useState)(null),r=Object(j.a)(n,2),a=r[0],u=r[1],i=Object(c.useState)([]),s=Object(j.a)(i,2),o=s[0],l=s[1];return Object(d.jsxs)("div",{className:"App",children:[Object(d.jsx)("h1",{children:"React SQL interpreter"}),Object(d.jsx)("textarea",{onChange:function(e){return function(e){try{l(t.exec(e)),u(null)}catch(n){u(n),l([])}}(e.target.value)},placeholder:"Enter some SQL. No inspiration ? Try \u201cselect sqlite_version()\u201d"}),Object(d.jsx)("pre",{className:"error",children:(a||"").toString()}),Object(d.jsx)("pre",{children:o.map((function(e,t){var n=e.columns,c=e.values;return Object(d.jsx)(p,{columns:n,values:c},t)}))})]})}function p(e){var t=e.columns,n=e.values;return Object(d.jsxs)("table",{children:[Object(d.jsx)("thead",{children:Object(d.jsx)("tr",{children:t.map((function(e,t){return Object(d.jsx)("td",{children:e},t)}))})}),Object(d.jsx)("tbody",{children:n.map((function(e,t){return Object(d.jsx)("tr",{children:e.map((function(e,t){return Object(d.jsx)("td",{children:e},t)}))},t)}))})]})}var x=document.getElementById("root");u.a.render(Object(d.jsx)(r.a.StrictMode,{children:Object(d.jsx)(O,{})}),x)}},[[225,1,2]]]);
//# sourceMappingURL=main.34c80d30.chunk.js.map