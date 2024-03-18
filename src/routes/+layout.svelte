<script>
	import IconButton from '../components/atoms/IconButton.svelte';
	import './../app.css';
	import { ArrowUp, Icon, SpeakerWave, Bars3, Sun, XMark, Link } from 'svelte-hero-icons';

	const onClick = () => {};

	let hidden = true;

	const onClickMenu = (e) => {
		hidden = false;
	};

	const onClickHide = () => {
		hidden = true;
	};
</script>

<div class="flex">
	<!-- Drawer -->
	<!-- Ref: https://tw-elements.com/docs/standard/navigation/sidenav/ -->
	<!-- <div
		class:hideDrawer={hidden}
		class="fixed top-0 left-0 z-40 w-64 h-screen overflow-y-auto transition-transform bg-white shadow-md"
		tabindex="-1"
	>
		<div class="flex h-[56px] border-b">
			<div class="p-2">
				<IconButton onClick={onClickHide} src={XMark} iconSize="sm" />
			</div>
			<div class="grow"></div>
		</div>
		<div class="overflow-y-auto p-4">
			<ul class="space-y-2 font-medium">
				<li>
					<a
						class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group"
					>
						<Icon src={Bars3} size="24" />
					</a>
				</li>
			</ul>
		</div>
	</div> -->
	<div
		class:hideDrawer={hidden}
		class:showDrawer={!hidden}
		class="top-0 left-0 z-40 w-64 h-screen overflow-y-auto transition-all bg-white shadow-md bg-gray-100"
		tabindex="-1"
	>
		<div class="flex h-[56px] border-b">
			<div class="p-2">
				<IconButton onClick={onClickHide} src={XMark} iconSize="sm" />
			</div>
			<div class="grow"></div>
		</div>
		<div class="overflow-y-auto p-4">
			<ul class="space-y-2 font-medium">
				<li>
					<a
						class="flex items-center p-2 text-gray-900 rounded-lg dark:text-white hover:bg-gray-100 dark:hover:bg-gray-700 group"
					>
						<Icon src={Bars3} size="24" />
					</a>
				</li>
			</ul>
		</div>
	</div>

	<!-- Main -->
	<div class="w-full" class:hideContent={hidden}>
		<header class="flex p-2 h-[56px] shadow-md">
			<div>
				<IconButton onClick={onClickMenu} src={Bars3} iconSize="sm" />
			</div>
			<div class="grow text-center">Bullet Wall</div>
			<div>
				<IconButton {onClick} src={SpeakerWave} iconSize="sm" />
				<IconButton {onClick} src={Sun} iconSize="sm" />
			</div>
		</header>
		<main class="h-[calc(100vh-56px)]">
			<slot />
		</main>
	</div>
</div>

<style>
	.hideDrawer {
		/* @apply -translate-x-full; */
		@apply w-0;
	}
	.showDrawer {
		@apply min-w-64;
	}
	/* .hideContent {
		@apply ;
	} */
</style>
