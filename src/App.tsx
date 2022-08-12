import { useEffect } from "react";
import styles from "./App.module.less";
import { invoke } from "@tauri-apps/api";
import { emit } from "@tauri-apps/api/event";
import { WebviewWindow } from "@tauri-apps/api/window";

function App() {
	useEffect(() => {
		setTimeout(() => {
			invoke("show_main_window");
		}, 2000);
	}, []);
	const handleClick = async () => {
		const ret = await invoke("hello_test", { word: "TRURI" });
		alert(ret);
	};

	const handleBackend = () => {
		emit("backend", { word: "FrontEnd" });
	};

	const handleOpenNewWindow = () => {
		const webview = new WebviewWindow("new-window", {
			url: "https://github.com/tauri-apps/tauri",
		});
		webview.once("tauri://created", function () {
			// webview window successfully created
		});
		webview.once("tauri://error", function (e) {
			// an error happened creating the webview window
		});
	};
	return (
		<div className={styles.box}>
			<h1 onClick={handleClick}> Hello Tauri!</h1>
			<button onClick={handleBackend}>call backend</button>
			<button onClick={handleOpenNewWindow}>open new window</button>
		</div>
	);
}

export default App;
