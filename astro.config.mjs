import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightBlogPlugin from 'starlight-blog';
import { ion } from "starlight-ion-theme";
import sitemap from '@astrojs/sitemap';

export default defineConfig({
  site:'https://official-afterchange.github.io',
  base: '/Heretic-Launcher/',
  srcDir: './docs',
  redirects: {
    '/': '/Heretic-Launcher/en/'
  },
  integrations: [
    sitemap(),
    starlight({
      title: 'Heretic Launcher',
      social: [
        { icon: 'github', label: 'Github', href: 'https://github.com/official-afterchange/heretic-launcher' }
      ],
      customCss: ['./docs/styles/custom.css'],
      defaultLocale: 'en',
      locales: {
        fr: {
          label: 'Français',
          lang: 'fr',
        },
        en: {
          label: 'English',
          lang: 'en',
        },
      },
      sidebar: [
        {
          label: 'Technical docs',
          translations: { fr: 'Documentation technique' },
          items: [
            'technical/installation',
            'technical/architecture',
            'technical/frontend',
            'technical/build',
            'technical/contributing'
          ]
        },
        {
          label: 'API',
          translations: { fr: 'API' },
          items: [
            'api/mojang',
            'api/fabric',
            'api/adoptium'
          ]
        },
        {
          label: 'Dev Log',
          translations: { fr: 'Journal de bord' },
          items: [
            { autogenerate: { directory: 'blog' } }
          ],
        },
        {
          label: 'Roadmap',
          translations: { fr: 'Feuille de route' },
          items: [
            'roadmap/roadmap',
          ]
        },
      ],
      plugins: [ion(), starlightBlogPlugin({
        title: {
          fr: 'Journal de bord',
          en: 'Dev log'
        }
      })],
    }),
  ],
  vite: {
    build: {
      cssMinify: 'esbuild'
    }
  }
});