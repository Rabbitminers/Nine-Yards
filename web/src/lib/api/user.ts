import type { User } from '../../types/user';
import type { Optional } from '../../types/utility';

import { authHeader } from './helpers/header';
import { API_URL } from '$env/static/private';

/*
export const getUser = (): Promise<Optional<User>> => {
	return await fetch(API_URL + 'users/')
		.then((response) => {
			if (!response.ok) {
				throw Error('User not found');
			}
			return response.body;
		})
		.then((body) => {
			body as user;
		})
		.catch((error) => {
			console.error(error);
			return null;
		});
};
*/
