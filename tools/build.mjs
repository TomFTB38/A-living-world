// Assemble les modules ES en un seul fichier HTML autonome, sans dépendance externe.
import { readFileSync, writeFileSync, mkdirSync, readdirSync } from 'node:fs';
const strip = (f) => readFileSync(new URL('../src/' + f, import.meta.url), 'utf8')
  .replace(/^import .*$/gm, '').replace(/^export const /gm, "const ").replace(/^export (class|function|async function) /gm, '$1 ');
const world = ['core/noise.js', 'world/blocks.js', 'world/gen.js', 'world/mesher.js', 'world/store.js'];
const workerSrc = `(()=>{${world.map(strip).join('\n')}
const svc=new WorldService((m,t)=>postMessage(m,t));onmessage=(e)=>svc.handle(e.data);})();`;
// shaders : espace de noms S.*
const shaders = strip('render/shaders.js');
const names = [...shaders.matchAll(/^const (\w+)\s*=/gm)].map((m) => m[1]).filter((n) => n !== 'COMMON' && n !== 'LIGHTING');
const renderer = strip('render/renderer.js').replace(/\bS\.(\w+)/g, '$1');
const game = [...world, 'core/math.js', 'render/textures.js'].map(strip).join('\n') + shaders + renderer + ['game/save.js', 'game/client.js', 'game/player.js', 'game/items.js', 'game/character.js', 'game/animals.js', 'game/weather.js', 'game/music.js', 'game/audio.js', 'game/ui.js', 'game/main.js'].map(strip).join('\n');
const soundDir = new URL('../assets/sounds/', import.meta.url), sounds = Object.fromEntries(readdirSync(soundDir).filter((f) => f.endsWith('.mp3')).map((f) => [f.slice(0, -4), readFileSync(new URL(f, soundDir)).toString('base64')]));
const js = `(()=>{const SOUND_DATA=${JSON.stringify(sounds)};${game}})();`.replace("'__WORKER_SOURCE__'", () => JSON.stringify(workerSrc));
void names;
const html = readFileSync(new URL('../src/index.template.html', import.meta.url), 'utf8').replace('__GAME__', () => js.replace(/<\/script>/g, '<\\/script>'));
mkdirSync(new URL('../dist/', import.meta.url), { recursive: true });
writeFileSync(new URL('../dist/index.html', import.meta.url), html);
console.log('dist/index.html', (html.length / 1024).toFixed(0), 'Ko');
