<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { listen } from '@tauri-apps/api/event';

	let last_response = '';
	// "In this example, any requests starting with /ubiclippy will be proxied to the Tauri backend running at tauri://localhost/. Make sure to adjust the configuration to match your specific setup. Choose an approach that best fits your project's requirements and constraints, and remember to test thoroughly in both development and production environments to ensure everything works as expected.";

	const unlisten = listen('ollama_chunk', (event: any) => {
		last_response += event.payload.response;
	});

	function trigger_message() {
		console.log('Pressed button...');
		invoke('trigger_response');
	}
</script>

<main class="h-screen w-screen content-center space-y-5">
	<div class="w-full text-center">
		<button class="btn variant-filled-primary" on:click={trigger_message}> Who are you? </button>
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
</main>
