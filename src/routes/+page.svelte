<script lang="ts">
	import { onMount } from 'svelte';
	import init, { greet } from '$lib/wasm/pkg';

	const CANVAS_ID = 'main-canvas';

	let screen_width = 500;
	let screen_height = 500;
	let canvas_ref: HTMLCanvasElement | null = null;

	onMount(async () => {
		await init();
		setTimeout(() => {
			console.log(canvas_ref);
		});
	});
</script>

<canvas id={CANVAS_ID} bind:this={canvas_ref} width={screen_width} height={screen_height} />
