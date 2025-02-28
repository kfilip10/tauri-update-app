<script lang="ts">
	let message = 'Click the button to run Rust backend!';
	import { Alert } from 'flowbite-svelte';
	import { Button } from 'flowbite-svelte';

	async function callRust() {
		const { invoke } = await import('@tauri-apps/api/core');
		message = await invoke('greet', { name: 'User' });
	}

	import { checkForAppUpdates } from '$lib/utils/updater';

	async function handleUpdateCheck() {
		await checkForAppUpdates(true);
	}
</script>

<main>
	<h1>Welcome to Tauri and SvelteKit: This is an updated version</h1>
	<p>{message}</p>
	<Button on:click={callRust}>Call Rusty</Button>
	<div class="p-8">
		<Alert>
			<span class="font-medium">Info alert!</span>
			Change a few things up and try submitting again.
		</Alert>
	</div>
	<Button on:click={handleUpdateCheck}>Check update</Button>
</main>
