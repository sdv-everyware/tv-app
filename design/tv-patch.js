// tv-patch.js: the screen of tv-app, a home screen in the style of a smart TV's launcher: a
// wallpaper over the whole screen, a big greeting, the time and the weather in the corner, a
// ribbon of device cards that scrolls sideways (#tile in a horizontal list) and a row of round
// buttons at the bottom. Every control is a slot of a Library variant; the app (packages/
// services/display_safety/utils/tv-app) drives the #tv/* names. Applied to the EveryWare
// Library .fig, then ui-lib's dark-theme-patch.js for the dark Library:
//
//   design/build.sh     # the Library, this patch, the dark theme, data/tv.dcf

// the wallpaper's colours (not in the Library's palette, so the dark theme leaves them)
const WALL = [[0.05, 0.06, 0.16], [0.16, 0.08, 0.30], [0.42, 0.12, 0.38]];
const CARD_W = 438, CARD_H = 248, GAP = 24, LIFT = 20;

// roundtrip.js hands over fig-helpers.js (the node authoring) as the second argument
module.exports = function (msg, { key, C, FONT, authoring }) {
  const A = authoring(msg, 'tv');
  const { nodes, kids, frame, text, instance, solid, log } = A;
  // the app's own pictures of the weather, a set next to the Library's (weather-pictures.js)
  require('./weather-pictures.js')(A, C, '#tv-weather');
  // the Library's .fig (ui-lib design/build-library.sh): the screen goes on its Page 1
  const page = nodes.find(n => n.type === 'CANVAS' && n.name === 'Page 1');
  if (!page) throw new Error('no Page 1: build on the EveryWare Library .fig');
  if (nodes.some(n => n.name === '#tv-root')) throw new Error('#tv-root exists already');
  const variant = (set, name) => {
    const s = nodes.find(n => n.isStateGroup && n.name === set);
    const v = s && kids(key(s.guid)).find(n => n.type === 'SYMBOL' && n.name === name);
    if (!v) throw new Error(`no variant ${set} / ${name}`);
    return v;
  };
  const slot = (parent, name, x, y, v) => {
    const f = frame(parent, name, x, y, v.size.x, v.size.y, {
      bordersTakeSpace: false, stackMode: 'HORIZONTAL', stackPrimaryAlignItems: 'CENTER', stackCounterAlignItems: 'CENTER',
      stackPrimarySizing: 'FIXED', stackCounterSizing: 'FIXED', stackLayoutVersion: 1,
    });
    instance(f, `${name}/preview`, 0, 0, v);
    return f;
  };
  const stop = (rgb, position) => ({ color: { r: rgb[0], g: rgb[1], b: rgb[2], a: 1 }, position });

  // ---- the wallpaper: a diagonal gradient over the whole screen
  const root = frame(page, '#tv-root', -2400, 1400, 1920, 1080, {
    frameMaskDisabled: false,
    fillPaints: [{ type: 'GRADIENT_LINEAR', opacity: 1, visible: true, blendMode: 'NORMAL',
      stops: WALL.map((c, i) => stop(c, i / (WALL.length - 1))),
      transform: { m00: 0.7, m01: 0.5, m02: -0.1, m10: -0.5, m11: 0.7, m12: 0.4 } }],
  });

  // ---- the corner: the weather and the time
  slot(root, '#tv/weather/icon', 1420, 56, variant('#tv-weather', 'kind=partly'));
  text(root, '#tv/weather/temp', 1472, 50, 110, 52, '18°', 40, FONT.regular, C.white);
  text(root, '#tv/time', 1600, 44, 224, 64, '20:42', 52, FONT.semibold, C.white, 'RIGHT');

  // ---- the greeting
  text(root, '#tv/greeting', 96, 300, 1200, 96, 'Good evening', 80, FONT.semibold, C.white);
  text(root, '#tv/date', 100, 404, 900, 36, 'SUN, 8 NOV', 28, FONT.semibold, C.white, 'LEFT', 'TOP', { opacity: 0.7 });
  text(root, '#tv/weather/condition', 100, 448, 900, 32, 'Partly cloudy in Berlin', 24, FONT.regular, C.white, 'LEFT', 'TOP', { opacity: 0.7 });

  // ---- the ribbon: a clipping list slot of #tile, LIFT higher than a card (the chosen card rises)
  text(root, 'Ribbon heading', 96, 620, 600, 34, 'Your home', 28, FONT.semibold, C.white);
  const ribbon = frame(root, '#tv/cards', 96, 664, 1728, CARD_H + LIFT, { frameMaskDisabled: false });
  const kinds = ['toggle', 'slider', 'segmented', 'stepper'];
  kinds.forEach((k, i) => instance(ribbon, `#tv/cards/preview-${i + 1}`, i * (CARD_W + GAP), i === 0 ? 0 : LIFT, variant('#tile', `kind=${k}, state=default`)));

  // ---- the bottom row: round buttons
  const button = icon => variant('#icon-button', `kind=tonal, state=default, icon=${icon}, size=regular`);
  ['house', 'sliders-horizontal', 'wifi', 'settings-2'].forEach((icon, i) => {
    const name = ['#tv/nav/home', '#tv/nav/scenes', '#tv/nav/wifi', '#tv/nav/settings'][i];
    slot(root, name, 84 + i * 96, 968, button(icon));
  });

  log(`#tv-root: ${A.count()} nodes`);
  return msg;
};
