import { join } from 'path';
import type { Config } from 'tailwindcss';

import forms from '@tailwindcss/forms';

import { skeleton } from '@skeletonlabs/tw-plugin';

const config = {
  darkMode: 'selector',
  content: [
    './src/**/*.{html,js,svelte,ts}',
    join(require.resolve(
      '@skeletonlabs/skeleton'),
      '../**/*.{html,js,svelte,ts}'
    )
  ],
  plugins: [
    forms,
    skeleton({
      themes: { preset: [
        {
          name: 'skeleton',
          enhancements: true,
        },
      ] }
    })
  ]
} satisfies Config;

export default config;
