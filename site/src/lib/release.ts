// What the site needs from GitHub, read once per build and shared by every language page.
export const repo = 'https://github.com/Flaeren/voidflare';
export const artcraft = 'https://getartcraft.com';

export interface Dl { url: string; mb: number; sha: string }
export interface Info {
  releasePage: string;
  zipUrl: string; // what the main button and the menu link to: the small hub, else the full bundle
  version: string;
  hub: Dl | null; // the hub alone, a few MB, installs the apps you pick
  full: Dl | null; // the whole thing in one ZIP, for a PC without a network
  downloads: number;
}

async function load(): Promise<Info> {
  const info: Info = { releasePage: repo + '/releases/latest', zipUrl: repo + '/releases/latest', version: '', hub: null, full: null, downloads: 0 };
  try {
    // VOIDFLARE_RELEASE_API points the build at another JSON shaped like GitHub's, to look at the page before a release exists.
    const res = await fetch(process.env.VOIDFLARE_RELEASE_API || 'https://api.github.com/repos/Flaeren/voidflare/releases/latest', { headers: { 'User-Agent': 'voidflare-site' } });
    if (res.ok) {
      const rel: any = await res.json();
      const assets: any[] = rel.assets || [];
      const own = (a: any) => a.browser_download_url.startsWith(repo + '/releases/download/');
      // By name, not "the first ZIP": a release carries the hub, the full bundle, and one ZIP per app and per engine.
      const hubAsset = assets.find((a) => /^voidflare-hub-.*-windows\.zip$/.test(a.name) && own(a));
      const fullAsset = assets.find((a) => /^voidflare-(?!hub-).*-windows\.zip$/.test(a.name) && own(a));
      let sums = '';
      const sumsAsset = assets.find((a) => a.name === 'SHA256SUMS.txt');
      if (sumsAsset) {
        try {
          sums = await (await fetch(sumsAsset.browser_download_url)).text();
        } catch {
          /* the buttons still work, only the checksum lines are left out */
        }
      }
      const describe = (a: any): Dl => {
        const line = sums.split('\n').find((l) => l.trim().endsWith(' ' + a.name));
        return { url: a.browser_download_url, mb: Math.max(1, Math.round(a.size / 1048576)), sha: line ? line.trim().split(/\s+/)[0] : '' };
      };
      if (hubAsset) info.hub = describe(hubAsset);
      if (fullAsset) info.full = describe(fullAsset);
      if (info.hub || info.full) {
        info.version = String(rel.tag_name).replace(/^v/, '');
        info.zipUrl = (info.hub || info.full)!.url;
      }
    }
  } catch {
    /* keep the fallback */
  }
  // Downloads so far: only the hub and the full bundle, every version. The checksum file and the per-app packs are left out,
  // otherwise one install would count several times. The hub's own self-update also counts, so it is a rough number.
  try {
    if (!process.env.VOIDFLARE_RELEASE_API) {
      const all = await fetch('https://api.github.com/repos/Flaeren/voidflare/releases?per_page=100', { headers: { 'User-Agent': 'voidflare-site' } });
      if (all.ok) {
        for (const r of (await all.json()) as any[]) for (const a of r.assets || []) if (/^voidflare-(hub-)?\d[\w.]*-windows\.zip$/.test(a.name)) info.downloads += a.download_count || 0;
      }
    }
  } catch {
    /* no counter then */
  }
  return info;
}

let cached: Promise<Info> | undefined;
export const release = () => (cached ??= load());

/** Fills {tokens} in a text of src/i18n.ts. */
export function fill(text: string, extra: Record<string, string> = {}): string {
  const vars: Record<string, string> = {
    repo,
    artcraft,
    credits: repo + '/blob/main/CREDITS.md',
    engines: repo + '/blob/main/src/engines.json',
    build: repo + '#build-from-source',
    issues: repo + '/issues',
    advisory: repo + '/security/advisories/new',
    ...extra,
  };
  return text.replace(/\{(\w+)\}/g, (m, k) => (k in vars ? vars[k] : m));
}
