<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';

	let last_response = '';

	const unlisten = listen('llm_chunk', (event: any) => {
		last_response += event.payload.response;
	});

	function trigger_message() {
		console.log('Pressed button...');
		invoke('prompt_with_cb', { model: 'llama3:instruct' });
	}

	function into_clipboard(text: string) {
		invoke('to_clipboard', { text });
	}
</script>

<main class="h-screen w-screen content-center space-y-5">
	<div class="w-full text-center">
		<button class="btn variant-filled-primary" on:click={trigger_message}>
			Summarize Clipboard
		</button>
	</div>
	<div class="w-full flex">
		<div class="w-1/4"></div>
		<div class="w-1/2">
			<div class="w-full text-center">
				Model:
				<span>{last_response}</span>
			</div>
		</div>
	</div>
	<div class="w-full text-center">
		<button class="btn variant-filled-secondary" on:click={() => into_clipboard(last_response)}>
			Copy resp to clipboard
		</button>
	</div>
</main>
