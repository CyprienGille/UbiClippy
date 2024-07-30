<script lang="ts">
	import { getModalStore, type ModalSettings } from '@skeletonlabs/skeleton';
	import { invoke } from '@tauri-apps/api/tauri';
	import { register } from '@tauri-apps/api/globalShortcut';
	import { listen } from '@tauri-apps/api/event';

	import ModalModelList from '$lib/ModalModelList.svelte';

	import cbIcon from '$lib/copy_clipboard_icon.png';

	const modalStore = getModalStore();

	let chatElement: HTMLElement;

	let chat: string[] = [];
	let currentMessage = '';

	let chosenModel: string;

	let pickingPrompt = true;
	let generating = false;
	let firstMsg = false;

	let promptsPromise: Promise<Array<Prompt>> = invoke('get_all_prompts');
	let modelsPromise: Promise<Array<ModelInfo>> = invoke('get_all_models');

	register('CommandOrControl+Alt+C', () => {
		invoke('summon_window');
	});

	const unlisten = listen('llm_chunk', (event: LLMChunk) => {
		if (!generating) {
			// First message
			firstMsg = true;
		}

		if (firstMsg) {
			chat = [...chat, event.payload.message.content];
			firstMsg = false;
			generating = true;
		} else if (event.payload.done) {
			// Last message
			generating = false;
			setTimeout(() => {
				scrollChatBottom();
			}, 0);
		} else {
			// Middle messages
			chat[chat.length - 1] += event.payload.message.content;
		}
	});

	function scrollChatBottom(): void {
		chatElement.scrollTo({ top: chatElement.scrollHeight, behavior: 'smooth' });
	}

	function sendMessage() {
		invoke('process_chat', {
			userChat: currentMessage,
			model: chosenModel
		});
		chat = [...chat, currentMessage];
		currentMessage = '';

		setTimeout(() => {
			scrollChatBottom();
		}, 0);
	}

	function handleInputKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			if (!event.shiftKey) {
				event.preventDefault();
				if (ableToSend()) {
					sendMessage();
				}
			}
		}
	}

	function copyMessage(i: number) {
		invoke('set_clipboard', { text: chat[i] });
	}

	function backToPrompts() {
		pickingPrompt = true;
		chat = [];
		currentMessage = '';
	}

	async function selectPrompt(id: number, content: string) {
		pickingPrompt = false;
		currentMessage = content.replaceAll('$CLIPBOARD$', await invoke('get_clipboard'));
		sendMessage();
	}

	function noSelectedModel(choice: string): boolean {
		return choice === null || choice === undefined;
	}

	function ableToSend(): boolean {
		return !noSelectedModel(chosenModel) && !generating;
	}

	function triggerModal() {
		new Promise<boolean>((resolve) => {
			const modelsModal: ModalSettings = {
				type: 'component',
				title: 'Pick a Model',
				body: '',
				component: {
					ref: ModalModelList,
					props: { modelsPromise: modelsPromise }
				},
				response: (r: boolean) => {
					resolve(r);
				}
			};
			modalStore.trigger(modelsModal);
		}).then((r: boolean | string) => {
			if (typeof r === 'string') {
				chosenModel = r;
			}
		});
	}
</script>

<main class="h-screen w-screen">
	{#if pickingPrompt}
		<button class="btn w-full justify-center text-2xl" on:click={triggerModal}
			>{chosenModel ?? 'Pick a Model'}</button
		>
		<div
			class="logo-cloud grid-cols-1 justify-center md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4"
		>
			{#await promptsPromise}
				Loading prompts...
			{:then all_prompts}
				{#each all_prompts as prompt, i}
					<button
						class="variant-filled-surface btn m-2 whitespace-pre-line text-wrap rounded-md"
						disabled={noSelectedModel(chosenModel)}
						on:click={() => selectPrompt(prompt.id, prompt.content)}>{prompt.content}</button
					>
				{/each}
			{/await}
		</div>
	{:else}
		<div class="grid h-full grid-rows-[1fr_auto]">
			<div class="overflow-y-auto bg-surface-500/10 p-4" bind:this={chatElement}>
				<section class="h-auto w-full space-y-2 overflow-y-auto p-4">
					{#each chat as msg, i}
						{#if i % 2 === 0}
							<!-- User Message Bubble -->
							<div class="grid grid-cols-[1fr_auto] gap-2">
								<div class="w-12" />
								<div class="card variant-soft-primary rounded-tr-none p-4">
									<header class="flex items-center justify-between">
										<button class="btn" on:click={() => copyMessage(i)}>
											<img class="w-6" src={cbIcon} alt="A clipboard icon" />
										</button>
										<p class="font-bold">User</p>
									</header>
									<p>{msg}</p>
								</div>
							</div>
						{:else}
							<!-- Assistant Message Bubble -->
							<div class="grid grid-cols-[auto_1fr] gap-2">
								<div class="card variant-soft-secondary space-y-2 rounded-tl-none p-4">
									<header class="flex items-center justify-between">
										<p class="font-bold">{chosenModel}</p>
										<button class="btn" on:click={() => copyMessage(i)}>
											<img class="w-6" src={cbIcon} alt="A clipboard icon" />
										</button>
									</header>
									<p>{msg}</p>
								</div>
								<div class="w-16" />
							</div>
						{/if}
					{/each}
				</section>
			</div>
			<div class="bg-surface-500/20 p-4">
				<div
					class="input-group input-group-divider grid-cols-[auto_1fr_auto] rounded-container-token"
				>
					<button class="variant-filled-primary" disabled={!ableToSend()} on:click={backToPrompts}
						>Back</button
					>
					<textarea
						bind:value={currentMessage}
						on:keydown={handleInputKeyDown}
						class="border-0 bg-transparent py-2 pl-2 ring-0"
						name="prompt"
						id="prompt"
						placeholder="Write a message..."
						rows="1"
					/>
					<button class="variant-filled-primary" disabled={!ableToSend()} on:click={sendMessage}
						>Send</button
					>
				</div>
			</div>
		</div>
	{/if}
</main>
