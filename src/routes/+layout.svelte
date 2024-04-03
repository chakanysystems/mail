<script lang="ts">
	import '../app.pcss';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let publickey: string = '';
	onMount(async () => {
		document.documentElement.classList.add('dark');
		invoke('get_pubkey').then((k) => {
			console.log('key from rust:', k);
		});
	});
</script>

<div>
	<div class="flex gap-3">
		<div class="flex flex-col basis-[50px] h-[100vh]">
			<div>Chaker</div>
			<div class="flex flex-col grow mb-auto">
				<div>Mail</div>
				<div>Messages</div>
				<div>Contacts</div>
			</div>
			<div class="flex mt-auto">
			</div>
		</div>

		<slot />
	</div>
</div>
