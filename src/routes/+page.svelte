<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';

	interface Message {
		role: string;
		content: string;
	}
	interface ChatResponse {
		message: Message;
		done: boolean;
	}
	interface LLMChunk {
		payload: ChatResponse;
	}

	let chatElement: HTMLElement;

	let chat: string[] = [];
	let currentMessage = '';

	let generating = false;
	let firstMsg = false;

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
			model: 'llama3:instruct'
		});
		chat = [...chat, currentMessage];
		currentMessage = '';

		setTimeout(() => {
			scrollChatBottom();
		}, 0);
	}

	function handleKeyDown(event: KeyboardEvent) {
		if (event.key === 'Enter') {
			if (!event.shiftKey) {
				event.preventDefault();
				sendMessage();
			}
		}
	}
</script>

<main class="h-screen w-screen content-center space-y-5">
	<div class="h-full grid grid-rows-[1fr_auto]">
		<div class="bg-surface-500/30 p-4 overflow-y-auto" bind:this={chatElement}>
			<section class="w-full h-auto p-4 overflow-y-auto space-y-2">
				{#each chat as msg, i}
					{#if i % 2 === 0}
						<!-- User Message Bubble -->
						<div class="grid grid-cols-[1fr_auto] gap-2">
							<div class="w-12" />
							<div class="card p-4 variant-soft-primary rounded-tr-none">
								<header class="flex justify-between items-center">
									<p class="font-bold">User</p>
								</header>
								<p>{msg}</p>
							</div>
						</div>
					{:else}
						<!-- Assistant Message Bubble -->
						<div class="grid grid-cols-[auto_1fr] gap-2">
							<div class="card p-4 rounded-tl-none space-y-2 variant-soft-secondary">
								<header class="flex justify-between items-center">
									<p class="font-bold">Assistant</p>
								</header>
								<p>{msg}</p>
							</div>
							<div class="w-16" />
						</div>
					{/if}
				{/each}
			</section>
		</div>
		<div class="bg-surface-500/30 p-4">
			<div class="input-group input-group-divider grid-cols-[1fr_auto] rounded-container-token">
				<textarea
					bind:value={currentMessage}
					on:keydown={handleKeyDown}
					class="bg-transparent border-0 ring-0 py-2 pl-2"
					name="prompt"
					id="prompt"
					placeholder="Write a message..."
					rows="1"
				/>
				<button class="variant-filled-primary" on:click={sendMessage}>Send</button>
			</div>
		</div>
	</div>
</main>
