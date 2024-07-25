<script lang="ts">
	import type { SvelteComponent } from 'svelte';

	import { ListBox, ListBoxItem, getModalStore } from '@skeletonlabs/skeleton';

	// Props
	/** Exposes parent props to this component. */
	export let parent: SvelteComponent;
	export let modelsPromise: Promise<Array<ModelInfo>>;

	// Local
	let model: string;
	const modalStore = getModalStore();

	// Handle Form Submission
	function onFormSubmit(): void {
		if ($modalStore[0].response) $modalStore[0].response(model);
		modalStore.close();
	}

	// Base Classes
	const cBase = 'card p-4 w-modal shadow-xl space-y-4';
	const cHeader = 'text-2xl font-bold';
</script>

{#if $modalStore[0]}
	<div class="modal-example-form {cBase}">
		<header class={cHeader}>{$modalStore[0].title ?? '(title missing)'}</header>
		<article>{$modalStore[0].body ?? '(body missing)'}</article>
		{#await modelsPromise}
			Loading models...
		{:then modelsList}
			<ListBox class="max-h-72 overflow-auto scroll-smooth p-4 rounded-container-token">
				{#each modelsList as modelInfo}
					<ListBoxItem bind:group={model} name="model" value={modelInfo.name}
						>{modelInfo.name}
						<svelte:fragment slot="trail">{modelInfo.details.parameter_size}</svelte:fragment>
					</ListBoxItem>
				{/each}
			</ListBox>
		{/await}

		<footer class="modal-footer {parent.regionFooter}">
			<button class="btn {parent.buttonNeutral}" on:click={parent.onClose}
				>{parent.buttonTextCancel}</button
			>
			<button class="btn {parent.buttonPositive}" on:click={onFormSubmit}>Select Model</button>
		</footer>
	</div>
{/if}
