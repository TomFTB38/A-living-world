"""Mesures objectives d'une image (zone de jeu, HUD exclu) pour comparer rendu et références."""
import sys, numpy as np
from PIL import Image
import colorsys
def stats(p):
    im=Image.open(p).convert('RGB'); w,h=im.size
    a=np.asarray(im).astype(float)[int(h*.08):int(h*.84),int(w*.02):int(w*.9)]/255
    lin=np.where(a<=.04045,a/12.92,((a+.055)/1.055)**2.4)
    L=(lin*[.2126,.7152,.0722]).sum(2)
    mx=a.max(2);mn=a.min(2);S=(mx-mn)/np.maximum(mx,1e-4)
    gy,gx=np.gradient(a.mean(2))
    detail=np.hypot(gx,gy).mean()*w/1672  # densité de détail normalisée
    q=Image.fromarray((a*255).astype('uint8')).resize((200,120)).quantize(5,method=Image.MEDIANCUT)
    pal=q.getpalette()[:15]; cnt=sorted(q.getcolors(),reverse=True)
    hexs=[('#%02x%02x%02x'%tuple(pal[i*3:i*3+3]),round(c/24000*100)) for c,i in cnt]
    return dict(lum=round(float(np.mean(L)),3),p5=round(float(np.percentile(L,5)),3),p95=round(float(np.percentile(L,95)),3),clip=round(float((mx>.98).mean()*100),2),sat=round(float(S.mean()),3),detail=round(float(detail),4),pal=hexs)
if __name__=='__main__':
    for p in sys.argv[1:]: print(p.split('/')[-1],stats(p))
