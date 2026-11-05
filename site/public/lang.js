// Language of the visitor. English pages (marked data-auto) send you to your own language once, the first time:
// the choice you make with the language menu is remembered in this browser only (localStorage, no cookie) and wins
// over the browser's language from then on. Pages already in another language are never redirected, so a shared
// /it/ link always opens in Italian. Nothing is sent anywhere.
(function () {
  var langs = ['en', 'it', 'es', 'fr', 'de'];
  var root = document.documentElement;

  document.addEventListener('click', function (e) {
    var a = e.target && e.target.closest ? e.target.closest('a[data-lang]') : null;
    if (!a) return;
    try { localStorage.setItem('vf-lang', a.getAttribute('data-lang')); } catch (err) { /* private mode: no memory, still works */ }
  });

  if (!root.hasAttribute('data-auto')) return;
  var want = null;
  try { want = localStorage.getItem('vf-lang'); } catch (err) { /* ignore */ }
  if (langs.indexOf(want) < 0) {
    want = null;
    var prefs = navigator.languages && navigator.languages.length ? navigator.languages : [navigator.language || ''];
    for (var i = 0; i < prefs.length; i++) {
      var code = String(prefs[i]).slice(0, 2).toLowerCase();
      if (langs.indexOf(code) >= 0) { want = code; break; }
    }
  }
  if (want && want !== 'en') {
    location.replace('/' + want + (root.getAttribute('data-page') ? '/' + root.getAttribute('data-page') : '') + location.hash);
  }
})();
