/** Extract a rough dominant color from album art.
 *  Fetches the bytes first so the canvas never gets tainted by
 *  cross-origin images (works for https, asset://, data: URLs). */
export async function dominantColor(url: string): Promise<string | null> {
  try {
    const blob = await (await fetch(url)).blob();
    const img = await createImageBitmap(blob);

    const c = document.createElement("canvas");
    const s = 16;
    c.width = c.height = s;
    const ctx = c.getContext("2d");
    if (!ctx) return null;
    ctx.drawImage(img, 0, 0, s, s);
    const d = ctx.getImageData(0, 0, s, s).data;

    let r = 0, g = 0, b = 0, n = 0;
    for (let i = 0; i < d.length; i += 4) {
      // skip very dark / near-white pixels for a punchier accent
      const [pr, pg, pb] = [d[i], d[i + 1], d[i + 2]];
      const lum = 0.2126 * pr + 0.7152 * pg + 0.0722 * pb;
      if (lum > 30 && lum < 235) {
        r += pr; g += pg; b += pb; n++;
      }
    }
    if (!n) return null;
    r = Math.round(r / n); g = Math.round(g / n); b = Math.round(b / n);
    // boost saturation a bit for accent use
    const max = Math.max(r, g, b), min = Math.min(r, g, b);
    const boost = 1.15;
    const mid = (max + min) / 2;
    r = Math.min(255, Math.round(mid + (r - mid) * boost));
    g = Math.min(255, Math.round(mid + (g - mid) * boost));
    b = Math.min(255, Math.round(mid + (b - mid) * boost));
    return `rgb(${r}, ${g}, ${b})`;
  } catch {
    return null; // fetch/decode failure — keep default accent
  }
}
