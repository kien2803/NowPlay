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

export async function fetchLyrics(
  title: string,
  artist: string,
  album: string,
  durationMs: number,
): Promise<Lyrics | null> {
  try {
    const q = new URLSearchParams({
      track_name: title,
      artist_name: artist,
      album_name: album,
      duration: String(Math.round(durationMs / 1000)),
    });
    const res = await fetch(`https://lrclib.net/api/get?${q}`);
    if (!res.ok) return null;
    const j = await res.json();
    return {
      synced: j.syncedLyrics ? parseLrc(j.syncedLyrics) : null,
      plain: j.plainLyrics || null,
    };
  } catch {
    return null;
  }
}
