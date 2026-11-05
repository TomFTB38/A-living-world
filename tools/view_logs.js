(()=>{window.__ids={y:[11,12,13],x:[32,34,36],z:[33,35,37]};ALW.freeze=true;ALW.fauna.update=()=>{};ALW.fauna.deer=[];ALW.fauna.birds=[];const w=ALW.world,p=ALW.player.p,x0=Math.floor(p[0])-7,z0=Math.floor(p[2])+1,y0=Math.floor(p[1])+3,B=[11,0,0];
const id=(n)=>[...Array(60).keys()].find(i=>0)||0;const ids=window.__ids;
ids.y.forEach((b,i)=>{w.set(x0+i*4,y0,z0,b);w.set(x0+i*4,y0+1,z0,b)});ids.x.forEach((b,i)=>{w.set(x0+i*4,y0,z0+3,b);w.set(x0+i*4+1,y0,z0+3,b);w.set(x0+i*4+2,y0,z0+3,b)});ids.z.forEach((b,i)=>{w.set(x0+i*4,y0,z0+5,b);w.set(x0+i*4,y0,z0+6,b);w.set(x0+i*4,y0,z0+7,b)});
const e=[x0+13,y0+5,z0-3],t=[x0+4,y0,z0+4],f=[t[0]-e[0],t[1]-e[1],t[2]-e[2]],L=Math.hypot(f[0],f[1],f[2]);ALW.camOverride={eye:e,f:f.map(v=>v/L)};})()
