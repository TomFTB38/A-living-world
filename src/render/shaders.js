// Rendering/shaders.js — GLSL ES 3.00. Portable vers WGSL pour le moteur Rust/wgpu.
import { matLayer } from '../world/blocks.js';
export const VS_FULL = `#version 300 es
out vec2 uv; void main(){ vec2 p=vec2((gl_VertexID<<1)&2,gl_VertexID&2); uv=p; gl_Position=vec4(p*2.-1.,0.,1.); }`;

const COMMON = `
precision highp float; precision highp int; precision highp sampler2DArray; precision highp sampler2DArrayShadow; precision highp sampler3D; precision highp sampler2DShadow;
#define sat(x) clamp(x,0.,1.)
float ign(vec2 p){ return fract(52.9829189*fract(dot(p,vec2(.06711056,.00583715)))); }
float hash13(vec3 p){ p=fract(p*.1031); p+=dot(p,p.zyx+31.32); return fract((p.x+p.y)*p.z); }
`;

export const VS_GEOM = `#version 300 es
${COMMON}
layout(location=0) in vec4 aPos; layout(location=1) in vec4 aNrm; layout(location=2) in vec4 aLit; layout(location=3) in vec4 aTint;
uniform mat4 uVP; uniform vec3 uOrigin,uCamPos; uniform float uPosScale,uTime,uFarHole,uDepthK; uniform mat4 uModel; uniform vec2 uJitter;
out vec3 vPos; out vec3 vN; flat out int vLayer; flat out int vKind; flat out int vRot; out vec3 vLit; out vec3 vTint; out vec3 vLocal; out vec3 vNl;
vec3 wind(vec3 p,float w){ if(w<=0.)return vec3(0.);
  float g=sin(uTime*.7+p.x*.05+p.z*.07)*.5+.5, t=uTime*1.8;
  vec3 d=vec3(sin(t+p.x*.45+p.z*.2+p.y*.3),sin(t*1.3+p.x*.3)*.3,cos(t*.9+p.z*.4+p.y*.2));
  return d*w*(.035+.085*g); }
void main(){
  vec3 lp=aPos.xyz*uPosScale; vec4 wp=uModel*vec4(lp,1.); vec3 world=wp.xyz+uOrigin;
  world+=wind(world,aNrm.w*2.);
  if(uFarHole>0. && length(world.xz-uCamPos.xz)<uFarHole) world.y-=90.;
  vPos=world; vLocal=lp; vN=mat3(uModel)*aNrm.xyz; vLayer=int(aPos.w+.5); vKind=int(aLit.w*255.+.5); vRot=int(aTint.a*255.+.5); vLit=aLit.xyz; vTint=aTint.rgb*2.;
  vNl=aNrm.xyz; gl_Position=uVP*vec4(world,1.); gl_Position.xy+=uJitter*gl_Position.w; gl_Position.z*=uDepthK;
}`;

export const FS_GEOM = `#version 300 es
${COMMON}
uniform sampler2DArray uAlbedo,uNormal; uniform sampler3D uNoise; uniform float uTexOn,uHero,uLocal;
in vec3 vPos; in vec3 vN; flat in int vLayer; flat in int vKind; flat in int vRot; in vec3 vLit; in vec3 vTint; in vec3 vLocal; in vec3 vNl;
layout(location=0) out vec4 gA; layout(location=1) out vec4 gN; layout(location=2) out vec4 gL;
void main(){
  vec3 n=normalize(vN); if(!gl_FrontFacing && vKind>=2) n=vec3(n.x,abs(n.y),n.z);
  vec3 alb=vTint; float rough=.9, trans=0., emis=0.; vec3 nn=n;
  if(vKind<=1){
    bool held=uLocal>.5; vec3 P=held?vLocal:vPos, ln=held?normalize(vNl):n; // objet en main : texture liée à l'objet, pas au monde
    vec3 cell=floor(P-ln*.02); vec3 an=abs(ln);
    vec2 uv; vec3 T,B;
    if(an.y>.5){ uv=P.xz; T=vec3(1,0,0); B=vec3(0,0,1);} else if(an.x>.5){ uv=vec2(P.z,-P.y); T=vec3(0,0,1); B=vec3(0,-1,0);} else { uv=vec2(P.x,-P.y); T=vec3(1,0,0); B=vec3(0,-1,0);}
    float h=hash13(cell+vec3(.5)), h2=hash13(cell.zxy+vec3(7.5));
    int v=int(h*4.)&3;
    vec2 f=fract(uv);
    vec2 dx=dFdx(uv),dy=dFdy(uv);
    bool swp=vRot==1||(vRot==0&&an.y>.5&&h2>.5); // le fil du bois suit l'axe du bloc
    if(swp){ f=f.yx; dx=dx.yx; dy=dy.yx; vec3 q=T; T=B; B=q; }
    vec3 tc=vec3(f,float(vLayer+v));
    vec4 a=textureGrad(uAlbedo,tc,dx,dy); vec4 m=textureGrad(uNormal,tc,dx,dy);
    alb=a.rgb*a.rgb*vTint; // texture sRGB approchée vers linéaire
    alb=mix(vec3(dot(alb,vec3(.3,.6,.1))),alb,.86);
    bool isBark=vLayer>=${matLayer('bark_oak')}&&vLayer<${matLayer('leaf_oak')};
    alb*=isBark?.84+.32*h2:.9+.2*h2;
    if(isBark||(vLayer>=${matLayer('stone')}&&vLayer<${matLayer('gravel')})){ vec2 e=min(f,1.-f); alb*=.8+.2*smoothstep(0.,.04,min(e.x,e.y)); }
    vec2 d=m.xy*2.-1.; if(!held) nn=normalize(n+(T*d.x+B*d.y)*.9); rough=m.b;
    if(vKind==1){ trans=1.; rough=.5; alb*=1.05; }
    // mousse en petits pixels sur les pierres et l'écorce, plus présente sur les faces hautes
    bool mossyRock=vLayer>=${matLayer('mossy_stone')}&&vLayer<${matLayer('mossy_stone') + 4};
    bool mossable=mossyRock||(vLayer>=${matLayer('bark_oak')}&&vLayer<${matLayer('bark_oak') + 4})||(vLayer>=${matLayer('bark_beech')}&&vLayer<${matLayer('bark_beech') + 4});
    if(mossable&&!held&&length(dFdx(vPos))<.07){ vec3 g=floor(vPos*9.)/9.; float mn=texture(uNoise,g*.031).r*.78+hash13(g)*.22+(n.y>.5?.14:0.);
      float mm=step(isBark?.7:.5,mn); alb=mix(alb,vec3(.13,.22,.045)*(.75+.7*hash13(g+3.)),mm); rough=mix(rough,.95,mm); }
  } else {
    float k=.92+.16*hash13(floor(vPos*7.)); alb*=alb*k*1.25;
    if(vKind==3){ trans=.45; alb=mix(vec3(dot(alb,vec3(.3,.6,.1))),alb,.72); }
    if(vKind==4) emis=1.;
  }
  gA=vec4(alb,vLit.x); gN=vec4(nn*.5+.5,rough); gL=vec4(vLit.y,vLit.z,trans,emis>0.?emis:uHero*.4);
}`;

export const VS_SHADOW = `#version 300 es
${COMMON}
layout(location=0) in vec4 aPos; layout(location=1) in vec4 aNrm;
uniform mat4 uVP; uniform vec3 uOrigin; uniform float uPosScale,uTime; uniform mat4 uModel;
void main(){ vec3 lp=aPos.xyz*uPosScale; vec3 w=(uModel*vec4(lp,1.)).xyz+uOrigin; float wnd=aNrm.w*2.;
  if(wnd>0.){ float t=uTime*1.8; w+=vec3(sin(t+w.x*.45+w.z*.2+w.y*.3),0.,cos(t*.9+w.z*.4+w.y*.2))*wnd*.08; }
  gl_Position=uVP*vec4(w,1.); }`;
export const FS_SHADOW = `#version 300 es
precision mediump float; void main(){}`;

const LIGHTING = `
uniform sampler2DArrayShadow uShadow; uniform mat4 uShadowVP[3]; uniform vec3 uCascade; uniform float uShadowTexel;
uniform vec3 uSunDir,uSunCol,uSkyTop,uSkyHor,uGround; uniform vec3 uCam; uniform float uTime,uCloudShadow;
uniform sampler3D uNoise; uniform sampler2DShadow uActorShadow; uniform mat4 uActorVP;
const vec2 VOGEL[8]=vec2[](vec2(.177,.0),vec2(-.25,.228),vec2(.04,-.446),vec2(.336,.391),vec2(-.578,-.137),vec2(.569,-.285),vec2(-.232,.649),vec2(-.301,-.667));
float shadowBase(vec3 wp,vec3 n,float dist,vec2 fc,int samples){
  int c=dist<uCascade.x?0:dist<uCascade.y?1:dist<uCascade.z?2:3; if(c>2) return 1.;
  float bias=(c==0?.035:c==1?.09:.26);
  vec3 p=wp+n*bias*1.5+uSunDir*bias;
  vec4 lp=uShadowVP[c]*vec4(p,1.); vec3 s=lp.xyz*.5+.5;
  if(s.x<.002||s.y<.002||s.x>.998||s.y>.998) return 1.;
  float a=ign(fc)*6.283, ca=cos(a), sa=sin(a), sum=0., rad=uShadowTexel*(c==0?2.6:c==1?1.8:1.2);
  for(int i=0;i<samples;i++){ vec2 o=VOGEL[i]; o=vec2(o.x*ca-o.y*sa,o.x*sa+o.y*ca)*rad; sum+=texture(uShadow,vec4(s.xy+o,float(c),s.z)); }
  float sh=sum/float(samples);
  float fade=sat((uCascade.z-dist)/(uCascade.z*.15)); return c==2?mix(1.,sh,fade):sh; }
// Ombres des personnages et animaux proches : carte dédiée, recalculée à chaque image.
float actorShade(vec3 wp,vec3 n){ vec3 s=(uActorVP*vec4(wp+n*.05+uSunDir*.05,1.)).xyz*.5+.5;
  if(s.x<.01||s.y<.01||s.x>.99||s.y>.99) return 1.; float t=1./1024., r=0.;
  for(int i=0;i<4;i++) r+=texture(uActorShadow,vec3(s.xy+VOGEL[i]*t*2.,s.z)); return r*.25; }
float shadowAt(vec3 wp,vec3 n,float dist,vec2 fc,int samples){ return shadowBase(wp,n,dist,fc,samples)*actorShade(wp,n); }
float cloudShade(vec3 wp){ if(uCloudShadow<=0.) return 1.; vec3 q=vec3(wp.xz*.0011+vec2(uTime*.004,uTime*.0017),.31);
  float n=texture(uNoise,q.xzy).r; return mix(1.,smoothstep(.56,.3,n)*.7+.3,uCloudShadow); }
vec3 skyColor(vec3 d){
  float y=max(d.y,0.), t=pow(1.-y,3.6), mu=sat(dot(d,uSunDir));
  vec3 c=mix(uSkyTop,uSkyHor,t);
  c+=uSunCol*(.0035/(1.02-mu*.985)+.05*pow(mu,8.))*(.25+.75*t);
  if(d.y<0.) c=mix(c,uGround,sat(-d.y*7.));
  return c; }
`;

export const FS_LIGHT = `#version 300 es
${COMMON}${LIGHTING}
in vec2 uv; out vec4 o;
uniform sampler2D uA,uN,uL,uD,uSSAO,uClouds; uniform mat4 uInvVP; uniform int uShadowSamples; uniform float uNight;
vec3 pos(vec2 p,float d){ vec4 c=uInvVP*vec4(p*2.-1.,d*2.-1.,1.); return c.xyz/c.w; }
void main(){
  float d=texture(uD,uv).r; vec3 wp=pos(uv,d); vec3 V=normalize(wp-uCam);
  if(d>=1.){ vec3 s=skyColor(V); float mu=dot(V,uSunDir); s+=uSunCol*22.*smoothstep(.99955,.99985,mu);
    if(uNight>0.&&V.y>0.){ vec3 g=floor(V*520.); float st=hash13(g); s+=vec3(.8,.87,1.)*step(.9985,st)*(.25+.75*hash13(g+2.))*.8*uNight*smoothstep(0.,.15,V.y); }
    o=vec4(s,1.); return; }
  vec4 A=texture(uA,uv), N=texture(uN,uv), L=texture(uL,uv);
  vec3 n=normalize(N.xyz*2.-1.), alb=A.rgb; float rough=N.a, skyL=L.x, blk=L.y, trans=L.z, emis=L.w, vao=A.a;
  if(emis>.5){ o=vec4(alb*5.,1.); return; }
  float dist=length(wp-uCam), ss=texture(uSSAO,uv).r;
  float sh=shadowAt(wp,n,dist,gl_FragCoord.xy,uShadowSamples)*cloudShade(wp);
  float NoL=dot(n,uSunDir), wrap=trans>0.?sat((NoL+.45)/1.45):sat(NoL);
  vec3 direct=uSunCol*wrap*sh;
  float back=pow(sat(dot(V,uSunDir)),5.)*trans;
  vec3 bl=uSunCol*alb*vec3(1.2,1.4,.6)*back*sh*.9;
  float sky2=skyL*skyL*(3.-2.*skyL); float vis=vao*mix(1.,ss,.9);
  vec3 skyAmb=mix(uGround*.9,mix(uSkyHor,uSkyTop,.4)*vec3(1.,.95,.82)*.78,n.y*.5+.5);
  // lumière rebondie du couvert : sous les arbres, l'ambiance se teinte de vert et reste lisible
  vec3 canopy=vec3(.27,.36,.13)*mix(1.,.5,n.y*.5+.5);
  vec3 amb=mix(canopy*1.75,skyAmb,sky2*.8+.2)*(.6+.4*skyL)*vis;
  vec3 blkc=vec3(1.,.62,.28)*pow(blk,2.1)*2.2*vao;
  vec3 c=alb*(direct+amb+blkc)+bl;
  // spéculaire de ciel et de soleil discret
  vec3 H=normalize(uSunDir-V); float NoH=sat(dot(n,H)), sp=pow(NoH,mix(90.,10.,rough))*(1.-rough)*.9;
  c+=uSunCol*sp*sh*sat(NoL)*.5;
  o=vec4(c,1.);
}`;

export const FS_SSAO = `#version 300 es
${COMMON}
in vec2 uv; out vec4 o; uniform sampler2D uD,uN; uniform mat4 uP,uInvP,uV; uniform float uFrame;
vec3 vp(vec2 p){ float d=texture(uD,p).r; vec4 c=uInvP*vec4(p*2.-1.,d*2.-1.,1.); return c.xyz/c.w; }
void main(){
  vec3 P=vp(uv); float dist=-P.z; if(dist>180.){ o=vec4(1.); return; }
  vec3 N=normalize(mat3(uV)*(texture(uN,uv).xyz*2.-1.));
  float a=ign(gl_FragCoord.xy+uFrame*5.588)*6.283, occ=0., R=clamp(.55+dist*.03,.5,3.);
  const int S=10;
  for(int i=0;i<S;i++){
    float f=(float(i)+.5)/float(S), t=a+f*38.8, r=sqrt(f)*R;
    vec3 dir=normalize(vec3(cos(t),sin(t),.4)); if(dot(dir,N)<0.) dir=-dir;
    vec3 q=P+dir*r+N*.04; vec4 c=uP*vec4(q,1.); vec2 s=c.xy/c.w*.5+.5;
    vec3 Q=vp(s); float dz=Q.z-q.z; 
    occ+=step(.03,dz)*smoothstep(R*2.,R*.6,abs(P.z-Q.z));
  }
  float ao=1.-occ/float(S)*1.25; o=vec4(pow(sat(ao),1.4));
}`;

export const FS_BLUR = `#version 300 es
${COMMON}
in vec2 uv; out vec4 o; uniform sampler2D uT,uD; uniform vec2 uDir; uniform float uNear;
float lin(float d){ return 1./max(1.-d,1e-5); }
void main(){ float d0=lin(texture(uD,uv).r); vec4 s=texture(uT,uv); float w=1.;
  for(int i=-3;i<=3;i++){ if(i==0)continue; vec2 p=uv+uDir*float(i); float e=abs(lin(texture(uD,p).r)-d0)/d0; float g=exp(-float(i*i)*.18)*exp(-e*24.); s+=texture(uT,p)*g; w+=g; }
  o=s/w; }`;

export const FS_SKYCLOUD = `#version 300 es
${COMMON}${LIGHTING}
in vec2 uv; out vec4 o; uniform mat4 uInvVP; uniform sampler2D uD; uniform float uFrame;
float dens(vec3 p,float cov){ float h=sat((p.y-210.)/170.);
  vec3 q=p*.00105+vec3(uTime*.0024,0.,uTime*.001);
  float b=texture(uNoise,q).r, d=texture(uNoise,q*3.7+.2).r;
  float prof=smoothstep(0.,.18,h)*smoothstep(1.,.45,h);
  return sat((b*.78+d*.22-(1.-cov)*.82-.1)*2.6)*prof; }
void main(){
  vec4 c=uInvVP*vec4(uv*2.-1.,1.,1.); vec3 V=normalize(c.xyz/c.w-uCam);
  if(V.y<.015){ o=vec4(0,0,0,1); return; }
  float t0=(210.-uCam.y)/V.y, t1=(380.-uCam.y)/V.y, step_=(t1-t0)/14., t=t0+step_*ign(gl_FragCoord.xy+uFrame*5.588);
  vec3 col=vec3(0.); float T=1.; float mu=dot(V,uSunDir), ph=.6+.9*pow(sat(mu),6.);
  vec3 amb=mix(uSkyHor,uSkyTop,.3)*1.25;
  for(int i=0;i<14&&T>.02;i++){ vec3 p=uCam+V*t; float d=dens(p,uCloudShadow*.5+.28);
    if(d>.01){ float sh=0.; for(int j=1;j<=3;j++) sh+=dens(p+uSunDir*float(j)*22.,uCloudShadow*.5+.28);
      float L=exp(-sh*1.5); vec3 s=(uSunCol*L*ph*.95+amb*(.45+.55*sat((p.y-210.)/170.)))*d;
      float a=1.-exp(-d*step_*.05); col+=T*s*a/max(d,.01); T*=1.-a; }
    t+=step_; }
  float fade=smoothstep(.015,.14,V.y);
  o=vec4(col*fade,mix(1.,T,fade));
}`;

export const FS_VOLUME = `#version 300 es
${COMMON}${LIGHTING}
in vec2 uv; out vec4 o; uniform sampler2D uD; uniform mat4 uInvVP; uniform float uFrame,uFog; uniform int uSteps;
float G(float c,float g){ float g2=g*g; return (1.-g2)/(12.57*pow(1.+g2-2.*g*c,1.5)); }
float sh(vec3 p,float dist){ int c=dist<uCascade.x?0:dist<uCascade.y?1:dist<uCascade.z?2:3; if(c>2)return 1.;
  vec4 lp=uShadowVP[c]*vec4(p,1.); vec3 s=lp.xyz*.5+.5; if(s.x<0.||s.y<0.||s.x>1.||s.y>1.)return 1.; return texture(uShadow,vec4(s.xy,float(c),s.z-.0006)); }
void main(){
  float d=texture(uD,uv).r; vec4 c=uInvVP*vec4(uv*2.-1.,d*2.-1.,1.); vec3 wp=c.xyz/c.w; vec3 ray=wp-uCam; float len=length(ray); vec3 V=ray/len;
  float end=min(len,150.), st=end/float(uSteps), t=st*ign(gl_FragCoord.xy+uFrame*5.588), mu=dot(V,uSunDir);
  float ph=min(G(mu,.8)*9.,3.2)+G(mu,-.2)*.5+.3;
  vec3 acc=vec3(0.); float tr=1.;
  for(int i=0;i<uSteps;i++){ vec3 p=uCam+V*t;
    float hf=exp(-max(p.y-52.,0.)*.055)+.25;
    float n=texture(uNoise,p*.016+vec3(uTime*.012,0.,uTime*.007)).r;
    float rho=uFog*hf*(.3+1.15*n*n);
    float s=sh(p,t);
    vec3 skyS=mix(uSkyHor,uSkyTop,.4)*.42;
    acc+=tr*(uSunCol*s*ph*.13+skyS*.14)*rho*st;
    tr*=exp(-rho*st); t+=st; }
  o=vec4(acc,tr);
}`;

export const FS_WATER = `#version 300 es
${COMMON}${LIGHTING}
in vec3 vPos; in vec3 vN; in vec3 vLit; in float vFoam; layout(location=0) out vec4 o;
uniform sampler2D uScene,uD; uniform vec2 uRes; uniform mat4 uVP; uniform mat4 uInvVP;
float wave(vec2 p,float t){ return texture(uNoise,vec3(p*.09+t*vec2(.022,.015),t*.016)).r+.5*texture(uNoise,vec3(p*.27-t*vec2(.03,.05),.4+t*.02)).r; }
vec3 pos(vec2 p,float d){ vec4 c=uInvVP*vec4(p*2.-1.,d*2.-1.,1.); return c.xyz/c.w; }
void main(){
  vec2 suv=gl_FragCoord.xy/uRes; vec3 N0=normalize(vN); vec3 V=normalize(vPos-uCam); float dist=length(vPos-uCam), fall=1.-abs(N0.y);
  float e=.12; vec3 n;
  if(fall>.5){ vec2 q=vec2(vPos.x+vPos.z,vPos.y*.35+uTime*2.6); float a=wave(q,uTime*2.),b=wave(q+vec2(e,0.),uTime*2.),c2=wave(q+vec2(0.,e),uTime*2.); n=normalize(N0+vec3(a-b,c2-a,a-b)*1.2); }
  else { vec2 q=vPos.xz; float a=wave(q,uTime),b=wave(q+vec2(e,0.),uTime),c2=wave(q+vec2(0.,e),uTime); n=normalize(vec3((a-b)*.55,1.,(a-c2)*.55)); }
  float sceneD=texture(uD,suv).r; vec3 bg=pos(suv,sceneD); float depth=length(bg-vPos);
  vec2 ruv=suv+n.xz*.03*min(1.,depth*.6)/(1.+dist*.03);
  vec3 scene=texture(uScene,ruv).rgb; float rd=texture(uD,ruv).r; if(length(pos(ruv,rd)-uCam)<dist-.05) scene=texture(uScene,suv).rgb;
  vec3 deep=vec3(.015,.075,.085), shallow=vec3(.19,.42,.36);
  float absorb=exp(-depth*.42); vec3 tint=mix(deep,shallow,exp(-depth*.28));
  float sh=shadowAt(vPos,vec3(0,1,0),dist,gl_FragCoord.xy,4); if(fall>.5) sh=.5+.2*sh; // une nappe en mouvement ne reçoit pas d'ombre nette
  vec3 water=mix(tint*(uSunCol*.35*sh+mix(uSkyHor,uSkyTop,.5)*.7*vLit.y),scene,absorb*.82);
  vec3 R=reflect(V,n); R.y=abs(R.y);
  float F=.03+.97*pow(1.-sat(dot(-V,n)),5.);
  vec3 refl=skyColor(R)*mix(.22,1.,vLit.y);
  // réflexion écran grossière : verdure et rives dans l'eau calme
  if(fall<.5){ vec3 q=vPos+R*.8; float st=1.8; vec2 hit=vec2(-1.);
    for(int i=0;i<14;i++){ vec4 c=uVP*vec4(q,1.); vec2 s=c.xy/c.w*.5+.5; if(s.x<0.||s.y<0.||s.x>1.||s.y>1.)break;
      float dq=texture(uD,s).r; vec3 P=pos(s,dq); if(dq<1.&&length(P-uCam)<length(q-uCam)&&length(P-q)<st*2.5){ hit=s; break; } q+=R*st; st*=1.18; }
    if(hit.x>=0.){ float ed=smoothstep(0.,.08,min(min(hit.x,hit.y),min(1.-hit.x,1.-hit.y))); refl=mix(refl,texture(uScene,hit).rgb,ed*.9); } }
  vec3 c=mix(water,refl,F);
  vec3 H=normalize(uSunDir-V); c+=uSunCol*pow(sat(dot(n,H)),420.)*sh*5.;
  float glint=pow(sat(dot(reflect(V,normalize(n+vec3(0,.2,0))),uSunDir)),1400.)*40.*sh; c+=uSunCol*min(glint,6.);
  // écume : contact avec les rives et chutes
  float fn=texture(uNoise,vec3(vPos.xz*.42,uTime*.09)).r;
  float boil=.6*texture(uNoise,vec3(vPos.xz*.37+.13,uTime*.3)).r+.4*texture(uNoise,vec3(vPos.zx*.91,.4-uTime*.45)).r;
  float foam=max(smoothstep(.7,.05,depth)*smoothstep(.42,.7,fn+.25),smoothstep(.2,1.5,vFoam)*smoothstep(.42,.56,boil))*(fall<.5?1.:0.);
  if(fall>.5){ float streak=texture(uNoise,vec3((vPos.x+vPos.z)*1.7,vPos.y*.09+uTime*.9,.2)).r; foam=smoothstep(.56,.76,streak)*.42; c=mix(c,vec3(.34,.5,.52)*(uSunCol*.3*sh+mix(uSkyHor,uSkyTop,.5)*.9*max(vLit.y,.4)),.3); }
  c=mix(c,vec3(1.)*(uSunCol*.55*sh+mix(uSkyHor,uSkyTop,.5)*.8*vLit.y),foam*.8);
  o=vec4(c,1.);
}`;
export const VS_WATER = `#version 300 es
${COMMON}
layout(location=0) in vec4 aPos; layout(location=1) in vec4 aNrm; layout(location=2) in vec4 aLit; layout(location=3) in vec4 aTint;
uniform mat4 uVP; uniform vec3 uOrigin; uniform float uPosScale; uniform vec2 uJitter;
out vec3 vPos; out vec3 vN; out vec3 vLit; out float vFoam;
void main(){ vPos=aPos.xyz*uPosScale+uOrigin; vN=aNrm.xyz; vLit=aLit.xyz; vFoam=aTint.r*2.; gl_Position=uVP*vec4(vPos,1.); gl_Position.xy+=uJitter*gl_Position.w; }`;

export const FS_COMPOSE = `#version 300 es
${COMMON}${LIGHTING}
in vec2 uv; out vec4 o; uniform sampler2D uHdr,uVol,uClouds,uD; uniform mat4 uInvVP; uniform float uFog,uMist; uniform vec2 uTexel;
void main(){
  vec3 c=texture(uHdr,uv).rgb; float d=texture(uD,uv).r;
  vec4 h=uInvVP*vec4(uv*2.-1.,d*2.-1.,1.); vec3 wp=h.xyz/h.w, ray=wp-uCam; float len=length(ray); vec3 V=ray/len;
  vec3 mist=skyColor(normalize(vec3(V.x,max(V.y,.03)*.25+.03,V.z)));
  if(d>=1.){ vec4 cl=texture(uClouds,uv); c=c*cl.a+cl.rgb; c=mix(c,mist,min(1.,uMist*160.)); }
  else {
    // perspective aérienne : au-delà de la brume marchée, l'horizon se dissout dans le ciel
    float far=max(len-70.,0.);
    float a=1.-exp(-far*.00125*(1.+exp(-max(wp.y-50.,0.)*.02)));
    vec3 air=skyColor(normalize(vec3(V.x,max(V.y,.03),V.z)));
    c=mix(c,air,a);
    c=mix(c,mist,1.-exp(-len*uMist)); // pluie, brouillard : la distance se perd dans un voile uniforme
  }
  // suréchantillonnage bilatéral du volume
  vec4 v=vec4(0.); float w=0., d0=1./max(1.-d,1e-5);
  for(int j=0;j<4;j++){ vec2 off=(vec2(float(j&1),float(j>>1))-.5)*uTexel*2.; vec2 p=uv+off; float e=abs(1./max(1.-texture(uD,p).r,1e-5)-d0)/d0, g=exp(-e*16.)+.01; v+=texture(uVol,p)*g; w+=g; }
  v/=w;
  c=c*v.a+v.rgb;
  o=vec4(c,1.);
}`;

export const FS_TAA = `#version 300 es
${COMMON}
in vec2 uv; layout(location=0) out vec4 o; uniform sampler2D uCur,uHist,uD,uL; uniform mat4 uInvVP,uPrevVP; uniform vec2 uTexel; uniform float uReset;
vec3 ycc(vec3 c){ return vec3(.25*c.r+.5*c.g+.25*c.b,.5*c.r-.5*c.b,-.25*c.r+.5*c.g-.25*c.b); }
vec3 rgb(vec3 c){ return vec3(c.x+c.y-c.z,c.x+c.z,c.x-c.y-c.z); }
vec3 ton(vec3 c){ return c/(1.+dot(c,vec3(.299,.587,.114))); } vec3 inv(vec3 c){ return c/max(1.-dot(c,vec3(.299,.587,.114)),.002); }
void main(){
  vec3 c=ton(texture(uCur,uv).rgb); if(uReset>.5){ o=vec4(inv(c),1.); return; }
  vec3 m=vec3(0.),s=vec3(0.),Y=ycc(c),mn=Y,mx=Y;
  for(int j=-1;j<=1;j++)for(int i=-1;i<=1;i++){ vec3 q=ycc(ton(texture(uCur,uv+vec2(i,j)*uTexel).rgb)); m+=q; s+=q*q; mn=min(mn,q); mx=max(mx,q); }
  m/=9.; s=sqrt(max(s/9.-m*m,0.)); vec3 lo=max(mn,m-s*1.25), hi=min(mx,m+s*1.25);
  float d=texture(uD,uv).r; vec4 w=uInvVP*vec4(uv*2.-1.,d*2.-1.,1.); w/=w.w; vec4 p=uPrevVP*w; vec2 q=p.xy/p.w*.5+.5;
  float hero=texture(uL,uv).a; if(hero>.2&&hero<.6) q=uv; // le héros suit la caméra : son historique est au même point de l'écran
  if(q.x<0.||q.y<0.||q.x>1.||q.y>1.){ o=vec4(inv(c),1.); return; }
  vec3 h=ycc(ton(texture(uHist,q).rgb)); h=clamp(h,lo,hi);
  float mv=length((q-uv)/uTexel), a=mix(.09,.28,sat(mv*.08));
  o=vec4(inv(rgb(mix(h,Y,a))),1.);
}`;

export const FS_BRIGHT = `#version 300 es
${COMMON}
in vec2 uv; out vec4 o; uniform sampler2D uT; uniform vec2 uTexel; uniform float uFirst;
void main(){ vec3 c=texture(uT,uv+uTexel*vec2(-1,-1)).rgb+texture(uT,uv+uTexel*vec2(1,-1)).rgb+texture(uT,uv+uTexel*vec2(-1,1)).rgb+texture(uT,uv+uTexel*vec2(1,1)).rgb; c*=.25;
  if(uFirst>.5){ float l=dot(c,vec3(.3,.6,.1)); c*=smoothstep(.95,2.2,l); c=min(c,vec3(9.)); } o=vec4(c,1.); }`;

export const FS_FINAL = `#version 300 es
${COMMON}
in vec2 uv; out vec4 o; uniform sampler2D uT,uB; uniform vec2 uTexel; uniform float uExposure,uBloom,uSharp;
vec3 aces(vec3 x){ return sat(x*(2.51*x+.03)/(x*(2.43*x+.59)+.14)); }
void main(){
  vec3 c=texture(uT,uv).rgb;
  // netteté adaptative légère après TAA
  vec3 nb=(texture(uT,uv+vec2(uTexel.x,0.)).rgb+texture(uT,uv-vec2(uTexel.x,0.)).rgb+texture(uT,uv+vec2(0.,uTexel.y)).rgb+texture(uT,uv-vec2(0.,uTexel.y)).rgb)*.25;
  c=max(c+(c-nb)*uSharp,vec3(0.));
  c+=texture(uB,uv).rgb*uBloom;
  c*=uExposure;
  c=aces(c);
  c=pow(c,vec3(1./2.2));
  // étalonnage : ombres légèrement froides, hautes lumières chaudes, saturation maîtrisée
  float l=dot(c,vec3(.3,.59,.11));
  c=mix(vec3(l),c,1.2);
  c*=mix(vec3(.97,1.,1.03),vec3(1.03,1.,.95),smoothstep(.1,.8,l));
  c=mix(c,c*c*(3.-2.*c),.22);
  c+=(ign(gl_FragCoord.xy)-.5)/255.;
  o=vec4(c,1.);
}`;

export const VS_LINE = `#version 300 es
layout(location=0) in vec3 aPos; uniform mat4 uVP; uniform vec3 uOffset; uniform vec3 uScale; void main(){ gl_Position=uVP*vec4(aPos*uScale+uOffset,1.); }`;
export const FS_LINE = `#version 300 es
precision mediump float; out vec4 o; uniform vec4 uColor; void main(){ o=uColor; }`;

export const VS_RAIN = `#version 300 es
precision highp float; uniform mat4 uVP; uniform vec3 uCam; uniform float uTime;
float h(float n){ return fract(sin(n*12.9898)*43758.5453); }
void main(){ float i=float(gl_VertexID/2), e=float(gl_VertexID&1); const vec3 B=vec3(28.,18.,28.);
  vec3 p=vec3(h(i),h(i+7.3),h(i+13.1))*B; p.y-=uTime*(20.+6.*h(i+3.)); p.xz+=uTime*vec2(1.8,.9);
  p=mod(p-uCam,B)-B*.5+uCam; p+=vec3(.1,-1.25,.05)*e; gl_Position=uVP*vec4(p,1.); }`;
