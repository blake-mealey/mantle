"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __commonJS = (cb, mod) => function __require() {
  return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));

// node_modules/.pnpm/color-name@1.1.4/node_modules/color-name/index.js
var require_color_name = __commonJS({
  "node_modules/.pnpm/color-name@1.1.4/node_modules/color-name/index.js"(exports, module2) {
    "use strict";
    module2.exports = {
      "aliceblue": [240, 248, 255],
      "antiquewhite": [250, 235, 215],
      "aqua": [0, 255, 255],
      "aquamarine": [127, 255, 212],
      "azure": [240, 255, 255],
      "beige": [245, 245, 220],
      "bisque": [255, 228, 196],
      "black": [0, 0, 0],
      "blanchedalmond": [255, 235, 205],
      "blue": [0, 0, 255],
      "blueviolet": [138, 43, 226],
      "brown": [165, 42, 42],
      "burlywood": [222, 184, 135],
      "cadetblue": [95, 158, 160],
      "chartreuse": [127, 255, 0],
      "chocolate": [210, 105, 30],
      "coral": [255, 127, 80],
      "cornflowerblue": [100, 149, 237],
      "cornsilk": [255, 248, 220],
      "crimson": [220, 20, 60],
      "cyan": [0, 255, 255],
      "darkblue": [0, 0, 139],
      "darkcyan": [0, 139, 139],
      "darkgoldenrod": [184, 134, 11],
      "darkgray": [169, 169, 169],
      "darkgreen": [0, 100, 0],
      "darkgrey": [169, 169, 169],
      "darkkhaki": [189, 183, 107],
      "darkmagenta": [139, 0, 139],
      "darkolivegreen": [85, 107, 47],
      "darkorange": [255, 140, 0],
      "darkorchid": [153, 50, 204],
      "darkred": [139, 0, 0],
      "darksalmon": [233, 150, 122],
      "darkseagreen": [143, 188, 143],
      "darkslateblue": [72, 61, 139],
      "darkslategray": [47, 79, 79],
      "darkslategrey": [47, 79, 79],
      "darkturquoise": [0, 206, 209],
      "darkviolet": [148, 0, 211],
      "deeppink": [255, 20, 147],
      "deepskyblue": [0, 191, 255],
      "dimgray": [105, 105, 105],
      "dimgrey": [105, 105, 105],
      "dodgerblue": [30, 144, 255],
      "firebrick": [178, 34, 34],
      "floralwhite": [255, 250, 240],
      "forestgreen": [34, 139, 34],
      "fuchsia": [255, 0, 255],
      "gainsboro": [220, 220, 220],
      "ghostwhite": [248, 248, 255],
      "gold": [255, 215, 0],
      "goldenrod": [218, 165, 32],
      "gray": [128, 128, 128],
      "green": [0, 128, 0],
      "greenyellow": [173, 255, 47],
      "grey": [128, 128, 128],
      "honeydew": [240, 255, 240],
      "hotpink": [255, 105, 180],
      "indianred": [205, 92, 92],
      "indigo": [75, 0, 130],
      "ivory": [255, 255, 240],
      "khaki": [240, 230, 140],
      "lavender": [230, 230, 250],
      "lavenderblush": [255, 240, 245],
      "lawngreen": [124, 252, 0],
      "lemonchiffon": [255, 250, 205],
      "lightblue": [173, 216, 230],
      "lightcoral": [240, 128, 128],
      "lightcyan": [224, 255, 255],
      "lightgoldenrodyellow": [250, 250, 210],
      "lightgray": [211, 211, 211],
      "lightgreen": [144, 238, 144],
      "lightgrey": [211, 211, 211],
      "lightpink": [255, 182, 193],
      "lightsalmon": [255, 160, 122],
      "lightseagreen": [32, 178, 170],
      "lightskyblue": [135, 206, 250],
      "lightslategray": [119, 136, 153],
      "lightslategrey": [119, 136, 153],
      "lightsteelblue": [176, 196, 222],
      "lightyellow": [255, 255, 224],
      "lime": [0, 255, 0],
      "limegreen": [50, 205, 50],
      "linen": [250, 240, 230],
      "magenta": [255, 0, 255],
      "maroon": [128, 0, 0],
      "mediumaquamarine": [102, 205, 170],
      "mediumblue": [0, 0, 205],
      "mediumorchid": [186, 85, 211],
      "mediumpurple": [147, 112, 219],
      "mediumseagreen": [60, 179, 113],
      "mediumslateblue": [123, 104, 238],
      "mediumspringgreen": [0, 250, 154],
      "mediumturquoise": [72, 209, 204],
      "mediumvioletred": [199, 21, 133],
      "midnightblue": [25, 25, 112],
      "mintcream": [245, 255, 250],
      "mistyrose": [255, 228, 225],
      "moccasin": [255, 228, 181],
      "navajowhite": [255, 222, 173],
      "navy": [0, 0, 128],
      "oldlace": [253, 245, 230],
      "olive": [128, 128, 0],
      "olivedrab": [107, 142, 35],
      "orange": [255, 165, 0],
      "orangered": [255, 69, 0],
      "orchid": [218, 112, 214],
      "palegoldenrod": [238, 232, 170],
      "palegreen": [152, 251, 152],
      "paleturquoise": [175, 238, 238],
      "palevioletred": [219, 112, 147],
      "papayawhip": [255, 239, 213],
      "peachpuff": [255, 218, 185],
      "peru": [205, 133, 63],
      "pink": [255, 192, 203],
      "plum": [221, 160, 221],
      "powderblue": [176, 224, 230],
      "purple": [128, 0, 128],
      "rebeccapurple": [102, 51, 153],
      "red": [255, 0, 0],
      "rosybrown": [188, 143, 143],
      "royalblue": [65, 105, 225],
      "saddlebrown": [139, 69, 19],
      "salmon": [250, 128, 114],
      "sandybrown": [244, 164, 96],
      "seagreen": [46, 139, 87],
      "seashell": [255, 245, 238],
      "sienna": [160, 82, 45],
      "silver": [192, 192, 192],
      "skyblue": [135, 206, 235],
      "slateblue": [106, 90, 205],
      "slategray": [112, 128, 144],
      "slategrey": [112, 128, 144],
      "snow": [255, 250, 250],
      "springgreen": [0, 255, 127],
      "steelblue": [70, 130, 180],
      "tan": [210, 180, 140],
      "teal": [0, 128, 128],
      "thistle": [216, 191, 216],
      "tomato": [255, 99, 71],
      "turquoise": [64, 224, 208],
      "violet": [238, 130, 238],
      "wheat": [245, 222, 179],
      "white": [255, 255, 255],
      "whitesmoke": [245, 245, 245],
      "yellow": [255, 255, 0],
      "yellowgreen": [154, 205, 50]
    };
  }
});

// node_modules/.pnpm/color-convert@2.0.1/node_modules/color-convert/conversions.js
var require_conversions = __commonJS({
  "node_modules/.pnpm/color-convert@2.0.1/node_modules/color-convert/conversions.js"(exports, module2) {
    var cssKeywords = require_color_name();
    var reverseKeywords = {};
    for (const key of Object.keys(cssKeywords)) {
      reverseKeywords[cssKeywords[key]] = key;
    }
    var convert = {
      rgb: { channels: 3, labels: "rgb" },
      hsl: { channels: 3, labels: "hsl" },
      hsv: { channels: 3, labels: "hsv" },
      hwb: { channels: 3, labels: "hwb" },
      cmyk: { channels: 4, labels: "cmyk" },
      xyz: { channels: 3, labels: "xyz" },
      lab: { channels: 3, labels: "lab" },
      lch: { channels: 3, labels: "lch" },
      hex: { channels: 1, labels: ["hex"] },
      keyword: { channels: 1, labels: ["keyword"] },
      ansi16: { channels: 1, labels: ["ansi16"] },
      ansi256: { channels: 1, labels: ["ansi256"] },
      hcg: { channels: 3, labels: ["h", "c", "g"] },
      apple: { channels: 3, labels: ["r16", "g16", "b16"] },
      gray: { channels: 1, labels: ["gray"] }
    };
    module2.exports = convert;
    for (const model of Object.keys(convert)) {
      if (!("channels" in convert[model])) {
        throw new Error("missing channels property: " + model);
      }
      if (!("labels" in convert[model])) {
        throw new Error("missing channel labels property: " + model);
      }
      if (convert[model].labels.length !== convert[model].channels) {
        throw new Error("channel and label counts mismatch: " + model);
      }
      const { channels, labels } = convert[model];
      delete convert[model].channels;
      delete convert[model].labels;
      Object.defineProperty(convert[model], "channels", { value: channels });
      Object.defineProperty(convert[model], "labels", { value: labels });
    }
    convert.rgb.hsl = function(rgb) {
      const r = rgb[0] / 255;
      const g = rgb[1] / 255;
      const b = rgb[2] / 255;
      const min = Math.min(r, g, b);
      const max = Math.max(r, g, b);
      const delta = max - min;
      let h;
      let s;
      if (max === min) {
        h = 0;
      } else if (r === max) {
        h = (g - b) / delta;
      } else if (g === max) {
        h = 2 + (b - r) / delta;
      } else if (b === max) {
        h = 4 + (r - g) / delta;
      }
      h = Math.min(h * 60, 360);
      if (h < 0) {
        h += 360;
      }
      const l = (min + max) / 2;
      if (max === min) {
        s = 0;
      } else if (l <= 0.5) {
        s = delta / (max + min);
      } else {
        s = delta / (2 - max - min);
      }
      return [h, s * 100, l * 100];
    };
    convert.rgb.hsv = function(rgb) {
      let rdif;
      let gdif;
      let bdif;
      let h;
      let s;
      const r = rgb[0] / 255;
      const g = rgb[1] / 255;
      const b = rgb[2] / 255;
      const v = Math.max(r, g, b);
      const diff = v - Math.min(r, g, b);
      const diffc = function(c) {
        return (v - c) / 6 / diff + 1 / 2;
      };
      if (diff === 0) {
        h = 0;
        s = 0;
      } else {
        s = diff / v;
        rdif = diffc(r);
        gdif = diffc(g);
        bdif = diffc(b);
        if (r === v) {
          h = bdif - gdif;
        } else if (g === v) {
          h = 1 / 3 + rdif - bdif;
        } else if (b === v) {
          h = 2 / 3 + gdif - rdif;
        }
        if (h < 0) {
          h += 1;
        } else if (h > 1) {
          h -= 1;
        }
      }
      return [
        h * 360,
        s * 100,
        v * 100
      ];
    };
    convert.rgb.hwb = function(rgb) {
      const r = rgb[0];
      const g = rgb[1];
      let b = rgb[2];
      const h = convert.rgb.hsl(rgb)[0];
      const w = 1 / 255 * Math.min(r, Math.min(g, b));
      b = 1 - 1 / 255 * Math.max(r, Math.max(g, b));
      return [h, w * 100, b * 100];
    };
    convert.rgb.cmyk = function(rgb) {
      const r = rgb[0] / 255;
      const g = rgb[1] / 255;
      const b = rgb[2] / 255;
      const k = Math.min(1 - r, 1 - g, 1 - b);
      const c = (1 - r - k) / (1 - k) || 0;
      const m = (1 - g - k) / (1 - k) || 0;
      const y = (1 - b - k) / (1 - k) || 0;
      return [c * 100, m * 100, y * 100, k * 100];
    };
    function comparativeDistance(x, y) {
      return (x[0] - y[0]) ** 2 + (x[1] - y[1]) ** 2 + (x[2] - y[2]) ** 2;
    }
    convert.rgb.keyword = function(rgb) {
      const reversed = reverseKeywords[rgb];
      if (reversed) {
        return reversed;
      }
      let currentClosestDistance = Infinity;
      let currentClosestKeyword;
      for (const keyword of Object.keys(cssKeywords)) {
        const value = cssKeywords[keyword];
        const distance = comparativeDistance(rgb, value);
        if (distance < currentClosestDistance) {
          currentClosestDistance = distance;
          currentClosestKeyword = keyword;
        }
      }
      return currentClosestKeyword;
    };
    convert.keyword.rgb = function(keyword) {
      return cssKeywords[keyword];
    };
    convert.rgb.xyz = function(rgb) {
      let r = rgb[0] / 255;
      let g = rgb[1] / 255;
      let b = rgb[2] / 255;
      r = r > 0.04045 ? ((r + 0.055) / 1.055) ** 2.4 : r / 12.92;
      g = g > 0.04045 ? ((g + 0.055) / 1.055) ** 2.4 : g / 12.92;
      b = b > 0.04045 ? ((b + 0.055) / 1.055) ** 2.4 : b / 12.92;
      const x = r * 0.4124 + g * 0.3576 + b * 0.1805;
      const y = r * 0.2126 + g * 0.7152 + b * 0.0722;
      const z = r * 0.0193 + g * 0.1192 + b * 0.9505;
      return [x * 100, y * 100, z * 100];
    };
    convert.rgb.lab = function(rgb) {
      const xyz = convert.rgb.xyz(rgb);
      let x = xyz[0];
      let y = xyz[1];
      let z = xyz[2];
      x /= 95.047;
      y /= 100;
      z /= 108.883;
      x = x > 8856e-6 ? x ** (1 / 3) : 7.787 * x + 16 / 116;
      y = y > 8856e-6 ? y ** (1 / 3) : 7.787 * y + 16 / 116;
      z = z > 8856e-6 ? z ** (1 / 3) : 7.787 * z + 16 / 116;
      const l = 116 * y - 16;
      const a = 500 * (x - y);
      const b = 200 * (y - z);
      return [l, a, b];
    };
    convert.hsl.rgb = function(hsl) {
      const h = hsl[0] / 360;
      const s = hsl[1] / 100;
      const l = hsl[2] / 100;
      let t2;
      let t3;
      let val;
      if (s === 0) {
        val = l * 255;
        return [val, val, val];
      }
      if (l < 0.5) {
        t2 = l * (1 + s);
      } else {
        t2 = l + s - l * s;
      }
      const t1 = 2 * l - t2;
      const rgb = [0, 0, 0];
      for (let i = 0; i < 3; i++) {
        t3 = h + 1 / 3 * -(i - 1);
        if (t3 < 0) {
          t3++;
        }
        if (t3 > 1) {
          t3--;
        }
        if (6 * t3 < 1) {
          val = t1 + (t2 - t1) * 6 * t3;
        } else if (2 * t3 < 1) {
          val = t2;
        } else if (3 * t3 < 2) {
          val = t1 + (t2 - t1) * (2 / 3 - t3) * 6;
        } else {
          val = t1;
        }
        rgb[i] = val * 255;
      }
      return rgb;
    };
    convert.hsl.hsv = function(hsl) {
      const h = hsl[0];
      let s = hsl[1] / 100;
      let l = hsl[2] / 100;
      let smin = s;
      const lmin = Math.max(l, 0.01);
      l *= 2;
      s *= l <= 1 ? l : 2 - l;
      smin *= lmin <= 1 ? lmin : 2 - lmin;
      const v = (l + s) / 2;
      const sv = l === 0 ? 2 * smin / (lmin + smin) : 2 * s / (l + s);
      return [h, sv * 100, v * 100];
    };
    convert.hsv.rgb = function(hsv) {
      const h = hsv[0] / 60;
      const s = hsv[1] / 100;
      let v = hsv[2] / 100;
      const hi = Math.floor(h) % 6;
      const f = h - Math.floor(h);
      const p = 255 * v * (1 - s);
      const q = 255 * v * (1 - s * f);
      const t = 255 * v * (1 - s * (1 - f));
      v *= 255;
      switch (hi) {
        case 0:
          return [v, t, p];
        case 1:
          return [q, v, p];
        case 2:
          return [p, v, t];
        case 3:
          return [p, q, v];
        case 4:
          return [t, p, v];
        case 5:
          return [v, p, q];
      }
    };
    convert.hsv.hsl = function(hsv) {
      const h = hsv[0];
      const s = hsv[1] / 100;
      const v = hsv[2] / 100;
      const vmin = Math.max(v, 0.01);
      let sl;
      let l;
      l = (2 - s) * v;
      const lmin = (2 - s) * vmin;
      sl = s * vmin;
      sl /= lmin <= 1 ? lmin : 2 - lmin;
      sl = sl || 0;
      l /= 2;
      return [h, sl * 100, l * 100];
    };
    convert.hwb.rgb = function(hwb) {
      const h = hwb[0] / 360;
      let wh = hwb[1] / 100;
      let bl = hwb[2] / 100;
      const ratio = wh + bl;
      let f;
      if (ratio > 1) {
        wh /= ratio;
        bl /= ratio;
      }
      const i = Math.floor(6 * h);
      const v = 1 - bl;
      f = 6 * h - i;
      if ((i & 1) !== 0) {
        f = 1 - f;
      }
      const n = wh + f * (v - wh);
      let r;
      let g;
      let b;
      switch (i) {
        default:
        case 6:
        case 0:
          r = v;
          g = n;
          b = wh;
          break;
        case 1:
          r = n;
          g = v;
          b = wh;
          break;
        case 2:
          r = wh;
          g = v;
          b = n;
          break;
        case 3:
          r = wh;
          g = n;
          b = v;
          break;
        case 4:
          r = n;
          g = wh;
          b = v;
          break;
        case 5:
          r = v;
          g = wh;
          b = n;
          break;
      }
      return [r * 255, g * 255, b * 255];
    };
    convert.cmyk.rgb = function(cmyk) {
      const c = cmyk[0] / 100;
      const m = cmyk[1] / 100;
      const y = cmyk[2] / 100;
      const k = cmyk[3] / 100;
      const r = 1 - Math.min(1, c * (1 - k) + k);
      const g = 1 - Math.min(1, m * (1 - k) + k);
      const b = 1 - Math.min(1, y * (1 - k) + k);
      return [r * 255, g * 255, b * 255];
    };
    convert.xyz.rgb = function(xyz) {
      const x = xyz[0] / 100;
      const y = xyz[1] / 100;
      const z = xyz[2] / 100;
      let r;
      let g;
      let b;
      r = x * 3.2406 + y * -1.5372 + z * -0.4986;
      g = x * -0.9689 + y * 1.8758 + z * 0.0415;
      b = x * 0.0557 + y * -0.204 + z * 1.057;
      r = r > 31308e-7 ? 1.055 * r ** (1 / 2.4) - 0.055 : r * 12.92;
      g = g > 31308e-7 ? 1.055 * g ** (1 / 2.4) - 0.055 : g * 12.92;
      b = b > 31308e-7 ? 1.055 * b ** (1 / 2.4) - 0.055 : b * 12.92;
      r = Math.min(Math.max(0, r), 1);
      g = Math.min(Math.max(0, g), 1);
      b = Math.min(Math.max(0, b), 1);
      return [r * 255, g * 255, b * 255];
    };
    convert.xyz.lab = function(xyz) {
      let x = xyz[0];
      let y = xyz[1];
      let z = xyz[2];
      x /= 95.047;
      y /= 100;
      z /= 108.883;
      x = x > 8856e-6 ? x ** (1 / 3) : 7.787 * x + 16 / 116;
      y = y > 8856e-6 ? y ** (1 / 3) : 7.787 * y + 16 / 116;
      z = z > 8856e-6 ? z ** (1 / 3) : 7.787 * z + 16 / 116;
      const l = 116 * y - 16;
      const a = 500 * (x - y);
      const b = 200 * (y - z);
      return [l, a, b];
    };
    convert.lab.xyz = function(lab) {
      const l = lab[0];
      const a = lab[1];
      const b = lab[2];
      let x;
      let y;
      let z;
      y = (l + 16) / 116;
      x = a / 500 + y;
      z = y - b / 200;
      const y2 = y ** 3;
      const x2 = x ** 3;
      const z2 = z ** 3;
      y = y2 > 8856e-6 ? y2 : (y - 16 / 116) / 7.787;
      x = x2 > 8856e-6 ? x2 : (x - 16 / 116) / 7.787;
      z = z2 > 8856e-6 ? z2 : (z - 16 / 116) / 7.787;
      x *= 95.047;
      y *= 100;
      z *= 108.883;
      return [x, y, z];
    };
    convert.lab.lch = function(lab) {
      const l = lab[0];
      const a = lab[1];
      const b = lab[2];
      let h;
      const hr = Math.atan2(b, a);
      h = hr * 360 / 2 / Math.PI;
      if (h < 0) {
        h += 360;
      }
      const c = Math.sqrt(a * a + b * b);
      return [l, c, h];
    };
    convert.lch.lab = function(lch) {
      const l = lch[0];
      const c = lch[1];
      const h = lch[2];
      const hr = h / 360 * 2 * Math.PI;
      const a = c * Math.cos(hr);
      const b = c * Math.sin(hr);
      return [l, a, b];
    };
    convert.rgb.ansi16 = function(args, saturation = null) {
      const [r, g, b] = args;
      let value = saturation === null ? convert.rgb.hsv(args)[2] : saturation;
      value = Math.round(value / 50);
      if (value === 0) {
        return 30;
      }
      let ansi = 30 + (Math.round(b / 255) << 2 | Math.round(g / 255) << 1 | Math.round(r / 255));
      if (value === 2) {
        ansi += 60;
      }
      return ansi;
    };
    convert.hsv.ansi16 = function(args) {
      return convert.rgb.ansi16(convert.hsv.rgb(args), args[2]);
    };
    convert.rgb.ansi256 = function(args) {
      const r = args[0];
      const g = args[1];
      const b = args[2];
      if (r === g && g === b) {
        if (r < 8) {
          return 16;
        }
        if (r > 248) {
          return 231;
        }
        return Math.round((r - 8) / 247 * 24) + 232;
      }
      const ansi = 16 + 36 * Math.round(r / 255 * 5) + 6 * Math.round(g / 255 * 5) + Math.round(b / 255 * 5);
      return ansi;
    };
    convert.ansi16.rgb = function(args) {
      let color = args % 10;
      if (color === 0 || color === 7) {
        if (args > 50) {
          color += 3.5;
        }
        color = color / 10.5 * 255;
        return [color, color, color];
      }
      const mult = (~~(args > 50) + 1) * 0.5;
      const r = (color & 1) * mult * 255;
      const g = (color >> 1 & 1) * mult * 255;
      const b = (color >> 2 & 1) * mult * 255;
      return [r, g, b];
    };
    convert.ansi256.rgb = function(args) {
      if (args >= 232) {
        const c = (args - 232) * 10 + 8;
        return [c, c, c];
      }
      args -= 16;
      let rem;
      const r = Math.floor(args / 36) / 5 * 255;
      const g = Math.floor((rem = args % 36) / 6) / 5 * 255;
      const b = rem % 6 / 5 * 255;
      return [r, g, b];
    };
    convert.rgb.hex = function(args) {
      const integer = ((Math.round(args[0]) & 255) << 16) + ((Math.round(args[1]) & 255) << 8) + (Math.round(args[2]) & 255);
      const string = integer.toString(16).toUpperCase();
      return "000000".substring(string.length) + string;
    };
    convert.hex.rgb = function(args) {
      const match = args.toString(16).match(/[a-f0-9]{6}|[a-f0-9]{3}/i);
      if (!match) {
        return [0, 0, 0];
      }
      let colorString = match[0];
      if (match[0].length === 3) {
        colorString = colorString.split("").map((char) => {
          return char + char;
        }).join("");
      }
      const integer = parseInt(colorString, 16);
      const r = integer >> 16 & 255;
      const g = integer >> 8 & 255;
      const b = integer & 255;
      return [r, g, b];
    };
    convert.rgb.hcg = function(rgb) {
      const r = rgb[0] / 255;
      const g = rgb[1] / 255;
      const b = rgb[2] / 255;
      const max = Math.max(Math.max(r, g), b);
      const min = Math.min(Math.min(r, g), b);
      const chroma = max - min;
      let grayscale;
      let hue;
      if (chroma < 1) {
        grayscale = min / (1 - chroma);
      } else {
        grayscale = 0;
      }
      if (chroma <= 0) {
        hue = 0;
      } else if (max === r) {
        hue = (g - b) / chroma % 6;
      } else if (max === g) {
        hue = 2 + (b - r) / chroma;
      } else {
        hue = 4 + (r - g) / chroma;
      }
      hue /= 6;
      hue %= 1;
      return [hue * 360, chroma * 100, grayscale * 100];
    };
    convert.hsl.hcg = function(hsl) {
      const s = hsl[1] / 100;
      const l = hsl[2] / 100;
      const c = l < 0.5 ? 2 * s * l : 2 * s * (1 - l);
      let f = 0;
      if (c < 1) {
        f = (l - 0.5 * c) / (1 - c);
      }
      return [hsl[0], c * 100, f * 100];
    };
    convert.hsv.hcg = function(hsv) {
      const s = hsv[1] / 100;
      const v = hsv[2] / 100;
      const c = s * v;
      let f = 0;
      if (c < 1) {
        f = (v - c) / (1 - c);
      }
      return [hsv[0], c * 100, f * 100];
    };
    convert.hcg.rgb = function(hcg) {
      const h = hcg[0] / 360;
      const c = hcg[1] / 100;
      const g = hcg[2] / 100;
      if (c === 0) {
        return [g * 255, g * 255, g * 255];
      }
      const pure = [0, 0, 0];
      const hi = h % 1 * 6;
      const v = hi % 1;
      const w = 1 - v;
      let mg = 0;
      switch (Math.floor(hi)) {
        case 0:
          pure[0] = 1;
          pure[1] = v;
          pure[2] = 0;
          break;
        case 1:
          pure[0] = w;
          pure[1] = 1;
          pure[2] = 0;
          break;
        case 2:
          pure[0] = 0;
          pure[1] = 1;
          pure[2] = v;
          break;
        case 3:
          pure[0] = 0;
          pure[1] = w;
          pure[2] = 1;
          break;
        case 4:
          pure[0] = v;
          pure[1] = 0;
          pure[2] = 1;
          break;
        default:
          pure[0] = 1;
          pure[1] = 0;
          pure[2] = w;
      }
      mg = (1 - c) * g;
      return [
        (c * pure[0] + mg) * 255,
        (c * pure[1] + mg) * 255,
        (c * pure[2] + mg) * 255
      ];
    };
    convert.hcg.hsv = function(hcg) {
      const c = hcg[1] / 100;
      const g = hcg[2] / 100;
      const v = c + g * (1 - c);
      let f = 0;
      if (v > 0) {
        f = c / v;
      }
      return [hcg[0], f * 100, v * 100];
    };
    convert.hcg.hsl = function(hcg) {
      const c = hcg[1] / 100;
      const g = hcg[2] / 100;
      const l = g * (1 - c) + 0.5 * c;
      let s = 0;
      if (l > 0 && l < 0.5) {
        s = c / (2 * l);
      } else if (l >= 0.5 && l < 1) {
        s = c / (2 * (1 - l));
      }
      return [hcg[0], s * 100, l * 100];
    };
    convert.hcg.hwb = function(hcg) {
      const c = hcg[1] / 100;
      const g = hcg[2] / 100;
      const v = c + g * (1 - c);
      return [hcg[0], (v - c) * 100, (1 - v) * 100];
    };
    convert.hwb.hcg = function(hwb) {
      const w = hwb[1] / 100;
      const b = hwb[2] / 100;
      const v = 1 - b;
      const c = v - w;
      let g = 0;
      if (c < 1) {
        g = (v - c) / (1 - c);
      }
      return [hwb[0], c * 100, g * 100];
    };
    convert.apple.rgb = function(apple) {
      return [apple[0] / 65535 * 255, apple[1] / 65535 * 255, apple[2] / 65535 * 255];
    };
    convert.rgb.apple = function(rgb) {
      return [rgb[0] / 255 * 65535, rgb[1] / 255 * 65535, rgb[2] / 255 * 65535];
    };
    convert.gray.rgb = function(args) {
      return [args[0] / 100 * 255, args[0] / 100 * 255, args[0] / 100 * 255];
    };
    convert.gray.hsl = function(args) {
      return [0, 0, args[0]];
    };
    convert.gray.hsv = convert.gray.hsl;
    convert.gray.hwb = function(gray) {
      return [0, 100, gray[0]];
    };
    convert.gray.cmyk = function(gray) {
      return [0, 0, 0, gray[0]];
    };
    convert.gray.lab = function(gray) {
      return [gray[0], 0, 0];
    };
    convert.gray.hex = function(gray) {
      const val = Math.round(gray[0] / 100 * 255) & 255;
      const integer = (val << 16) + (val << 8) + val;
      const string = integer.toString(16).toUpperCase();
      return "000000".substring(string.length) + string;
    };
    convert.rgb.gray = function(rgb) {
      const val = (rgb[0] + rgb[1] + rgb[2]) / 3;
      return [val / 255 * 100];
    };
  }
});

// node_modules/.pnpm/color-convert@2.0.1/node_modules/color-convert/route.js
var require_route = __commonJS({
  "node_modules/.pnpm/color-convert@2.0.1/node_modules/color-convert/route.js"(exports, module2) {
    var conversions = require_conversions();
    function buildGraph() {
      const graph = {};
      const models = Object.keys(conversions);
      for (let len = models.length, i = 0; i < len; i++) {
        graph[models[i]] = {
          distance: -1,
          parent: null
        };
      }
      return graph;
    }
    function deriveBFS(fromModel) {
      const graph = buildGraph();
      const queue = [fromModel];
      graph[fromModel].distance = 0;
      while (queue.length) {
        const current = queue.pop();
        const adjacents = Object.keys(conversions[current]);
        for (let len = adjacents.length, i = 0; i < len; i++) {
          const adjacent = adjacents[i];
          const node = graph[adjacent];
          if (node.distance === -1) {
            node.distance = graph[current].distance + 1;
            node.parent = current;
            queue.unshift(adjacent);
          }
        }
      }
      return graph;
    }
    function link(from, to) {
      return function(args) {
        return to(from(args));
      };
    }
    function wrapConversion(toModel, graph) {
      const path = [graph[toModel].parent, toModel];
      let fn = conversions[graph[toModel].parent][toModel];
      let cur = graph[toModel].parent;
      while (graph[cur].parent) {
        path.unshift(graph[cur].parent);
        fn = link(conversions[graph[cur].parent][cur], fn);
        cur = graph[cur].parent;
      }
      fn.conversion = path;
      return fn;
    }
    module2.exports = function(fromModel) {
      const graph = deriveBFS(fromModel);
      const conversion = {};
      const models = Object.keys(graph);
      for (let len = models.length, i = 0; i < len; i++) {
        const toModel = models[i];
        const node = graph[toModel];
        if (node.parent === null) {
          continue;
        }
        conversion[toModel] = wrapConversion(toModel, graph);
      }
      return conversion;
    };
  }
});

// node_modules/.pnpm/color-convert@2.0.1/node_modules/color-convert/index.js
var require_color_convert = __commonJS({
  "node_modules/.pnpm/color-convert@2.0.1/node_modules/color-convert/index.js"(exports, module2) {
    var conversions = require_conversions();
    var route = require_route();
    var convert = {};
    var models = Object.keys(conversions);
    function wrapRaw(fn) {
      const wrappedFn = function(...args) {
        const arg0 = args[0];
        if (arg0 === void 0 || arg0 === null) {
          return arg0;
        }
        if (arg0.length > 1) {
          args = arg0;
        }
        return fn(args);
      };
      if ("conversion" in fn) {
        wrappedFn.conversion = fn.conversion;
      }
      return wrappedFn;
    }
    function wrapRounded(fn) {
      const wrappedFn = function(...args) {
        const arg0 = args[0];
        if (arg0 === void 0 || arg0 === null) {
          return arg0;
        }
        if (arg0.length > 1) {
          args = arg0;
        }
        const result = fn(args);
        if (typeof result === "object") {
          for (let len = result.length, i = 0; i < len; i++) {
            result[i] = Math.round(result[i]);
          }
        }
        return result;
      };
      if ("conversion" in fn) {
        wrappedFn.conversion = fn.conversion;
      }
      return wrappedFn;
    }
    models.forEach((fromModel) => {
      convert[fromModel] = {};
      Object.defineProperty(convert[fromModel], "channels", { value: conversions[fromModel].channels });
      Object.defineProperty(convert[fromModel], "labels", { value: conversions[fromModel].labels });
      const routes = route(fromModel);
      const routeModels = Object.keys(routes);
      routeModels.forEach((toModel) => {
        const fn = routes[toModel];
        convert[fromModel][toModel] = wrapRounded(fn);
        convert[fromModel][toModel].raw = wrapRaw(fn);
      });
    });
    module2.exports = convert;
  }
});

// node_modules/.pnpm/ansi-styles@4.3.0/node_modules/ansi-styles/index.js
var require_ansi_styles = __commonJS({
  "node_modules/.pnpm/ansi-styles@4.3.0/node_modules/ansi-styles/index.js"(exports, module2) {
    "use strict";
    var wrapAnsi16 = (fn, offset) => (...args) => {
      const code = fn(...args);
      return `\x1B[${code + offset}m`;
    };
    var wrapAnsi256 = (fn, offset) => (...args) => {
      const code = fn(...args);
      return `\x1B[${38 + offset};5;${code}m`;
    };
    var wrapAnsi16m = (fn, offset) => (...args) => {
      const rgb = fn(...args);
      return `\x1B[${38 + offset};2;${rgb[0]};${rgb[1]};${rgb[2]}m`;
    };
    var ansi2ansi = (n) => n;
    var rgb2rgb = (r, g, b) => [r, g, b];
    var setLazyProperty = (object, property, get) => {
      Object.defineProperty(object, property, {
        get: () => {
          const value = get();
          Object.defineProperty(object, property, {
            value,
            enumerable: true,
            configurable: true
          });
          return value;
        },
        enumerable: true,
        configurable: true
      });
    };
    var colorConvert;
    var makeDynamicStyles = (wrap, targetSpace, identity, isBackground) => {
      if (colorConvert === void 0) {
        colorConvert = require_color_convert();
      }
      const offset = isBackground ? 10 : 0;
      const styles = {};
      for (const [sourceSpace, suite] of Object.entries(colorConvert)) {
        const name = sourceSpace === "ansi16" ? "ansi" : sourceSpace;
        if (sourceSpace === targetSpace) {
          styles[name] = wrap(identity, offset);
        } else if (typeof suite === "object") {
          styles[name] = wrap(suite[targetSpace], offset);
        }
      }
      return styles;
    };
    function assembleStyles() {
      const codes = /* @__PURE__ */ new Map();
      const styles = {
        modifier: {
          reset: [0, 0],
          bold: [1, 22],
          dim: [2, 22],
          italic: [3, 23],
          underline: [4, 24],
          inverse: [7, 27],
          hidden: [8, 28],
          strikethrough: [9, 29]
        },
        color: {
          black: [30, 39],
          red: [31, 39],
          green: [32, 39],
          yellow: [33, 39],
          blue: [34, 39],
          magenta: [35, 39],
          cyan: [36, 39],
          white: [37, 39],
          blackBright: [90, 39],
          redBright: [91, 39],
          greenBright: [92, 39],
          yellowBright: [93, 39],
          blueBright: [94, 39],
          magentaBright: [95, 39],
          cyanBright: [96, 39],
          whiteBright: [97, 39]
        },
        bgColor: {
          bgBlack: [40, 49],
          bgRed: [41, 49],
          bgGreen: [42, 49],
          bgYellow: [43, 49],
          bgBlue: [44, 49],
          bgMagenta: [45, 49],
          bgCyan: [46, 49],
          bgWhite: [47, 49],
          bgBlackBright: [100, 49],
          bgRedBright: [101, 49],
          bgGreenBright: [102, 49],
          bgYellowBright: [103, 49],
          bgBlueBright: [104, 49],
          bgMagentaBright: [105, 49],
          bgCyanBright: [106, 49],
          bgWhiteBright: [107, 49]
        }
      };
      styles.color.gray = styles.color.blackBright;
      styles.bgColor.bgGray = styles.bgColor.bgBlackBright;
      styles.color.grey = styles.color.blackBright;
      styles.bgColor.bgGrey = styles.bgColor.bgBlackBright;
      for (const [groupName, group] of Object.entries(styles)) {
        for (const [styleName, style] of Object.entries(group)) {
          styles[styleName] = {
            open: `\x1B[${style[0]}m`,
            close: `\x1B[${style[1]}m`
          };
          group[styleName] = styles[styleName];
          codes.set(style[0], style[1]);
        }
        Object.defineProperty(styles, groupName, {
          value: group,
          enumerable: false
        });
      }
      Object.defineProperty(styles, "codes", {
        value: codes,
        enumerable: false
      });
      styles.color.close = "\x1B[39m";
      styles.bgColor.close = "\x1B[49m";
      setLazyProperty(styles.color, "ansi", () => makeDynamicStyles(wrapAnsi16, "ansi16", ansi2ansi, false));
      setLazyProperty(styles.color, "ansi256", () => makeDynamicStyles(wrapAnsi256, "ansi256", ansi2ansi, false));
      setLazyProperty(styles.color, "ansi16m", () => makeDynamicStyles(wrapAnsi16m, "rgb", rgb2rgb, false));
      setLazyProperty(styles.bgColor, "ansi", () => makeDynamicStyles(wrapAnsi16, "ansi16", ansi2ansi, true));
      setLazyProperty(styles.bgColor, "ansi256", () => makeDynamicStyles(wrapAnsi256, "ansi256", ansi2ansi, true));
      setLazyProperty(styles.bgColor, "ansi16m", () => makeDynamicStyles(wrapAnsi16m, "rgb", rgb2rgb, true));
      return styles;
    }
    Object.defineProperty(module2, "exports", {
      enumerable: true,
      get: assembleStyles
    });
  }
});

// node_modules/.pnpm/has-flag@4.0.0/node_modules/has-flag/index.js
var require_has_flag = __commonJS({
  "node_modules/.pnpm/has-flag@4.0.0/node_modules/has-flag/index.js"(exports, module2) {
    "use strict";
    module2.exports = (flag, argv = process.argv) => {
      const prefix = flag.startsWith("-") ? "" : flag.length === 1 ? "-" : "--";
      const position = argv.indexOf(prefix + flag);
      const terminatorPosition = argv.indexOf("--");
      return position !== -1 && (terminatorPosition === -1 || position < terminatorPosition);
    };
  }
});

// node_modules/.pnpm/supports-color@7.2.0/node_modules/supports-color/index.js
var require_supports_color = __commonJS({
  "node_modules/.pnpm/supports-color@7.2.0/node_modules/supports-color/index.js"(exports, module2) {
    "use strict";
    var os = require("os");
    var tty = require("tty");
    var hasFlag = require_has_flag();
    var { env } = process;
    var forceColor;
    if (hasFlag("no-color") || hasFlag("no-colors") || hasFlag("color=false") || hasFlag("color=never")) {
      forceColor = 0;
    } else if (hasFlag("color") || hasFlag("colors") || hasFlag("color=true") || hasFlag("color=always")) {
      forceColor = 1;
    }
    if ("FORCE_COLOR" in env) {
      if (env.FORCE_COLOR === "true") {
        forceColor = 1;
      } else if (env.FORCE_COLOR === "false") {
        forceColor = 0;
      } else {
        forceColor = env.FORCE_COLOR.length === 0 ? 1 : Math.min(parseInt(env.FORCE_COLOR, 10), 3);
      }
    }
    function translateLevel(level) {
      if (level === 0) {
        return false;
      }
      return {
        level,
        hasBasic: true,
        has256: level >= 2,
        has16m: level >= 3
      };
    }
    function supportsColor(haveStream, streamIsTTY) {
      if (forceColor === 0) {
        return 0;
      }
      if (hasFlag("color=16m") || hasFlag("color=full") || hasFlag("color=truecolor")) {
        return 3;
      }
      if (hasFlag("color=256")) {
        return 2;
      }
      if (haveStream && !streamIsTTY && forceColor === void 0) {
        return 0;
      }
      const min = forceColor || 0;
      if (env.TERM === "dumb") {
        return min;
      }
      if (process.platform === "win32") {
        const osRelease = os.release().split(".");
        if (Number(osRelease[0]) >= 10 && Number(osRelease[2]) >= 10586) {
          return Number(osRelease[2]) >= 14931 ? 3 : 2;
        }
        return 1;
      }
      if ("CI" in env) {
        if (["TRAVIS", "CIRCLECI", "APPVEYOR", "GITLAB_CI", "GITHUB_ACTIONS", "BUILDKITE"].some((sign) => sign in env) || env.CI_NAME === "codeship") {
          return 1;
        }
        return min;
      }
      if ("TEAMCITY_VERSION" in env) {
        return /^(9\.(0*[1-9]\d*)\.|\d{2,}\.)/.test(env.TEAMCITY_VERSION) ? 1 : 0;
      }
      if (env.COLORTERM === "truecolor") {
        return 3;
      }
      if ("TERM_PROGRAM" in env) {
        const version = parseInt((env.TERM_PROGRAM_VERSION || "").split(".")[0], 10);
        switch (env.TERM_PROGRAM) {
          case "iTerm.app":
            return version >= 3 ? 3 : 2;
          case "Apple_Terminal":
            return 2;
        }
      }
      if (/-256(color)?$/i.test(env.TERM)) {
        return 2;
      }
      if (/^screen|^xterm|^vt100|^vt220|^rxvt|color|ansi|cygwin|linux/i.test(env.TERM)) {
        return 1;
      }
      if ("COLORTERM" in env) {
        return 1;
      }
      return min;
    }
    function getSupportLevel(stream) {
      const level = supportsColor(stream, stream && stream.isTTY);
      return translateLevel(level);
    }
    module2.exports = {
      supportsColor: getSupportLevel,
      stdout: translateLevel(supportsColor(true, tty.isatty(1))),
      stderr: translateLevel(supportsColor(true, tty.isatty(2)))
    };
  }
});

// node_modules/.pnpm/chalk@4.1.2/node_modules/chalk/source/util.js
var require_util = __commonJS({
  "node_modules/.pnpm/chalk@4.1.2/node_modules/chalk/source/util.js"(exports, module2) {
    "use strict";
    var stringReplaceAll = (string, substring, replacer) => {
      let index = string.indexOf(substring);
      if (index === -1) {
        return string;
      }
      const substringLength = substring.length;
      let endIndex = 0;
      let returnValue = "";
      do {
        returnValue += string.substr(endIndex, index - endIndex) + substring + replacer;
        endIndex = index + substringLength;
        index = string.indexOf(substring, endIndex);
      } while (index !== -1);
      returnValue += string.substr(endIndex);
      return returnValue;
    };
    var stringEncaseCRLFWithFirstIndex = (string, prefix, postfix, index) => {
      let endIndex = 0;
      let returnValue = "";
      do {
        const gotCR = string[index - 1] === "\r";
        returnValue += string.substr(endIndex, (gotCR ? index - 1 : index) - endIndex) + prefix + (gotCR ? "\r\n" : "\n") + postfix;
        endIndex = index + 1;
        index = string.indexOf("\n", endIndex);
      } while (index !== -1);
      returnValue += string.substr(endIndex);
      return returnValue;
    };
    module2.exports = {
      stringReplaceAll,
      stringEncaseCRLFWithFirstIndex
    };
  }
});

// node_modules/.pnpm/chalk@4.1.2/node_modules/chalk/source/templates.js
var require_templates = __commonJS({
  "node_modules/.pnpm/chalk@4.1.2/node_modules/chalk/source/templates.js"(exports, module2) {
    "use strict";
    var TEMPLATE_REGEX2 = /(?:\\(u(?:[a-f\d]{4}|\{[a-f\d]{1,6}\})|x[a-f\d]{2}|.))|(?:\{(~)?(\w+(?:\([^)]*\))?(?:\.\w+(?:\([^)]*\))?)*)(?:[ \t]|(?=\r?\n)))|(\})|((?:.|[\r\n\f])+?)/gi;
    var STYLE_REGEX2 = /(?:^|\.)(\w+)(?:\(([^)]*)\))?/g;
    var STRING_REGEX2 = /^(['"])((?:\\.|(?!\1)[^\\])*)\1$/;
    var ESCAPE_REGEX2 = /\\(u(?:[a-f\d]{4}|{[a-f\d]{1,6}})|x[a-f\d]{2}|.)|([^\\])/gi;
    var ESCAPES2 = /* @__PURE__ */ new Map([
      ["n", "\n"],
      ["r", "\r"],
      ["t", "	"],
      ["b", "\b"],
      ["f", "\f"],
      ["v", "\v"],
      ["0", "\0"],
      ["\\", "\\"],
      ["e", "\x1B"],
      ["a", "\x07"]
    ]);
    function unescape2(c) {
      const u2 = c[0] === "u";
      const bracket = c[1] === "{";
      if (u2 && !bracket && c.length === 5 || c[0] === "x" && c.length === 3) {
        return String.fromCharCode(parseInt(c.slice(1), 16));
      }
      if (u2 && bracket) {
        return String.fromCodePoint(parseInt(c.slice(2, -1), 16));
      }
      return ESCAPES2.get(c) || c;
    }
    function parseArguments2(name, arguments_) {
      const results = [];
      const chunks = arguments_.trim().split(/\s*,\s*/g);
      let matches;
      for (const chunk of chunks) {
        const number = Number(chunk);
        if (!Number.isNaN(number)) {
          results.push(number);
        } else if (matches = chunk.match(STRING_REGEX2)) {
          results.push(matches[2].replace(ESCAPE_REGEX2, (m, escape, character) => escape ? unescape2(escape) : character));
        } else {
          throw new Error(`Invalid Chalk template style argument: ${chunk} (in style '${name}')`);
        }
      }
      return results;
    }
    function parseStyle2(style) {
      STYLE_REGEX2.lastIndex = 0;
      const results = [];
      let matches;
      while ((matches = STYLE_REGEX2.exec(style)) !== null) {
        const name = matches[1];
        if (matches[2]) {
          const args = parseArguments2(name, matches[2]);
          results.push([name].concat(args));
        } else {
          results.push([name]);
        }
      }
      return results;
    }
    function buildStyle(chalk2, styles) {
      const enabled = {};
      for (const layer of styles) {
        for (const style of layer.styles) {
          enabled[style[0]] = layer.inverse ? null : style.slice(1);
        }
      }
      let current = chalk2;
      for (const [styleName, styles2] of Object.entries(enabled)) {
        if (!Array.isArray(styles2)) {
          continue;
        }
        if (!(styleName in current)) {
          throw new Error(`Unknown Chalk style: ${styleName}`);
        }
        current = styles2.length > 0 ? current[styleName](...styles2) : current[styleName];
      }
      return current;
    }
    module2.exports = (chalk2, temporary) => {
      const styles = [];
      const chunks = [];
      let chunk = [];
      temporary.replace(TEMPLATE_REGEX2, (m, escapeCharacter, inverse, style, close, character) => {
        if (escapeCharacter) {
          chunk.push(unescape2(escapeCharacter));
        } else if (style) {
          const string = chunk.join("");
          chunk = [];
          chunks.push(styles.length === 0 ? string : buildStyle(chalk2, styles)(string));
          styles.push({ inverse, styles: parseStyle2(style) });
        } else if (close) {
          if (styles.length === 0) {
            throw new Error("Found extraneous } in Chalk template literal");
          }
          chunks.push(buildStyle(chalk2, styles)(chunk.join("")));
          chunk = [];
          styles.pop();
        } else {
          chunk.push(character);
        }
      });
      chunks.push(chunk.join(""));
      if (styles.length > 0) {
        const errMessage = `Chalk template literal is missing ${styles.length} closing bracket${styles.length === 1 ? "" : "s"} (\`}\`)`;
        throw new Error(errMessage);
      }
      return chunks.join("");
    };
  }
});

// node_modules/.pnpm/chalk@4.1.2/node_modules/chalk/source/index.js
var require_source = __commonJS({
  "node_modules/.pnpm/chalk@4.1.2/node_modules/chalk/source/index.js"(exports, module2) {
    "use strict";
    var ansiStyles = require_ansi_styles();
    var { stdout: stdoutColor, stderr: stderrColor } = require_supports_color();
    var {
      stringReplaceAll,
      stringEncaseCRLFWithFirstIndex
    } = require_util();
    var { isArray } = Array;
    var levelMapping = [
      "ansi",
      "ansi",
      "ansi256",
      "ansi16m"
    ];
    var styles = /* @__PURE__ */ Object.create(null);
    var applyOptions = (object, options = {}) => {
      if (options.level && !(Number.isInteger(options.level) && options.level >= 0 && options.level <= 3)) {
        throw new Error("The `level` option should be an integer from 0 to 3");
      }
      const colorLevel = stdoutColor ? stdoutColor.level : 0;
      object.level = options.level === void 0 ? colorLevel : options.level;
    };
    var ChalkClass = class {
      constructor(options) {
        return chalkFactory(options);
      }
    };
    var chalkFactory = (options) => {
      const chalk3 = {};
      applyOptions(chalk3, options);
      chalk3.template = (...arguments_) => chalkTag(chalk3.template, ...arguments_);
      Object.setPrototypeOf(chalk3, Chalk.prototype);
      Object.setPrototypeOf(chalk3.template, chalk3);
      chalk3.template.constructor = () => {
        throw new Error("`chalk.constructor()` is deprecated. Use `new chalk.Instance()` instead.");
      };
      chalk3.template.Instance = ChalkClass;
      return chalk3.template;
    };
    function Chalk(options) {
      return chalkFactory(options);
    }
    for (const [styleName, style] of Object.entries(ansiStyles)) {
      styles[styleName] = {
        get() {
          const builder = createBuilder(this, createStyler(style.open, style.close, this._styler), this._isEmpty);
          Object.defineProperty(this, styleName, { value: builder });
          return builder;
        }
      };
    }
    styles.visible = {
      get() {
        const builder = createBuilder(this, this._styler, true);
        Object.defineProperty(this, "visible", { value: builder });
        return builder;
      }
    };
    var usedModels = ["rgb", "hex", "keyword", "hsl", "hsv", "hwb", "ansi", "ansi256"];
    for (const model of usedModels) {
      styles[model] = {
        get() {
          const { level } = this;
          return function(...arguments_) {
            const styler = createStyler(ansiStyles.color[levelMapping[level]][model](...arguments_), ansiStyles.color.close, this._styler);
            return createBuilder(this, styler, this._isEmpty);
          };
        }
      };
    }
    for (const model of usedModels) {
      const bgModel = "bg" + model[0].toUpperCase() + model.slice(1);
      styles[bgModel] = {
        get() {
          const { level } = this;
          return function(...arguments_) {
            const styler = createStyler(ansiStyles.bgColor[levelMapping[level]][model](...arguments_), ansiStyles.bgColor.close, this._styler);
            return createBuilder(this, styler, this._isEmpty);
          };
        }
      };
    }
    var proto = Object.defineProperties(() => {
    }, {
      ...styles,
      level: {
        enumerable: true,
        get() {
          return this._generator.level;
        },
        set(level) {
          this._generator.level = level;
        }
      }
    });
    var createStyler = (open, close, parent) => {
      let openAll;
      let closeAll;
      if (parent === void 0) {
        openAll = open;
        closeAll = close;
      } else {
        openAll = parent.openAll + open;
        closeAll = close + parent.closeAll;
      }
      return {
        open,
        close,
        openAll,
        closeAll,
        parent
      };
    };
    var createBuilder = (self, _styler, _isEmpty) => {
      const builder = (...arguments_) => {
        if (isArray(arguments_[0]) && isArray(arguments_[0].raw)) {
          return applyStyle(builder, chalkTag(builder, ...arguments_));
        }
        return applyStyle(builder, arguments_.length === 1 ? "" + arguments_[0] : arguments_.join(" "));
      };
      Object.setPrototypeOf(builder, proto);
      builder._generator = self;
      builder._styler = _styler;
      builder._isEmpty = _isEmpty;
      return builder;
    };
    var applyStyle = (self, string) => {
      if (self.level <= 0 || !string) {
        return self._isEmpty ? "" : string;
      }
      let styler = self._styler;
      if (styler === void 0) {
        return string;
      }
      const { openAll, closeAll } = styler;
      if (string.indexOf("\x1B") !== -1) {
        while (styler !== void 0) {
          string = stringReplaceAll(string, styler.close, styler.open);
          styler = styler.parent;
        }
      }
      const lfIndex = string.indexOf("\n");
      if (lfIndex !== -1) {
        string = stringEncaseCRLFWithFirstIndex(string, closeAll, openAll, lfIndex);
      }
      return openAll + string + closeAll;
    };
    var template2;
    var chalkTag = (chalk3, ...strings) => {
      const [firstString] = strings;
      if (!isArray(firstString) || !isArray(firstString.raw)) {
        return strings.join(" ");
      }
      const arguments_ = strings.slice(1);
      const parts = [firstString.raw[0]];
      for (let i = 1; i < firstString.length; i++) {
        parts.push(
          String(arguments_[i - 1]).replace(/[{}\\]/g, "\\$&"),
          String(firstString.raw[i])
        );
      }
      if (template2 === void 0) {
        template2 = require_templates();
      }
      return template2(chalk3, parts.join(""));
    };
    Object.defineProperties(Chalk.prototype, styles);
    var chalk2 = Chalk();
    chalk2.supportsColor = stdoutColor;
    chalk2.stderr = Chalk({ level: stderrColor ? stderrColor.level : 0 });
    chalk2.stderr.supportsColor = stderrColor;
    module2.exports = chalk2;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/has-own-property.js
var require_has_own_property = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/has-own-property.js"(exports, module2) {
    "use strict";
    var own = {}.hasOwnProperty;
    module2.exports = own;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/splice.js
var require_splice = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/splice.js"(exports, module2) {
    "use strict";
    var splice = [].splice;
    module2.exports = splice;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/chunked-splice.js
var require_chunked_splice = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/chunked-splice.js"(exports, module2) {
    "use strict";
    var splice = require_splice();
    function chunkedSplice(list, start, remove, items) {
      var end = list.length;
      var chunkStart = 0;
      var parameters;
      if (start < 0) {
        start = -start > end ? 0 : end + start;
      } else {
        start = start > end ? end : start;
      }
      remove = remove > 0 ? remove : 0;
      if (items.length < 1e4) {
        parameters = Array.from(items);
        parameters.unshift(start, remove);
        splice.apply(list, parameters);
      } else {
        if (remove)
          splice.apply(list, [start, remove]);
        while (chunkStart < items.length) {
          parameters = items.slice(chunkStart, chunkStart + 1e4);
          parameters.unshift(start, 0);
          splice.apply(list, parameters);
          chunkStart += 1e4;
          start += 1e4;
        }
      }
    }
    module2.exports = chunkedSplice;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/miniflat.js
var require_miniflat = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/miniflat.js"(exports, module2) {
    "use strict";
    function miniflat(value) {
      return value === null || value === void 0 ? [] : "length" in value ? value : [value];
    }
    module2.exports = miniflat;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/combine-extensions.js
var require_combine_extensions = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/combine-extensions.js"(exports, module2) {
    "use strict";
    var hasOwnProperty = require_has_own_property();
    var chunkedSplice = require_chunked_splice();
    var miniflat = require_miniflat();
    function combineExtensions(extensions) {
      var all = {};
      var index = -1;
      while (++index < extensions.length) {
        extension(all, extensions[index]);
      }
      return all;
    }
    function extension(all, extension2) {
      var hook;
      var left;
      var right;
      var code;
      for (hook in extension2) {
        left = hasOwnProperty.call(all, hook) ? all[hook] : all[hook] = {};
        right = extension2[hook];
        for (code in right) {
          left[code] = constructs(
            miniflat(right[code]),
            hasOwnProperty.call(left, code) ? left[code] : []
          );
        }
      }
    }
    function constructs(list, existing) {
      var index = -1;
      var before = [];
      while (++index < list.length) {
        ;
        (list[index].add === "after" ? existing : before).push(list[index]);
      }
      chunkedSplice(existing, 0, 0, before);
      return existing;
    }
    module2.exports = combineExtensions;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/from-char-code.js
var require_from_char_code = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/from-char-code.js"(exports, module2) {
    "use strict";
    var fromCharCode = String.fromCharCode;
    module2.exports = fromCharCode;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/regex-check.js
var require_regex_check = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/regex-check.js"(exports, module2) {
    "use strict";
    var fromCharCode = require_from_char_code();
    function regexCheck(regex) {
      return check;
      function check(code) {
        return regex.test(fromCharCode(code));
      }
    }
    module2.exports = regexCheck;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/ascii-alpha.js
var require_ascii_alpha = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/ascii-alpha.js"(exports, module2) {
    "use strict";
    var regexCheck = require_regex_check();
    var asciiAlpha = regexCheck(/[A-Za-z]/);
    module2.exports = asciiAlpha;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/ascii-alphanumeric.js
var require_ascii_alphanumeric = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/ascii-alphanumeric.js"(exports, module2) {
    "use strict";
    var regexCheck = require_regex_check();
    var asciiAlphanumeric = regexCheck(/[\dA-Za-z]/);
    module2.exports = asciiAlphanumeric;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/ascii-control.js
var require_ascii_control = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/ascii-control.js"(exports, module2) {
    "use strict";
    function asciiControl(code) {
      return code < 32 || code === 127;
    }
    module2.exports = asciiControl;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/markdown-line-ending.js
var require_markdown_line_ending = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/markdown-line-ending.js"(exports, module2) {
    "use strict";
    function markdownLineEnding(code) {
      return code < -2;
    }
    module2.exports = markdownLineEnding;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/unicode-punctuation-regex.js
var require_unicode_punctuation_regex = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/unicode-punctuation-regex.js"(exports, module2) {
    "use strict";
    var unicodePunctuation = /[!-\/:-@\[-`\{-~\xA1\xA7\xAB\xB6\xB7\xBB\xBF\u037E\u0387\u055A-\u055F\u0589\u058A\u05BE\u05C0\u05C3\u05C6\u05F3\u05F4\u0609\u060A\u060C\u060D\u061B\u061E\u061F\u066A-\u066D\u06D4\u0700-\u070D\u07F7-\u07F9\u0830-\u083E\u085E\u0964\u0965\u0970\u09FD\u0A76\u0AF0\u0C77\u0C84\u0DF4\u0E4F\u0E5A\u0E5B\u0F04-\u0F12\u0F14\u0F3A-\u0F3D\u0F85\u0FD0-\u0FD4\u0FD9\u0FDA\u104A-\u104F\u10FB\u1360-\u1368\u1400\u166E\u169B\u169C\u16EB-\u16ED\u1735\u1736\u17D4-\u17D6\u17D8-\u17DA\u1800-\u180A\u1944\u1945\u1A1E\u1A1F\u1AA0-\u1AA6\u1AA8-\u1AAD\u1B5A-\u1B60\u1BFC-\u1BFF\u1C3B-\u1C3F\u1C7E\u1C7F\u1CC0-\u1CC7\u1CD3\u2010-\u2027\u2030-\u2043\u2045-\u2051\u2053-\u205E\u207D\u207E\u208D\u208E\u2308-\u230B\u2329\u232A\u2768-\u2775\u27C5\u27C6\u27E6-\u27EF\u2983-\u2998\u29D8-\u29DB\u29FC\u29FD\u2CF9-\u2CFC\u2CFE\u2CFF\u2D70\u2E00-\u2E2E\u2E30-\u2E4F\u2E52\u3001-\u3003\u3008-\u3011\u3014-\u301F\u3030\u303D\u30A0\u30FB\uA4FE\uA4FF\uA60D-\uA60F\uA673\uA67E\uA6F2-\uA6F7\uA874-\uA877\uA8CE\uA8CF\uA8F8-\uA8FA\uA8FC\uA92E\uA92F\uA95F\uA9C1-\uA9CD\uA9DE\uA9DF\uAA5C-\uAA5F\uAADE\uAADF\uAAF0\uAAF1\uABEB\uFD3E\uFD3F\uFE10-\uFE19\uFE30-\uFE52\uFE54-\uFE61\uFE63\uFE68\uFE6A\uFE6B\uFF01-\uFF03\uFF05-\uFF0A\uFF0C-\uFF0F\uFF1A\uFF1B\uFF1F\uFF20\uFF3B-\uFF3D\uFF3F\uFF5B\uFF5D\uFF5F-\uFF65]/;
    module2.exports = unicodePunctuation;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/unicode-punctuation.js
var require_unicode_punctuation = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/unicode-punctuation.js"(exports, module2) {
    "use strict";
    var unicodePunctuationRegex = require_unicode_punctuation_regex();
    var regexCheck = require_regex_check();
    var unicodePunctuation = regexCheck(unicodePunctuationRegex);
    module2.exports = unicodePunctuation;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/unicode-whitespace.js
var require_unicode_whitespace = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/unicode-whitespace.js"(exports, module2) {
    "use strict";
    var regexCheck = require_regex_check();
    var unicodeWhitespace = regexCheck(/\s/);
    module2.exports = unicodeWhitespace;
  }
});

// node_modules/.pnpm/micromark-extension-gfm-autolink-literal@0.5.7/node_modules/micromark-extension-gfm-autolink-literal/syntax.js
var require_syntax = __commonJS({
  "node_modules/.pnpm/micromark-extension-gfm-autolink-literal@0.5.7/node_modules/micromark-extension-gfm-autolink-literal/syntax.js"(exports) {
    var asciiAlpha = require_ascii_alpha();
    var asciiAlphanumeric = require_ascii_alphanumeric();
    var asciiControl = require_ascii_control();
    var markdownLineEnding = require_markdown_line_ending();
    var unicodePunctuation = require_unicode_punctuation();
    var unicodeWhitespace = require_unicode_whitespace();
    var www = { tokenize: tokenizeWww, partial: true };
    var domain = { tokenize: tokenizeDomain, partial: true };
    var path = { tokenize: tokenizePath, partial: true };
    var punctuation = { tokenize: tokenizePunctuation, partial: true };
    var namedCharacterReference = {
      tokenize: tokenizeNamedCharacterReference,
      partial: true
    };
    var wwwAutolink = { tokenize: tokenizeWwwAutolink, previous: previousWww };
    var httpAutolink = { tokenize: tokenizeHttpAutolink, previous: previousHttp };
    var emailAutolink = { tokenize: tokenizeEmailAutolink, previous: previousEmail };
    var text = {};
    exports.text = text;
    var code = 48;
    while (code < 123) {
      text[code] = emailAutolink;
      code++;
      if (code === 58)
        code = 65;
      else if (code === 91)
        code = 97;
    }
    text[43] = emailAutolink;
    text[45] = emailAutolink;
    text[46] = emailAutolink;
    text[95] = emailAutolink;
    text[72] = [emailAutolink, httpAutolink];
    text[104] = [emailAutolink, httpAutolink];
    text[87] = [emailAutolink, wwwAutolink];
    text[119] = [emailAutolink, wwwAutolink];
    function tokenizeEmailAutolink(effects, ok, nok) {
      var self = this;
      var hasDot;
      return start;
      function start(code2) {
        if (!gfmAtext(code2) || !previousEmail(self.previous) || previous(self.events)) {
          return nok(code2);
        }
        effects.enter("literalAutolink");
        effects.enter("literalAutolinkEmail");
        return atext(code2);
      }
      function atext(code2) {
        if (gfmAtext(code2)) {
          effects.consume(code2);
          return atext;
        }
        if (code2 === 64) {
          effects.consume(code2);
          return label;
        }
        return nok(code2);
      }
      function label(code2) {
        if (code2 === 46) {
          return effects.check(punctuation, done, dotContinuation)(code2);
        }
        if (code2 === 45 || code2 === 95) {
          return effects.check(punctuation, nok, dashOrUnderscoreContinuation)(code2);
        }
        if (asciiAlphanumeric(code2)) {
          effects.consume(code2);
          return label;
        }
        return done(code2);
      }
      function dotContinuation(code2) {
        effects.consume(code2);
        hasDot = true;
        return label;
      }
      function dashOrUnderscoreContinuation(code2) {
        effects.consume(code2);
        return afterDashOrUnderscore;
      }
      function afterDashOrUnderscore(code2) {
        if (code2 === 46) {
          return effects.check(punctuation, nok, dotContinuation)(code2);
        }
        return label(code2);
      }
      function done(code2) {
        if (hasDot) {
          effects.exit("literalAutolinkEmail");
          effects.exit("literalAutolink");
          return ok(code2);
        }
        return nok(code2);
      }
    }
    function tokenizeWwwAutolink(effects, ok, nok) {
      var self = this;
      return start;
      function start(code2) {
        if (code2 !== 87 && code2 - 32 !== 87 || !previousWww(self.previous) || previous(self.events)) {
          return nok(code2);
        }
        effects.enter("literalAutolink");
        effects.enter("literalAutolinkWww");
        return effects.check(
          www,
          effects.attempt(domain, effects.attempt(path, done), nok),
          nok
        )(code2);
      }
      function done(code2) {
        effects.exit("literalAutolinkWww");
        effects.exit("literalAutolink");
        return ok(code2);
      }
    }
    function tokenizeHttpAutolink(effects, ok, nok) {
      var self = this;
      return start;
      function start(code2) {
        if (code2 !== 72 && code2 - 32 !== 72 || !previousHttp(self.previous) || previous(self.events)) {
          return nok(code2);
        }
        effects.enter("literalAutolink");
        effects.enter("literalAutolinkHttp");
        effects.consume(code2);
        return t1;
      }
      function t1(code2) {
        if (code2 === 84 || code2 - 32 === 84) {
          effects.consume(code2);
          return t2;
        }
        return nok(code2);
      }
      function t2(code2) {
        if (code2 === 84 || code2 - 32 === 84) {
          effects.consume(code2);
          return p;
        }
        return nok(code2);
      }
      function p(code2) {
        if (code2 === 80 || code2 - 32 === 80) {
          effects.consume(code2);
          return s;
        }
        return nok(code2);
      }
      function s(code2) {
        if (code2 === 83 || code2 - 32 === 83) {
          effects.consume(code2);
          return colon;
        }
        return colon(code2);
      }
      function colon(code2) {
        if (code2 === 58) {
          effects.consume(code2);
          return slash1;
        }
        return nok(code2);
      }
      function slash1(code2) {
        if (code2 === 47) {
          effects.consume(code2);
          return slash2;
        }
        return nok(code2);
      }
      function slash2(code2) {
        if (code2 === 47) {
          effects.consume(code2);
          return after;
        }
        return nok(code2);
      }
      function after(code2) {
        return asciiControl(code2) || unicodeWhitespace(code2) || unicodePunctuation(code2) ? nok(code2) : effects.attempt(domain, effects.attempt(path, done), nok)(code2);
      }
      function done(code2) {
        effects.exit("literalAutolinkHttp");
        effects.exit("literalAutolink");
        return ok(code2);
      }
    }
    function tokenizeWww(effects, ok, nok) {
      return start;
      function start(code2) {
        effects.consume(code2);
        return w2;
      }
      function w2(code2) {
        if (code2 === 87 || code2 - 32 === 87) {
          effects.consume(code2);
          return w3;
        }
        return nok(code2);
      }
      function w3(code2) {
        if (code2 === 87 || code2 - 32 === 87) {
          effects.consume(code2);
          return dot;
        }
        return nok(code2);
      }
      function dot(code2) {
        if (code2 === 46) {
          effects.consume(code2);
          return after;
        }
        return nok(code2);
      }
      function after(code2) {
        return code2 === null || markdownLineEnding(code2) ? nok(code2) : ok(code2);
      }
    }
    function tokenizeDomain(effects, ok, nok) {
      var hasUnderscoreInLastSegment;
      var hasUnderscoreInLastLastSegment;
      return domain2;
      function domain2(code2) {
        if (code2 === 38) {
          return effects.check(
            namedCharacterReference,
            done,
            punctuationContinuation
          )(code2);
        }
        if (code2 === 46 || code2 === 95) {
          return effects.check(punctuation, done, punctuationContinuation)(code2);
        }
        if (asciiControl(code2) || unicodeWhitespace(code2) || code2 !== 45 && unicodePunctuation(code2)) {
          return done(code2);
        }
        effects.consume(code2);
        return domain2;
      }
      function punctuationContinuation(code2) {
        if (code2 === 46) {
          hasUnderscoreInLastLastSegment = hasUnderscoreInLastSegment;
          hasUnderscoreInLastSegment = void 0;
          effects.consume(code2);
          return domain2;
        }
        if (code2 === 95)
          hasUnderscoreInLastSegment = true;
        effects.consume(code2);
        return domain2;
      }
      function done(code2) {
        if (!hasUnderscoreInLastLastSegment && !hasUnderscoreInLastSegment) {
          return ok(code2);
        }
        return nok(code2);
      }
    }
    function tokenizePath(effects, ok) {
      var balance = 0;
      return inPath;
      function inPath(code2) {
        if (code2 === 38) {
          return effects.check(
            namedCharacterReference,
            ok,
            continuedPunctuation
          )(code2);
        }
        if (code2 === 40) {
          balance++;
        }
        if (code2 === 41) {
          return effects.check(
            punctuation,
            parenAtPathEnd,
            continuedPunctuation
          )(code2);
        }
        if (pathEnd(code2)) {
          return ok(code2);
        }
        if (trailingPunctuation(code2)) {
          return effects.check(punctuation, ok, continuedPunctuation)(code2);
        }
        effects.consume(code2);
        return inPath;
      }
      function continuedPunctuation(code2) {
        effects.consume(code2);
        return inPath;
      }
      function parenAtPathEnd(code2) {
        balance--;
        return balance < 0 ? ok(code2) : continuedPunctuation(code2);
      }
    }
    function tokenizeNamedCharacterReference(effects, ok, nok) {
      return start;
      function start(code2) {
        effects.consume(code2);
        return inside;
      }
      function inside(code2) {
        if (asciiAlpha(code2)) {
          effects.consume(code2);
          return inside;
        }
        if (code2 === 59) {
          effects.consume(code2);
          return after;
        }
        return nok(code2);
      }
      function after(code2) {
        return pathEnd(code2) ? ok(code2) : nok(code2);
      }
    }
    function tokenizePunctuation(effects, ok, nok) {
      return start;
      function start(code2) {
        effects.consume(code2);
        return after;
      }
      function after(code2) {
        if (trailingPunctuation(code2)) {
          effects.consume(code2);
          return after;
        }
        return pathEnd(code2) ? ok(code2) : nok(code2);
      }
    }
    function trailingPunctuation(code2) {
      return code2 === 33 || code2 === 34 || code2 === 39 || code2 === 41 || code2 === 42 || code2 === 44 || code2 === 46 || code2 === 58 || code2 === 59 || code2 === 60 || code2 === 63 || code2 === 95 || code2 === 126;
    }
    function pathEnd(code2) {
      return code2 === null || code2 < 0 || code2 === 32 || code2 === 60;
    }
    function gfmAtext(code2) {
      return code2 === 43 || code2 === 45 || code2 === 46 || code2 === 95 || asciiAlphanumeric(code2);
    }
    function previousWww(code2) {
      return code2 === null || code2 < 0 || code2 === 32 || code2 === 40 || code2 === 42 || code2 === 95 || code2 === 126;
    }
    function previousHttp(code2) {
      return code2 === null || !asciiAlpha(code2);
    }
    function previousEmail(code2) {
      return code2 !== 47 && previousHttp(code2);
    }
    function previous(events) {
      var index = events.length;
      while (index--) {
        if ((events[index][1].type === "labelLink" || events[index][1].type === "labelImage") && !events[index][1]._balanced) {
          return true;
        }
      }
    }
  }
});

// node_modules/.pnpm/micromark-extension-gfm-autolink-literal@0.5.7/node_modules/micromark-extension-gfm-autolink-literal/index.js
var require_micromark_extension_gfm_autolink_literal = __commonJS({
  "node_modules/.pnpm/micromark-extension-gfm-autolink-literal@0.5.7/node_modules/micromark-extension-gfm-autolink-literal/index.js"(exports, module2) {
    module2.exports = require_syntax();
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/markdown-line-ending-or-space.js
var require_markdown_line_ending_or_space = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/markdown-line-ending-or-space.js"(exports, module2) {
    "use strict";
    function markdownLineEndingOrSpace(code) {
      return code < 0 || code === 32;
    }
    module2.exports = markdownLineEndingOrSpace;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/classify-character.js
var require_classify_character = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/classify-character.js"(exports, module2) {
    "use strict";
    var markdownLineEndingOrSpace = require_markdown_line_ending_or_space();
    var unicodePunctuation = require_unicode_punctuation();
    var unicodeWhitespace = require_unicode_whitespace();
    function classifyCharacter(code) {
      if (code === null || markdownLineEndingOrSpace(code) || unicodeWhitespace(code)) {
        return 1;
      }
      if (unicodePunctuation(code)) {
        return 2;
      }
    }
    module2.exports = classifyCharacter;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/resolve-all.js
var require_resolve_all = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/resolve-all.js"(exports, module2) {
    "use strict";
    function resolveAll(constructs, events, context) {
      var called = [];
      var index = -1;
      var resolve;
      while (++index < constructs.length) {
        resolve = constructs[index].resolveAll;
        if (resolve && called.indexOf(resolve) < 0) {
          events = resolve(events, context);
          called.push(resolve);
        }
      }
      return events;
    }
    module2.exports = resolveAll;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/assign.js
var require_assign = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/constant/assign.js"(exports, module2) {
    "use strict";
    var assign = Object.assign;
    module2.exports = assign;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/shallow.js
var require_shallow = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/shallow.js"(exports, module2) {
    "use strict";
    var assign = require_assign();
    function shallow(object) {
      return assign({}, object);
    }
    module2.exports = shallow;
  }
});

// node_modules/.pnpm/micromark-extension-gfm-strikethrough@0.6.5/node_modules/micromark-extension-gfm-strikethrough/index.js
var require_micromark_extension_gfm_strikethrough = __commonJS({
  "node_modules/.pnpm/micromark-extension-gfm-strikethrough@0.6.5/node_modules/micromark-extension-gfm-strikethrough/index.js"(exports, module2) {
    module2.exports = create;
    var classifyCharacter = require_classify_character();
    var chunkedSplice = require_chunked_splice();
    var resolveAll = require_resolve_all();
    var shallow = require_shallow();
    function create(options) {
      var settings = options || {};
      var single = settings.singleTilde;
      var tokenizer = {
        tokenize: tokenizeStrikethrough,
        resolveAll: resolveAllStrikethrough
      };
      if (single === null || single === void 0) {
        single = true;
      }
      return { text: { 126: tokenizer }, insideSpan: { null: tokenizer } };
      function resolveAllStrikethrough(events, context) {
        var index = -1;
        var strikethrough;
        var text;
        var open;
        var nextEvents;
        while (++index < events.length) {
          if (events[index][0] === "enter" && events[index][1].type === "strikethroughSequenceTemporary" && events[index][1]._close) {
            open = index;
            while (open--) {
              if (events[open][0] === "exit" && events[open][1].type === "strikethroughSequenceTemporary" && events[open][1]._open && events[index][1].end.offset - events[index][1].start.offset === events[open][1].end.offset - events[open][1].start.offset) {
                events[index][1].type = "strikethroughSequence";
                events[open][1].type = "strikethroughSequence";
                strikethrough = {
                  type: "strikethrough",
                  start: shallow(events[open][1].start),
                  end: shallow(events[index][1].end)
                };
                text = {
                  type: "strikethroughText",
                  start: shallow(events[open][1].end),
                  end: shallow(events[index][1].start)
                };
                nextEvents = [
                  ["enter", strikethrough, context],
                  ["enter", events[open][1], context],
                  ["exit", events[open][1], context],
                  ["enter", text, context]
                ];
                chunkedSplice(
                  nextEvents,
                  nextEvents.length,
                  0,
                  resolveAll(
                    context.parser.constructs.insideSpan.null,
                    events.slice(open + 1, index),
                    context
                  )
                );
                chunkedSplice(nextEvents, nextEvents.length, 0, [
                  ["exit", text, context],
                  ["enter", events[index][1], context],
                  ["exit", events[index][1], context],
                  ["exit", strikethrough, context]
                ]);
                chunkedSplice(events, open - 1, index - open + 3, nextEvents);
                index = open + nextEvents.length - 2;
                break;
              }
            }
          }
        }
        return removeRemainingSequences(events);
      }
      function removeRemainingSequences(events) {
        var index = -1;
        var length = events.length;
        while (++index < length) {
          if (events[index][1].type === "strikethroughSequenceTemporary") {
            events[index][1].type = "data";
          }
        }
        return events;
      }
      function tokenizeStrikethrough(effects, ok, nok) {
        var previous = this.previous;
        var events = this.events;
        var size = 0;
        return start;
        function start(code) {
          if (code !== 126 || previous === 126 && events[events.length - 1][1].type !== "characterEscape") {
            return nok(code);
          }
          effects.enter("strikethroughSequenceTemporary");
          return more(code);
        }
        function more(code) {
          var before = classifyCharacter(previous);
          var token;
          var after;
          if (code === 126) {
            if (size > 1)
              return nok(code);
            effects.consume(code);
            size++;
            return more;
          }
          if (size < 2 && !single)
            return nok(code);
          token = effects.exit("strikethroughSequenceTemporary");
          after = classifyCharacter(code);
          token._open = !after || after === 2 && before;
          token._close = !before || before === 2 && after;
          return ok(code);
        }
      }
    }
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/markdown-space.js
var require_markdown_space = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/character/markdown-space.js"(exports, module2) {
    "use strict";
    function markdownSpace(code) {
      return code === -2 || code === -1 || code === 32;
    }
    module2.exports = markdownSpace;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/tokenize/factory-space.js
var require_factory_space = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/tokenize/factory-space.js"(exports, module2) {
    "use strict";
    var markdownSpace = require_markdown_space();
    function spaceFactory(effects, ok, type, max) {
      var limit = max ? max - 1 : Infinity;
      var size = 0;
      return start;
      function start(code) {
        if (markdownSpace(code)) {
          effects.enter(type);
          return prefix(code);
        }
        return ok(code);
      }
      function prefix(code) {
        if (markdownSpace(code) && size++ < limit) {
          effects.consume(code);
          return prefix;
        }
        effects.exit(type);
        return ok(code);
      }
    }
    module2.exports = spaceFactory;
  }
});

// node_modules/.pnpm/micromark-extension-gfm-table@0.4.3/node_modules/micromark-extension-gfm-table/syntax.js
var require_syntax2 = __commonJS({
  "node_modules/.pnpm/micromark-extension-gfm-table@0.4.3/node_modules/micromark-extension-gfm-table/syntax.js"(exports) {
    exports.flow = {
      null: { tokenize: tokenizeTable, resolve: resolveTable, interruptible: true }
    };
    var createSpace = require_factory_space();
    var setextUnderlineMini = { tokenize: tokenizeSetextUnderlineMini, partial: true };
    var nextPrefixedOrBlank = { tokenize: tokenizeNextPrefixedOrBlank, partial: true };
    function resolveTable(events, context) {
      var length = events.length;
      var index = -1;
      var token;
      var inHead;
      var inDelimiterRow;
      var inRow;
      var cell;
      var content;
      var text;
      var contentStart;
      var contentEnd;
      var cellStart;
      while (++index < length) {
        token = events[index][1];
        if (inRow) {
          if (token.type === "temporaryTableCellContent") {
            contentStart = contentStart || index;
            contentEnd = index;
          }
          if ((token.type === "tableCellDivider" || token.type === "tableRow") && contentEnd) {
            content = {
              type: "tableContent",
              start: events[contentStart][1].start,
              end: events[contentEnd][1].end
            };
            text = {
              type: "chunkText",
              start: content.start,
              end: content.end,
              contentType: "text"
            };
            events.splice(
              contentStart,
              contentEnd - contentStart + 1,
              ["enter", content, context],
              ["enter", text, context],
              ["exit", text, context],
              ["exit", content, context]
            );
            index -= contentEnd - contentStart - 3;
            length = events.length;
            contentStart = void 0;
            contentEnd = void 0;
          }
        }
        if (events[index][0] === "exit" && cellStart && cellStart + 1 < index && (token.type === "tableCellDivider" || token.type === "tableRow" && (cellStart + 3 < index || events[cellStart][1].type !== "whitespace"))) {
          cell = {
            type: inDelimiterRow ? "tableDelimiter" : inHead ? "tableHeader" : "tableData",
            start: events[cellStart][1].start,
            end: events[index][1].end
          };
          events.splice(index + (token.type === "tableCellDivider" ? 1 : 0), 0, [
            "exit",
            cell,
            context
          ]);
          events.splice(cellStart, 0, ["enter", cell, context]);
          index += 2;
          length = events.length;
          cellStart = index + 1;
        }
        if (token.type === "tableRow") {
          inRow = events[index][0] === "enter";
          if (inRow) {
            cellStart = index + 1;
          }
        }
        if (token.type === "tableDelimiterRow") {
          inDelimiterRow = events[index][0] === "enter";
          if (inDelimiterRow) {
            cellStart = index + 1;
          }
        }
        if (token.type === "tableHead") {
          inHead = events[index][0] === "enter";
        }
      }
      return events;
    }
    function tokenizeTable(effects, ok, nok) {
      var align = [];
      var tableHeaderCount = 0;
      var seenDelimiter;
      var hasDash;
      return start;
      function start(code) {
        if (code === null || code === -5 || code === -4 || code === -3) {
          return nok(code);
        }
        effects.enter("table")._align = align;
        effects.enter("tableHead");
        effects.enter("tableRow");
        if (code === 124) {
          return cellDividerHead(code);
        }
        tableHeaderCount++;
        effects.enter("temporaryTableCellContent");
        return inCellContentHead(code);
      }
      function cellDividerHead(code) {
        effects.enter("tableCellDivider");
        effects.consume(code);
        effects.exit("tableCellDivider");
        seenDelimiter = true;
        return cellBreakHead;
      }
      function cellBreakHead(code) {
        if (code === null || code === -5 || code === -4 || code === -3) {
          return atRowEndHead(code);
        }
        if (code === -2 || code === -1 || code === 32) {
          effects.enter("whitespace");
          effects.consume(code);
          return inWhitespaceHead;
        }
        if (seenDelimiter) {
          seenDelimiter = void 0;
          tableHeaderCount++;
        }
        if (code === 124) {
          return cellDividerHead(code);
        }
        effects.enter("temporaryTableCellContent");
        return inCellContentHead(code);
      }
      function inWhitespaceHead(code) {
        if (code === -2 || code === -1 || code === 32) {
          effects.consume(code);
          return inWhitespaceHead;
        }
        effects.exit("whitespace");
        return cellBreakHead(code);
      }
      function inCellContentHead(code) {
        if (code === null || code < 0 || code === 32 || code === 124) {
          effects.exit("temporaryTableCellContent");
          return cellBreakHead(code);
        }
        effects.consume(code);
        return code === 92 ? inCellContentEscapeHead : inCellContentHead;
      }
      function inCellContentEscapeHead(code) {
        if (code === 92 || code === 124) {
          effects.consume(code);
          return inCellContentHead;
        }
        return inCellContentHead(code);
      }
      function atRowEndHead(code) {
        if (code === null) {
          return nok(code);
        }
        effects.exit("tableRow");
        effects.exit("tableHead");
        effects.enter("lineEnding");
        effects.consume(code);
        effects.exit("lineEnding");
        return effects.check(
          setextUnderlineMini,
          nok,
          createSpace(effects, rowStartDelimiter, "linePrefix", 4)
        );
      }
      function rowStartDelimiter(code) {
        if (code === null || code < 0 || code === 32) {
          return nok(code);
        }
        effects.enter("tableDelimiterRow");
        return atDelimiterRowBreak(code);
      }
      function atDelimiterRowBreak(code) {
        if (code === null || code === -5 || code === -4 || code === -3) {
          return rowEndDelimiter(code);
        }
        if (code === -2 || code === -1 || code === 32) {
          effects.enter("whitespace");
          effects.consume(code);
          return inWhitespaceDelimiter;
        }
        if (code === 45) {
          effects.enter("tableDelimiterFiller");
          effects.consume(code);
          hasDash = true;
          align.push(null);
          return inFillerDelimiter;
        }
        if (code === 58) {
          effects.enter("tableDelimiterAlignment");
          effects.consume(code);
          effects.exit("tableDelimiterAlignment");
          align.push("left");
          return afterLeftAlignment;
        }
        if (code === 124) {
          effects.enter("tableCellDivider");
          effects.consume(code);
          effects.exit("tableCellDivider");
          return atDelimiterRowBreak;
        }
        return nok(code);
      }
      function inWhitespaceDelimiter(code) {
        if (code === -2 || code === -1 || code === 32) {
          effects.consume(code);
          return inWhitespaceDelimiter;
        }
        effects.exit("whitespace");
        return atDelimiterRowBreak(code);
      }
      function inFillerDelimiter(code) {
        if (code === 45) {
          effects.consume(code);
          return inFillerDelimiter;
        }
        effects.exit("tableDelimiterFiller");
        if (code === 58) {
          effects.enter("tableDelimiterAlignment");
          effects.consume(code);
          effects.exit("tableDelimiterAlignment");
          align[align.length - 1] = align[align.length - 1] === "left" ? "center" : "right";
          return afterRightAlignment;
        }
        return atDelimiterRowBreak(code);
      }
      function afterLeftAlignment(code) {
        if (code === 45) {
          effects.enter("tableDelimiterFiller");
          effects.consume(code);
          hasDash = true;
          return inFillerDelimiter;
        }
        return nok(code);
      }
      function afterRightAlignment(code) {
        if (code === null || code === -5 || code === -4 || code === -3) {
          return rowEndDelimiter(code);
        }
        if (code === -2 || code === -1 || code === 32) {
          effects.enter("whitespace");
          effects.consume(code);
          return inWhitespaceDelimiter;
        }
        if (code === 124) {
          effects.enter("tableCellDivider");
          effects.consume(code);
          effects.exit("tableCellDivider");
          return atDelimiterRowBreak;
        }
        return nok(code);
      }
      function rowEndDelimiter(code) {
        effects.exit("tableDelimiterRow");
        if (!hasDash || tableHeaderCount !== align.length) {
          return nok(code);
        }
        if (code === null) {
          return tableClose(code);
        }
        return effects.check(nextPrefixedOrBlank, tableClose, tableContinue)(code);
      }
      function tableClose(code) {
        effects.exit("table");
        return ok(code);
      }
      function tableContinue(code) {
        effects.enter("lineEnding");
        effects.consume(code);
        effects.exit("lineEnding");
        return createSpace(effects, bodyStart, "linePrefix", 4);
      }
      function bodyStart(code) {
        effects.enter("tableBody");
        return rowStartBody(code);
      }
      function rowStartBody(code) {
        effects.enter("tableRow");
        if (code === 124) {
          return cellDividerBody(code);
        }
        effects.enter("temporaryTableCellContent");
        return inCellContentBody(code);
      }
      function cellDividerBody(code) {
        effects.enter("tableCellDivider");
        effects.consume(code);
        effects.exit("tableCellDivider");
        return cellBreakBody;
      }
      function cellBreakBody(code) {
        if (code === null || code === -5 || code === -4 || code === -3) {
          return atRowEndBody(code);
        }
        if (code === -2 || code === -1 || code === 32) {
          effects.enter("whitespace");
          effects.consume(code);
          return inWhitespaceBody;
        }
        if (code === 124) {
          return cellDividerBody(code);
        }
        effects.enter("temporaryTableCellContent");
        return inCellContentBody(code);
      }
      function inWhitespaceBody(code) {
        if (code === -2 || code === -1 || code === 32) {
          effects.consume(code);
          return inWhitespaceBody;
        }
        effects.exit("whitespace");
        return cellBreakBody(code);
      }
      function inCellContentBody(code) {
        if (code === null || code < 0 || code === 32 || code === 124) {
          effects.exit("temporaryTableCellContent");
          return cellBreakBody(code);
        }
        effects.consume(code);
        return code === 92 ? inCellContentEscapeBody : inCellContentBody;
      }
      function inCellContentEscapeBody(code) {
        if (code === 92 || code === 124) {
          effects.consume(code);
          return inCellContentBody;
        }
        return inCellContentBody(code);
      }
      function atRowEndBody(code) {
        effects.exit("tableRow");
        if (code === null) {
          return tableBodyClose(code);
        }
        return effects.check(
          nextPrefixedOrBlank,
          tableBodyClose,
          tableBodyContinue
        )(code);
      }
      function tableBodyClose(code) {
        effects.exit("tableBody");
        return tableClose(code);
      }
      function tableBodyContinue(code) {
        effects.enter("lineEnding");
        effects.consume(code);
        effects.exit("lineEnding");
        return createSpace(effects, rowStartBody, "linePrefix", 4);
      }
    }
    function tokenizeSetextUnderlineMini(effects, ok, nok) {
      return start;
      function start(code) {
        if (code !== 45) {
          return nok(code);
        }
        effects.enter("setextUnderline");
        return sequence(code);
      }
      function sequence(code) {
        if (code === 45) {
          effects.consume(code);
          return sequence;
        }
        return whitespace(code);
      }
      function whitespace(code) {
        if (code === -2 || code === -1 || code === 32) {
          effects.consume(code);
          return whitespace;
        }
        if (code === null || code === -5 || code === -4 || code === -3) {
          return ok(code);
        }
        return nok(code);
      }
    }
    function tokenizeNextPrefixedOrBlank(effects, ok, nok) {
      var size = 0;
      return start;
      function start(code) {
        effects.enter("check");
        effects.consume(code);
        return whitespace;
      }
      function whitespace(code) {
        if (code === -1 || code === 32) {
          effects.consume(code);
          size++;
          return size === 4 ? ok : whitespace;
        }
        if (code === null || code < 0) {
          return ok(code);
        }
        return nok(code);
      }
    }
  }
});

// node_modules/.pnpm/micromark-extension-gfm-table@0.4.3/node_modules/micromark-extension-gfm-table/index.js
var require_micromark_extension_gfm_table = __commonJS({
  "node_modules/.pnpm/micromark-extension-gfm-table@0.4.3/node_modules/micromark-extension-gfm-table/index.js"(exports, module2) {
    module2.exports = require_syntax2();
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/size-chunks.js
var require_size_chunks = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/size-chunks.js"(exports, module2) {
    "use strict";
    function sizeChunks(chunks) {
      var index = -1;
      var size = 0;
      while (++index < chunks.length) {
        size += typeof chunks[index] === "string" ? chunks[index].length : 1;
      }
      return size;
    }
    module2.exports = sizeChunks;
  }
});

// node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/prefix-size.js
var require_prefix_size = __commonJS({
  "node_modules/.pnpm/micromark@2.11.4/node_modules/micromark/dist/util/prefix-size.js"(exports, module2) {
    "use strict";
    var sizeChunks = require_size_chunks();
    function prefixSize(events, type) {
      var tail = events[events.length - 1];
      if (!tail || tail[1].type !== type)
        return 0;
      return sizeChunks(tail[2].sliceStream(tail[1]));
    }
    module2.exports = prefixSize;
  }
});

// node_modules/.pnpm/micromark-extension-gfm-task-list-item@0.3.3/node_modules/micromark-extension-gfm-task-list-item/syntax.js
var require_syntax3 = __commonJS({
  "node_modules/.pnpm/micromark-extension-gfm-task-list-item@0.3.3/node_modules/micromark-extension-gfm-task-list-item/syntax.js"(exports) {
    var markdownLineEndingOrSpace = require_markdown_line_ending_or_space();
    var spaceFactory = require_factory_space();
    var prefixSize = require_prefix_size();
    var tasklistCheck = { tokenize: tokenizeTasklistCheck };
    exports.text = { 91: tasklistCheck };
    function tokenizeTasklistCheck(effects, ok, nok) {
      var self = this;
      return open;
      function open(code) {
        if (code !== 91 || self.previous !== null || !self._gfmTasklistFirstContentOfListItem) {
          return nok(code);
        }
        effects.enter("taskListCheck");
        effects.enter("taskListCheckMarker");
        effects.consume(code);
        effects.exit("taskListCheckMarker");
        return inside;
      }
      function inside(code) {
        if (code === -2 || code === 32) {
          effects.enter("taskListCheckValueUnchecked");
          effects.consume(code);
          effects.exit("taskListCheckValueUnchecked");
          return close;
        }
        if (code === 88 || code === 120) {
          effects.enter("taskListCheckValueChecked");
          effects.consume(code);
          effects.exit("taskListCheckValueChecked");
          return close;
        }
        return nok(code);
      }
      function close(code) {
        if (code === 93) {
          effects.enter("taskListCheckMarker");
          effects.consume(code);
          effects.exit("taskListCheckMarker");
          effects.exit("taskListCheck");
          return effects.check({ tokenize: spaceThenNonSpace }, ok, nok);
        }
        return nok(code);
      }
    }
    function spaceThenNonSpace(effects, ok, nok) {
      var self = this;
      return spaceFactory(effects, after, "whitespace");
      function after(code) {
        return prefixSize(self.events, "whitespace") && code !== null && !markdownLineEndingOrSpace(code) ? ok(code) : nok(code);
      }
    }
  }
});

// node_modules/.pnpm/micromark-extension-gfm-task-list-item@0.3.3/node_modules/micromark-extension-gfm-task-list-item/index.js
var require_micromark_extension_gfm_task_list_item = __commonJS({
  "node_modules/.pnpm/micromark-extension-gfm-task-list-item@0.3.3/node_modules/micromark-extension-gfm-task-list-item/index.js"(exports, module2) {
    module2.exports = require_syntax3();
  }
});

// node_modules/.pnpm/micromark-extension-gfm@0.3.3/node_modules/micromark-extension-gfm/syntax.js
var require_syntax4 = __commonJS({
  "node_modules/.pnpm/micromark-extension-gfm@0.3.3/node_modules/micromark-extension-gfm/syntax.js"(exports, module2) {
    var combine = require_combine_extensions();
    var autolink = require_micromark_extension_gfm_autolink_literal();
    var strikethrough = require_micromark_extension_gfm_strikethrough();
    var table = require_micromark_extension_gfm_table();
    var tasklist = require_micromark_extension_gfm_task_list_item();
    module2.exports = create;
    function create(options) {
      return combine([autolink, strikethrough(options), table, tasklist]);
    }
  }
});

// node_modules/.pnpm/micromark-extension-gfm@0.3.3/node_modules/micromark-extension-gfm/index.js
var require_micromark_extension_gfm = __commonJS({
  "node_modules/.pnpm/micromark-extension-gfm@0.3.3/node_modules/micromark-extension-gfm/index.js"(exports, module2) {
    module2.exports = require_syntax4();
  }
});

// node_modules/.pnpm/ccount@1.1.0/node_modules/ccount/index.js
var require_ccount = __commonJS({
  "node_modules/.pnpm/ccount@1.1.0/node_modules/ccount/index.js"(exports, module2) {
    "use strict";
    module2.exports = ccount;
    function ccount(source, character) {
      var value = String(source);
      var count = 0;
      var index;
      if (typeof character !== "string") {
        throw new Error("Expected character");
      }
      index = value.indexOf(character);
      while (index !== -1) {
        count++;
        index = value.indexOf(character, index + character.length);
      }
      return count;
    }
  }
});

// node_modules/.pnpm/unist-util-is@4.1.0/node_modules/unist-util-is/convert.js
var require_convert = __commonJS({
  "node_modules/.pnpm/unist-util-is@4.1.0/node_modules/unist-util-is/convert.js"(exports, module2) {
    "use strict";
    module2.exports = convert;
    function convert(test) {
      if (test == null) {
        return ok;
      }
      if (typeof test === "string") {
        return typeFactory(test);
      }
      if (typeof test === "object") {
        return "length" in test ? anyFactory(test) : allFactory(test);
      }
      if (typeof test === "function") {
        return test;
      }
      throw new Error("Expected function, string, or object as test");
    }
    function allFactory(test) {
      return all;
      function all(node) {
        var key;
        for (key in test) {
          if (node[key] !== test[key])
            return false;
        }
        return true;
      }
    }
    function anyFactory(tests) {
      var checks = [];
      var index = -1;
      while (++index < tests.length) {
        checks[index] = convert(tests[index]);
      }
      return any;
      function any() {
        var index2 = -1;
        while (++index2 < checks.length) {
          if (checks[index2].apply(this, arguments)) {
            return true;
          }
        }
        return false;
      }
    }
    function typeFactory(test) {
      return type;
      function type(node) {
        return Boolean(node && node.type === test);
      }
    }
    function ok() {
      return true;
    }
  }
});

// node_modules/.pnpm/unist-util-visit-parents@3.1.1/node_modules/unist-util-visit-parents/color.js
var require_color = __commonJS({
  "node_modules/.pnpm/unist-util-visit-parents@3.1.1/node_modules/unist-util-visit-parents/color.js"(exports, module2) {
    module2.exports = color;
    function color(d) {
      return "\x1B[33m" + d + "\x1B[39m";
    }
  }
});

// node_modules/.pnpm/unist-util-visit-parents@3.1.1/node_modules/unist-util-visit-parents/index.js
var require_unist_util_visit_parents = __commonJS({
  "node_modules/.pnpm/unist-util-visit-parents@3.1.1/node_modules/unist-util-visit-parents/index.js"(exports, module2) {
    "use strict";
    module2.exports = visitParents;
    var convert = require_convert();
    var color = require_color();
    var CONTINUE = true;
    var SKIP = "skip";
    var EXIT = false;
    visitParents.CONTINUE = CONTINUE;
    visitParents.SKIP = SKIP;
    visitParents.EXIT = EXIT;
    function visitParents(tree, test, visitor, reverse) {
      var step;
      var is;
      if (typeof test === "function" && typeof visitor !== "function") {
        reverse = visitor;
        visitor = test;
        test = null;
      }
      is = convert(test);
      step = reverse ? -1 : 1;
      factory(tree, null, [])();
      function factory(node, index, parents) {
        var value = typeof node === "object" && node !== null ? node : {};
        var name;
        if (typeof value.type === "string") {
          name = typeof value.tagName === "string" ? value.tagName : typeof value.name === "string" ? value.name : void 0;
          visit2.displayName = "node (" + color(value.type + (name ? "<" + name + ">" : "")) + ")";
        }
        return visit2;
        function visit2() {
          var grandparents = parents.concat(node);
          var result = [];
          var subresult;
          var offset;
          if (!test || is(node, index, parents[parents.length - 1] || null)) {
            result = toResult(visitor(node, parents));
            if (result[0] === EXIT) {
              return result;
            }
          }
          if (node.children && result[0] !== SKIP) {
            offset = (reverse ? node.children.length : -1) + step;
            while (offset > -1 && offset < node.children.length) {
              subresult = factory(node.children[offset], offset, grandparents)();
              if (subresult[0] === EXIT) {
                return subresult;
              }
              offset = typeof subresult[1] === "number" ? subresult[1] : offset + step;
            }
          }
          return result;
        }
      }
    }
    function toResult(value) {
      if (value !== null && typeof value === "object" && "length" in value) {
        return value;
      }
      if (typeof value === "number") {
        return [CONTINUE, value];
      }
      return [value];
    }
  }
});

// node_modules/.pnpm/escape-string-regexp@4.0.0/node_modules/escape-string-regexp/index.js
var require_escape_string_regexp = __commonJS({
  "node_modules/.pnpm/escape-string-regexp@4.0.0/node_modules/escape-string-regexp/index.js"(exports, module2) {
    "use strict";
    module2.exports = (string) => {
      if (typeof string !== "string") {
        throw new TypeError("Expected a string");
      }
      return string.replace(/[|\\{}()[\]^$+*?.]/g, "\\$&").replace(/-/g, "\\x2d");
    };
  }
});

// node_modules/.pnpm/mdast-util-find-and-replace@1.1.1/node_modules/mdast-util-find-and-replace/index.js
var require_mdast_util_find_and_replace = __commonJS({
  "node_modules/.pnpm/mdast-util-find-and-replace@1.1.1/node_modules/mdast-util-find-and-replace/index.js"(exports, module2) {
    "use strict";
    module2.exports = findAndReplace;
    var visit2 = require_unist_util_visit_parents();
    var convert = require_convert();
    var escape = require_escape_string_regexp();
    var splice = [].splice;
    function findAndReplace(tree, find, replace, options) {
      var settings;
      var schema;
      if (typeof find === "string" || find && typeof find.exec === "function") {
        schema = [[find, replace]];
      } else {
        schema = find;
        options = replace;
      }
      settings = options || {};
      search(tree, settings, handlerFactory(toPairs(schema)));
      return tree;
      function handlerFactory(pairs) {
        var pair = pairs[0];
        return handler;
        function handler(node, parent) {
          var find2 = pair[0];
          var replace2 = pair[1];
          var nodes = [];
          var start = 0;
          var index = parent.children.indexOf(node);
          var position;
          var match;
          var subhandler;
          var value;
          find2.lastIndex = 0;
          match = find2.exec(node.value);
          while (match) {
            position = match.index;
            value = replace2.apply(
              null,
              [].concat(match, { index: match.index, input: match.input })
            );
            if (value !== false) {
              if (start !== position) {
                nodes.push({ type: "text", value: node.value.slice(start, position) });
              }
              if (typeof value === "string" && value.length > 0) {
                value = { type: "text", value };
              }
              if (value) {
                nodes = [].concat(nodes, value);
              }
              start = position + match[0].length;
            }
            if (!find2.global) {
              break;
            }
            match = find2.exec(node.value);
          }
          if (position === void 0) {
            nodes = [node];
            index--;
          } else {
            if (start < node.value.length) {
              nodes.push({ type: "text", value: node.value.slice(start) });
            }
            nodes.unshift(index, 1);
            splice.apply(parent.children, nodes);
          }
          if (pairs.length > 1) {
            subhandler = handlerFactory(pairs.slice(1));
            position = -1;
            while (++position < nodes.length) {
              node = nodes[position];
              if (node.type === "text") {
                subhandler(node, parent);
              } else {
                search(node, settings, subhandler);
              }
            }
          }
          return index + nodes.length + 1;
        }
      }
    }
    function search(tree, settings, handler) {
      var ignored = convert(settings.ignore || []);
      var result = [];
      visit2(tree, "text", visitor);
      return result;
      function visitor(node, parents) {
        var index = -1;
        var parent;
        var grandparent;
        while (++index < parents.length) {
          parent = parents[index];
          if (ignored(
            parent,
            grandparent ? grandparent.children.indexOf(parent) : void 0,
            grandparent
          )) {
            return;
          }
          grandparent = parent;
        }
        return handler(node, grandparent);
      }
    }
    function toPairs(schema) {
      var result = [];
      var key;
      var index;
      if (typeof schema !== "object") {
        throw new Error("Expected array or object as schema");
      }
      if ("length" in schema) {
        index = -1;
        while (++index < schema.length) {
          result.push([
            toExpression(schema[index][0]),
            toFunction(schema[index][1])
          ]);
        }
      } else {
        for (key in schema) {
          result.push([toExpression(key), toFunction(schema[key])]);
        }
      }
      return result;
    }
    function toExpression(find) {
      return typeof find === "string" ? new RegExp(escape(find), "g") : find;
    }
    function toFunction(replace) {
      return typeof replace === "function" ? replace : returner;
      function returner() {
        return replace;
      }
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm-autolink-literal@0.1.3/node_modules/mdast-util-gfm-autolink-literal/from-markdown.js
var require_from_markdown = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm-autolink-literal@0.1.3/node_modules/mdast-util-gfm-autolink-literal/from-markdown.js"(exports) {
    var ccount = require_ccount();
    var findAndReplace = require_mdast_util_find_and_replace();
    var unicodePunctuation = require_unicode_punctuation();
    var unicodeWhitespace = require_unicode_whitespace();
    exports.transforms = [transformGfmAutolinkLiterals];
    exports.enter = {
      literalAutolink: enterLiteralAutolink,
      literalAutolinkEmail: enterLiteralAutolinkValue,
      literalAutolinkHttp: enterLiteralAutolinkValue,
      literalAutolinkWww: enterLiteralAutolinkValue
    };
    exports.exit = {
      literalAutolink: exitLiteralAutolink,
      literalAutolinkEmail: exitLiteralAutolinkEmail,
      literalAutolinkHttp: exitLiteralAutolinkHttp,
      literalAutolinkWww: exitLiteralAutolinkWww
    };
    function enterLiteralAutolink(token) {
      this.enter({ type: "link", title: null, url: "", children: [] }, token);
    }
    function enterLiteralAutolinkValue(token) {
      this.config.enter.autolinkProtocol.call(this, token);
    }
    function exitLiteralAutolinkHttp(token) {
      this.config.exit.autolinkProtocol.call(this, token);
    }
    function exitLiteralAutolinkWww(token) {
      this.config.exit.data.call(this, token);
      this.stack[this.stack.length - 1].url = "http://" + this.sliceSerialize(token);
    }
    function exitLiteralAutolinkEmail(token) {
      this.config.exit.autolinkEmail.call(this, token);
    }
    function exitLiteralAutolink(token) {
      this.exit(token);
    }
    function transformGfmAutolinkLiterals(tree) {
      findAndReplace(
        tree,
        [
          [/(https?:\/\/|www(?=\.))([-.\w]+)([^ \t\r\n]*)/i, findUrl],
          [/([-.\w+]+)@([-\w]+(?:\.[-\w]+)+)/, findEmail]
        ],
        { ignore: ["link", "linkReference"] }
      );
    }
    function findUrl($0, protocol, domain, path, match) {
      var prefix = "";
      var parts;
      var result;
      if (!previous(match)) {
        return false;
      }
      if (/^w/i.test(protocol)) {
        domain = protocol + domain;
        protocol = "";
        prefix = "http://";
      }
      if (!isCorrectDomain(domain)) {
        return false;
      }
      parts = splitUrl(domain + path);
      if (!parts[0])
        return false;
      result = {
        type: "link",
        title: null,
        url: prefix + protocol + parts[0],
        children: [{ type: "text", value: protocol + parts[0] }]
      };
      if (parts[1]) {
        result = [result, { type: "text", value: parts[1] }];
      }
      return result;
    }
    function findEmail($0, atext, label, match) {
      if (!previous(match, true) || /[_-]$/.test(label)) {
        return false;
      }
      return {
        type: "link",
        title: null,
        url: "mailto:" + atext + "@" + label,
        children: [{ type: "text", value: atext + "@" + label }]
      };
    }
    function isCorrectDomain(domain) {
      var parts = domain.split(".");
      if (parts.length < 2 || parts[parts.length - 1] && (/_/.test(parts[parts.length - 1]) || !/[a-zA-Z\d]/.test(parts[parts.length - 1])) || parts[parts.length - 2] && (/_/.test(parts[parts.length - 2]) || !/[a-zA-Z\d]/.test(parts[parts.length - 2]))) {
        return false;
      }
      return true;
    }
    function splitUrl(url) {
      var trail = /[!"&'),.:;<>?\]}]+$/.exec(url);
      var closingParenIndex;
      var openingParens;
      var closingParens;
      if (trail) {
        url = url.slice(0, trail.index);
        trail = trail[0];
        closingParenIndex = trail.indexOf(")");
        openingParens = ccount(url, "(");
        closingParens = ccount(url, ")");
        while (closingParenIndex !== -1 && openingParens > closingParens) {
          url += trail.slice(0, closingParenIndex + 1);
          trail = trail.slice(closingParenIndex + 1);
          closingParenIndex = trail.indexOf(")");
          closingParens++;
        }
      }
      return [url, trail];
    }
    function previous(match, email) {
      var code = match.input.charCodeAt(match.index - 1);
      return (code !== code || unicodeWhitespace(code) || unicodePunctuation(code)) && (!email || code !== 47);
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm-strikethrough@0.2.3/node_modules/mdast-util-gfm-strikethrough/from-markdown.js
var require_from_markdown2 = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm-strikethrough@0.2.3/node_modules/mdast-util-gfm-strikethrough/from-markdown.js"(exports) {
    exports.canContainEols = ["delete"];
    exports.enter = { strikethrough: enterStrikethrough };
    exports.exit = { strikethrough: exitStrikethrough };
    function enterStrikethrough(token) {
      this.enter({ type: "delete", children: [] }, token);
    }
    function exitStrikethrough(token) {
      this.exit(token);
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm-table@0.1.6/node_modules/mdast-util-gfm-table/from-markdown.js
var require_from_markdown3 = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm-table@0.1.6/node_modules/mdast-util-gfm-table/from-markdown.js"(exports) {
    exports.enter = {
      table: enterTable,
      tableData: enterCell,
      tableHeader: enterCell,
      tableRow: enterRow
    };
    exports.exit = {
      codeText: exitCodeText,
      table: exitTable,
      tableData: exit,
      tableHeader: exit,
      tableRow: exit
    };
    function enterTable(token) {
      this.enter({ type: "table", align: token._align, children: [] }, token);
      this.setData("inTable", true);
    }
    function exitTable(token) {
      this.exit(token);
      this.setData("inTable");
    }
    function enterRow(token) {
      this.enter({ type: "tableRow", children: [] }, token);
    }
    function exit(token) {
      this.exit(token);
    }
    function enterCell(token) {
      this.enter({ type: "tableCell", children: [] }, token);
    }
    function exitCodeText(token) {
      var value = this.resume();
      if (this.getData("inTable")) {
        value = value.replace(/\\([\\|])/g, replace);
      }
      this.stack[this.stack.length - 1].value = value;
      this.exit(token);
    }
    function replace($0, $1) {
      return $1 === "|" ? $1 : $0;
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm-task-list-item@0.1.6/node_modules/mdast-util-gfm-task-list-item/from-markdown.js
var require_from_markdown4 = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm-task-list-item@0.1.6/node_modules/mdast-util-gfm-task-list-item/from-markdown.js"(exports) {
    exports.exit = {
      taskListCheckValueChecked: exitCheck,
      taskListCheckValueUnchecked: exitCheck,
      paragraph: exitParagraphWithTaskListItem
    };
    function exitCheck(token) {
      this.stack[this.stack.length - 2].checked = token.type === "taskListCheckValueChecked";
    }
    function exitParagraphWithTaskListItem(token) {
      var parent = this.stack[this.stack.length - 2];
      var node = this.stack[this.stack.length - 1];
      var siblings = parent.children;
      var head = node.children[0];
      var index = -1;
      var firstParaghraph;
      if (parent && parent.type === "listItem" && typeof parent.checked === "boolean" && head && head.type === "text") {
        while (++index < siblings.length) {
          if (siblings[index].type === "paragraph") {
            firstParaghraph = siblings[index];
            break;
          }
        }
        if (firstParaghraph === node) {
          head.value = head.value.slice(1);
          if (head.value.length === 0) {
            node.children.shift();
          } else {
            head.position.start.column++;
            head.position.start.offset++;
            node.position.start = Object.assign({}, head.position.start);
          }
        }
      }
      this.exit(token);
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm@0.1.2/node_modules/mdast-util-gfm/from-markdown.js
var require_from_markdown5 = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm@0.1.2/node_modules/mdast-util-gfm/from-markdown.js"(exports, module2) {
    var autolinkLiteral = require_from_markdown();
    var strikethrough = require_from_markdown2();
    var table = require_from_markdown3();
    var taskListItem = require_from_markdown4();
    var own = {}.hasOwnProperty;
    module2.exports = configure([
      autolinkLiteral,
      strikethrough,
      table,
      taskListItem
    ]);
    function configure(extensions) {
      var config2 = { transforms: [], canContainEols: [] };
      var length = extensions.length;
      var index = -1;
      while (++index < length) {
        extension(config2, extensions[index]);
      }
      return config2;
    }
    function extension(config2, extension2) {
      var key;
      var left;
      var right;
      for (key in extension2) {
        left = own.call(config2, key) ? config2[key] : config2[key] = {};
        right = extension2[key];
        if (key === "canContainEols" || key === "transforms") {
          config2[key] = [].concat(left, right);
        } else {
          Object.assign(left, right);
        }
      }
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm-autolink-literal@0.1.3/node_modules/mdast-util-gfm-autolink-literal/to-markdown.js
var require_to_markdown = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm-autolink-literal@0.1.3/node_modules/mdast-util-gfm-autolink-literal/to-markdown.js"(exports) {
    var inConstruct = "phrasing";
    var notInConstruct = ["autolink", "link", "image", "label"];
    exports.unsafe = [
      {
        character: "@",
        before: "[+\\-.\\w]",
        after: "[\\-.\\w]",
        inConstruct,
        notInConstruct
      },
      {
        character: ".",
        before: "[Ww]",
        after: "[\\-.\\w]",
        inConstruct,
        notInConstruct
      },
      {
        character: ":",
        before: "[ps]",
        after: "\\/",
        inConstruct,
        notInConstruct
      }
    ];
  }
});

// node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/container-phrasing.js
var require_container_phrasing = __commonJS({
  "node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/container-phrasing.js"(exports, module2) {
    module2.exports = phrasing;
    function phrasing(parent, context, safeOptions) {
      var children = parent.children || [];
      var results = [];
      var index = -1;
      var before = safeOptions.before;
      var after;
      var handle;
      var child;
      while (++index < children.length) {
        child = children[index];
        if (index + 1 < children.length) {
          handle = context.handle.handlers[children[index + 1].type];
          if (handle && handle.peek)
            handle = handle.peek;
          after = handle ? handle(children[index + 1], parent, context, {
            before: "",
            after: ""
          }).charAt(0) : "";
        } else {
          after = safeOptions.after;
        }
        if (results.length > 0 && (before === "\r" || before === "\n") && child.type === "html") {
          results[results.length - 1] = results[results.length - 1].replace(
            /(\r?\n|\r)$/,
            " "
          );
          before = " ";
        }
        results.push(
          context.handle(child, parent, context, {
            before,
            after
          })
        );
        before = results[results.length - 1].slice(-1);
      }
      return results.join("");
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm-strikethrough@0.2.3/node_modules/mdast-util-gfm-strikethrough/to-markdown.js
var require_to_markdown2 = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm-strikethrough@0.2.3/node_modules/mdast-util-gfm-strikethrough/to-markdown.js"(exports) {
    var phrasing = require_container_phrasing();
    exports.unsafe = [{ character: "~", inConstruct: "phrasing" }];
    exports.handlers = { delete: handleDelete };
    handleDelete.peek = peekDelete;
    function handleDelete(node, _, context) {
      var exit = context.enter("emphasis");
      var value = phrasing(node, context, { before: "~", after: "~" });
      exit();
      return "~~" + value + "~~";
    }
    function peekDelete() {
      return "~";
    }
  }
});

// node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/pattern-compile.js
var require_pattern_compile = __commonJS({
  "node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/pattern-compile.js"(exports, module2) {
    module2.exports = patternCompile;
    function patternCompile(pattern) {
      var before;
      var after;
      if (!pattern._compiled) {
        before = pattern.before ? "(?:" + pattern.before + ")" : "";
        after = pattern.after ? "(?:" + pattern.after + ")" : "";
        if (pattern.atBreak) {
          before = "[\\r\\n][\\t ]*" + before;
        }
        pattern._compiled = new RegExp(
          (before ? "(" + before + ")" : "") + (/[|\\{}()[\]^$+*?.-]/.test(pattern.character) ? "\\" : "") + pattern.character + (after || ""),
          "g"
        );
      }
      return pattern._compiled;
    }
  }
});

// node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/handle/inline-code.js
var require_inline_code = __commonJS({
  "node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/handle/inline-code.js"(exports, module2) {
    module2.exports = inlineCode;
    inlineCode.peek = inlineCodePeek;
    var patternCompile = require_pattern_compile();
    function inlineCode(node, parent, context) {
      var value = node.value || "";
      var sequence = "`";
      var index = -1;
      var pattern;
      var expression;
      var match;
      var position;
      while (new RegExp("(^|[^`])" + sequence + "([^`]|$)").test(value)) {
        sequence += "`";
      }
      if (/[^ \r\n]/.test(value) && (/[ \r\n`]/.test(value.charAt(0)) || /[ \r\n`]/.test(value.charAt(value.length - 1)))) {
        value = " " + value + " ";
      }
      while (++index < context.unsafe.length) {
        pattern = context.unsafe[index];
        if (!pattern.atBreak)
          continue;
        expression = patternCompile(pattern);
        while (match = expression.exec(value)) {
          position = match.index;
          if (value.charCodeAt(position) === 10 && value.charCodeAt(position - 1) === 13) {
            position--;
          }
          value = value.slice(0, position) + " " + value.slice(match.index + 1);
        }
      }
      return sequence + value + sequence;
    }
    function inlineCodePeek() {
      return "`";
    }
  }
});

// node_modules/.pnpm/repeat-string@1.6.1/node_modules/repeat-string/index.js
var require_repeat_string = __commonJS({
  "node_modules/.pnpm/repeat-string@1.6.1/node_modules/repeat-string/index.js"(exports, module2) {
    "use strict";
    var res = "";
    var cache;
    module2.exports = repeat;
    function repeat(str, num) {
      if (typeof str !== "string") {
        throw new TypeError("expected a string");
      }
      if (num === 1)
        return str;
      if (num === 2)
        return str + str;
      var max = str.length * num;
      if (cache !== str || typeof cache === "undefined") {
        cache = str;
        res = "";
      } else if (res.length >= max) {
        return res.substr(0, max);
      }
      while (max > res.length && num > 1) {
        if (num & 1) {
          res += str;
        }
        num >>= 1;
        str += str;
      }
      res += str;
      res = res.substr(0, max);
      return res;
    }
  }
});

// node_modules/.pnpm/markdown-table@2.0.0/node_modules/markdown-table/index.js
var require_markdown_table = __commonJS({
  "node_modules/.pnpm/markdown-table@2.0.0/node_modules/markdown-table/index.js"(exports, module2) {
    "use strict";
    var repeat = require_repeat_string();
    module2.exports = markdownTable;
    var trailingWhitespace = / +$/;
    var space = " ";
    var lineFeed = "\n";
    var dash = "-";
    var colon = ":";
    var verticalBar = "|";
    var x = 0;
    var C = 67;
    var L = 76;
    var R = 82;
    var c = 99;
    var l = 108;
    var r = 114;
    function markdownTable(table, options) {
      var settings = options || {};
      var padding = settings.padding !== false;
      var start = settings.delimiterStart !== false;
      var end = settings.delimiterEnd !== false;
      var align = (settings.align || []).concat();
      var alignDelimiters = settings.alignDelimiters !== false;
      var alignments = [];
      var stringLength = settings.stringLength || defaultStringLength;
      var rowIndex = -1;
      var rowLength = table.length;
      var cellMatrix = [];
      var sizeMatrix = [];
      var row = [];
      var sizes = [];
      var longestCellByColumn = [];
      var mostCellsPerRow = 0;
      var cells;
      var columnIndex;
      var columnLength;
      var largest;
      var size;
      var cell;
      var lines;
      var line;
      var before;
      var after;
      var code;
      while (++rowIndex < rowLength) {
        cells = table[rowIndex];
        columnIndex = -1;
        columnLength = cells.length;
        row = [];
        sizes = [];
        if (columnLength > mostCellsPerRow) {
          mostCellsPerRow = columnLength;
        }
        while (++columnIndex < columnLength) {
          cell = serialize(cells[columnIndex]);
          if (alignDelimiters === true) {
            size = stringLength(cell);
            sizes[columnIndex] = size;
            largest = longestCellByColumn[columnIndex];
            if (largest === void 0 || size > largest) {
              longestCellByColumn[columnIndex] = size;
            }
          }
          row.push(cell);
        }
        cellMatrix[rowIndex] = row;
        sizeMatrix[rowIndex] = sizes;
      }
      columnIndex = -1;
      columnLength = mostCellsPerRow;
      if (typeof align === "object" && "length" in align) {
        while (++columnIndex < columnLength) {
          alignments[columnIndex] = toAlignment(align[columnIndex]);
        }
      } else {
        code = toAlignment(align);
        while (++columnIndex < columnLength) {
          alignments[columnIndex] = code;
        }
      }
      columnIndex = -1;
      columnLength = mostCellsPerRow;
      row = [];
      sizes = [];
      while (++columnIndex < columnLength) {
        code = alignments[columnIndex];
        before = "";
        after = "";
        if (code === l) {
          before = colon;
        } else if (code === r) {
          after = colon;
        } else if (code === c) {
          before = colon;
          after = colon;
        }
        size = alignDelimiters ? Math.max(
          1,
          longestCellByColumn[columnIndex] - before.length - after.length
        ) : 1;
        cell = before + repeat(dash, size) + after;
        if (alignDelimiters === true) {
          size = before.length + size + after.length;
          if (size > longestCellByColumn[columnIndex]) {
            longestCellByColumn[columnIndex] = size;
          }
          sizes[columnIndex] = size;
        }
        row[columnIndex] = cell;
      }
      cellMatrix.splice(1, 0, row);
      sizeMatrix.splice(1, 0, sizes);
      rowIndex = -1;
      rowLength = cellMatrix.length;
      lines = [];
      while (++rowIndex < rowLength) {
        row = cellMatrix[rowIndex];
        sizes = sizeMatrix[rowIndex];
        columnIndex = -1;
        columnLength = mostCellsPerRow;
        line = [];
        while (++columnIndex < columnLength) {
          cell = row[columnIndex] || "";
          before = "";
          after = "";
          if (alignDelimiters === true) {
            size = longestCellByColumn[columnIndex] - (sizes[columnIndex] || 0);
            code = alignments[columnIndex];
            if (code === r) {
              before = repeat(space, size);
            } else if (code === c) {
              if (size % 2 === 0) {
                before = repeat(space, size / 2);
                after = before;
              } else {
                before = repeat(space, size / 2 + 0.5);
                after = repeat(space, size / 2 - 0.5);
              }
            } else {
              after = repeat(space, size);
            }
          }
          if (start === true && columnIndex === 0) {
            line.push(verticalBar);
          }
          if (padding === true && !(alignDelimiters === false && cell === "") && (start === true || columnIndex !== 0)) {
            line.push(space);
          }
          if (alignDelimiters === true) {
            line.push(before);
          }
          line.push(cell);
          if (alignDelimiters === true) {
            line.push(after);
          }
          if (padding === true) {
            line.push(space);
          }
          if (end === true || columnIndex !== columnLength - 1) {
            line.push(verticalBar);
          }
        }
        line = line.join("");
        if (end === false) {
          line = line.replace(trailingWhitespace, "");
        }
        lines.push(line);
      }
      return lines.join(lineFeed);
    }
    function serialize(value) {
      return value === null || value === void 0 ? "" : String(value);
    }
    function defaultStringLength(value) {
      return value.length;
    }
    function toAlignment(value) {
      var code = typeof value === "string" ? value.charCodeAt(0) : x;
      return code === L || code === l ? l : code === R || code === r ? r : code === C || code === c ? c : x;
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm-table@0.1.6/node_modules/mdast-util-gfm-table/to-markdown.js
var require_to_markdown3 = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm-table@0.1.6/node_modules/mdast-util-gfm-table/to-markdown.js"(exports, module2) {
    var phrasing = require_container_phrasing();
    var defaultInlineCode = require_inline_code();
    var markdownTable = require_markdown_table();
    module2.exports = toMarkdown;
    function toMarkdown(options) {
      var settings = options || {};
      var padding = settings.tableCellPadding;
      var alignDelimiters = settings.tablePipeAlign;
      var stringLength = settings.stringLength;
      var around = padding ? " " : "|";
      return {
        unsafe: [
          { character: "\r", inConstruct: "tableCell" },
          { character: "\n", inConstruct: "tableCell" },
          { atBreak: true, character: "|", after: "[	 :-]" },
          { character: "|", inConstruct: "tableCell" },
          { atBreak: true, character: ":", after: "-" },
          { atBreak: true, character: "-", after: "[:|-]" }
        ],
        handlers: {
          table: handleTable,
          tableRow: handleTableRow,
          tableCell: handleTableCell,
          inlineCode: inlineCodeWithTable
        }
      };
      function handleTable(node, _, context) {
        return serializeData(handleTableAsData(node, context), node.align);
      }
      function handleTableRow(node, _, context) {
        var row = handleTableRowAsData(node, context);
        var value = serializeData([row]);
        return value.slice(0, value.indexOf("\n"));
      }
      function handleTableCell(node, _, context) {
        var exit = context.enter("tableCell");
        var value = phrasing(node, context, { before: around, after: around });
        exit();
        return value;
      }
      function serializeData(matrix, align) {
        return markdownTable(matrix, {
          align,
          alignDelimiters,
          padding,
          stringLength
        });
      }
      function handleTableAsData(node, context) {
        var children = node.children;
        var index = -1;
        var length = children.length;
        var result = [];
        var subexit = context.enter("table");
        while (++index < length) {
          result[index] = handleTableRowAsData(children[index], context);
        }
        subexit();
        return result;
      }
      function handleTableRowAsData(node, context) {
        var children = node.children;
        var index = -1;
        var length = children.length;
        var result = [];
        var subexit = context.enter("tableRow");
        while (++index < length) {
          result[index] = handleTableCell(children[index], node, context);
        }
        subexit();
        return result;
      }
      function inlineCodeWithTable(node, parent, context) {
        var value = defaultInlineCode(node, parent, context);
        if (context.stack.indexOf("tableCell") !== -1) {
          value = value.replace(/\|/g, "\\$&");
        }
        return value;
      }
    }
  }
});

// node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/check-bullet.js
var require_check_bullet = __commonJS({
  "node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/check-bullet.js"(exports, module2) {
    module2.exports = checkBullet;
    function checkBullet(context) {
      var marker = context.options.bullet || "*";
      if (marker !== "*" && marker !== "+" && marker !== "-") {
        throw new Error(
          "Cannot serialize items with `" + marker + "` for `options.bullet`, expected `*`, `+`, or `-`"
        );
      }
      return marker;
    }
  }
});

// node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/check-list-item-indent.js
var require_check_list_item_indent = __commonJS({
  "node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/check-list-item-indent.js"(exports, module2) {
    module2.exports = checkListItemIndent;
    function checkListItemIndent(context) {
      var style = context.options.listItemIndent || "tab";
      if (style === 1 || style === "1") {
        return "one";
      }
      if (style !== "tab" && style !== "one" && style !== "mixed") {
        throw new Error(
          "Cannot serialize items with `" + style + "` for `options.listItemIndent`, expected `tab`, `one`, or `mixed`"
        );
      }
      return style;
    }
  }
});

// node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/container-flow.js
var require_container_flow = __commonJS({
  "node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/container-flow.js"(exports, module2) {
    module2.exports = flow;
    var repeat = require_repeat_string();
    function flow(parent, context) {
      var children = parent.children || [];
      var results = [];
      var index = -1;
      var child;
      while (++index < children.length) {
        child = children[index];
        results.push(
          context.handle(child, parent, context, { before: "\n", after: "\n" })
        );
        if (index + 1 < children.length) {
          results.push(between(child, children[index + 1]));
        }
      }
      return results.join("");
      function between(left, right) {
        var index2 = -1;
        var result;
        while (++index2 < context.join.length) {
          result = context.join[index2](left, right, parent, context);
          if (result === true || result === 1) {
            break;
          }
          if (typeof result === "number") {
            return repeat("\n", 1 + Number(result));
          }
          if (result === false) {
            return "\n\n<!---->\n\n";
          }
        }
        return "\n\n";
      }
    }
  }
});

// node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/indent-lines.js
var require_indent_lines = __commonJS({
  "node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/util/indent-lines.js"(exports, module2) {
    module2.exports = indentLines;
    var eol = /\r?\n|\r/g;
    function indentLines(value, map) {
      var result = [];
      var start = 0;
      var line = 0;
      var match;
      while (match = eol.exec(value)) {
        one(value.slice(start, match.index));
        result.push(match[0]);
        start = match.index + match[0].length;
        line++;
      }
      one(value.slice(start));
      return result.join("");
      function one(value2) {
        result.push(map(value2, line, !value2));
      }
    }
  }
});

// node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/handle/list-item.js
var require_list_item = __commonJS({
  "node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/handle/list-item.js"(exports, module2) {
    module2.exports = listItem;
    var repeat = require_repeat_string();
    var checkBullet = require_check_bullet();
    var checkListItemIndent = require_check_list_item_indent();
    var flow = require_container_flow();
    var indentLines = require_indent_lines();
    function listItem(node, parent, context) {
      var bullet = checkBullet(context);
      var listItemIndent = checkListItemIndent(context);
      var size;
      var value;
      var exit;
      if (parent && parent.ordered) {
        bullet = (parent.start > -1 ? parent.start : 1) + (context.options.incrementListMarker === false ? 0 : parent.children.indexOf(node)) + ".";
      }
      size = bullet.length + 1;
      if (listItemIndent === "tab" || listItemIndent === "mixed" && (parent && parent.spread || node.spread)) {
        size = Math.ceil(size / 4) * 4;
      }
      exit = context.enter("listItem");
      value = indentLines(flow(node, context), map);
      exit();
      return value;
      function map(line, index, blank) {
        if (index) {
          return (blank ? "" : repeat(" ", size)) + line;
        }
        return (blank ? bullet : bullet + repeat(" ", size - bullet.length)) + line;
      }
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm-task-list-item@0.1.6/node_modules/mdast-util-gfm-task-list-item/to-markdown.js
var require_to_markdown4 = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm-task-list-item@0.1.6/node_modules/mdast-util-gfm-task-list-item/to-markdown.js"(exports) {
    var defaultListItem = require_list_item();
    exports.unsafe = [{ atBreak: true, character: "-", after: "[:|-]" }];
    exports.handlers = {
      listItem: listItemWithTaskListItem
    };
    function listItemWithTaskListItem(node, parent, context) {
      var value = defaultListItem(node, parent, context);
      var head = node.children[0];
      if (typeof node.checked === "boolean" && head && head.type === "paragraph") {
        value = value.replace(/^(?:[*+-]|\d+\.)([\r\n]| {1,3})/, check);
      }
      return value;
      function check($0) {
        return $0 + "[" + (node.checked ? "x" : " ") + "] ";
      }
    }
  }
});

// node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/configure.js
var require_configure = __commonJS({
  "node_modules/.pnpm/mdast-util-to-markdown@0.6.5/node_modules/mdast-util-to-markdown/lib/configure.js"(exports, module2) {
    module2.exports = configure;
    function configure(base, extension) {
      var index = -1;
      var key;
      if (extension.extensions) {
        while (++index < extension.extensions.length) {
          configure(base, extension.extensions[index]);
        }
      }
      for (key in extension) {
        if (key === "extensions") {
        } else if (key === "unsafe" || key === "join") {
          base[key] = base[key].concat(extension[key] || []);
        } else if (key === "handlers") {
          base[key] = Object.assign(base[key], extension[key] || {});
        } else {
          base.options[key] = extension[key];
        }
      }
      return base;
    }
  }
});

// node_modules/.pnpm/mdast-util-gfm@0.1.2/node_modules/mdast-util-gfm/to-markdown.js
var require_to_markdown5 = __commonJS({
  "node_modules/.pnpm/mdast-util-gfm@0.1.2/node_modules/mdast-util-gfm/to-markdown.js"(exports, module2) {
    var autolinkLiteral = require_to_markdown();
    var strikethrough = require_to_markdown2();
    var table = require_to_markdown3();
    var taskListItem = require_to_markdown4();
    var configure = require_configure();
    module2.exports = toMarkdown;
    function toMarkdown(options) {
      var config2 = configure(
        { handlers: {}, join: [], unsafe: [], options: {} },
        {
          extensions: [autolinkLiteral, strikethrough, table(options), taskListItem]
        }
      );
      return Object.assign(config2.options, {
        handlers: config2.handlers,
        join: config2.join,
        unsafe: config2.unsafe
      });
    }
  }
});

// node_modules/.pnpm/remark-gfm@1.0.0/node_modules/remark-gfm/index.js
var require_remark_gfm = __commonJS({
  "node_modules/.pnpm/remark-gfm@1.0.0/node_modules/remark-gfm/index.js"(exports, module2) {
    "use strict";
    var syntax = require_micromark_extension_gfm();
    var fromMarkdown = require_from_markdown5();
    var toMarkdown = require_to_markdown5();
    var warningIssued;
    module2.exports = gfm;
    function gfm(options) {
      var data = this.data();
      if (!warningIssued && (this.Parser && this.Parser.prototype && this.Parser.prototype.blockTokenizers || this.Compiler && this.Compiler.prototype && this.Compiler.prototype.visitors)) {
        warningIssued = true;
        console.warn(
          "[remark-gfm] Warning: please upgrade to remark 13 to use this plugin"
        );
      }
      add("micromarkExtensions", syntax(options));
      add("fromMarkdownExtensions", fromMarkdown);
      add("toMarkdownExtensions", toMarkdown(options));
      function add(field, value) {
        if (data[field])
          data[field].push(value);
        else
          data[field] = [value];
      }
    }
  }
});

// node_modules/.pnpm/dotenv@16.0.3/node_modules/dotenv/package.json
var require_package = __commonJS({
  "node_modules/.pnpm/dotenv@16.0.3/node_modules/dotenv/package.json"(exports, module2) {
    module2.exports = {
      name: "dotenv",
      version: "16.0.3",
      description: "Loads environment variables from .env file",
      main: "lib/main.js",
      types: "lib/main.d.ts",
      exports: {
        ".": {
          require: "./lib/main.js",
          types: "./lib/main.d.ts",
          default: "./lib/main.js"
        },
        "./config": "./config.js",
        "./config.js": "./config.js",
        "./lib/env-options": "./lib/env-options.js",
        "./lib/env-options.js": "./lib/env-options.js",
        "./lib/cli-options": "./lib/cli-options.js",
        "./lib/cli-options.js": "./lib/cli-options.js",
        "./package.json": "./package.json"
      },
      scripts: {
        "dts-check": "tsc --project tests/types/tsconfig.json",
        lint: "standard",
        "lint-readme": "standard-markdown",
        pretest: "npm run lint && npm run dts-check",
        test: "tap tests/*.js --100 -Rspec",
        prerelease: "npm test",
        release: "standard-version"
      },
      repository: {
        type: "git",
        url: "git://github.com/motdotla/dotenv.git"
      },
      keywords: [
        "dotenv",
        "env",
        ".env",
        "environment",
        "variables",
        "config",
        "settings"
      ],
      readmeFilename: "README.md",
      license: "BSD-2-Clause",
      devDependencies: {
        "@types/node": "^17.0.9",
        decache: "^4.6.1",
        dtslint: "^3.7.0",
        sinon: "^12.0.1",
        standard: "^16.0.4",
        "standard-markdown": "^7.1.0",
        "standard-version": "^9.3.2",
        tap: "^15.1.6",
        tar: "^6.1.11",
        typescript: "^4.5.4"
      },
      engines: {
        node: ">=12"
      }
    };
  }
});

// node_modules/.pnpm/dotenv@16.0.3/node_modules/dotenv/lib/main.js
var require_main = __commonJS({
  "node_modules/.pnpm/dotenv@16.0.3/node_modules/dotenv/lib/main.js"(exports, module2) {
    var fs = require("fs");
    var path = require("path");
    var os = require("os");
    var packageJson = require_package();
    var version = packageJson.version;
    var LINE = /(?:^|^)\s*(?:export\s+)?([\w.-]+)(?:\s*=\s*?|:\s+?)(\s*'(?:\\'|[^'])*'|\s*"(?:\\"|[^"])*"|\s*`(?:\\`|[^`])*`|[^#\r\n]+)?\s*(?:#.*)?(?:$|$)/mg;
    function parse(src) {
      const obj = {};
      let lines = src.toString();
      lines = lines.replace(/\r\n?/mg, "\n");
      let match;
      while ((match = LINE.exec(lines)) != null) {
        const key = match[1];
        let value = match[2] || "";
        value = value.trim();
        const maybeQuote = value[0];
        value = value.replace(/^(['"`])([\s\S]*)\1$/mg, "$2");
        if (maybeQuote === '"') {
          value = value.replace(/\\n/g, "\n");
          value = value.replace(/\\r/g, "\r");
        }
        obj[key] = value;
      }
      return obj;
    }
    function _log(message) {
      console.log(`[dotenv@${version}][DEBUG] ${message}`);
    }
    function _resolveHome(envPath) {
      return envPath[0] === "~" ? path.join(os.homedir(), envPath.slice(1)) : envPath;
    }
    function config2(options) {
      let dotenvPath = path.resolve(process.cwd(), ".env");
      let encoding = "utf8";
      const debug = Boolean(options && options.debug);
      const override = Boolean(options && options.override);
      if (options) {
        if (options.path != null) {
          dotenvPath = _resolveHome(options.path);
        }
        if (options.encoding != null) {
          encoding = options.encoding;
        }
      }
      try {
        const parsed = DotenvModule.parse(fs.readFileSync(dotenvPath, { encoding }));
        Object.keys(parsed).forEach(function(key) {
          if (!Object.prototype.hasOwnProperty.call(process.env, key)) {
            process.env[key] = parsed[key];
          } else {
            if (override === true) {
              process.env[key] = parsed[key];
            }
            if (debug) {
              if (override === true) {
                _log(`"${key}" is already defined in \`process.env\` and WAS overwritten`);
              } else {
                _log(`"${key}" is already defined in \`process.env\` and was NOT overwritten`);
              }
            }
          }
        });
        return { parsed };
      } catch (e) {
        if (debug) {
          _log(`Failed to load ${dotenvPath} ${e.message}`);
        }
        return { error: e };
      }
    }
    var DotenvModule = {
      config: config2,
      parse
    };
    module2.exports.config = DotenvModule.config;
    module2.exports.parse = DotenvModule.parse;
    module2.exports = DotenvModule;
  }
});

// lib/releases.ts
var import_promises = require("fs/promises");
var import_fs = require("fs");
var import_path = require("path");
var import_rest = require("@octokit/rest");

// lib/is-defined.ts
function isDefined(value) {
  return value !== void 0 && value !== null;
}

// node_modules/.pnpm/chalk-template@0.5.0/node_modules/chalk-template/index.js
var import_chalk = __toESM(require_source(), 1);
var TEMPLATE_REGEX = /(?:\\(u(?:[a-f\d]{4}|{[a-f\d]{1,6}})|x[a-f\d]{2}|.))|(?:{(~)?(#?[\w:]+(?:\([^)]*\))?(?:\.#?[\w:]+(?:\([^)]*\))?)*)(?:[ \t]|(?=\r?\n)))|(})|((?:.|[\r\n\f])+?)/gi;
var STYLE_REGEX = /(?:^|\.)(?:(?:(\w+)(?:\(([^)]*)\))?)|(?:#(?=[:a-fA-F\d]{2,})([a-fA-F\d]{6})?(?::([a-fA-F\d]{6}))?))/g;
var STRING_REGEX = /^(['"])((?:\\.|(?!\1)[^\\])*)\1$/;
var ESCAPE_REGEX = /\\(u(?:[a-f\d]{4}|{[a-f\d]{1,6}})|x[a-f\d]{2}|.)|([^\\])/gi;
var ESCAPES = /* @__PURE__ */ new Map([
  ["n", "\n"],
  ["r", "\r"],
  ["t", "	"],
  ["b", "\b"],
  ["f", "\f"],
  ["v", "\v"],
  ["0", "\0"],
  ["\\", "\\"],
  ["e", "\x1B"],
  ["a", "\x07"]
]);
function unescape(c) {
  const u2 = c[0] === "u";
  const bracket = c[1] === "{";
  if (u2 && !bracket && c.length === 5 || c[0] === "x" && c.length === 3) {
    return String.fromCodePoint(Number.parseInt(c.slice(1), 16));
  }
  if (u2 && bracket) {
    return String.fromCodePoint(Number.parseInt(c.slice(2, -1), 16));
  }
  return ESCAPES.get(c) || c;
}
function parseArguments(name, arguments_) {
  const results = [];
  const chunks = arguments_.trim().split(/\s*,\s*/g);
  let matches;
  for (const chunk of chunks) {
    const number = Number(chunk);
    if (!Number.isNaN(number)) {
      results.push(number);
    } else if (matches = chunk.match(STRING_REGEX)) {
      results.push(matches[2].replace(ESCAPE_REGEX, (_, escape, character) => escape ? unescape(escape) : character));
    } else {
      throw new Error(`Invalid Chalk template style argument: ${chunk} (in style '${name}')`);
    }
  }
  return results;
}
function parseHex(hex) {
  const n = Number.parseInt(hex, 16);
  return [
    n >> 16 & 255,
    n >> 8 & 255,
    n & 255
  ];
}
function parseStyle(style) {
  STYLE_REGEX.lastIndex = 0;
  const results = [];
  let matches;
  while ((matches = STYLE_REGEX.exec(style)) !== null) {
    const name = matches[1];
    if (matches[2]) {
      results.push([name, ...parseArguments(name, matches[2])]);
    } else if (matches[3] || matches[4]) {
      if (matches[3]) {
        results.push(["rgb", ...parseHex(matches[3])]);
      }
      if (matches[4]) {
        results.push(["bgRgb", ...parseHex(matches[4])]);
      }
    } else {
      results.push([name]);
    }
  }
  return results;
}
function makeTemplate(chalk2) {
  function buildStyle(styles) {
    const enabled = {};
    for (const layer of styles) {
      for (const style of layer.styles) {
        enabled[style[0]] = layer.inverse ? null : style.slice(1);
      }
    }
    let current = chalk2;
    for (const [styleName, styles2] of Object.entries(enabled)) {
      if (!Array.isArray(styles2)) {
        continue;
      }
      if (!(styleName in current)) {
        throw new Error(`Unknown Chalk style: ${styleName}`);
      }
      current = styles2.length > 0 ? current[styleName](...styles2) : current[styleName];
    }
    return current;
  }
  function template2(string) {
    const styles = [];
    const chunks = [];
    let chunk = [];
    string.replace(TEMPLATE_REGEX, (_, escapeCharacter, inverse, style, close, character) => {
      if (escapeCharacter) {
        chunk.push(unescape(escapeCharacter));
      } else if (style) {
        const string2 = chunk.join("");
        chunk = [];
        chunks.push(styles.length === 0 ? string2 : buildStyle(styles)(string2));
        styles.push({ inverse, styles: parseStyle(style) });
      } else if (close) {
        if (styles.length === 0) {
          throw new Error("Found extraneous } in Chalk template literal");
        }
        chunks.push(buildStyle(styles)(chunk.join("")));
        chunk = [];
        styles.pop();
      } else {
        chunk.push(character);
      }
    });
    chunks.push(chunk.join(""));
    if (styles.length > 0) {
      throw new Error(`Chalk template literal is missing ${styles.length} closing bracket${styles.length === 1 ? "" : "s"} (\`}\`)`);
    }
    return chunks.join("");
  }
  return template2;
}
function makeChalkTemplate(template2) {
  function chalkTemplate(firstString, ...arguments_) {
    if (!Array.isArray(firstString) || !Array.isArray(firstString.raw)) {
      throw new TypeError("A tagged template literal must be provided");
    }
    const parts = [firstString.raw[0]];
    for (let index = 1; index < firstString.raw.length; index++) {
      parts.push(
        String(arguments_[index - 1]).replace(/[{}\\]/g, "\\$&"),
        String(firstString.raw[index])
      );
    }
    return template2(parts.join(""));
  }
  return chalkTemplate;
}
var template = makeTemplate(import_chalk.default);
var chalk_template_default = makeChalkTemplate(template);
var templateStderr = makeTemplate(import_chalk.default.stderr);
var chalkTemplateStderr = makeChalkTemplate(templateStderr);

// lib/releases.ts
var import_remark = __toESM(require("remark"));
var import_remark_gfm = __toESM(require_remark_gfm());
var import_remark_mdx = __toESM(require("remark-mdx"));

// lib/remark-plugins/translate-to-nextra.ts
var import_unist_builder = __toESM(require("unist-builder"));
var import_unist_util_visit = __toESM(require("unist-util-visit"));
var admonitionTypeToCalloutType = {
  note: "default",
  tip: "default",
  info: "info",
  caution: "warning",
  danger: "error"
};
var translateToNextra = function() {
  return (tree, _file, done) => {
    (0, import_unist_util_visit.default)(tree, [{ type: "code" }], (node) => {
      const codeNode = node;
      if (codeNode.lang === "yml") {
        codeNode.lang = "yaml";
      }
      if (codeNode.meta) {
        codeNode.meta = codeNode.meta.replace(/title="(.*)"/, 'filename="$1"');
      }
    });
    (0, import_unist_util_visit.default)(tree, [{ type: "paragraph" }], (node, index, parent) => {
      const paragraphNode = node;
      const firstChild = paragraphNode.children[0];
      if (firstChild?.value.startsWith(":::")) {
        const match = firstChild.value.match(/^:::(\w*)\s*\n/);
        const type = match?.[1];
        firstChild.value = firstChild?.value.replace(/^:::.*\n/, "");
        const lastChild = paragraphNode.children[paragraphNode.children.length - 1];
        lastChild.value = lastChild.value.split("\n").slice(0, -1).join("\n");
        const calloutType = admonitionTypeToCalloutType[type ?? "note"] ?? "default";
        const callout = (0, import_unist_builder.default)("mdxJsxFlowElement", {
          name: "Callout",
          attributes: [
            { type: "mdxJsxAttribute", name: "type", value: calloutType }
          ],
          children: paragraphNode.children,
          data: { _mdxExplicitJsx: true }
        });
        parent.children.splice(index, 1, callout);
      }
    });
    done();
  };
};

// lib/transform-schema-markdown.ts
function createSchemaTransformer(processor, options) {
  const propagateTitle = options?.propagateTitle ?? false;
  const descriptionKey = options?.descriptionKey ?? "description";
  const transform = async (schema, title) => {
    if (typeof schema === "boolean") {
      return;
    }
    if (propagateTitle && title) {
      schema.title = title;
    }
    if (schema.description) {
      const { contents } = await processor.process(schema.description);
      schema[descriptionKey] = contents.toString();
    }
    const changes = [];
    if (schema.properties) {
      changes.push(
        ...Object.entries(schema.properties).map(
          ([name, definition]) => transform(definition, name)
        )
      );
    }
    if (schema.oneOf) {
      changes.push(...schema.oneOf.map((definition) => transform(definition)));
    }
    if (schema.anyOf) {
      changes.push(...schema.anyOf.map((definition) => transform(definition)));
    }
    if (typeof schema.items === "object") {
      const items = Array.isArray(schema.items) ? schema.items : [schema.items];
      changes.push(...items.map((definition) => transform(definition)));
    }
    if (typeof schema.additionalProperties === "object") {
      changes.push(transform(schema.additionalProperties));
    }
    return changes;
  };
  return transform;
}

// lib/releases.ts
var CACHE_DIR = (0, import_path.join)(__dirname, ".releases-cache");
var GITHUB_OWNER = "blake-mealey";
var GITHUB_REPO = "mantle";
async function loadFromGitHub() {
  const client = new import_rest.Octokit({ auth: process.env.GITHUB_TOKEN });
  const repoParams = { owner: GITHUB_OWNER, repo: GITHUB_REPO };
  const githubReleases = await client.paginate(
    client.rest.repos.listReleases,
    repoParams
  );
  const processor = (0, import_remark.default)().use(import_remark_gfm.default).use(translateToNextra).use(import_remark_mdx.default);
  const transformSchema = createSchemaTransformer(processor);
  return (await Promise.all(
    githubReleases.map(async (githubRelease) => {
      const asset_id = githubRelease.assets.find(
        (asset) => asset.name === "schema.json"
      )?.id;
      if (!asset_id)
        return;
      const response = await client.rest.repos.getReleaseAsset({
        ...repoParams,
        asset_id,
        headers: {
          accept: "application/octet-stream"
        }
      });
      const configurationSchema = JSON.parse(
        Buffer.from(response.data).toString("utf8")
      );
      await transformSchema(configurationSchema);
      return {
        version: githubRelease.tag_name,
        configurationSchema
      };
    })
  )).filter(isDefined);
}
async function saveToCache(releases) {
  console.log(`Saving ${releases.length} releases to cache...`);
  if ((0, import_fs.existsSync)(CACHE_DIR)) {
    await (0, import_promises.rm)(CACHE_DIR, { recursive: true });
  }
  await (0, import_promises.mkdir)(CACHE_DIR);
  await Promise.all(
    releases.map(async (release) => {
      const versionDir = (0, import_path.join)(CACHE_DIR, release.version);
      console.log(
        chalk_template_default`{grey Saving} ${release.version} {grey to} ${versionDir}`
      );
      await (0, import_promises.mkdir)(versionDir, { recursive: true });
      await (0, import_promises.writeFile)(
        (0, import_path.join)(versionDir, "configurationSchema.json"),
        JSON.stringify(release.configurationSchema)
      );
    })
  );
}
async function refreshReleasesCache() {
  const releases = await loadFromGitHub();
  saveToCache(releases);
}

// scripts/download-releases.ts
var import_dotenv = __toESM(require_main());
(0, import_dotenv.config)();
refreshReleasesCache().catch(console.error);
/*!
 * repeat-string <https://github.com/jonschlinkert/repeat-string>
 *
 * Copyright (c) 2014-2015, Jon Schlinkert.
 * Licensed under the MIT License.
 */
