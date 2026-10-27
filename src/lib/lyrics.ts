/** Lyrics via lrclib.net — free, no API key.
 *  Prefers synced (LRC) lyrics; falls back to plain text. */

export interface LrcLine {
  /** milliseconds */
  t: number;
  text: string;
}

export interface Lyrics {
  synced: LrcLine[] | null;
  plain: string | null;
}

export function parseLrc(lrc: string): LrcLine[] {
  const out: LrcLine[] = [];
  const re = /\[(\d+):(\d+(?:\.\d+)?)\]/g;
  for (const raw of lrc.split("\n")) {
    const text = raw.replace(re, "").trim();
    let m: RegExpExecArray | null;
    re.lastIndex = 0;
    while ((m = re.exec(raw))) {
      const t = parseInt(m[1]) * 60000 + parseFloat(m[2]) * 1000;
      if (text) out.push({ t, text });
    }
  }
  return out.sort((a, b) => a.t - b.t);
}

interface LrcEntry {
  syncedLyrics?: string | null;
  plainLyrics?: string | null;
}

function toLyrics(j: LrcEntry): Lyrics | null {
  const synced = j.syncedLyrics ? parseLrc(j.syncedLyrics) : null;
  const plain = j.plainLyrics || null;
  return synced || plain ? { synced, plain } : null;
}

export async function fetchLyrics(
  title: string,
  artist: string,
  album: string,
  durationMs: number,
): Promise<Lyrics | null> {
  try {
    // Exact lookup — 404s when duration is off by >2s or metadata
    // doesn't match their record, so fall back to search.
    const exact = new URLSearchParams({
      track_name: title,
      artist_name: artist,
      album_name: album,
      duration: String(Math.round(durationMs / 1000)),
    });
    const res = await fetch(`https://lrclib.net/api/get?${exact}`);
    if (res.ok) {
      const l = toLyrics(await res.json());
      if (l) return l;
    }

    const search = new URLSearchParams({
      track_name: title,
      artist_name: artist,
    });
    const res2 = await fetch(`https://lrclib.net/api/search?${search}`);
    if (!res2.ok) return null;
    const hits: LrcEntry[] = await res2.json();
    if (!Array.isArray(hits)) return null;
    // Prefer a hit that has synced lyrics.
    for (const h of hits) {
      const l = toLyrics(h);
      if (l?.synced) return l;
    }
    return hits.length ? toLyrics(hits[0]) : null;
  } catch {
    return null;
  }
}
