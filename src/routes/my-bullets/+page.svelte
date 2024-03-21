<script lang="ts">
	import Input from '../../components/atoms/Input.svelte';

	let searchQuery: string = '';

	const mockBullets = [
		{
			id: 'hosyuer243243',
			title: 'Bullet1'
		},
		{
			id: '234juer243243',
			title: 'Bullet2'
		},
		{
			id: 'hosdf0234js',
			title: 'Bullet3'
		}
	];
</script>

<div class="flex flex-col h-full">
	<div class="w-full border-b p-2 flex">
		<div class="m-auto">
			<Input
				bind:value={searchQuery}
				className="w-[500px]"
				placeholder="Please Input Search Keyword."
			/>
		</div>
	</div>

	<div class="p-2">
		{#each mockBullets as bullet}
			<div>
				{bullet.title}
			</div>
		{/each}
	</div>
</div>
