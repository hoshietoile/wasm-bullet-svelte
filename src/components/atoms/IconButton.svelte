<script lang="ts">
	import { Icon, type IconSource } from 'svelte-hero-icons';
	import type { IconSize } from '../../@types/icon';

	export let src: IconSource;
	export let onClick: () => void;
	export let iconSize: IconSize;

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
</script>

<button on:click={onClick} class="border rounded p-1 hover:bg-gray-200">
	<Icon {src} {size} />
</button>
