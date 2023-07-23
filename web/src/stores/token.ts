import { writable, get } from 'svelte/store';
import type { Token } from '../types/user';

export const TOKEN_KEY: string = 'token';

/**
 * Read the last used token from local storage
 * @returns The last token used or null if none is found
 */
const getInitialToken = (): Token => {
	const storedToken = localStorage.getItem(TOKEN_KEY);
	return storedToken ? JSON.parse(storedToken) : null;
};

export const tokenStore = writable<Token>(getInitialToken());

// Back the store to local storage to persist on reload
tokenStore.subscribe((value) => {
	localStorage.setItem(TOKEN_KEY, JSON.stringify(value));
});

// Utility functiosns

/**
 * Removes token from store and local storage
 */
export const logoutUser = () => {
	tokenStore.set(null);
	localStorage.clear();
};

/**
 * Shorthand for collecting the current token
 * @returns the current token
 */
export const getToken = (): Token => {
	return get(tokenStore);
};
