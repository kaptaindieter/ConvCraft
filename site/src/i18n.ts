// Every visible text of voidflare.app, in the five languages of the app itself.
// English is the source; the others follow the same keys (TypeScript checks it). The translations have not been
// reviewed by native speakers yet, like the app's. Strings with HTML are written by us and inserted with set:html;
// {tokens} such as {repo} are filled in by fill() in the components.

export const LANGS = ['en', 'it', 'es', 'fr', 'de'] as const;
export type Lang = (typeof LANGS)[number];
export const NATIVE: Record<Lang, string> = { en: 'English', it: 'Italiano', es: 'Español', fr: 'Français', de: 'Deutsch' };
export const LOCALE: Record<Lang, string> = { en: 'en_US', it: 'it_IT', es: 'es_ES', fr: 'fr_FR', de: 'de_DE' };

export interface Strings {
  meta: { title: string; desc: string; og: string; tw: string; ld: string; alt: string };
  nav: { finder: string; apps: string; offline: string; get: string; github: string; lang: string };
  hero: { eyebrow: string; h1: [string, string, string]; lead: string; dl: string; gh: string; hubWord: string; zipWord: string; convertTo: string };
  fmt: { pdf: string; word: string; ppt: string; html: string; rtf: string; txt: string; png: string; jpg: string; dwgSheets: string; dwgWhole: string };
  more: { std: string; slides: string; pdf: string; dwg: string };
  finder: { h2: string; sub: string; aria: string; picked: string; becomes: string; doneBy: string };
  apps: { h2: string; faint: string; text: [string, string, string, string, string, string, string, string, string]; soon: string; soonText: string; langs: string };
  offline: { h2: string; items: [string, string, string, string] };
  how: { h2: string; p1: string; p2: string };
  safe: { h2: string; sub: string; items: [string, string, string, string]; warn: string };
  get: {
    h2: string; sub: string; hubText: string; noNet: string; larger: string; zipOnly: string; smart: string;
    links: string; check: string; hubSha: string; fullSha: string; counter: string; release: string; source: string; build: string;
    btnHub: string; btnFull: string; btnZip: string; star: string;
    winLabel: string; linuxLabel: string; linuxBtn: string; linuxNote: string;
  };
  foot: [string, string, string, string];
  ctx: { dl: string; source: string; privacy: string; copy: string; copied: string };
  privacy: {
    title: string; desc: string; og: string; back: string; h1: string; updated: string; intro: string;
    siteH: string; site: [string, string, string]; dlH: string; dl: string; progH: string; prog: [string, string, string, string, string];
    rightsH: string; rights: string; contactH: string; contact: [string, string, string];
  };
}

const en: Strings = {
  meta: {
    title: 'voidflare: offline file converters for Windows',
    desc: 'Drop a file, pick a format. Convert Office documents, PDF, DWG, images, audio, video and e-books on your PC. Nothing to install, nothing uploaded.',
    og: 'Drop a file, pick a format. Convert Office documents, PDF, DWG, images, audio, video and e-books on your PC. Nothing to install, nothing uploaded.',
    tw: 'Drop a file, pick a format. Nothing to install, nothing uploaded.',
    ld: 'Free, open-source offline file converters for Windows (Office documents, PDF, DWG, images, audio, video and e-books), built on the ArtCraft open-source engines. Files are converted on your PC and never uploaded.',
    alt: 'voidflare: the indigo flame mark and the text Offline file converters for Windows',
  },
  nav: { finder: 'CONVERT', apps: 'APPS', offline: 'OFFLINE', get: 'DOWNLOAD', github: 'GITHUB ↗', lang: 'Language' },
  hero: {
    eyebrow: 'OFFLINE FILE CONVERTERS FOR WINDOWS',
    h1: ['Drop a file.', 'Pick a format.', 'It never leaves your PC.'],
    lead: 'Convert Office documents, PDF, DWG, images, audio, video and e-books entirely on your machine. Nothing to install, nothing uploaded, no account.',
    dl: 'DOWNLOAD FOR WINDOWS', gh: 'VIEW ON GITHUB ↗', hubWord: 'HUB', zipWord: 'PORTABLE ZIP', convertTo: 'CONVERT TO →',
  },
  fmt: { pdf: 'PDF', word: 'Word (DOCX)', ppt: 'PowerPoint (PPTX)', html: 'HTML', rtf: 'RTF', txt: 'Text (TXT)', png: 'PNG pictures', jpg: 'JPG pictures', dwgSheets: 'PDF · sheets, merged', dwgWhole: 'PDF · whole drawing' },
  more: {
    std: 'One file or a whole folder. The new PDF lands next to the original.',
    slides: 'Slide for slide. The new PDF lands next to the original.',
    pdf: 'The same app also merges, splits, extracts pages, shrinks and reads scanned pages (OCR).',
    dwg: 'Choose the sheets of the drawing or the whole drawing on one page.',
  },
  finder: { h2: 'What file do you have?', sub: 'Pick it and see what it can become.', aria: 'File type', picked: 'YOU PICKED', becomes: 'IT CAN BECOME', doneBy: 'Done by {app}. ' },
  apps: {
    h2: 'Nine apps, one hub', faint: 'EACH APP CARRIES ITS OWN ENGINE',
    text: [
      'DOCX, DOC, RTF and ODT to PDF, one file or a whole folder.',
      'XLSX, CSV and TSV spreadsheets to PDF. No Office needed.',
      'PowerPoint presentations to PDF, slide for slide.',
      'Convert to Word, PowerPoint, HTML, RTF, text or pictures. Merge, split, extract pages, shrink, OCR.',
      'Drawings to PDF: the sheets merged or separate, or the whole drawing on one page.',
      'Convert pictures between PNG, JPG, WebP, TIFF, BMP and more.',
      'Convert audio to WAV, FLAC or AIFF (no MP3 output).',
      'Convert video to MP4, ProRes or GIF, or keep just the sound.',
      'EPUB and Kindle (AZW3, MOBI) books to EPUB, text, HTML, PDF or Word, and back. DRM-protected books are refused.',
    ],
    soon: 'FOR LLMS', soonText: 'voidflare-cli.exe and an MCP server let Claude and other AI assistants convert your files, only inside folders you allow.',
    langs: 'Available in English, Italiano, Español, Français and Deutsch.',
  },
  offline: {
    h2: 'Offline is the point, not a feature.',
    items: [
      'Files are converted on your PC and never sent anywhere.',
      'No account, no sign-in, no telemetry to switch off.',
      'No Microsoft Office or any other program required.',
      'Open source, with every licence and credit in the folder.',
    ],
  },
  how: {
    h2: 'Who does the converting?',
    p1: 'Almost every conversion is done by a command-line engine of the <a href="{artcraft}">ArtCraft</a> open-source suite (WordCraft, GridCraft, DeckCraft, PdfCraft, CADCraft, PhotoCraft, SoundCraft and FilmCraft), written by its authors and used under their MIT OR Apache-2.0 licences. The exception is reading and writing EPUB and Kindle books, which is our own Rust code. voidflare adds the drag-and-drop interface. Each app is installed with its engine and that engine\'s licences, or, if your PC already has a version of ArtCraft that we have tested, it uses yours and does not download a second copy.',
    p2: 'It is an independent project, not affiliated with or endorsed by the ArtCraft team. <a href="{credits}">Credits and licences ↗</a>',
  },
  safe: {
    h2: 'Is it safe to run?', sub: 'What you can check yourself, and what is still missing.',
    items: [
      'The source is public, and so are the build instructions: <a href="{build}">build it yourself</a> instead of trusting the zip.',
      'The engines are pinned: their exact versions and SHA-256 hashes are in <a href="{engines}">engines.json</a>, and the build refuses any file that differs.',
      'The converters do not use the network. The hub asks GitHub for the latest release when it starts (you can switch that off with AUTO-CHECK), and downloads an app only when you press Install; the requests carry no file and no file names.',
      'The downloads are plain files on GitHub Releases, with their SHA-256 shown next to the buttons; the hub checks every pack against the release\'s SHA256SUMS.txt before installing it.',
    ],
    warn: 'The programs are not code-signed yet, so Windows SmartScreen may warn on first run. Code signing is planned.',
  },
  get: {
    h2: 'Get voidflare', sub: 'Windows 10 and 11 · portable',
    hubText: 'Download the small hub, unzip it to a short path such as <code>C:\\voidflare</code> and run <code>voidflare.exe</code>. Then press <em>Install</em> on the converters you need: each one is a separate download (a few MB, plus its engine unless your PC already has a tested ArtCraft one), checked against the release\'s SHA-256 before it is installed. Nothing is downloaded until you click.',
    noNet: 'No network where you work? The full bundle has everything in one file ({full}).', larger: 'larger',
    zipOnly: 'Unzip the whole folder first (do not run it from inside the zip), then put it in a short path such as <code>C:\\voidflare</code> and run <code>voidflare.exe</code>.',
    smart: 'The programs are not code-signed yet, so Windows SmartScreen may warn on first run: choose <em>More info</em>, then <em>Run anyway</em>.',
    links: 'The buttons link straight to the files on GitHub Releases, nothing is served from this site.', check: ' Check them after downloading:',
    hubSha: 'HUB · SHA-256', fullSha: 'FULL BUNDLE · SHA-256',
    counter: '{n} downloads so far (hub and full bundle, all versions)',
    release: 'Release page ↗', source: 'Source code ↗', build: 'Build it yourself ↗', star: 'If voidflare is useful to you, a star on GitHub helps other people find it.',
    btnHub: 'DOWNLOAD HUB · {mb} MB', btnFull: 'FULL BUNDLE · {mb} MB', btnZip: 'DOWNLOAD ZIP',
    winLabel: 'WINDOWS 10 / 11 · AVAILABLE', linuxLabel: 'LINUX · COMING SOON', linuxBtn: 'DOWNLOAD FOR LINUX', linuxNote: 'Planned. Watch the repository on GitHub to hear when it lands.',
  },
  foot: [
    'VOIDFLARE.APP · MIT OR APACHE-2.0',
    'OPEN SOURCE · <a href="{artcraft}">BUILT ON THE ARTCRAFT SUITE ↗</a> · <a href="{credits}">CREDITS</a>',
    'MADE BY GABRIELE DENARO · <a href="https://github.com/Flaeren">GITHUB ↗</a> · <a href="https://gabrieledenaro.com">GABRIELEDENARO.COM ↗</a>',
    'CONTACT · <a href="{issues}">ISSUES ↗</a> · <a href="{advisory}">REPORT A VULNERABILITY ↗</a> · <a href="{privacy}">PRIVACY</a>',
  ],
  ctx: { dl: 'DOWNLOAD FOR WINDOWS', source: 'SOURCE ON GITHUB ↗', privacy: 'PRIVACY', copy: 'COPY PAGE ADDRESS', copied: 'COPIED' },
  privacy: {
    title: 'Privacy: what voidflare does with your data',
    desc: 'What voidflare.app and the voidflare programs do with your data: no cookies, no analytics, files never leave your PC.',
    og: 'No cookies, no analytics, files never leave your PC.',
    back: '← VOIDFLARE', h1: 'Privacy', updated: 'LAST UPDATED 1 NOVEMBER 2026',
    intro: 'voidflare is a free, open-source project by Gabriele Denaro. The short version: your files are converted on your own PC, this website sets no cookies and uses no analytics, and I do not collect personal data myself.',
    siteH: 'This website',
    site: [
      'No cookies, no analytics, no advertising or tracking scripts, no forms and no accounts. If you pick a language, your browser remembers that choice on your device only (local storage); it is never sent to anyone.',
      'The fonts are served from this site. Nothing is loaded from Google or any other third party.',
      'The site is hosted on Cloudflare. Like any host, Cloudflare processes technical data such as your IP address to deliver and protect the pages. That is Cloudflare\'s processing, covered by <a href="https://www.cloudflare.com/privacypolicy/">its privacy policy</a>; I do not receive it as a list of visitors.',
    ],
    dlH: 'Downloading',
    dl: 'The download buttons link straight to the files on GitHub Releases. When you download one, GitHub receives the usual technical data (your IP address, for example) under <a href="https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement">its privacy statement</a>. Nothing is requested from this site. The download counter on the page is the number GitHub keeps for the files, read when the site is built; it does not know who downloaded.',
    progH: 'The programs',
    prog: [
      'Conversions run entirely on your PC with an ArtCraft engine, the one that comes with the app or, if you have a tested version of it installed, yours. Your files and their names are never uploaded.',
      'There is no account, no telemetry and no usage statistics.',
      'Two things use the network, and only these: when the hub starts it asks GitHub for the latest release, so it can tell you a newer version exists; and when you press Install or Update on an app (or UPDATE NOW for the hub itself), it downloads that file from the same GitHub release and checks its SHA-256. Neither request carries a file or a file name, only the request itself (so GitHub sees your IP address). You can switch the first off with <code>AUTO-CHECK</code> in the hub, or by setting the environment variable <code>VOIDFLARE_NO_UPDATE_CHECK</code>. Nothing is installed without your click.',
      'The programs keep a few settings on your PC (the language and whether to check for updates), in <code>%APPDATA%\\voidflare</code>. They stay there.',
      'The converter apps themselves never use the network.',
    ],
    rightsH: 'Your rights',
    rights: 'Because I hold no personal data about you, there is nothing for me to show, correct or delete. If you think otherwise, or have any question, write to me through one of the contacts below.',
    contactH: 'Contact',
    contact: [
      'Questions and bugs: <a href="{issues}">GitHub issues</a>.',
      'Security problems, in private: <a href="{advisory}">report a vulnerability</a>.',
      'Me: <a href="https://gabrieledenaro.com">gabrieledenaro.com</a>.',
    ],
  },
};

const it: Strings = {
  meta: {
    title: 'voidflare: convertitori di file offline per Windows',
    desc: 'Trascina un file, scegli il formato. Conversioni di documenti Office, PDF, DWG, immagini, audio, video ed e-book che girano sul tuo PC. Niente da installare, niente caricato online.',
    og: 'Trascina un file, scegli il formato. Conversioni di documenti Office, PDF, DWG, immagini, audio, video ed e-book sul tuo PC. Niente da installare, niente caricato online.',
    tw: 'Trascina un file, scegli il formato. Niente da installare, niente caricato online.',
    ld: 'Convertitori di file offline per Windows, gratuiti e open source (documenti Office, PDF, DWG, immagini, audio, video ed e-book), costruiti sui motori open source di ArtCraft. I file vengono convertiti sul tuo PC e non vengono mai caricati.',
    alt: 'voidflare: il simbolo della fiamma indaco e il testo Convertitori di file offline per Windows',
  },
  nav: { finder: 'CONVERTI', apps: 'APP', offline: 'OFFLINE', get: 'SCARICA', github: 'GITHUB ↗', lang: 'Lingua' },
  hero: {
    eyebrow: 'CONVERTITORI DI FILE OFFLINE PER WINDOWS',
    h1: ['Trascina un file.', 'Scegli un formato.', 'Non lascia mai il tuo PC.'],
    lead: 'Conversioni di documenti Office, PDF, DWG, immagini, audio, video ed e-book che girano interamente sul tuo computer. Niente da installare, niente caricato online, nessun account.',
    dl: 'SCARICA PER WINDOWS', gh: 'VEDI SU GITHUB ↗', hubWord: 'HUB', zipWord: 'ZIP PORTABILE', convertTo: 'CONVERTI IN →',
  },
  fmt: { pdf: 'PDF', word: 'Word (DOCX)', ppt: 'PowerPoint (PPTX)', html: 'HTML', rtf: 'RTF', txt: 'Testo (TXT)', png: 'Immagini PNG', jpg: 'Immagini JPG', dwgSheets: 'PDF · fogli, uniti', dwgWhole: 'PDF · disegno intero' },
  more: {
    std: 'Un file o un\'intera cartella. Il nuovo PDF compare accanto all\'originale.',
    slides: 'Diapositiva per diapositiva. Il nuovo PDF compare accanto all\'originale.',
    pdf: 'La stessa app unisce, divide, estrae pagine, comprime e legge le pagine scansionate (OCR).',
    dwg: 'Scegli i fogli del disegno oppure il disegno intero su una sola pagina.',
  },
  finder: { h2: 'Che file hai?', sub: 'Sceglilo e guarda in cosa può trasformarsi.', aria: 'Tipo di file', picked: 'HAI SCELTO', becomes: 'PUÒ DIVENTARE', doneBy: 'Lo fa {app}. ' },
  apps: {
    h2: 'Nove app, un solo hub', faint: 'OGNI APP PORTA IL SUO MOTORE',
    text: [
      'DOCX, DOC, RTF e ODT in PDF, un file o un\'intera cartella.',
      'Fogli XLSX, CSV e TSV in PDF. Non serve Office.',
      'Presentazioni PowerPoint in PDF, diapositiva per diapositiva.',
      'Converti in Word, PowerPoint, HTML, RTF, testo o immagini. Unisci, dividi, estrai pagine, comprimi, OCR.',
      'Disegni in PDF: i fogli uniti o separati, oppure l\'intero disegno su una pagina.',
      'Converti immagini tra PNG, JPG, WebP, TIFF, BMP e altri formati.',
      'Converti l\'audio in WAV, FLAC o AIFF (niente MP3 in uscita).',
      'Converti i video in MP4, ProRes o GIF, oppure tieni solo l\'audio.',
      'Libri EPUB e Kindle (AZW3, MOBI) in EPUB, testo, HTML, PDF o Word, e viceversa. I libri con DRM vengono rifiutati.',
    ],
    soon: 'PER GLI LLM', soonText: 'voidflare-cli.exe e un server MCP permettono a Claude e ad altri assistenti IA di convertire i tuoi file, solo nelle cartelle che autorizzi.',
    langs: 'Disponibile in English, Italiano, Español, Français e Deutsch.',
  },
  offline: {
    h2: 'Offline è il punto, non un optional.',
    items: [
      'I file vengono convertiti sul tuo PC e non sono mai inviati da nessuna parte.',
      'Nessun account, nessun accesso, nessuna telemetria da disattivare.',
      'Non serve Microsoft Office né altri programmi.',
      'Open source, con tutte le licenze e i crediti nella cartella.',
    ],
  },
  how: {
    h2: 'Chi fa la conversione?',
    p1: 'Quasi ogni conversione la fa un motore a riga di comando della suite open source <a href="{artcraft}">ArtCraft</a> (WordCraft, GridCraft, DeckCraft, PdfCraft, CADCraft, PhotoCraft, SoundCraft e FilmCraft), scritto dai suoi autori e usato con le loro licenze MIT OR Apache-2.0. L\'eccezione è la lettura e scrittura dei libri EPUB e Kindle, che è codice Rust nostro. voidflare aggiunge l\'interfaccia con il trascinamento. Ogni app viene installata con il suo motore e le relative licenze oppure, se sul tuo PC c\'è già una versione di ArtCraft che abbiamo provato, usa la tua e non scarica una seconda copia.',
    p2: 'È un progetto indipendente, non affiliato né approvato dal team di ArtCraft. <a href="{credits}">Crediti e licenze ↗</a>',
  },
  safe: {
    h2: 'È sicuro da eseguire?', sub: 'Cosa puoi controllare da solo e cosa ancora manca.',
    items: [
      'Il codice sorgente è pubblico, e lo sono anche le istruzioni di compilazione: <a href="{build}">compilalo tu</a> invece di fidarti dello zip.',
      'I motori sono fissati: le versioni esatte e gli hash SHA-256 sono in <a href="{engines}">engines.json</a> e la compilazione rifiuta qualsiasi file diverso.',
      'I convertitori non usano la rete. L\'hub chiede a GitHub l\'ultima versione quando si avvia (puoi disattivarlo con AUTO-CHECK) e scarica un\'app solo quando premi Installa; le richieste non contengono file né nomi di file.',
      'I download sono semplici file su GitHub Releases, con lo SHA-256 mostrato accanto ai pulsanti; l\'hub controlla ogni pacchetto con lo SHA256SUMS.txt della release prima di installarlo.',
    ],
    warn: 'I programmi non sono ancora firmati digitalmente, quindi Windows SmartScreen può avvisarti al primo avvio. La firma è in programma.',
  },
  get: {
    h2: 'Scarica voidflare', sub: 'Windows 10 e 11 · portabile',
    hubText: 'Scarica l\'hub, che è piccolo, estrailo in un percorso corto come <code>C:\\voidflare</code> ed esegui <code>voidflare.exe</code>. Poi premi <em>Installa</em> sui convertitori che ti servono: ognuno è un download separato (pochi MB, più il suo motore se sul PC non c\'è già un ArtCraft provato), verificato con lo SHA-256 della release prima dell\'installazione. Non viene scaricato nulla finché non clicchi.',
    noNet: 'Dove lavori non c\'è rete? Il pacchetto completo ha tutto in un unico file ({full}).', larger: 'più grande',
    zipOnly: 'Estrai prima l\'intera cartella (non eseguirlo da dentro lo zip), mettila in un percorso corto come <code>C:\\voidflare</code> ed esegui <code>voidflare.exe</code>.',
    smart: 'I programmi non sono ancora firmati digitalmente, quindi Windows SmartScreen può avvisarti al primo avvio: scegli <em>Ulteriori informazioni</em>, poi <em>Esegui comunque</em>.',
    links: 'I pulsanti puntano direttamente ai file su GitHub Releases, da questo sito non viene servito nulla.', check: ' Controllali dopo il download:',
    hubSha: 'HUB · SHA-256', fullSha: 'PACCHETTO COMPLETO · SHA-256',
    counter: '{n} download finora (hub e pacchetto completo, tutte le versioni)',
    release: 'Pagina della release ↗', source: 'Codice sorgente ↗', build: 'Compilalo tu ↗', star: 'Se voidflare ti è utile, una stella su GitHub aiuta altri a trovarlo.',
    btnHub: 'SCARICA L\'HUB · {mb} MB', btnFull: 'PACCHETTO COMPLETO · {mb} MB', btnZip: 'SCARICA LO ZIP',
    winLabel: 'WINDOWS 10 / 11 · DISPONIBILE', linuxLabel: 'LINUX · IN ARRIVO', linuxBtn: 'SCARICA PER LINUX', linuxNote: 'In programma. Segui il repository su GitHub per sapere quando arriva.',
  },
  foot: [
    'VOIDFLARE.APP · MIT OR APACHE-2.0',
    'OPEN SOURCE · <a href="{artcraft}">COSTRUITO SULLA SUITE ARTCRAFT ↗</a> · <a href="{credits}">CREDITI</a>',
    'REALIZZATO DA GABRIELE DENARO · <a href="https://github.com/Flaeren">GITHUB ↗</a> · <a href="https://gabrieledenaro.com">GABRIELEDENARO.COM ↗</a>',
    'CONTATTI · <a href="{issues}">SEGNALAZIONI ↗</a> · <a href="{advisory}">SEGNALA UNA VULNERABILITÀ ↗</a> · <a href="{privacy}">PRIVACY</a>',
  ],
  ctx: { dl: 'SCARICA PER WINDOWS', source: 'CODICE SU GITHUB ↗', privacy: 'PRIVACY', copy: 'COPIA L\'INDIRIZZO DELLA PAGINA', copied: 'COPIATO' },
  privacy: {
    title: 'Privacy: cosa fa voidflare con i tuoi dati',
    desc: 'Cosa fanno voidflare.app e i programmi voidflare con i tuoi dati: niente cookie, niente analytics, i file non lasciano mai il tuo PC.',
    og: 'Niente cookie, niente analytics, i file non lasciano mai il tuo PC.',
    back: '← VOIDFLARE', h1: 'Privacy', updated: 'ULTIMO AGGIORNAMENTO 1 NOVEMBRE 2026',
    intro: 'voidflare è un progetto gratuito e open source di Gabriele Denaro. In breve: i tuoi file vengono convertiti sul tuo PC, questo sito non imposta cookie e non usa analytics, e io non raccolgo dati personali.',
    siteH: 'Questo sito',
    site: [
      'Nessun cookie, nessun analytics, nessuno script pubblicitario o di tracciamento, nessun modulo e nessun account. Se scegli una lingua, il browser ricorda la scelta solo sul tuo dispositivo (archivio locale); non viene mai inviata a nessuno.',
      'I font sono serviti da questo sito. Non viene caricato nulla da Google o da altre terze parti.',
      'Il sito è ospitato su Cloudflare. Come ogni host, Cloudflare tratta dati tecnici come il tuo indirizzo IP per consegnare e proteggere le pagine. È un trattamento di Cloudflare, coperto dalla <a href="https://www.cloudflare.com/privacypolicy/">sua informativa</a>; io non lo ricevo come elenco di visitatori.',
    ],
    dlH: 'Download',
    dl: 'I pulsanti di download puntano direttamente ai file su GitHub Releases. Quando ne scarichi uno, GitHub riceve i soliti dati tecnici (ad esempio il tuo indirizzo IP) secondo <a href="https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement">la sua informativa</a>. Da questo sito non viene richiesto nulla. Il contatore dei download nella pagina è il numero che GitHub conserva per i file, letto quando il sito viene compilato: non sa chi ha scaricato.',
    progH: 'I programmi',
    prog: [
      'Le conversioni girano interamente sul tuo PC con un motore ArtCraft: quello che arriva con l\'app oppure, se ne hai installata una versione provata, il tuo. I tuoi file e i loro nomi non vengono mai caricati.',
      'Non c\'è nessun account, nessuna telemetria e nessuna statistica d\'uso.',
      'Due cose usano la rete, e solo queste: all\'avvio l\'hub chiede a GitHub l\'ultima versione, per dirti se ne esiste una più nuova; e quando premi Installa o Aggiorna su un\'app (o UPDATE NOW per l\'hub stesso), scarica quel file dalla stessa release di GitHub e ne verifica lo SHA-256. Nessuna delle due richieste contiene un file o un nome di file, solo la richiesta in sé (quindi GitHub vede il tuo indirizzo IP). La prima si disattiva con <code>AUTO-CHECK</code> nell\'hub o impostando la variabile d\'ambiente <code>VOIDFLARE_NO_UPDATE_CHECK</code>. Non viene installato nulla senza un tuo clic.',
      'I programmi conservano alcune impostazioni sul tuo PC (la lingua e se controllare gli aggiornamenti), in <code>%APPDATA%\\voidflare</code>. Restano lì.',
      'Le app di conversione, di per sé, non usano mai la rete.',
    ],
    rightsH: 'I tuoi diritti',
    rights: 'Poiché non conservo alcun dato personale su di te, non c\'è nulla che io possa mostrare, correggere o cancellare. Se pensi il contrario, o hai domande, scrivimi a uno dei contatti qui sotto.',
    contactH: 'Contatti',
    contact: [
      'Domande e bug: <a href="{issues}">segnalazioni su GitHub</a>.',
      'Problemi di sicurezza, in privato: <a href="{advisory}">segnala una vulnerabilità</a>.',
      'Io: <a href="https://gabrieledenaro.com">gabrieledenaro.com</a>.',
    ],
  },
};

const es: Strings = {
  meta: {
    title: 'voidflare: convertidores de archivos sin conexión para Windows',
    desc: 'Suelta un archivo, elige un formato. Conversiones de documentos de Office, PDF, DWG, imágenes, audio, vídeo y libros electrónicos que se ejecutan en tu PC. Nada que instalar, nada que subir.',
    og: 'Suelta un archivo, elige un formato. Conversiones de documentos de Office, PDF, DWG, imágenes, audio, vídeo y libros electrónicos en tu PC. Nada que instalar, nada que subir.',
    tw: 'Suelta un archivo, elige un formato. Nada que instalar, nada que subir.',
    ld: 'Convertidores de archivos sin conexión para Windows, gratuitos y de código abierto (documentos de Office, PDF, DWG, imágenes, audio, vídeo y libros electrónicos), creados sobre los motores de código abierto de ArtCraft. Los archivos se convierten en tu PC y nunca se suben.',
    alt: 'voidflare: el símbolo de la llama índigo y el texto Convertidores de archivos sin conexión para Windows',
  },
  nav: { finder: 'CONVERTIR', apps: 'APPS', offline: 'SIN CONEXIÓN', get: 'DESCARGAR', github: 'GITHUB ↗', lang: 'Idioma' },
  hero: {
    eyebrow: 'CONVERTIDORES DE ARCHIVOS SIN CONEXIÓN PARA WINDOWS',
    h1: ['Suelta un archivo.', 'Elige un formato.', 'Nunca sale de tu PC.'],
    lead: 'Conversiones de documentos de Office, PDF, DWG, imágenes, audio, vídeo y libros electrónicos que se ejecutan por completo en tu equipo. Nada que instalar, nada que subir, sin cuenta.',
    dl: 'DESCARGAR PARA WINDOWS', gh: 'VER EN GITHUB ↗', hubWord: 'HUB', zipWord: 'ZIP PORTÁTIL', convertTo: 'CONVERTIR A →',
  },
  fmt: { pdf: 'PDF', word: 'Word (DOCX)', ppt: 'PowerPoint (PPTX)', html: 'HTML', rtf: 'RTF', txt: 'Texto (TXT)', png: 'Imágenes PNG', jpg: 'Imágenes JPG', dwgSheets: 'PDF · hojas, unidas', dwgWhole: 'PDF · dibujo completo' },
  more: {
    std: 'Un archivo o una carpeta entera. El nuevo PDF aparece junto al original.',
    slides: 'Diapositiva por diapositiva. El nuevo PDF aparece junto al original.',
    pdf: 'La misma app también une, divide, extrae páginas, reduce el tamaño y lee páginas escaneadas (OCR).',
    dwg: 'Elige las hojas del dibujo o el dibujo completo en una sola página.',
  },
  finder: { h2: '¿Qué archivo tienes?', sub: 'Elígelo y mira en qué puede convertirse.', aria: 'Tipo de archivo', picked: 'HAS ELEGIDO', becomes: 'PUEDE CONVERTIRSE EN', doneBy: 'Lo hace {app}. ' },
  apps: {
    h2: 'Nueve apps, un solo hub', faint: 'CADA APP LLEVA SU PROPIO MOTOR',
    text: [
      'DOCX, DOC, RTF y ODT a PDF, un archivo o una carpeta entera.',
      'Hojas de cálculo XLSX, CSV y TSV a PDF. No hace falta Office.',
      'Presentaciones de PowerPoint a PDF, diapositiva por diapositiva.',
      'Convierte a Word, PowerPoint, HTML, RTF, texto o imágenes. Une, divide, extrae páginas, reduce, OCR.',
      'Dibujos a PDF: las hojas unidas o separadas, o el dibujo completo en una página.',
      'Convierte imágenes entre PNG, JPG, WebP, TIFF, BMP y más.',
      'Convierte audio a WAV, FLAC o AIFF (sin salida MP3).',
      'Convierte vídeo a MP4, ProRes o GIF, o quédate solo con el sonido.',
      'Libros EPUB y Kindle (AZW3, MOBI) a EPUB, texto, HTML, PDF o Word, y viceversa. Los libros con DRM se rechazan.',
    ],
    soon: 'PARA LLM', soonText: 'voidflare-cli.exe y un servidor MCP permiten que Claude y otros asistentes de IA conviertan tus archivos, solo dentro de las carpetas que autorices.',
    langs: 'Disponible en English, Italiano, Español, Français y Deutsch.',
  },
  offline: {
    h2: 'Sin conexión es el objetivo, no una función.',
    items: [
      'Los archivos se convierten en tu PC y no se envían a ninguna parte.',
      'Sin cuenta, sin inicio de sesión, sin telemetría que desactivar.',
      'No hace falta Microsoft Office ni ningún otro programa.',
      'Código abierto, con todas las licencias y créditos en la carpeta.',
    ],
  },
  how: {
    h2: '¿Quién hace la conversión?',
    p1: 'Casi cada conversión la hace un motor de línea de comandos de la suite de código abierto <a href="{artcraft}">ArtCraft</a> (WordCraft, GridCraft, DeckCraft, PdfCraft, CADCraft, PhotoCraft, SoundCraft y FilmCraft), escrito por sus autores y usado bajo sus licencias MIT OR Apache-2.0. La excepción es la lectura y escritura de libros EPUB y Kindle, que es código Rust nuestro. voidflare añade la interfaz de arrastrar y soltar. Cada app se instala con su motor y las licencias de ese motor o, si tu PC ya tiene una versión de ArtCraft que hemos probado, usa la tuya y no descarga una segunda copia.',
    p2: 'Es un proyecto independiente, sin afiliación ni respaldo del equipo de ArtCraft. <a href="{credits}">Créditos y licencias ↗</a>',
  },
  safe: {
    h2: '¿Es seguro ejecutarlo?', sub: 'Lo que puedes comprobar tú mismo y lo que aún falta.',
    items: [
      'El código fuente es público, y también las instrucciones de compilación: <a href="{build}">compílalo tú</a> en lugar de fiarte del zip.',
      'Los motores están fijados: sus versiones exactas y sus hashes SHA-256 están en <a href="{engines}">engines.json</a>, y la compilación rechaza cualquier archivo distinto.',
      'Los convertidores no usan la red. El hub pregunta a GitHub por la última versión al arrancar (puedes desactivarlo con AUTO-CHECK) y descarga una app solo cuando pulsas Instalar; las peticiones no llevan ningún archivo ni nombre de archivo.',
      'Las descargas son archivos normales en GitHub Releases, con su SHA-256 junto a los botones; el hub comprueba cada paquete con el SHA256SUMS.txt de la versión antes de instalarlo.',
    ],
    warn: 'Los programas aún no están firmados digitalmente, así que Windows SmartScreen puede avisarte la primera vez. La firma está prevista.',
  },
  get: {
    h2: 'Descarga voidflare', sub: 'Windows 10 y 11 · portátil',
    hubText: 'Descarga el hub, que es pequeño, descomprímelo en una ruta corta como <code>C:\\voidflare</code> y ejecuta <code>voidflare.exe</code>. Después pulsa <em>Instalar</em> en los convertidores que necesites: cada uno es una descarga aparte (unos pocos MB, más su motor si tu PC no tiene ya un ArtCraft probado), comprobada con el SHA-256 de la versión antes de instalarse. No se descarga nada hasta que hagas clic.',
    noNet: '¿Sin red donde trabajas? El paquete completo lo trae todo en un solo archivo ({full}).', larger: 'más grande',
    zipOnly: 'Descomprime primero la carpeta entera (no lo ejecutes desde dentro del zip), colócala en una ruta corta como <code>C:\\voidflare</code> y ejecuta <code>voidflare.exe</code>.',
    smart: 'Los programas aún no están firmados digitalmente, así que Windows SmartScreen puede avisarte la primera vez: elige <em>Más información</em> y luego <em>Ejecutar de todas formas</em>.',
    links: 'Los botones enlazan directamente con los archivos de GitHub Releases; este sitio no sirve nada.', check: ' Compruébalos tras la descarga:',
    hubSha: 'HUB · SHA-256', fullSha: 'PAQUETE COMPLETO · SHA-256',
    counter: '{n} descargas hasta ahora (hub y paquete completo, todas las versiones)',
    release: 'Página de la versión ↗', source: 'Código fuente ↗', build: 'Compílalo tú ↗', star: 'Si voidflare te resulta útil, una estrella en GitHub ayuda a que otras personas lo encuentren.',
    btnHub: 'DESCARGAR EL HUB · {mb} MB', btnFull: 'PAQUETE COMPLETO · {mb} MB', btnZip: 'DESCARGAR ZIP',
    winLabel: 'WINDOWS 10 / 11 · DISPONIBLE', linuxLabel: 'LINUX · PRÓXIMAMENTE', linuxBtn: 'DESCARGAR PARA LINUX', linuxNote: 'Previsto. Sigue el repositorio en GitHub para saber cuándo llega.',
  },
  foot: [
    'VOIDFLARE.APP · MIT OR APACHE-2.0',
    'CÓDIGO ABIERTO · <a href="{artcraft}">CREADO SOBRE LA SUITE ARTCRAFT ↗</a> · <a href="{credits}">CRÉDITOS</a>',
    'HECHO POR GABRIELE DENARO · <a href="https://github.com/Flaeren">GITHUB ↗</a> · <a href="https://gabrieledenaro.com">GABRIELEDENARO.COM ↗</a>',
    'CONTACTO · <a href="{issues}">INCIDENCIAS ↗</a> · <a href="{advisory}">NOTIFICAR UNA VULNERABILIDAD ↗</a> · <a href="{privacy}">PRIVACIDAD</a>',
  ],
  ctx: { dl: 'DESCARGAR PARA WINDOWS', source: 'CÓDIGO EN GITHUB ↗', privacy: 'PRIVACIDAD', copy: 'COPIAR DIRECCIÓN DE LA PÁGINA', copied: 'COPIADO' },
  privacy: {
    title: 'Privacidad: qué hace voidflare con tus datos',
    desc: 'Qué hacen voidflare.app y los programas de voidflare con tus datos: sin cookies, sin analítica, los archivos nunca salen de tu PC.',
    og: 'Sin cookies, sin analítica, los archivos nunca salen de tu PC.',
    back: '← VOIDFLARE', h1: 'Privacidad', updated: 'ÚLTIMA ACTUALIZACIÓN: 1 DE NOVIEMBRE DE 2026',
    intro: 'voidflare es un proyecto gratuito y de código abierto de Gabriele Denaro. En resumen: tus archivos se convierten en tu propio PC, este sitio no usa cookies ni analítica, y yo no recojo datos personales.',
    siteH: 'Este sitio web',
    site: [
      'Sin cookies, sin analítica, sin scripts de publicidad ni de seguimiento, sin formularios y sin cuentas. Si eliges un idioma, tu navegador recuerda esa elección solo en tu dispositivo (almacenamiento local); nunca se envía a nadie.',
      'Las fuentes se sirven desde este sitio. No se carga nada de Google ni de otros terceros.',
      'El sitio está alojado en Cloudflare. Como cualquier proveedor, Cloudflare trata datos técnicos como tu dirección IP para entregar y proteger las páginas. Es un tratamiento de Cloudflare, cubierto por <a href="https://www.cloudflare.com/privacypolicy/">su política de privacidad</a>; yo no lo recibo como una lista de visitantes.',
    ],
    dlH: 'Descargas',
    dl: 'Los botones de descarga enlazan directamente con los archivos de GitHub Releases. Al descargar uno, GitHub recibe los datos técnicos habituales (tu dirección IP, por ejemplo) conforme a <a href="https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement">su declaración de privacidad</a>. A este sitio no se le pide nada. El contador de descargas de la página es el número que GitHub guarda para los archivos, leído al compilar el sitio; no sabe quién descargó.',
    progH: 'Los programas',
    prog: [
      'Las conversiones se ejecutan por completo en tu PC con un motor de ArtCraft: el que viene con la app o, si tienes instalada una versión probada, el tuyo. Tus archivos y sus nombres nunca se suben.',
      'No hay cuenta, ni telemetría, ni estadísticas de uso.',
      'Dos cosas usan la red, y solo estas: al arrancar, el hub pregunta a GitHub por la última versión para avisarte si hay una más nueva; y cuando pulsas Instalar o Actualizar en una app (o UPDATE NOW para el propio hub), descarga ese archivo de la misma versión de GitHub y comprueba su SHA-256. Ninguna de las dos peticiones lleva un archivo ni un nombre de archivo, solo la petición en sí (así que GitHub ve tu dirección IP). La primera se desactiva con <code>AUTO-CHECK</code> en el hub o con la variable de entorno <code>VOIDFLARE_NO_UPDATE_CHECK</code>. No se instala nada sin tu clic.',
      'Los programas guardan algunos ajustes en tu PC (el idioma y si buscar actualizaciones), en <code>%APPDATA%\\voidflare</code>. Se quedan ahí.',
      'Las apps de conversión en sí nunca usan la red.',
    ],
    rightsH: 'Tus derechos',
    rights: 'Como no guardo ningún dato personal tuyo, no hay nada que pueda mostrar, corregir ni borrar. Si piensas lo contrario, o tienes cualquier duda, escríbeme por uno de los contactos de abajo.',
    contactH: 'Contacto',
    contact: [
      'Preguntas y errores: <a href="{issues}">incidencias en GitHub</a>.',
      'Problemas de seguridad, en privado: <a href="{advisory}">notificar una vulnerabilidad</a>.',
      'Yo: <a href="https://gabrieledenaro.com">gabrieledenaro.com</a>.',
    ],
  },
};

const fr: Strings = {
  meta: {
    title: 'voidflare : convertisseurs de fichiers hors ligne pour Windows',
    desc: 'Déposez un fichier, choisissez un format. Des conversions de documents Office, PDF, DWG, images, audio, vidéo et livres numériques qui s\'exécutent sur votre PC. Rien à installer, rien d\'envoyé en ligne.',
    og: 'Déposez un fichier, choisissez un format. Conversions de documents Office, PDF, DWG, images, audio, vidéo et livres numériques sur votre PC. Rien à installer, rien d\'envoyé en ligne.',
    tw: 'Déposez un fichier, choisissez un format. Rien à installer, rien d\'envoyé en ligne.',
    ld: 'Convertisseurs de fichiers hors ligne pour Windows, gratuits et open source (documents Office, PDF, DWG, images, audio, vidéo et livres numériques), construits sur les moteurs open source d\'ArtCraft. Les fichiers sont convertis sur votre PC et ne sont jamais envoyés.',
    alt: 'voidflare : le symbole de la flamme indigo et le texte Convertisseurs de fichiers hors ligne pour Windows',
  },
  nav: { finder: 'CONVERTIR', apps: 'APPLIS', offline: 'HORS LIGNE', get: 'TÉLÉCHARGER', github: 'GITHUB ↗', lang: 'Langue' },
  hero: {
    eyebrow: 'CONVERTISSEURS DE FICHIERS HORS LIGNE POUR WINDOWS',
    h1: ['Déposez un fichier.', 'Choisissez un format.', 'Il ne quitte jamais votre PC.'],
    lead: 'Des conversions de documents Office, PDF, DWG, images, audio, vidéo et livres numériques qui s\'exécutent entièrement sur votre machine. Rien à installer, rien d\'envoyé, aucun compte.',
    dl: 'TÉLÉCHARGER POUR WINDOWS', gh: 'VOIR SUR GITHUB ↗', hubWord: 'HUB', zipWord: 'ZIP PORTABLE', convertTo: 'CONVERTIR EN →',
  },
  fmt: { pdf: 'PDF', word: 'Word (DOCX)', ppt: 'PowerPoint (PPTX)', html: 'HTML', rtf: 'RTF', txt: 'Texte (TXT)', png: 'Images PNG', jpg: 'Images JPG', dwgSheets: 'PDF · feuilles, fusionnées', dwgWhole: 'PDF · dessin entier' },
  more: {
    std: 'Un fichier ou un dossier entier. Le nouveau PDF apparaît à côté de l\'original.',
    slides: 'Diapositive par diapositive. Le nouveau PDF apparaît à côté de l\'original.',
    pdf: 'La même appli fusionne aussi, divise, extrait des pages, réduit la taille et lit les pages numérisées (OCR).',
    dwg: 'Choisissez les feuilles du dessin ou le dessin entier sur une seule page.',
  },
  finder: { h2: 'Quel fichier avez-vous ?', sub: 'Choisissez-le et voyez ce qu\'il peut devenir.', aria: 'Type de fichier', picked: 'VOUS AVEZ CHOISI', becomes: 'IL PEUT DEVENIR', doneBy: 'C\'est {app} qui s\'en charge. ' },
  apps: {
    h2: 'Neuf applis, un seul hub', faint: 'CHAQUE APPLI EMBARQUE SON MOTEUR',
    text: [
      'DOCX, DOC, RTF et ODT en PDF, un fichier ou un dossier entier.',
      'Tableurs XLSX, CSV et TSV en PDF. Pas besoin d\'Office.',
      'Présentations PowerPoint en PDF, diapositive par diapositive.',
      'Convertissez en Word, PowerPoint, HTML, RTF, texte ou images. Fusion, division, extraction de pages, réduction, OCR.',
      'Dessins en PDF : les feuilles fusionnées ou séparées, ou le dessin entier sur une page.',
      'Convertissez des images entre PNG, JPG, WebP, TIFF, BMP et plus.',
      'Convertissez l\'audio en WAV, FLAC ou AIFF (pas de sortie MP3).',
      'Convertissez la vidéo en MP4, ProRes ou GIF, ou gardez seulement le son.',
      'Livres EPUB et Kindle (AZW3, MOBI) en EPUB, texte, HTML, PDF ou Word, et inversement. Les livres protégés par DRM sont refusés.',
    ],
    soon: 'POUR LES LLM', soonText: 'voidflare-cli.exe et un serveur MCP permettent à Claude et à d\'autres assistants IA de convertir vos fichiers, uniquement dans les dossiers que vous autorisez.',
    langs: 'Disponible en English, Italiano, Español, Français et Deutsch.',
  },
  offline: {
    h2: 'Le hors ligne est le but, pas une option.',
    items: [
      'Les fichiers sont convertis sur votre PC et ne sont envoyés nulle part.',
      'Pas de compte, pas de connexion, pas de télémétrie à désactiver.',
      'Ni Microsoft Office ni aucun autre programme n\'est nécessaire.',
      'Open source, avec toutes les licences et tous les crédits dans le dossier.',
    ],
  },
  how: {
    h2: 'Qui fait la conversion ?',
    p1: 'Presque chaque conversion est faite par un moteur en ligne de commande de la suite open source <a href="{artcraft}">ArtCraft</a> (WordCraft, GridCraft, DeckCraft, PdfCraft, CADCraft, PhotoCraft, SoundCraft et FilmCraft), écrit par ses auteurs et utilisé sous leurs licences MIT OR Apache-2.0. L\'exception est la lecture et l\'écriture des livres EPUB et Kindle, qui est notre propre code Rust. voidflare ajoute l\'interface par glisser-déposer. Chaque appli est installée avec son moteur et les licences de ce moteur ou, si votre PC a déjà une version d\'ArtCraft que nous avons testée, utilise la vôtre et ne télécharge pas une seconde copie.',
    p2: 'C\'est un projet indépendant, non affilié à l\'équipe d\'ArtCraft et non approuvé par elle. <a href="{credits}">Crédits et licences ↗</a>',
  },
  safe: {
    h2: 'Est-ce sûr à exécuter ?', sub: 'Ce que vous pouvez vérifier vous-même, et ce qui manque encore.',
    items: [
      'Le code source est public, tout comme les instructions de compilation : <a href="{build}">compilez-le vous-même</a> plutôt que de vous fier au zip.',
      'Les moteurs sont figés : leurs versions exactes et leurs empreintes SHA-256 sont dans <a href="{engines}">engines.json</a>, et la compilation refuse tout fichier différent.',
      'Les convertisseurs n\'utilisent pas le réseau. Le hub demande à GitHub la dernière version au démarrage (vous pouvez le désactiver avec AUTO-CHECK) et ne télécharge une appli que lorsque vous appuyez sur Installer ; les requêtes ne contiennent ni fichier ni nom de fichier.',
      'Les téléchargements sont de simples fichiers sur GitHub Releases, avec leur SHA-256 affiché à côté des boutons ; le hub vérifie chaque paquet avec le SHA256SUMS.txt de la version avant de l\'installer.',
    ],
    warn: 'Les programmes ne sont pas encore signés numériquement ; Windows SmartScreen peut donc afficher un avertissement au premier lancement. La signature est prévue.',
  },
  get: {
    h2: 'Obtenir voidflare', sub: 'Windows 10 et 11 · portable',
    hubText: 'Téléchargez le petit hub, décompressez-le dans un chemin court comme <code>C:\\voidflare</code> et lancez <code>voidflare.exe</code>. Appuyez ensuite sur <em>Installer</em> pour les convertisseurs dont vous avez besoin : chacun est un téléchargement séparé (quelques Mo, plus son moteur si votre PC n\'a pas déjà un ArtCraft testé), vérifié avec le SHA-256 de la version avant d\'être installé. Rien n\'est téléchargé avant votre clic.',
    noNet: 'Pas de réseau là où vous travaillez ? Le paquet complet contient tout dans un seul fichier ({full}).', larger: 'plus volumineux',
    zipOnly: 'Décompressez d\'abord le dossier entier (ne lancez pas le programme depuis l\'intérieur du zip), placez-le dans un chemin court comme <code>C:\\voidflare</code> et lancez <code>voidflare.exe</code>.',
    smart: 'Les programmes ne sont pas encore signés numériquement ; Windows SmartScreen peut donc afficher un avertissement au premier lancement : choisissez <em>Informations complémentaires</em>, puis <em>Exécuter quand même</em>.',
    links: 'Les boutons pointent directement vers les fichiers de GitHub Releases ; ce site ne sert rien.', check: ' Vérifiez-les après le téléchargement :',
    hubSha: 'HUB · SHA-256', fullSha: 'PAQUET COMPLET · SHA-256',
    counter: '{n} téléchargements à ce jour (hub et paquet complet, toutes versions)',
    release: 'Page de la version ↗', source: 'Code source ↗', build: 'Compilez-le vous-même ↗', star: 'Si voidflare vous est utile, une étoile sur GitHub aide d\'autres personnes à le trouver.',
    btnHub: 'TÉLÉCHARGER LE HUB · {mb} Mo', btnFull: 'PAQUET COMPLET · {mb} Mo', btnZip: 'TÉLÉCHARGER LE ZIP',
    winLabel: 'WINDOWS 10 / 11 · DISPONIBLE', linuxLabel: 'LINUX · BIENTÔT', linuxBtn: 'TÉLÉCHARGER POUR LINUX', linuxNote: 'Prévu. Suivez le dépôt GitHub pour savoir quand il arrive.',
  },
  foot: [
    'VOIDFLARE.APP · MIT OR APACHE-2.0',
    'OPEN SOURCE · <a href="{artcraft}">CONSTRUIT SUR LA SUITE ARTCRAFT ↗</a> · <a href="{credits}">CRÉDITS</a>',
    'RÉALISÉ PAR GABRIELE DENARO · <a href="https://github.com/Flaeren">GITHUB ↗</a> · <a href="https://gabrieledenaro.com">GABRIELEDENARO.COM ↗</a>',
    'CONTACT · <a href="{issues}">SIGNALEMENTS ↗</a> · <a href="{advisory}">SIGNALER UNE VULNÉRABILITÉ ↗</a> · <a href="{privacy}">CONFIDENTIALITÉ</a>',
  ],
  ctx: { dl: 'TÉLÉCHARGER POUR WINDOWS', source: 'CODE SUR GITHUB ↗', privacy: 'CONFIDENTIALITÉ', copy: 'COPIER L\'ADRESSE DE LA PAGE', copied: 'COPIÉ' },
  privacy: {
    title: 'Confidentialité : ce que voidflare fait de vos données',
    desc: 'Ce que voidflare.app et les programmes voidflare font de vos données : pas de cookies, pas d\'analytique, vos fichiers ne quittent jamais votre PC.',
    og: 'Pas de cookies, pas d\'analytique, vos fichiers ne quittent jamais votre PC.',
    back: '← VOIDFLARE', h1: 'Confidentialité', updated: 'DERNIÈRE MISE À JOUR : 1ER NOVEMBRE 2026',
    intro: 'voidflare est un projet gratuit et open source de Gabriele Denaro. En bref : vos fichiers sont convertis sur votre propre PC, ce site ne dépose aucun cookie et n\'utilise aucun outil d\'analytique, et je ne collecte moi-même aucune donnée personnelle.',
    siteH: 'Ce site web',
    site: [
      'Pas de cookies, pas d\'analytique, pas de scripts publicitaires ni de suivi, pas de formulaires et pas de comptes. Si vous choisissez une langue, votre navigateur mémorise ce choix uniquement sur votre appareil (stockage local) ; il n\'est jamais envoyé à personne.',
      'Les polices sont servies depuis ce site. Rien n\'est chargé depuis Google ou un autre tiers.',
      'Le site est hébergé chez Cloudflare. Comme tout hébergeur, Cloudflare traite des données techniques telles que votre adresse IP pour délivrer et protéger les pages. C\'est un traitement de Cloudflare, couvert par <a href="https://www.cloudflare.com/privacypolicy/">sa politique de confidentialité</a> ; je ne le reçois pas sous forme de liste de visiteurs.',
    ],
    dlH: 'Téléchargement',
    dl: 'Les boutons de téléchargement pointent directement vers les fichiers de GitHub Releases. Quand vous en téléchargez un, GitHub reçoit les données techniques habituelles (votre adresse IP, par exemple) selon <a href="https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement">sa déclaration de confidentialité</a>. Rien n\'est demandé à ce site. Le compteur de téléchargements de la page est le nombre que GitHub conserve pour les fichiers, lu lors de la compilation du site ; il ne sait pas qui a téléchargé.',
    progH: 'Les programmes',
    prog: [
      'Les conversions s\'exécutent entièrement sur votre PC avec un moteur ArtCraft : celui qui est fourni avec l\'appli ou, si vous en avez installé une version testée, le vôtre. Vos fichiers et leurs noms ne sont jamais envoyés.',
      'Il n\'y a ni compte, ni télémétrie, ni statistiques d\'utilisation.',
      'Deux choses utilisent le réseau, et seulement celles-ci : au démarrage, le hub demande à GitHub la dernière version, pour vous dire qu\'une plus récente existe ; et quand vous appuyez sur Installer ou Mettre à jour pour une appli (ou sur UPDATE NOW pour le hub lui-même), il télécharge ce fichier depuis la même version GitHub et vérifie son SHA-256. Aucune de ces requêtes ne contient de fichier ni de nom de fichier, seulement la requête elle-même (GitHub voit donc votre adresse IP). La première se désactive avec <code>AUTO-CHECK</code> dans le hub, ou en définissant la variable d\'environnement <code>VOIDFLARE_NO_UPDATE_CHECK</code>. Rien n\'est installé sans votre clic.',
      'Les programmes conservent quelques réglages sur votre PC (la langue et la vérification des mises à jour), dans <code>%APPDATA%\\voidflare</code>. Ils y restent.',
      'Les applis de conversion elles-mêmes n\'utilisent jamais le réseau.',
    ],
    rightsH: 'Vos droits',
    rights: 'Comme je ne détiens aucune donnée personnelle vous concernant, je n\'ai rien à montrer, corriger ou supprimer. Si vous pensez le contraire, ou si vous avez une question, écrivez-moi via l\'un des contacts ci-dessous.',
    contactH: 'Contact',
    contact: [
      'Questions et bugs : <a href="{issues}">signalements sur GitHub</a>.',
      'Problèmes de sécurité, en privé : <a href="{advisory}">signaler une vulnérabilité</a>.',
      'Moi : <a href="https://gabrieledenaro.com">gabrieledenaro.com</a>.',
    ],
  },
};

const de: Strings = {
  meta: {
    title: 'voidflare: Offline-Dateikonverter für Windows',
    desc: 'Datei ablegen, Format wählen. Office-, PDF-, DWG-, Bild-, Audio-, Video- und E-Book-Konvertierungen, die auf deinem PC laufen. Nichts zu installieren, nichts wird hochgeladen.',
    og: 'Datei ablegen, Format wählen. Office-, PDF-, DWG-, Bild-, Audio-, Video- und E-Book-Konvertierungen auf deinem PC. Nichts zu installieren, nichts wird hochgeladen.',
    tw: 'Datei ablegen, Format wählen. Nichts zu installieren, nichts wird hochgeladen.',
    ld: 'Kostenlose Open-Source-Offline-Dateikonverter für Windows (Office-Dokumente, PDF, DWG, Bilder, Audio, Video und E-Books), gebaut auf den Open-Source-Engines von ArtCraft. Dateien werden auf deinem PC konvertiert und nie hochgeladen.',
    alt: 'voidflare: das indigofarbene Flammensymbol und der Text Offline-Dateikonverter für Windows',
  },
  nav: { finder: 'KONVERTIEREN', apps: 'APPS', offline: 'OFFLINE', get: 'DOWNLOAD', github: 'GITHUB ↗', lang: 'Sprache' },
  hero: {
    eyebrow: 'OFFLINE-DATEIKONVERTER FÜR WINDOWS',
    h1: ['Datei ablegen.', 'Format wählen.', 'Sie verlässt nie deinen PC.'],
    lead: 'Office-, PDF-, DWG-, Bild-, Audio-, Video- und E-Book-Konvertierungen, die komplett auf deinem Rechner laufen. Nichts zu installieren, nichts hochgeladen, kein Konto.',
    dl: 'FÜR WINDOWS HERUNTERLADEN', gh: 'AUF GITHUB ANSEHEN ↗', hubWord: 'HUB', zipWord: 'PORTABLES ZIP', convertTo: 'KONVERTIEREN IN →',
  },
  fmt: { pdf: 'PDF', word: 'Word (DOCX)', ppt: 'PowerPoint (PPTX)', html: 'HTML', rtf: 'RTF', txt: 'Text (TXT)', png: 'PNG-Bilder', jpg: 'JPG-Bilder', dwgSheets: 'PDF · Blätter, zusammengeführt', dwgWhole: 'PDF · ganze Zeichnung' },
  more: {
    std: 'Eine Datei oder ein ganzer Ordner. Das neue PDF landet neben dem Original.',
    slides: 'Folie für Folie. Das neue PDF landet neben dem Original.',
    pdf: 'Dieselbe App führt auch zusammen, teilt, extrahiert Seiten, verkleinert und liest gescannte Seiten (OCR).',
    dwg: 'Wähle die Blätter der Zeichnung oder die ganze Zeichnung auf einer Seite.',
  },
  finder: { h2: 'Welche Datei hast du?', sub: 'Wähle sie aus und sieh, was daraus werden kann.', aria: 'Dateityp', picked: 'DU HAST GEWÄHLT', becomes: 'KANN WERDEN', doneBy: 'Das erledigt {app}. ' },
  apps: {
    h2: 'Neun Apps, ein Hub', faint: 'JEDE APP BRINGT IHRE EIGENE ENGINE MIT',
    text: [
      'DOCX, DOC, RTF und ODT in PDF, eine Datei oder ein ganzer Ordner.',
      'Tabellen in XLSX, CSV und TSV zu PDF. Kein Office nötig.',
      'PowerPoint-Präsentationen zu PDF, Folie für Folie.',
      'Konvertiere in Word, PowerPoint, HTML, RTF, Text oder Bilder. Zusammenführen, teilen, Seiten extrahieren, verkleinern, OCR.',
      'Zeichnungen zu PDF: die Blätter zusammengeführt oder einzeln, oder die ganze Zeichnung auf einer Seite.',
      'Konvertiere Bilder zwischen PNG, JPG, WebP, TIFF, BMP und mehr.',
      'Konvertiere Audio in WAV, FLAC oder AIFF (keine MP3-Ausgabe).',
      'Konvertiere Video in MP4, ProRes oder GIF, oder behalte nur den Ton.',
      'EPUB- und Kindle-Bücher (AZW3, MOBI) in EPUB, Text, HTML, PDF oder Word, und zurück. Bücher mit DRM werden abgelehnt.',
    ],
    soon: 'FÜR LLMS', soonText: 'voidflare-cli.exe und ein MCP-Server lassen Claude und andere KI-Assistenten deine Dateien konvertieren, nur in Ordnern, die du freigibst.',
    langs: 'Verfügbar in English, Italiano, Español, Français und Deutsch.',
  },
  offline: {
    h2: 'Offline ist der Kern, kein Extra.',
    items: [
      'Dateien werden auf deinem PC konvertiert und nirgendwohin gesendet.',
      'Kein Konto, keine Anmeldung, keine Telemetrie zum Abschalten.',
      'Weder Microsoft Office noch ein anderes Programm nötig.',
      'Open Source, mit allen Lizenzen und Danksagungen im Ordner.',
    ],
  },
  how: {
    h2: 'Wer konvertiert eigentlich?',
    p1: 'Fast jede Konvertierung übernimmt eine Kommandozeilen-Engine der Open-Source-Suite <a href="{artcraft}">ArtCraft</a> (WordCraft, GridCraft, DeckCraft, PdfCraft, CADCraft, PhotoCraft, SoundCraft und FilmCraft), geschrieben von deren Autoren und genutzt unter deren Lizenzen MIT OR Apache-2.0. Die Ausnahme ist das Lesen und Schreiben von EPUB- und Kindle-Büchern, das ist unser eigener Rust-Code. voidflare liefert die Drag-and-drop-Oberfläche dazu. Jede App wird mit ihrer Engine und deren Lizenzen installiert oder, wenn dein PC bereits eine von uns getestete ArtCraft-Version hat, nutzt sie deine und lädt keine zweite Kopie herunter.',
    p2: 'Es ist ein unabhängiges Projekt, weder mit dem ArtCraft-Team verbunden noch von ihm unterstützt. <a href="{credits}">Danksagungen und Lizenzen ↗</a>',
  },
  safe: {
    h2: 'Ist das sicher?', sub: 'Was du selbst prüfen kannst und was noch fehlt.',
    items: [
      'Der Quellcode ist öffentlich, ebenso die Bauanleitung: <a href="{build}">Baue es selbst</a>, statt dem Zip zu vertrauen.',
      'Die Engines sind festgelegt: Ihre genauen Versionen und SHA-256-Hashes stehen in <a href="{engines}">engines.json</a>, und der Build lehnt jede abweichende Datei ab.',
      'Die Konverter nutzen das Netzwerk nicht. Der Hub fragt beim Start GitHub nach der neuesten Version (mit AUTO-CHECK abschaltbar) und lädt eine App nur herunter, wenn du auf Installieren drückst; die Anfragen enthalten weder Dateien noch Dateinamen.',
      'Die Downloads sind einfache Dateien auf GitHub Releases, mit ihrem SHA-256 neben den Schaltflächen; der Hub prüft jedes Paket anhand der SHA256SUMS.txt der Version, bevor er es installiert.',
    ],
    warn: 'Die Programme sind noch nicht code-signiert, daher kann Windows SmartScreen beim ersten Start warnen. Eine Signatur ist geplant.',
  },
  get: {
    h2: 'voidflare herunterladen', sub: 'Windows 10 und 11 · portabel',
    hubText: 'Lade den kleinen Hub herunter, entpacke ihn in einen kurzen Pfad wie <code>C:\\voidflare</code> und starte <code>voidflare.exe</code>. Drücke dann bei den benötigten Konvertern auf <em>Installieren</em>: Jeder ist ein eigener Download (ein paar MB, plus seine Engine, falls dein PC nicht schon ein getestetes ArtCraft hat), vor der Installation mit dem SHA-256 der Version geprüft. Nichts wird heruntergeladen, bevor du klickst.',
    noNet: 'Kein Netzwerk an deinem Arbeitsplatz? Das Komplettpaket enthält alles in einer Datei ({full}).', larger: 'größer',
    zipOnly: 'Entpacke zuerst den ganzen Ordner (nicht aus dem Zip heraus starten), lege ihn in einen kurzen Pfad wie <code>C:\\voidflare</code> und starte <code>voidflare.exe</code>.',
    smart: 'Die Programme sind noch nicht code-signiert, daher kann Windows SmartScreen beim ersten Start warnen: Wähle <em>Weitere Informationen</em>, dann <em>Trotzdem ausführen</em>.',
    links: 'Die Schaltflächen verweisen direkt auf die Dateien bei GitHub Releases, von dieser Seite wird nichts ausgeliefert.', check: ' Prüfe sie nach dem Download:',
    hubSha: 'HUB · SHA-256', fullSha: 'KOMPLETTPAKET · SHA-256',
    counter: 'Bisher {n} Downloads (Hub und Komplettpaket, alle Versionen)',
    release: 'Release-Seite ↗', source: 'Quellcode ↗', build: 'Selbst bauen ↗', star: 'Wenn dir voidflare nützt, hilft ein Stern auf GitHub anderen, es zu finden.',
    btnHub: 'HUB HERUNTERLADEN · {mb} MB', btnFull: 'KOMPLETTPAKET · {mb} MB', btnZip: 'ZIP HERUNTERLADEN',
    winLabel: 'WINDOWS 10 / 11 · VERFÜGBAR', linuxLabel: 'LINUX · BALD', linuxBtn: 'FÜR LINUX HERUNTERLADEN', linuxNote: 'Geplant. Verfolge das Repository auf GitHub, um zu erfahren, wann es erscheint.',
  },
  foot: [
    'VOIDFLARE.APP · MIT OR APACHE-2.0',
    'OPEN SOURCE · <a href="{artcraft}">GEBAUT AUF DER ARTCRAFT-SUITE ↗</a> · <a href="{credits}">DANKSAGUNGEN</a>',
    'ENTWICKELT VON GABRIELE DENARO · <a href="https://github.com/Flaeren">GITHUB ↗</a> · <a href="https://gabrieledenaro.com">GABRIELEDENARO.COM ↗</a>',
    'KONTAKT · <a href="{issues}">ISSUES ↗</a> · <a href="{advisory}">SCHWACHSTELLE MELDEN ↗</a> · <a href="{privacy}">DATENSCHUTZ</a>',
  ],
  ctx: { dl: 'FÜR WINDOWS HERUNTERLADEN', source: 'QUELLCODE AUF GITHUB ↗', privacy: 'DATENSCHUTZ', copy: 'SEITENADRESSE KOPIEREN', copied: 'KOPIERT' },
  privacy: {
    title: 'Datenschutz: was voidflare mit deinen Daten macht',
    desc: 'Was voidflare.app und die voidflare-Programme mit deinen Daten machen: keine Cookies, keine Analyse, Dateien verlassen nie deinen PC.',
    og: 'Keine Cookies, keine Analyse, Dateien verlassen nie deinen PC.',
    back: '← VOIDFLARE', h1: 'Datenschutz', updated: 'ZULETZT AKTUALISIERT: 1. NOVEMBER 2026',
    intro: 'voidflare ist ein kostenloses Open-Source-Projekt von Gabriele Denaro. Kurz gesagt: Deine Dateien werden auf deinem eigenen PC konvertiert, diese Website setzt keine Cookies und nutzt keine Analyse, und ich selbst erhebe keine personenbezogenen Daten.',
    siteH: 'Diese Website',
    site: [
      'Keine Cookies, keine Analyse, keine Werbe- oder Tracking-Skripte, keine Formulare und keine Konten. Wenn du eine Sprache wählst, merkt sich dein Browser diese Wahl nur auf deinem Gerät (lokaler Speicher); sie wird nie an jemanden gesendet.',
      'Die Schriften werden von dieser Seite ausgeliefert. Es wird nichts von Google oder anderen Dritten geladen.',
      'Die Seite wird bei Cloudflare gehostet. Wie jeder Host verarbeitet Cloudflare technische Daten wie deine IP-Adresse, um die Seiten auszuliefern und zu schützen. Das ist eine Verarbeitung durch Cloudflare, geregelt in <a href="https://www.cloudflare.com/privacypolicy/">dessen Datenschutzerklärung</a>; ich erhalte sie nicht als Besucherliste.',
    ],
    dlH: 'Download',
    dl: 'Die Download-Schaltflächen verweisen direkt auf die Dateien bei GitHub Releases. Wenn du eine herunterlädst, erhält GitHub die üblichen technischen Daten (zum Beispiel deine IP-Adresse) gemäß <a href="https://docs.github.com/site-policy/privacy-policies/github-general-privacy-statement">dessen Datenschutzerklärung</a>. Von dieser Seite wird nichts angefordert. Der Download-Zähler auf der Seite ist die Zahl, die GitHub für die Dateien führt, gelesen beim Erstellen der Seite; er weiß nicht, wer heruntergeladen hat.',
    progH: 'Die Programme',
    prog: [
      'Konvertierungen laufen komplett auf deinem PC mit einer ArtCraft-Engine: der, die mit der App kommt, oder, wenn du eine getestete Version installiert hast, deiner. Deine Dateien und deren Namen werden nie hochgeladen.',
      'Es gibt kein Konto, keine Telemetrie und keine Nutzungsstatistik.',
      'Zwei Dinge nutzen das Netzwerk, und nur diese: Beim Start fragt der Hub GitHub nach der neuesten Version, um dir zu sagen, dass es eine neuere gibt; und wenn du bei einer App auf Installieren oder Aktualisieren drückst (oder beim Hub selbst auf UPDATE NOW), lädt er diese Datei aus demselben GitHub-Release herunter und prüft ihren SHA-256. Keine der Anfragen enthält eine Datei oder einen Dateinamen, nur die Anfrage selbst (GitHub sieht also deine IP-Adresse). Die erste lässt sich mit <code>AUTO-CHECK</code> im Hub oder über die Umgebungsvariable <code>VOIDFLARE_NO_UPDATE_CHECK</code> abschalten. Ohne deinen Klick wird nichts installiert.',
      'Die Programme speichern einige Einstellungen auf deinem PC (die Sprache und ob nach Updates gesucht wird), in <code>%APPDATA%\\voidflare</code>. Sie bleiben dort.',
      'Die Konverter-Apps selbst nutzen nie das Netzwerk.',
    ],
    rightsH: 'Deine Rechte',
    rights: 'Da ich keine personenbezogenen Daten über dich speichere, gibt es nichts, was ich dir zeigen, berichtigen oder löschen könnte. Wenn du das anders siehst oder eine Frage hast, schreib mir über einen der Kontakte unten.',
    contactH: 'Kontakt',
    contact: [
      'Fragen und Fehler: <a href="{issues}">GitHub-Issues</a>.',
      'Sicherheitsprobleme, vertraulich: <a href="{advisory}">Schwachstelle melden</a>.',
      'Ich: <a href="https://gabrieledenaro.com">gabrieledenaro.com</a>.',
    ],
  },
};

export const STRINGS: Record<Lang, Strings> = { en, it, es, fr, de };

/** Path of a page in a language: English lives at the root, the others under /xx (no trailing slash, like /privacy). */
export function pagePath(lang: Lang, page: '' | 'privacy'): string {
  if (lang === 'en') return page ? '/privacy' : '/';
  return page ? `/${lang}/privacy` : `/${lang}`;
}
