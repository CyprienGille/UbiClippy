<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { SlideToggle } from '@skeletonlabs/skeleton';

	let promptsPromise: Promise<Array<Prompt>> = invoke('get_all_prompts');

	function togglePrompt(id: number) {
		invoke('toggle_prompt', { id });
		refreshPrompts();
	}

	function refreshPrompts() {
		promptsPromise = invoke('get_all_prompts');
	}
</script>

<main>
	<ol class="breadcrumb pl-3 pt-2 text-xl">
		<li class="crumb"><a class="anchor" href="/">Home</a></li>
		<li class="crumb-separator" aria-hidden="true">&rsaquo;</li>
		<li class="crumb"><a class="anchor" href="/settings">Settings</a></li>
		<li class="crumb-separator" aria-hidden="true">&rsaquo;</li>
		<li>Prompt Library</li>
	</ol>

	<div
		class="logo-cloud mx-2 my-3 grid-cols-1 justify-center gap-1 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4"
	>
		{#await promptsPromise}
			Loading prompts...
		{:then allPrompts}
			{#each allPrompts as prompt}
				<div class="logo-item relative whitespace-pre-line text-wrap hover:variant-filled-tertiary">
					{prompt.content}
					<div class="absolute bottom-0 right-0 mr-1">
						<SlideToggle
							name="Slide${prompt.id}"
							active="bg-primary-500"
							size="sm"
							bind:checked={prompt.enabled}
							on:change={() => togglePrompt(prompt.id)}
						/>
					</div>
				</div>
			{/each}
		{/await}
	</div>
</main>
