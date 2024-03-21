<script lang="ts">
	import { Plus, XMark } from 'svelte-hero-icons';
	import IconButton from '../atoms/IconButton.svelte';

	export let title: string;
	export let onClickAdd: (() => void) | null = null;
	export let onClickClose: (() => void) | null = null;
</script>

<div class="border-r shadow-lg w-[280px] px-2 h-full">
	<div class="flex py-2">
		<div class="font-bold text-cyan-700 dark:text-cyan-600">{title}</div>
		<div class="grow"></div>
		<div class="">
			{#if onClickAdd}
				<IconButton src={Plus} iconSize="xs" onClick={() => {}} />
			{/if}
			{#if onClickClose}
				<IconButton src={XMark} iconSize="xs" onClick={onClickClose} />
			{/if}
		</div>
	</div>
	<div class="border-b"></div>

	<div class="p-2">
		<slot />
	</div>
</div>
