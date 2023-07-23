/** @type {import('tailwindcss').Config} */
export default {
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		colors: {
			container: '#f3f6fd',
			orange: '#FEE4CB',
			'orange-accent': '#FF942E',
			purple: '#E9E7FD',
			'purple-accent': '#4F3FF0',
			blue: '#DBF6FD',
			'blue-accent': '#096C86',
			pink: '#FFD3E2',
			'pink-accent': '#DF3670',
			green: '#C8F7DC',
			'green-accent': '#34C471',
			'darker-blue': '#D5DEFF',
			'darker-blue-accent': '#4067F9'
		},
		extend: {}
	},
	daisyui: {
		themes: [
			{
				nineyardslight: {
					primary: '#570df8',
					secondary: '#f000b8',
					accent: '#1F1C2E',
					neutral: '#FFFFFF',
					'base-100': '#F3F6FD',
					info: '#3abff8',
					success: '#36d399',
					warning: '#fbbd23',
					error: '#f87272'
				},
				nineyardsdark: {
					primary: '#641ae6',
					secondary: '#d926a9',
					accent: '#353C50',
					neutral: '#1F2937',
					'base-100': '#111827',
					info: '#3abff8',
					success: '#36d399',
					warning: '#fbbd23',
					error: '#f87272'
				}
			},
			'light',
			'dark',
			'cupcake'
		]
	},
	plugins: [require('daisyui')]
};
