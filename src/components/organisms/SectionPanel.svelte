<script lang="ts">
	import { Plus } from 'svelte-hero-icons';
	import IconButton from '../atoms/IconButton.svelte';

	export let title: string;
</script>

<div class="border-r shadow-lg w-[280px] px-2 h-full">
	<div class="flex py-2">
		<div class="font-bold text-cyan-700">{title}</div>
		<div class="grow"></div>
		<div class="">
			<IconButton src={Plus} iconSize="xs" onClick={() => {}} />
		</div>
	</div>
	<div class="border-b"></div>

	<div class="p-2">
		<slot />
	</div>
</div>
