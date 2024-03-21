<script lang="ts">
	import { Icon, type IconSource } from 'svelte-hero-icons';
	import type { IconSize } from '../../@types/icon';

	export let src: IconSource;
	export let onClick: () => void;
	export let iconSize: IconSize = 'sm';
	export let className: string = '';

	const resolveSize = (iconSize: IconSize) => {
		switch (iconSize) {
			case 'lg':
				return '48';
			case 'md':
				return '36';
			case 'xs':
				return '16';
			default:
				return '24';
		}
	};

	$: size = resolveSize(iconSize);
	$: customClass =
		`border rounded p-1 hover:bg-gray-200 dark:hover:bg-gray-600 focus:ring-2 ring-offset-1 ring-cyan-800 dark:ring-cyan-600 ${className}`.trim();
</script>

<button on:click={onClick} class={customClass}>
	<Icon {src} {size} />
</button>
