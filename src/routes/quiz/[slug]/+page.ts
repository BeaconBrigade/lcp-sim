import { error } from '@sveltejs/kit';

export const load: ({ params }: { params: { slug: number } }) => { id: number } = ({ params }) => {
	if (params.slug > 0 && params.slug <= 9) {
		return {
			id: params.slug
		};
	}
	throw error(404, 'page not found');
};
