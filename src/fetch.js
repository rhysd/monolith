export class FetchedData {
    constructor(d, t, m, u) {
        this.data = d;
        this.text = t;
        this.mime = m;
        this.url = u;
    }
}

function timeout10s(url) {
    return new Promise((_, reject) => {
        setTimeout(() => reject(new Error(`Fetching ${url} did not finish after 10 seconds`)), 10000);
    });
}

export async function fetchData(url, userAgent, wantBinary) {
    const req = new Request(url);
    if (userAgent) {
        req.headers.set('User-Agent', userAgent);
    }

    // Send request with 10 seconds timeout
    const res = await Promise.race([fetch(req), timeout10s(url)]);

    if (!res.ok) {
        throw new Error(`Fetching ${url} failed with status ${res.status} (${res.statusText})`);
    }

    const mime = res.headers.get('Content-Type') || ''; // Fallback to empty string
    if (wantBinary) {
        const buf = await res.arrayBuffer();
        return new FetchedData(new Uint8Array(buf), undefined, mime, res.url);
    } else {
        const text = await res.text();
        return new FetchedData(undefined, text, mime, res.url);
    }
}
