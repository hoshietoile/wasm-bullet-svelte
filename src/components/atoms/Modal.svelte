<script lang="ts">
	import { XMark } from 'svelte-hero-icons';
	import IconButton from './IconButton.svelte';

	export let title: string;

	export let onCloseClick: () => void;
</script>

<!-- Main modal -->
<div
	class="overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full 100 bd-blur"
>
	<div class="relative w-full max-w-2xl max-h-full m-auto modal-content">
		<!-- Modal content -->
		<div class="relative bg-white rounded-lg shadow bg-white dark:bg-gray-700">
			<!--  dark:bg-gray-700 -->
			<!-- Modal header -->
			<div class="flex items-center justify-between py-2 px-4 border-b rounded-t border-gray-600">
				<h3 class="text-xl font-semibold">
					<!-- text-gray-900 dark:text-white -->
					{title}
				</h3>
				<IconButton src={XMark} onClick={onCloseClick}></IconButton>
			</div>
			<!-- Modal body -->
			<div class="p-4 md:p-5 space-y-4">
				<slot />
			</div>
			<!-- Modal footer -->
			<div class="flex items-center py-2 px-4 border-gray-200 rounded-b dark:border-gray-600 gap-2">
				<button
					data-modal-hide="static-modal"
					type="button"
					class="py-2.5 px-5 text-sm font-medium text-gray-900 focus:outline-none bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-4 focus:ring-gray-100"
					>Decline</button
				>
				<button
					data-modal-hide="static-modal"
					type="button"
					class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center"
					>Ok</button
				>
				<!-- dark:focus:ring-gray-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700 -->
				<!-- dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800 -->
			</div>
		</div>
	</div>
</div>

<style>
	.bd-blur {
		backdrop-filter: blur(20px);
	}
	.modal-content {
		transform: translateY(-50%);
		top: 50%;
	}
</style>
