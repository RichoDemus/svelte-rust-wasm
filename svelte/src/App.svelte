<script>
	import {onMount} from "svelte";
	import {from_wasm, objFromWasm, throwException} from "./call_wasm";

	export let name;
	let fromWasm = "";
	let obj = null;

	onMount( async () => {
		fromWasm = await from_wasm();
		const temp_obj = await objFromWasm();
		console.log("rust structs look weird:", temp_obj);
		console.log("obj.age:", temp_obj.age);
		obj = temp_obj.age;
	});

	async function click() {
		const f = await throwException();
		try {
			f();
		}catch (e) {
			alert("rust code returned Err(): " + e);
		}
	}
</script>

<style>
	h1 {
		color: purple;
	}
</style>

<h1>Hello {name}!</h1>
<span>from was: {fromWasm}</span> <br/>
<span>complex object(struct) from wasm: {obj}</span>
<div>
	<button on:click={click}>Click me!</button>
</div>
