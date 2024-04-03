<script lang="ts">
	import '../app.css';
	import { ndk } from '$lib/nostr';
	import Avatar from '../components/Avatar.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	localStorage.debug = 'ndk:*';

	let publickey: string = '';
	onMount(async () => {
		invoke('get_pubkey').then((k) => {
			console.log('HOLY FUCKING SHIT THE KEY!!!!', k);
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
				<Avatar pubkey={publickey} />
			</div>
		</div>

		<slot />
	</div>
</div>
