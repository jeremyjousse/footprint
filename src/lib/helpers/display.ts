export const displayTimeFromSeconds = (seconds: number): string => {
	if (isNaN(seconds) || !isFinite(seconds) || seconds < 0) {
		return '00:00';
	}
	const mins = Math.floor(seconds / 60);
	const secs = seconds % 60;
	return `${mins}:${secs}`;
};
