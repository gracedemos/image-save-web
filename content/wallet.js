const PROGRAM_ID = "A3dGa9KV1NUrVyTWrZRAF3QJqBohi2vmipmfQfXc2ww1";

function getProvider() {
	if ("phantom" in window) {
		const provider = window.phantom?.solana;

		if (provider?.isPhantom) {
			return provider;
		}
	}

	window.open("https://phantom.app", "_blank");
}

async function connectPhantom() {
	const provider = getProvider();
	const walletAddress = document.getElementById("walletAddress");

	try {
		const resp = await provider.connect();
		walletAddress.innerHTML = resp.publicKey.toString();
	} catch (err) {
		alert(err);
	}
}

async function addImage() {
	const url = document.getElementById("urlInput").value;
	const title = document.getElementById("titleInput").value;
	const image = {
		url: url,
		title: title
	};

	const signer = document.getElementById("walletAddress").innerHTML;

	fetch("/get-data-and-pda", {
		method: "POST",
		body: JSON.stringify({
			signer: signer,
			image: image
		}),
		headers: {
			"Content-type": "application/json"
		}
	}).then(async (response) => {
		const provider = getProvider();
		const dataAndPDA = await response.json();
		const data = dataAndPDA.data;
		const pda = dataAndPDA.pda;
		const signerPubkey = new solanaWeb3.PublicKey(signer);
		const pdaPubkey = new solanaWeb3.PublicKey(pda);

		let transaction = new solanaWeb3.Transaction();

		const instruction = new solanaWeb3.TransactionInstruction({
			programId: new solanaWeb3.PublicKey(PROGRAM_ID),
			keys: [
				{
					pubkey: signerPubkey,
					isSigner: true,
					isWritable: false
				},
				{
					pubkey: pdaPubkey,
					isSigner: false,
					isWritable: true
				},
				{
					pubkey: solanaWeb3.SystemProgram.programId,
					isSigner: false,
					isWritable: false
				}
			],
			data: data
		});

		transaction.add(instruction);

		const connection = new solanaWeb3.Connection("http://localhost:8899");
		const blockhash = (await connection.getLatestBlockhash("finalized")).blockhash;

		transaction.recentBlockhash = blockhash;
		transaction.feePayer = signerPubkey;

		const signedTransaction = await provider.signTransaction(transaction);
		const signature = await connection.sendRawTransaction(signedTransaction.serialize());

		alert(signature);
	});
}

document.getElementById("walletButton").addEventListener("click", async () => {
	await connectPhantom();
});

document.getElementById("addImageButton").addEventListener("click", async () => {
	await addImage();
});
