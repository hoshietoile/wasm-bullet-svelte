<script lang="ts">
	import type { InputEvent } from '../../@types/event';
	import Input from '../atoms/Input.svelte';

	// TODO: Reactの...Propsみたいにできないのか
	// propsを片付できないのは不便かも
	export let id: string | null = null;
	export let value: string | number;
	export let label: string;
	export let placeholder: string = '';
	export let className: string = '';
	export let maxlength: number | null = null;
	export let min: number | null = null;
	export let max: number | null = null;
	export let onInput: InputEvent | null = null;
</script>

<div class="input-group flex flex-col">
	<label for={id}>{label}</label>
	<Input {id} bind:value {className} {placeholder} {maxlength} {min} {max} {onInput} />
</div>
