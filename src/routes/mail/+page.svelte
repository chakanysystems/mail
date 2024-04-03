<script lang="ts">
	import { NDKEvent, NDKNip07Signer, NDKPrivateKeySigner } from '@nostr-dev-kit/ndk';
	import { generateSecretKey, getPublicKey, nip19 } from 'nostr-tools';
	import { browser } from '$app/environment';
	import { ndk } from '$lib/nostr';
	import { convertToUint8Array } from '$lib/index';
	import { createRumor, createSeal, createWrap, nip44Decrypt, nip44Encrypt } from '$lib/encrypt';
	import { bytesToHex } from '@noble/hashes/utils';

	let ourPrivateKey: Uint8Array;

	$: {
		load();
	}

	const temp = generateSecretKey();
	console.log(temp.toString(), getPublicKey(temp));

	let keyEvents: NDKEvent[] = [];
	let mailEvents: NDKEvent[] = [];

	interface Contact {
		actualKey: string;
		theirEphermeralKey: string;
		ourEphemeralKey: Uint8Array;
	}

	let contacts: Contact[] = [];

	function setupDatabase() {
		// check if absent
		const convoGet = localStorage.getItem('contacts');
		const privkey = localStorage.getItem('privkey');
		if (!convoGet) localStorage.setItem('contacts', JSON.stringify([]));
		if (!privkey) localStorage.setItem('privkey', generateSecretKey().toString());
		contacts = JSON.parse(JSON.parse(localStorage.getItem('contacts') || '[]'));
		console.log(contacts);
		for (let i = 0; contacts.length > i; i++) {
			contacts[i].ourEphemeralKey = Uint8Array.from(
				Array.from<number>(
					// @ts-ignore
					contacts[i].ourEphemeralKey.split(',').map((v) => parseInt(v))
				)
			);
		}
		ourPrivateKey = localStorage
			.getItem('privkey')!
			.split(',')
			.map((v) => parseInt(v));
		console.log(contacts);
	}

	async function load() {
		setupDatabase();
		await $ndk.connect();
		const contactKeysToSubTo: string[] = [];
		for (let i = 0; contacts.length > i; i++) {
			contactKeysToSubTo.push(getPublicKey(contacts[i].ourEphemeralKey));
		}

		const mailEvs = await $ndk.fetchEvents([
			{
				kinds: [1059],
				'#p': contactKeysToSubTo
			}
		]);

		mailEvs.forEach((e) => {
			try {
				const unwrappedSeal = nip44Decrypt(
					e.rawEvent() as any,
					contacts.find((c) => c.theirEphermeralKey === e.pubkey)!.ourEphemeralKey
				);
				const unsealedRumor = nip44Decrypt(unwrappedSeal, ourPrivateKey);
				console.log(unsealedRumor, 'FUCK');
				mailEvents.push(new NDKEvent($ndk, unsealedRumor));
				mailEvents = mailEvents;
			} catch (e) {
				console.log(e);
			}
		});
	}

	async function postMailMessage() {
		const toKeys = to.split(' ');
		const ccKeys = cc.split(' ');

		let toTags: string[][] = [];
		for (let i = 0; toKeys.length > i; i++) {
			toTags.push(['p', toKeys[i], 'to']);
		}
		let ccTags: string[][] = [];
		for (let i = 0; ccKeys.length > i; i++) {
			ccTags.push(['p', ccKeys[i], 'cc']);
		}

		const rumor = createRumor(
			{
				kind: 2024,
				tags: [...toTags, ...ccTags],
				content: body
			},
			getPublicKey(ourPrivateKey)
		);

		if (subject) rumor.tags.push(['subject', subject]);

		for (let i = 0; toKeys.length > i; i++) {
			const keyContact = contacts.find((c) => toKeys[i]);
			if (keyContact) {
				const seal = createSeal(rumor, ourPrivateKey, keyContact.actualKey);
				console.log(keyContact.theirEphermeralKey);
				const newEv = new NDKEvent($ndk);
				newEv.tags = [['p', keyContact.theirEphermeralKey]];
				newEv.kind = 1059;
				newEv.content = nip44Encrypt(
					seal,
					keyContact.ourEphemeralKey,
					keyContact.theirEphermeralKey
				);
				newEv.created_at = Math.round(
					Math.round(Date.now() / 1000) - Math.random() * (2 * 24 * 60 * 60)
				);
				console.log(newEv, getPublicKey(keyContact.ourEphemeralKey));
				try {
					await newEv.sign(new NDKPrivateKeySigner(bytesToHex(keyContact.ourEphemeralKey)));
					await newEv.publish();
				} catch (error) {
					console.error(error);
				}
			}
		}

		for (let i = 0; toKeys.length > i; i++) {
			const keyContact = contacts.find((c) => toKeys[i]);
			if (keyContact) {
				const seal = createSeal(rumor, ourPrivateKey, keyContact.actualKey);
				const wrap = createWrap(seal, keyContact.ourEphemeralKey, keyContact.theirEphermeralKey);
				await new NDKEvent($ndk, wrap).publish();
			}
		}

		// Recipient unwraps with his/her private key.

		//const unwrappedSeal = nip44Decrypt(wrap, recipientPrivateKey);
		//const unsealedRumor = nip44Decrypt(unwrappedSeal, recipientPrivateKey);
	}

	let privkeyToLoginTo = '';
	function login() {
		localStorage.setItem('privkey', JSON.stringify(convertToUint8Array(privkeyToLoginTo)));
	}

	let showCompose = false;
	let body = '';
	let subject = '';
	let to = '';
	let cc = '';

	let selectedMail: NDKEvent;
</script>

<div class="flex flex-col">
	<div class="flex gap-5">
		<button on:click={() => (showCompose = true)}>Compose</button>
		{#if ourPrivateKey}
			<div>
				Logged in as: {getPublicKey(ourPrivateKey)}
			</div>
		{/if}
	</div>
	{#if showCompose}
		<div class="flex flex-col">
			To: <input bind:value={to} type="text" />
			Cc: <input bind:value={cc} type="text" />
			Bcc: <input type="text" />
			Subject: <input type="text" bind:value={subject} />
			<textarea bind:value={body} />
			<button on:click={postMailMessage}>Send</button>
		</div>
	{/if}
	<div class="flex gap-3">
		<div>
			people
			<ul>
				{#each contacts as contact}
					<li>{contact.actualKey}</li>
				{/each}
			</ul>
		</div>
		<div>
			<div>Mail Requests</div>
			<div>
				Recent
				<div class="flex flex-col gap-3 text-left">
					{#each Array.from(mailEvents).sort((a, b) => b.created_at - a.created_at) as ev}
						<button on:click={() => (selectedMail = ev)}
							>{ev.getMatchingTags('subject')[0][1]}</button
						>
					{/each}
				</div>
			</div>
		</div>
		<div>
			{#if selectedMail}
				From: {selectedMail.pubkey}
				{selectedMail.getMatchingTags('subject')[0][1]}
				{selectedMail.content}
			{/if}
		</div>
	</div>
</div>
