import { Toast } from '$domain';
import { writable } from 'svelte/store';

export const toasts = writable<Toast[]>([]);

export const addToast = (toast: Toast) => {
	toasts.update((all) => [{ ...toast }, ...all]);

	if (toast.timeout) setTimeout(() => dismissToast(toast.id), toast.timeout);
};

export const dismissToast = (id: number) => {
	toasts.update((all) => all.filter((t) => t.id !== id));
};
