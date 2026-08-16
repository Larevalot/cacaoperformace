import { en } from './en';
import { es } from './es';
import { it } from './it';
import { ja } from './ja';
import { zh } from './zh';
import type { Language } from '../types';

export const dictionaries = { en, es, it, ja, zh };

export function translate(key: string, lang: Language = 'en', params?: Record<string, string | number>): string {
  const dict = dictionaries[lang] || dictionaries.en;
  let text = (dict as Record<string, string>)[key] || (dictionaries.en as Record<string, string>)[key] || key;

  if (params) {
    Object.entries(params).forEach(([paramKey, val]) => {
      text = text.replace(new RegExp(`\\{${paramKey}\\}`, 'g'), String(val));
    });
  }

  return text;
}
