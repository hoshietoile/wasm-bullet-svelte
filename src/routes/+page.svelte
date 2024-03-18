<script lang="ts">
	import { onMount } from 'svelte';
	import init, { initialize, type Screen } from '$lib/wasm/pkg';
	import Input from '../components/atoms/Input.svelte';
	import {
		Icon,
		Plus,
		Play,
		Pause,
		Camera,
		PencilSquare,
		Trash,
		CloudArrowUp,
		GlobeAlt
	} from 'svelte-hero-icons';
	import IconButton from '../components/atoms/IconButton.svelte';
	import SectionPanel from '../components/organisms/SectionPanel.svelte';
	import InputForm from '../components/molecules/InputForm.svelte';

	const CANVAS_ID = 'main-canvas';

	let canvasRef: HTMLCanvasElement | null = null;
	let screen: Screen | null = null;
	let request: number | null = null;
	let isSuspended = false;

	type Settings = {
		canvas_id: string;
		width: number;
		height: number;
		speed: number;
		degree: number;
		disk_num: number;
		interval_degree: number;
		spawn_interval: number;
		degree_change_per: number;
		offset: number;
		update_angle: number;
		update_speed: number;
		change_angle: number;
		change_speed: number;
	};

	let settings: Settings = {
		canvas_id: CANVAS_ID,
		width: 700,
		height: 700,
		speed: 5,
		degree: 90,
		disk_num: 4,
		interval_degree: 90,
		spawn_interval: 60,
		degree_change_per: 10,
		offset: 0,
		update_angle: 0,
		update_speed: 0,
		change_angle: 0,
		change_speed: 0
	};

	// header part
	let isPlaying = false;

	const updateFrame = () => {
		if (!screen || isSuspended) {
			return;
		}
		screen?.update_frame(request || 0);
		request = requestAnimationFrame(updateFrame);
	};

	const onClickStart = () => {
		isSuspended = false;
		updateFrame();
	};

	const onClickStop = () => {
		isSuspended = true;
		request = null;
	};

	const toggleIsPlaying = () => {
		isPlaying = !isPlaying;
		if (isPlaying) {
			onClickStart();
		} else {
			onClickStop();
		}
	};

	// const onReset = async () => {
	// 	isSuspended = true;
	// 	request = null;
	// 	screen = await initialize({
	// 		...settings
	// 	});
	// };

	const isAnyEmpty = (settings: Settings) => {
		return Object.values(settings).some((value) => !value && value !== 0);
	};

	const onReset = async (settings: Settings) => {
		isSuspended = true;
		request = null;
		if (isAnyEmpty(settings)) {
			return;
		}
		screen = await initialize({
			...settings,
			// TODO: なんかうまいことstring, numberの変換したい
			width: parseInt(settings.width, 10),
			height: parseInt(settings.height, 10),
			speed: parseInt(settings.speed, 10),
			degree: parseInt(settings.degree, 10),
			disk_num: parseInt(settings.disk_num, 10),
			interval_degree: parseInt(settings.interval_degree, 10),
			spawn_interval: parseInt(settings.spawn_interval, 10),
			degree_change_per: parseInt(settings.degree_change_per, 10),
			offset: parseInt(settings.offset, 10),
			update_angle: parseInt(settings.update_angle, 10),
			update_speed: parseInt(settings.update_speed, 10),
			change_angle: parseInt(settings.change_angle, 10),
			change_speed: parseInt(settings.change_speed, 10)
		});
		isSuspended = false;
		updateFrame();
	};

	let selectedEventId: number | null = null;
	const eventsMock = [
		{
			id: 1,
			name: 'Event01'
		},
		{
			id: 2,
			name: 'Event02'
		},
		{
			id: 3,
			name: 'Event03'
		}
	];

	$: playingIconSrc = isPlaying ? Pause : Play;

	let isMounted = false;
	let canvasWrapperRef: HTMLDivElement | null = null;
	onMount(async () => {
		await init();
		isMounted = true;
		// screen = await initialize({
		// 	...settings
		// });

		const ref = canvasWrapperRef;
		if (ref) {
			console.log('3## ref');
			const observer = new ResizeObserver(() => {
				const { width, height } = ref.getBoundingClientRect();
				// TODO: なんか変
				// console.log(`### ${width}:${height}`);
				// settings.width = width;
				// settings.height = height;
			});
			observer.observe(ref);
		}
	});

	// Play/Reset
	// TODO: timeoutIdの更新は補足させたくない ReactのuseRefみたいに扱いたい
	// -> 関数は変更とか検知されないので、更新関数を外部に切り出せばいい

	let timeoutId: number | undefined = undefined;
	const clearTimeoutId = () => {
		clearTimeout(timeoutId);
	};
	const updateTimeoutId = (id: number | undefined) => {
		timeoutId = id;
	};

	$: {
		if (isMounted) {
			clearTimeoutId();
			const id = setTimeout(() => {
				onReset(settings);
				updateTimeoutId(undefined);
			}, 500);
			updateTimeoutId(id);
		}
	}
</script>

<div class="flex flex-col h-full">
	<!-- Setting section -->
	<!-- <div></div> -->
	<div class="w-full border-b p-2 flex">
		<div>
			<Input className="w-[500px]" placeholder="Please Input Title" onInput={() => {}} />
		</div>
		<div class="grow"></div>

		<div>
			<!-- <IconButton src={Play} onClick={() => {}} /> -->
			<IconButton src={playingIconSrc} onClick={toggleIsPlaying} />
			<IconButton src={Camera} onClick={() => {}} />
			<IconButton src={CloudArrowUp} onClick={() => {}} />
			<IconButton src={GlobeAlt} onClick={() => {}} />
		</div>
	</div>

	<div class="flex h-full">
		<section class="flex">
			<!-- Section Events -->
			<SectionPanel title="Events">
				<ul>
					{#each eventsMock as event}
						<!-- REF: https://v1.tailwindcss.com/components/alerts -->
						<li class="py-1">
							<div
								class="bg-orange-100 border-l-4 border-orange-500 text-orange-700 p-4 flex"
								role="alert"
								on:click={() => {
									selectedEventId = event.id;
								}}
							>
								<p class="font-bold grow">{event.name}</p>
								<div class="flex">
									<Icon class="" src={PencilSquare} size="16" />
									<Icon class="text-red-600" src={Trash} size="16" />
								</div>
							</div>
							<!-- <IconButton src={Trash} onClick={() => {}} /> -->
							<!-- <IconButton src={PencilSquare} onClick={() => {}} /> -->
						</li>
					{/each}
				</ul>
				<button class="border rounded w-full px-2 py-1 flex flex-center">
					<Icon class="m-auto" src={Plus} size="24"></Icon>
				</button>
			</SectionPanel>

			{#if selectedEventId !== null}
				<!-- Section Setting-->
				<SectionPanel title="Settings">
					<!-- <div>
					Event Type: 弾追加 弾更新 エフェクト追加 背景更新 オブジェクト追加 オブジェクト更新...
				</div> -->
					<InputForm
						id="spawnInterval"
						label={'発射間隔'}
						min={0}
						max={60}
						bind:value={settings.spawn_interval}
					></InputForm>
					<InputForm id="speed" label={'速度'} min={0} max={60} bind:value={settings.speed}
					></InputForm>
					<InputForm id="diskNum" label={'弾数'} min={0} max={60} bind:value={settings.disk_num}
					></InputForm>
					<InputForm
						id="intervalDegree"
						label={'弾の角度'}
						min={0}
						max={360}
						bind:value={settings.interval_degree}
					></InputForm>
					<InputForm id="degree" label={'射出角度'} min={0} max={360} bind:value={settings.degree}
					></InputForm>
					<InputForm
						id="degreeChangePer"
						label={'射出角度変化'}
						bind:value={settings.degree_change_per}
					></InputForm>
					<InputForm
						id="updateAngle"
						label={'各FR毎角度変化'}
						min={0}
						max={360}
						bind:value={settings.update_angle}
					></InputForm>
					<InputForm
						id="updateSpeed"
						label={'各FR毎速度変化'}
						min={0}
						max={10}
						bind:value={settings.update_speed}
					></InputForm>
					<InputForm
						id="changeAngle"
						label={'角度更新'}
						min={0}
						max={360}
						bind:value={settings.change_angle}
					></InputForm>
					<InputForm
						id="changeSpeed"
						label={'速度更新'}
						min={0}
						max={10}
						bind:value={settings.change_speed}
					></InputForm>
					<InputForm id="offset" label={'オフセット'} bind:value={settings.offset}></InputForm>
				</SectionPanel>
			{:else}{/if}
		</section>

		<!-- Section Canvas -->
		<div bind:this={canvasWrapperRef} class="w-full">
			<canvas
				id={settings.canvas_id}
				bind:this={canvasRef}
				width={settings.width}
				height={settings.height}
			/>
		</div>
	</div>
</div>

<style lang="postcss">
	#main-canvas {
		border: 1px solid black;
	}
</style>
