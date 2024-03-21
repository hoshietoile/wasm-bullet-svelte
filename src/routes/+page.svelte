<script lang="ts">
	import { onMount } from 'svelte';
	import init, { initialize, type Screen } from '$lib/wasm/pkg';
	import Input from '../components/atoms/Input.svelte';
	import { slide } from 'svelte/transition';
	import { quintOut } from 'svelte/easing';
	import {
		Icon,
		Plus,
		Play,
		Pause,
		Camera,
		PencilSquare,
		Trash,
		CloudArrowUp,
		GlobeAlt,
		BugAnt,
		PaintBrush
	} from 'svelte-hero-icons';
	import IconButton from '../components/atoms/IconButton.svelte';
	import SectionPanel from '../components/organisms/SectionPanel.svelte';
	import InputForm from '../components/molecules/InputForm.svelte';
	import Alert from '../components/atoms/Alert.svelte';
	import Modal from '../components/atoms/Modal.svelte';

	const CANVAS_ID = 'main-canvas';

	let canvasRef: HTMLCanvasElement | null = null;
	let screen: Screen | null = null;
	let request: number | null = null;
	let isSuspended = false;
	let title: string = '';

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
		if (!screen || !isPlaying) {
			return;
		}
		screen?.update_frame(request || 0);
		request = requestAnimationFrame(updateFrame);
	};

	const onClickStart = () => {
		isPlaying = true;
		request = null;
		updateFrame();
	};

	const onClickStop = () => {
		isPlaying = false;
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

	const isAnyEmpty = (settings: Settings) => {
		return Object.values(settings).some((value) => !value && value !== 0);
	};

	const onReset = async (settings: Settings) => {
		isPlaying = false;

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
		isPlaying = true;
		updateFrame();
	};

	let selectedEventId: number | null = null;
	const eventsMock = [
		{
			id: 1,
			name: 'Event01',
			eventType: 'effect'
		},
		{
			id: 2,
			name: 'Event02',
			eventType: 'actor'
		},
		{
			id: 3,
			name: 'Event03',
			eventType: 'bg-update'
		},
		{
			id: 4,
			name: 'Event04',
			eventType: 'bullet'
		}
	];

	const onClickCloseEvent = () => {
		console.log('### click');
		selectedEventId = null;
	};

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

	let isShownBulletEditor = false;
	const showBulletEditor = () => {
		isShownBulletEditor = true;
	};
	const hideBulletEditor = () => {
		isShownBulletEditor = false;
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
			<Input
				bind:value={title}
				className="w-[500px]"
				placeholder="Please Input Title"
				onInput={() => {}}
			/>
		</div>
		<div class="grow"></div>

		<div>
			<IconButton className="text-red-400" src={BugAnt} onClick={toggleIsPlaying} />
			<span class="border-r mx-2"></span>
			<IconButton src={PaintBrush} onClick={showBulletEditor} />
			<IconButton src={playingIconSrc} onClick={toggleIsPlaying} />
			<IconButton src={playingIconSrc} onClick={toggleIsPlaying} />
			<IconButton src={Camera} onClick={() => {}} />
			<IconButton src={CloudArrowUp} onClick={() => {}} />
			<IconButton src={GlobeAlt} onClick={() => {}} />
		</div>
	</div>

	<div class="flex h-full">
		<section class="flex">
			<!-- Section Events -->
			<SectionPanel title="Events" onClickAdd={() => {}}>
				<ul>
					{#each eventsMock as event}
						<li class="py-1">
							<Alert title={event.name} eventType={event.eventType}>
								<div class="flex">
									<button on:click={() => (selectedEventId = event.id)}>
										<Icon class="" src={PencilSquare} size="16" />
									</button>
									<button on:click={() => {}}>
										<Icon class="text-red-600" src={Trash} size="16" />
									</button>
								</div>
							</Alert>
						</li>
					{/each}
				</ul>
				<button class="border rounded w-full px-2 py-1 flex flex-center">
					<Icon class="m-auto" src={Plus} size="24"></Icon>
				</button>
			</SectionPanel>

			{#if selectedEventId !== null}
				<!-- Section Setting-->
				<!-- <div transition:slide={{ delay: 250, duration: 300, easing: quintOut, axios: 'x' }}> -->
				<div transition:slide={{ duration: 300, easing: quintOut, axis: 'x' }}>
					<SectionPanel title="Settings" onClickClose={onClickCloseEvent}>
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
				</div>
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

	{#if isShownBulletEditor}
		<Modal title="スプライト編集" onCloseClick={hideBulletEditor} />
	{/if}
</div>

<style lang="postcss">
	#main-canvas {
		border: 1px solid black;
	}
</style>
