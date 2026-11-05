(()=>{ALW.freeze=true;ALW.fauna.update=()=>{};ALW.fauna.deer=[];ALW.fauna.birds=[];const w=ALW.world,p=ALW.player.p,x0=Math.floor(p[0])-6,z0=Math.floor(p[2])+1;const y0=Math.floor(p[1])+3;
for(let i=0;i<3;i++)for(let k=0;k<3;k++){for(let h=0;h<=k;h++)w.set(x0+i*3,y0+h,z0+k*0,[11,12,13][i])}
for(let i=0;i<3;i++){w.set(x0+i*3,y0,z0+3,[11,12,13][i]);w.set(x0+i*3,y0+1,z0+3,[11,12,13][i])}
const e=[x0+3.5,y0+5,z0-3.5],t=[x0+3.5,y0+1,z0+2],f=[t[0]-e[0],t[1]-e[1],t[2]-e[2]],L=Math.hypot(f[0],f[1],f[2]);ALW.camOverride={eye:e,f:f.map(v=>v/L)};})()
