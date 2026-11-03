type TranslationValue = string | number | boolean | null | undefined;
type TranslationObject = {
  [key: string]: TranslationValue | TranslationObject;
};

const localeFiles = import.meta.glob<TranslationObject>('../locales/*.json', {
  eager: true,
  import: 'default',
});

const dictionaries: Record<string, TranslationObject> = {};

for (const path in localeFiles) {
  const lang = path.split('/').pop()?.replace('.json', '');
  if (lang) {
    dictionaries[lang] = localeFiles[path];
  }
}

function getInitialLang(): string {
  if (typeof window !== 'undefined') {
    const saved = localStorage.getItem('app_lang');
    if (saved && dictionaries[saved]) return saved;
  }
  return 'fr';
}

let currentLang: string = getInitialLang();

if (typeof document !== 'undefined') {
  const dict = dictionaries[currentLang] || dictionaries['fr'] || {};
  document.documentElement.lang = currentLang;
  
  const appObj = dict.app;
  const appDir = appObj && typeof appObj === 'object' && 'dir' in appObj ? (appObj.dir as string) : undefined;
  
  const direction = appDir || (currentLang === 'ar' ? 'rtl' : 'ltr');
  document.documentElement.dir = direction;
}

export function setLanguage(lang: string) {
  if (dictionaries[lang]) {
    currentLang = lang;
    if (typeof document !== 'undefined') {
      const dict = dictionaries[lang];
      document.documentElement.lang = lang;

      const appObj = dict.app;
      const appDir = appObj && typeof appObj === 'object' && 'dir' in appObj ? (appObj.dir as string) : undefined;
      const direction = appDir || (lang === 'ar' ? 'rtl' : 'ltr');
      
      document.documentElement.dir = direction;
    }
    localStorage.setItem('app_lang', lang);

    window.dispatchEvent(new Event('languagechange'));
  }
}

export function getLanguage(): string {
  return currentLang;
}

// Remplacement de 'any' par 'unknown' avec vérification de type sécurisée
function getNestedValue(obj: unknown, path: string): string | null {
  const result = path.split('.').reduce((prev, curr) => {
    if (prev && typeof prev === 'object' && curr in prev) {
      return (prev as Record<string, unknown>)[curr];
    }
    return null;
  }, obj);

  return typeof result === 'string' ? result : null;
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
    const appObj = dict.app;
    const appValue = appObj && typeof appObj === 'object' && appKey in appObj ? appObj[appKey] : undefined;
    
    const replacement = typeof appValue === 'string' ? appValue : `{{app.${appKey}}}`;
    text = text.replace(match[0], replacement);
    match = appRegex.exec(text);
  }

  return text;
}