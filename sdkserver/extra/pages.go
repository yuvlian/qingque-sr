package extra

const luaExecutorPage = `<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<title>Lua Executor</title>
	<style>
		body {
			background-color: #1e1e2f;
			color: #f0f0f0;
			font-family: 'Arial';
			margin: 40px;
		}
		h2 {
			color: #ffffff;
			margin-bottom: 20px;
		}
		#editor {
			width: 100%;
			height: 300px;
			background-color: #2c2c3e;
			color: #ffffff;
			border: 1px solid #444;
			border-radius: 6px;
			padding: 12px;
			font-family: monospace;
			font-size: 14px;
			resize: vertical;
		}
		#response {
			white-space: pre-wrap;
			background-color: #1c1c2b;
			color: #c0ffc0;
			border: 1px solid #444;
			border-radius: 6px;
			padding: 12px;
			margin-top: 20px;
			font-family: monospace;
			font-size: 14px;
		}
		button {
			background-color: #3a3a5a;
			color: #ffffff;
			border: none;
			padding: 10px 16px;
			border-radius: 6px;
			cursor: pointer;
			font-size: 14px;
			transition: background-color 0.2s ease;
		}
		button:hover {
			background-color: #505075;
		}
		button:active {
			background-color: #2e2e4d;
		}
		button + button {
			margin-left: 10px;
		}
		#fileInput {
			display: none;
		}
	</style>
</head>
<body>
	<h2>Lua Executor</h2>
	<textarea id="editor" placeholder="Lua code here..."></textarea>

	<div style="margin-top: 20px;">
		<button onclick="triggerFileInput()">📂 Open</button>
		<button onclick="saveCode()">💾 Save</button>
		<button onclick="executeCode()">▶️ Execute</button>
		<button onclick="triggerFileInputExecute()">⚡ Open & Execute</button>
	</div>

	<input type="file" id="fileInput" accept=".lua,.txt" />
	<input type="file" id="fileInputExecute" accept=".lua,.txt" style="display: none;" />

	<pre id="response"></pre>

	<script>
		function triggerFileInput() {
			document.getElementById('fileInput').click();
		}

		document.getElementById('fileInput').addEventListener('change', function () {
			const file = this.files[0];
			if (!file) return;

			const reader = new FileReader();
			reader.onload = function (e) {
				document.getElementById('editor').value = e.target.result;
			};
			reader.readAsText(file);
		});

		function triggerFileInputExecute() {
			document.getElementById('fileInputExecute').click();
		}

		document.getElementById('fileInputExecute').addEventListener('change', function () {
			const file = this.files[0];
			if (!file) return;

			const reader = new FileReader();
			reader.onload = function (e) {
				const code = e.target.result;
				document.getElementById('editor').value = code;

				const payload = "EXECUTE###" + code;
				sendToServer(payload);
			};
			reader.readAsText(file);
		});

		function saveCode() {
			const code = document.getElementById('editor').value;
			const filename = prompt("Enter filename to save (e.g., 'my_script'):");
			if (!filename) return;

			const payload = "SAVE###" + filename + "###" + code;
			sendToServer(payload);
		}

		function executeCode() {
			const code = document.getElementById('editor').value;
			const payload = "EXECUTE###" + code;
			sendToServer(payload);
		}

		function sendToServer(content) {
			fetch('/lua_executor', {
				method: 'POST',
				headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
				body: new URLSearchParams({ data: content })
			})
			.then(res => res.text())
			.then(data => document.getElementById('response').textContent = data)
			.catch(err => document.getElementById('response').textContent = "Error: " + err);
		}
	</script>
</body>
</html>`

const srToolsManagerPage = `<!DOCTYPE html>
<html lang="en">
<head>
	<meta charset="UTF-8">
	<title>SRTools Manager</title>
	<style>
		body {
			background-color: #1e1e2f;
			color: #f0f0f0;
			font-family: 'Arial';
			margin: 40px;
		}
		h2 {
			color: #ffffff;
			margin-bottom: 20px;
		}
		#editor {
			width: 100%;
			height: 300px;
			background-color: #2c2c3e;
			color: #ffffff;
			border: 1px solid #444;
			border-radius: 6px;
			padding: 12px;
			font-family: monospace;
			font-size: 14px;
			resize: vertical;
		}
		#response {
			white-space: pre-wrap;
			background-color: #1c1c2b;
			color: #c0ffc0;
			border: 1px solid #444;
			border-radius: 6px;
			padding: 12px;
			margin-top: 20px;
			font-family: monospace;
			font-size: 14px;
		}
		button {
			background-color: #3a3a5a;
			color: #ffffff;
			border: none;
			padding: 10px 16px;
			border-radius: 6px;
			cursor: pointer;
			font-size: 14px;
			transition: background-color 0.2s ease;
		}
		button:hover {
			background-color: #505075;
		}
		button:active {
			background-color: #2e2e4d;
		}
		button + button {
			margin-left: 10px;
		}
		#fileInput {
			display: none;
		}
	</style>
</head>
<body>
	<h2>SRTools Manager</h2>
	<textarea id="editor" placeholder="JSON config here..."></textarea>

	<div style="margin-top: 20px;">
		<button onclick="triggerFileInput()">📂 Open</button>
		<button onclick="saveCode()">💾 Save</button>
		<button onclick="loadConfig()">📥 Set for Battle</button>
		<button onclick="triggerFileInputSet()">⚔️ Open & Set for Battle</button>
	</div>

	<input type="file" id="fileInput" accept=".json,.txt" />
	<input type="file" id="fileInputSet" accept=".json,.txt" style="display: none;" />

	<pre id="response"></pre>

	<script>
		function triggerFileInput() {
			document.getElementById('fileInput').click();
		}

		function triggerFileInputSet() {
			document.getElementById('fileInputSet').click();
		}

		document.getElementById('fileInput').addEventListener('change', function () {
			const file = this.files[0];
			if (!file) return;

			const reader = new FileReader();
			reader.onload = function (e) {
				document.getElementById('editor').value = e.target.result;
			};
			reader.readAsText(file);
		});

		document.getElementById('fileInputSet').addEventListener('change', function () {
			const file = this.files[0];
			if (!file) return;

			const reader = new FileReader();
			reader.onload = function (e) {
				const content = e.target.result;
				document.getElementById('editor').value = content
				const payload = "LOAD###" + content;
				sendToServer(payload);
			};
			reader.readAsText(file);
		});

		function saveCode() {
			const code = document.getElementById('editor').value;
			const filename = prompt("Enter filename to save (e.g., 'cool_team'):");
			if (!filename) return;

			const payload = "SAVE###" + filename + "###" + code;
			sendToServer(payload);
		}

		function loadConfig() {
			const code = document.getElementById('editor').value;
			const payload = "LOAD###" + code;
			sendToServer(payload);
		}

		function sendToServer(content) {
			fetch('/srtools_manager', {
				method: 'POST',
				headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
				body: new URLSearchParams({ data: content })
			})
			.then(res => res.text())
			.then(data => document.getElementById('response').textContent = data)
			.catch(err => document.getElementById('response').textContent = "Error: " + err);
		}
	</script>
</body>
</html>`
