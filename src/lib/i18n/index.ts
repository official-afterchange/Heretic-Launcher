// Charge automatiquement tous les .json du dossier locales en récupérant directement leur contenu
const localeFiles = import.meta.glob<Record<string, any>>('../locales/*.json', {
  eager: true,
  import: 'default',
});

const dictionaries: Record<string, Record<string, any>> = {};

for (const path in localeFiles) {
  const lang = path.split('/').pop()?.replace('.json', '');
  if (lang) {
    dictionaries[lang] = localeFiles[path];
  }
}

// Récupère la langue sauvegardée ou 'fr' par défaut
function getInitialLang(): string {
  if (typeof window !== 'undefined') {
    const saved = localStorage.getItem('app_lang');
    if (saved && dictionaries[saved]) return saved;
  }
  return 'fr';
}

let currentLang: string = getInitialLang();

// Applique les attributs sur le DOM au démarrage
if (typeof document !== 'undefined') {
  const dict = dictionaries[currentLang] || dictionaries['fr'] || {};
  document.documentElement.lang = currentLang;
  const direction = dict.app?.dir || (currentLang === 'ar' ? 'rtl' : 'ltr');
  document.documentElement.dir = direction;
}

export function setLanguage(lang: string) {
  if (dictionaries[lang]) {
    currentLang = lang;
    if (typeof document !== 'undefined') {
      const dict = dictionaries[lang];
      document.documentElement.lang = lang;

      const direction = dict.app?.dir || (lang === 'ar' ? 'rtl' : 'ltr');
      document.documentElement.dir = direction;
    }
    localStorage.setItem('app_lang', lang);

    // Déclenche un événement personnalisé pour notifier les composants React de re-render
    window.dispatchEvent(new Event('languagechange'));
  }
}

export function getLanguage(): string {
  return currentLang;
}

function getNestedValue(obj: any, path: string): string | null {
  return path.split('.').reduce((prev, curr) => (prev && prev[curr] !== undefined ? prev[curr] : null), obj);
}

export function t(key: string, vars: Record<string, string | number> = {}): string {
  const dict = dictionaries[currentLang] || dictionaries['fr'] || {};
  let text = getNestedValue(dict, key);

  if (!text) {
    console.warn(`[i18n] Clé manquante : "${key}" pour la langue "${currentLang}"`);
    return key;
  }

  Object.entries(vars).forEach(([varKey, value]) => {
    text = text!.replace(new RegExp(`{{\\s*${varKey}\\s*}}`, 'g'), String(value));
  });

  const appRegex = /{{\s*app\.([a-zA-Z0-9_]+)\s*}}/;
  let match = appRegex.exec(text);

  while (match) {
    const appKey = match[1];
    const replacement = dict.app?.[appKey] ?? `{{app.${appKey}}}`;
    text = text.replace(match[0], replacement);
    match = appRegex.exec(text);
  }

  return text;
}