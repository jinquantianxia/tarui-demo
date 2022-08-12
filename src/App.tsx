import { useState } from "react";
import styles from "./App.module.less";
import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";

function App() {
	const handleClick = async () => {
		const ret = await invoke("hello_test", { word: "TRURI" });
		alert(ret);
	};

	const handleBackend = () => {
		emit("backend", { word: "FrontEnd" });
	};
	return (
		<div className={styles.box}>
			<h1 onClick={handleClick}> Hello Tauri!</h1>
			<button onClick={handleBackend}>call backend</button>
		</div>
	);
}

export default App;
