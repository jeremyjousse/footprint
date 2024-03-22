export const toSnakeCase = (input: string): string => {
	return input
		.replace(/([a-z])([A-Z])/g, '$1_$2')
		.replace(/[\s_]+/g, '_')
		.toLowerCase();
};
