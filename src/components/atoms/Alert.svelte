<script lang="ts">
	export let title: string;
	export let eventType: string;

	let className: string;
	$: {
		let bg = '';
		if (eventType === 'bullet') {
			bg =
				'bg-blue-100 border-blue-500 text-blue-700 dark:bg-blue-200 dark:border-blue-600 dark:text-blue-800';
		} else if (eventType === 'effect') {
			bg =
				'bg-orange-100 border-orange-500 text-orange-700 dark:bg-orange-200 dark:border-orange-600 dark:text-orange-800';
		} else if (eventType === 'actor') {
			bg =
				'bg-pink-100 border-pink-500 text-pink-700 dark:bg-pink-200 dark:border-pink-600 dark:text-pink-800';
		} else {
			bg =
				'bg-green-100 border-green-500 text-green-700 dark:bg-green-200 dark:border-green-600 dark:text-green-800';
		}
		className = `border-l-4 px-4 py-2 flex ${bg}`.trim();
	}
</script>

<!-- REF: https://v1.tailwindcss.com/components/alerts -->
<div class={className}>
	<p class="font-bold grow">{title}</p>
	<div class="flex">
		<slot />
	</div>
</div>
