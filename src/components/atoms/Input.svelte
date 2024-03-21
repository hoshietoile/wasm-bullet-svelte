<script lang="ts">
	import type { InputEvent } from '../../@types/event';

	// type InputType = 'text' | 'number';

	export let id: string | null = null;
	// export const inputType: InputType = 'text';
	export let value: string | number;
	export let placeholder: string = '';
	export let className: string = '';
	export let maxlength: number | null = null;
	export let min: number | null = null;
	export let max: number | null = null;
	export let onInput: InputEvent | null = null;

	$: cssClass = `border rounded px-2 py-1 outline-none  focus:ring-1  ring-offset-1 ring-cyan-800 dark:ring-cyan-600 dark:bg-gray-800 dark:border-gray-400 ${className}`;
</script>

<!-- https://qiita.com/mkin/items/6be23f43fbc0da0fc03f -->
<!-- TODO: http://psychedelicnekopunch.com/archives/2990 -->
<!-- type 動的にできない? -->
<input {id} class={cssClass} {placeholder} bind:value on:input={onInput} {min} {max} {maxlength} />

<!-- TODO: appendix? -->
