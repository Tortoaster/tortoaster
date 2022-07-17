<script lang='ts'>
	async function newGame() {
		let room = await fetch('/api/new');
		let roomId = await room.json();
		window.location.href = `/room?id=${roomId}`;
	}
</script>

<button on:click={newGame} class='px-4 py-2 rounded-lg bg-blue-500 text-white font-bold hover:bg-blue-300'
				type='button'>New Game
</button>
