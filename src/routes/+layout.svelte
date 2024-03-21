<script lang="ts">
	import IconButton from '../components/atoms/IconButton.svelte';
	import './../app.css';
	import { ArrowUp, Icon, SpeakerWave, Bars3, Sun, XMark, Link, Moon } from 'svelte-hero-icons';
	import { slide } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';

	const onClick = () => {};

	let hidden = true;

	const onClickMenu = (e) => {
		hidden = false;
	};

	const onClickHide = () => {
		hidden = true;
	};

	let isDarkMode: boolean = false;

	const onClickToggleTheme = () => {
		const html = document.querySelector('html');
		document.startViewTransition(async () => {
			if (isDarkMode) {
				html?.classList.remove('dark');
				isDarkMode = false;
			} else {
				html?.classList.add('dark');
				isDarkMode = true;
			}
		});
	};
</script>

<div class="flex bg-white text-gray-600 dark:text-gray-200 dark:bg-gray-700">
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

	<!-- class:hideDrawer={hidden}
			class:showDrawer={!hidden} -->
	<!-- ref: https://svelte.jp/docs/svelte-transition -->
	{#if !hidden}
		<div
			transition:slide={{ duration: 300, easing: quintOut, axis: 'x' }}
			class="top-0 left-0 z-40 w-64 h-screen overflow-y-auto transition-all shadow-md dark:border-r"
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
						<a class="text-cyan-700 font-bold" href="/my-bullets"> MyBullets </a>
					</li>
					<li>
						<a href="/market"> BulletMarket</a>
					</li>
				</ul>
			</div>
			<div class="border-t">Logout</div>
		</div>
	{/if}

	<!-- Main -->
	<div class="w-full" class:hideContent={hidden}>
		<header class="flex p-2 h-[56px] shadow-md border-b">
			<div>
				<IconButton onClick={onClickMenu} src={Bars3} iconSize="sm" />
			</div>
			<div class="grow text-center">Bullet Wall</div>
			<div>
				<IconButton {onClick} src={SpeakerWave} iconSize="sm" />
				<IconButton onClick={onClickToggleTheme} src={isDarkMode ? Sun : Moon} iconSize="sm" />
			</div>
		</header>
		<main class="h-[calc(100vh-56px)]">
			<slot />
		</main>
	</div>
</div>

<style>
	.hideDrawer {
		@apply w-0;
	}
	.showDrawer {
		@apply min-w-64;
	}
	/* TODO:  */

	::view-transition-old {
		animation: 1s transition-out 0s ease;
	}
	::view-transition-new {
		animation: 1s transition-in 0s ease;
	}
	html::view-transition {
		animation: 1s transition-in 0s ease;
	}
	@keyframes transition-out {
		from {
			clip-path: circle(0%);
		}
		to {
			clip-path: circle(100%);
		}
	}
	@keyframes transition-in {
		from {
			clip-path: circle(0%);
		}
		to {
			clip-path: circle(100%);
		}
	}
</style>
