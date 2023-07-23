import { getToken } from '../../../stores/token';
import type { Headers } from '../../../types/utility';

/**
 * Generates the Authorization header for making authenticated API requests.
 * @returns The header object with the bearer token
 */
export const authHeader = (): Headers => {
	return {
		Authorization: `Bearer ${getToken()}`
	};
};
