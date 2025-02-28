<script lang="ts">
	let message = 'Click the button to run Rust backend!';
	import { Alert } from 'flowbite-svelte';
	import { Button } from 'flowbite-svelte';

	async function callRust() {
		const { invoke } = await import('@tauri-apps/api/core');
		message = await invoke('greet', { name: 'User' });
	}
</script>

<main>
	<h1>Welcome to Tauri + SvelteKit</h1>
	<p>{message}</p>
	<Button on:click={callRust}>Call Rust</Button>
	<div class="p-8">
		<Alert>
			<span class="font-medium">Info alert!</span>
			Change a few things up and try submitting again.
		</Alert>
	</div>
</main>
