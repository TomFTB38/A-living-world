// Persistence/save.js — format versionné, delta des modifications uniquement, somme de contrôle.
export const SAVE_VERSION = 1;
export function fnv(s) { let h = 2166136261; for (let i = 0; i < s.length; i++) h = Math.imul(h ^ s.charCodeAt(i), 16777619); return (h >>> 0).toString(16); }

export function serialize(st) {
  const body = JSON.stringify({ version: SAVE_VERSION, id: st.id, seed: st.seed, genVersion: st.genVersion, time: st.time, day: st.day, player: st.player, inventory: st.inventory, edits: st.edits });
  return JSON.stringify({ sum: fnv(body), body });
}
export function deserialize(text) {
  const o = JSON.parse(text);
  if (!o || typeof o.body !== 'string') throw new Error('Sauvegarde illisible.');
  if (fnv(o.body) !== o.sum) throw new Error('Sauvegarde corrompue : somme de contrôle invalide.');
  const b = JSON.parse(o.body);
  if (b.version > SAVE_VERSION) throw new Error('Sauvegarde créée par une version plus récente.');
  return b;
}
// Écriture en deux temps : si elle est interrompue, la version précédente reste intacte.
export function atomicWrite(storage, key, text) {
  storage.setItem(key + '.tmp', text);
  deserialize(storage.getItem(key + '.tmp'));
  storage.setItem(key, text);
  storage.removeItem(key + '.tmp');
}
export function safeRead(storage, key) {
  for (const k of [key, key + '.tmp']) {
    try { const t = storage.getItem(k); if (t) return deserialize(t); } catch (e) { /* version suivante */ }
  }
  return null;
}
