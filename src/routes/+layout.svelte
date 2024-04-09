<script lang="ts">
	import '../app.pcss';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import Avatar from '$lib/components/Avatar.svelte';
	import Name from '$lib/components/Name.svelte';
	import NostrAddress from '$lib/components/NostrAddress.svelte';

	let publickey: string = '';
	onMount(async () => {
		document.documentElement.classList.add('dark');
		invoke('get_pubkey').then((k) => {
			console.log('key from rust:', k);
		});
	});
</script>

<div>
	<div class="flex gap-3 h-full min-h-screen">
		<div class="flex border-r">
			<aside class="flex flex-col gap-5 h-full max-h-screen">
				<div class="flex font-semibold">
					Mail
				</div>
				<nav class="flex flex-col px-2 gap-3 h-full max-h-screen text-sm">
					<div class="flex flex-col gap-3 grow">
						<a
							href="##"
							class="flex items-center gap-3 rounded-lg bg-muted px-3 py-2 text-primary transition-all hover:text-primary"
						>
							Mail
						</a>
						<a
							href="##"
							class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary"
						>
							Messages
						</a>
					</div>
					<div class="flex-none">
						<div class="flex flex-col gap-6">
							<div class="flex flex-col gap-1">
								<a
									href="##"
									class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary"
								>
									Support
								</a>
								<a
									href="##"
									class="flex items-center gap-3 rounded-lg px-3 py-2 text-muted-foreground transition-all hover:text-primary"
								>
									Settings
								</a>
							</div>
							<hr />
							<div class="flex gap-3 pb-6 px-4">
								<Avatar />
								<div>
									<div>
										<Name />
									</div>
									<div>
										<NostrAddress />
									</div>
								</div>
							</div>
						</div>
					</div>
				</nav>
			</aside>
		</div>

		<slot />
	</div>
</div>
