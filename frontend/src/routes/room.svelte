<script lang='ts'>
	import { onMount } from 'svelte';

	let players = [];
	let roomId;

	onMount(() => {
		const params = new URLSearchParams(window.location.search);
		roomId = params.get('id');
		update();
		setInterval(update, 5000);
	});

	async function update() {
		let status = await fetch(`/api/status/${roomId}`);
		let room = await status.json();
		players = room.players;
		if(room.status !== 'waiting') {
			window.location.href = `/play?id=${roomId}`;
		}
	}

	async function joinGame() {
		let join = await fetch(`/api/join/${roomId}`);
		let room = await join.json();
		players = room.players;
	}

	async function leaveGame() {
		let join = await fetch(`/api/leave/${roomId}`);
		let room = await join.json();
		players = room.players;
	}

	async function startGame() {
		let room = await fetch(`/api/start/${roomId}`);
		if(room.ok) {
			window.location.href = `/play?id=${roomId}`;
		}
	}
</script>

<div>
	<ul>
		{#each players as p}
			<li>{p}</li>
		{/each}
	</ul>
	<div>
		<button on:click={joinGame} class='px-4 py-2 rounded-lg bg-blue-500 text-white font-bold hover:bg-blue-300'
						type='button'>Join
		</button>
		<button on:click={leaveGame} class='px-4 py-2 rounded-lg bg-blue-500 text-white font-bold hover:bg-blue-300'
						type='button'>Leave
		</button>
		<button on:click={startGame} class='px-4 py-2 rounded-lg bg-blue-500 text-white font-bold hover:bg-blue-300'
						type='button'>Start
		</button>
	</div>
</div>
