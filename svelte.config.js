import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/kit/vite';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		adapter: adapter(),
		prerender: {
			entries: [
				'/',
				'/learn-more',
				'/sandbox',
				'/quiz/1',
				'/quiz/2',
				'/quiz/3',
				'/quiz/4',
				'/quiz/5'
			]
		}
	}
};

export default config;
