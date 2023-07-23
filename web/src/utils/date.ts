export const daysUntil = (timestamp: number): number => {
	const now = new Date().getTime();
	const day = 24 * 60 * 60 * 1000;
	return Math.ceil((timestamp - now) / day);
};

export const formatDate = (timestamp: number): String => {
	const date = new Date(timestamp);
	const month = date.toLocaleString('en-US', { month: 'long' });
	return `${month}, ${date.getDate()}, ${date.getFullYear()}`;
};
