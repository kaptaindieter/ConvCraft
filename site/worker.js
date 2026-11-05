// voidflare.app is static (the assets binding). This code does one thing before serving them:
// a request that arrives over plain HTTP is sent to HTTPS with a permanent redirect.
export default {
  async fetch(request, env) {
    const url = new URL(request.url);
    if (url.protocol === 'http:' && url.hostname === 'voidflare.app') {
      url.protocol = 'https:';
      return Response.redirect(url.toString(), 301);
    }
    return env.ASSETS.fetch(request);
  },
};
