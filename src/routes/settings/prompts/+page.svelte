<script lang="ts">
	import { invoke } from '@tauri-apps/api';
	import { SlideToggle } from '@skeletonlabs/skeleton';
	import editIcon from '$lib/edit.svg';
	import trashIcon from '$lib/trash.svg';
	import plusIcon from '$lib/plus.svg';

	let promptsPromise: Promise<Array<Prompt>> = invoke('get_all_prompts');
	let editingId: Number = -1;

	let addingPrompt: Boolean = false;
	let new_content: String = 'Enter Prompt Here';

	function startEditPrompt(id: Number) {
		editingId = id;
	}

	function endEditPrompt(id: Number, content: String) {
		invoke('edit_prompt_content', { id, content });
		editingId = -1;
		refreshPrompts();
	}

	function startAddPrompt() {
		editingId = -2;
	}

	function endAddPrompt() {
		editingId = -1;
		invoke('add_prompt', { newPrompt: new_content });
		refreshPrompts();
	}

	function togglePrompt(id: Number) {
		invoke('toggle_prompt', { id });
		refreshPrompts();
	}

	function removePrompt(id: Number) {
		invoke('remove_prompt', { id });
		refreshPrompts();
	}

	function refreshPrompts() {
		promptsPromise = invoke('get_all_prompts');
	}

	function handleEnterKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			if (!event.shiftKey) {
				event.preventDefault();
				document.getElementById('checkEdit')?.click();
			}
		}
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
				<div class="logo-item relative whitespace-pre-line text-wrap">
					{#if editingId == -1}
						{prompt.content}
						<button class="btn absolute right-0 top-0" on:click={() => removePrompt(prompt.id)}>
							<img class="dark:invert" src={trashIcon} alt="An icon of a bin" />
						</button>
						<button
							class="btn absolute bottom-0 left-0"
							on:click={() => startEditPrompt(prompt.id)}
						>
							<img class="dark:invert" src={editIcon} alt="An icon of a pen" />
						</button>
						<div class="absolute bottom-0 right-0 pb-1 pr-1">
							<SlideToggle
								name="Slide${prompt.id}"
								active="bg-primary-500"
								size="sm"
								bind:checked={prompt.enabled}
								on:change={() => togglePrompt(prompt.id)}
							/>
						</div>
					{:else if editingId == prompt.id}
						<textarea
							class="textarea mx-3"
							bind:value={prompt.content}
							on:keydown={handleEnterKeyDown}
						/>
						<button
							class="btn absolute bottom-0 left-0"
							id="checkEdit"
							on:click={() => endEditPrompt(prompt.id, prompt.content)}>✅</button
						>
					{:else}
						{prompt.content}
						<span class="absolute bottom-0 left-0 mb-1 text-sm font-light italic">
							Another prompt is being edited right now
						</span>
					{/if}
				</div>
			{/each}
			{#if editingId == -2}
				<div class="logo-item relative">
					<textarea
						class="textarea mx-3"
						bind:value={new_content}
						on:keydown={handleEnterKeyDown}
					/>
					<button class="btn absolute bottom-0 right-0" id="checkEdit" on:click={endAddPrompt}
						>✅</button
					>
				</div>
			{:else if editingId == -1}
				<button class="logo-item hover:variant-ghost-primary" on:click={startAddPrompt}
					><img src={plusIcon} class="w-12 dark:invert" alt="A plus icon" /></button
				>
			{:else}
				<button class="logo-item relative" disabled>
					<img src={plusIcon} class="w-12 dark:invert" alt="A plus icon" />
					<span class="absolute bottom-0 left-0 mb-1 text-sm font-light italic">
						Another prompt is being edited right now
					</span>
				</button>
			{/if}
		{/await}
	</div>
</main>
