<script lang="ts">
	let message = 'Click the button to run Rust backend!';
	import { Alert } from 'flowbite-svelte';
	import { Button } from 'flowbite-svelte';

	async function callRust() {
		const { invoke } = await import('@tauri-apps/api/core');
		message = await invoke('greet', { name: 'User' });
	}
	async function testDownload() {
		const { invoke } = await import('@tauri-apps/api/core');
		const url =
			'https://github.com/kfilip10/tauri-update-app/releases/download/v0.0.2/tauri-updater_0.0.2_x64_en-US.msi.zip';
		try {
			await invoke('download_file', { url });
			console.log('Download successful!');
		} catch (error) {
			console.error('Download error:', error);
		}
	}
	import { getCurrentVersion, checkForAppUpdates, displayVersionInfo } from '$lib/utils/updater';
	let currentVersion = 'Loading...';

	async function handleUpdateCheck() {
		await checkForAppUpdates(true);
	}
	async function showVersionInfo() {
		const info = await displayVersionInfo();
		alert(info);
	}
</script>

<main>
	<h1>Welcome to Tauri and SvelteKit: This is an updated version v2</h1>
	<p>{message}</p>
	<Button on:click={callRust}>Call Rusty</Button>
	<div class="p-8">
		<Alert>
			<span class="font-medium">Info alert!</span>
			Change a few things up and try submitting again.
		</Alert>
	</div>
	<Button on:click={handleUpdateCheck}>Check for Updates</Button>
	<Button on:click={showVersionInfo}>Version Info</Button>

	<Button on:click={testDownload}>Manual download</Button>
</main>
