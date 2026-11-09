// weather-pictures.js: this app's own pictures of the weather, a component set of the app's
// design (not of the EveryWare Library: the Library has components, what an app shows with them
// is the app's). Seven pictures of 40 x 40 in the Library's palette: clouds muted, the sun, rain
// and the like in the accent colour. The app's patch calls it before it places an instance:
//
//   const { symbols } = require('./weather-pictures.js')(A, C, '#<app>-weather');
//
// The set goes into the Library frame of this app's file, below the Library's sets, so the
// .dcf has it (fig-to-dcf.sh keeps the root and that frame). The app's code names the same
// kinds (src/weather.rs): change both.
const KINDS = ['clear', 'partly', 'cloudy', 'fog', 'rain', 'snow', 'thunder'];

module.exports = function (A, C, name) {
  const { nodes, rect, ellipse, at, componentSet } = A;
  const lib = nodes.find(n => n.type === 'FRAME' && n.name === 'Library');
  if (!lib) throw new Error('no Library frame: build on the EveryWare Library .fig');
  const picture = (g, kind) => {
    // a cloud 32 x 20 with its top-left at (dx, dy)
    const cloud = (dx, dy, rgb) => {
      ellipse(g, 'cloud-1', dx + 0, dy + 8, 12, 12, rgb);
      ellipse(g, 'cloud-2', dx + 8, dy + 0, 17, 17, rgb);
      ellipse(g, 'cloud-3', dx + 20, dy + 7, 12, 12, rgb);
      rect(g, 'cloud-4', dx + 6, dy + 12, 22, 8, rgb, 4);
    };
    // a sun of radius r at (cx, cy): a disc and eight rays
    const sun = (cx, cy, r, rgb) => {
      ellipse(g, 'sun', cx - r, cy - r, 2 * r, 2 * r, rgb);
      const w = 0.3 * r, h = 0.55 * r, d = r * 1.55;
      for (let i = 0; i < 8; i++) {
        const a = i * Math.PI / 4, t = a - Math.PI / 2, c = Math.cos(t), sn = Math.sin(t);
        const mx = cx + d * Math.cos(a), my = cy + d * Math.sin(a);
        rect(g, 'ray', 0, 0, w, h, rgb, w / 2, { transform: at(mx - (w / 2 * c - h / 2 * sn), my - (w / 2 * sn + h / 2 * c), t * 180 / Math.PI) });
      }
    };
    // a short bar of w x h centred at (cx, cy), turned by deg
    const bar = (part, cx, cy, w, h, deg, rgb) => {
      const t = deg * Math.PI / 180, c = Math.cos(t), sn = Math.sin(t);
      rect(g, part, 0, 0, w, h, rgb, w / 2, { transform: at(cx - (w / 2 * c - h / 2 * sn), cy - (w / 2 * sn + h / 2 * c), deg) });
    };
    switch (kind) {
      case 'clear': sun(20, 20, 8, C.accent); break;
      case 'partly': sun(13, 13, 6, C.accent); cloud(6, 14, C.muted); break;
      case 'cloudy': cloud(4, 10, C.muted); break;
      case 'fog': cloud(4, 5, C.muted); bar('fog-1', 20, 30, 24, 2.5, 0, C.muted); bar('fog-2', 18, 35.5, 16, 2.5, 0, C.muted); break;
      case 'rain': cloud(4, 4, C.muted); [12, 20, 28].forEach((x, i) => bar(`drop-${i + 1}`, x, 32, 2.5, 8, 20, C.accent)); break;
      case 'snow': cloud(4, 4, C.muted); [12, 20, 28].forEach((x, i) => ellipse(g, `flake-${i + 1}`, x - 2, 30, 4, 4, C.accent)); break;
      case 'thunder': cloud(4, 3, C.muted); bar('bolt-1', 19, 27, 3, 10, 25, C.accent); bar('bolt-2', 21.5, 34, 3, 10, 25, C.accent); break;
    }
  };
  const made = componentSet(lib, name, 80, lib.size.y, [['kind', KINDS]], 40, 40, 24, 24, (s, v) => picture(s, v.kind));
  lib.size = { x: lib.size.x, y: lib.size.y + made.set.size.y + 80 };
  return made;
};
module.exports.KINDS = KINDS;
