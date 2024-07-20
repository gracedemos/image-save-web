function findImage() {
	const address = document.getElementById("addressInput").value;
	const imageTitle = document.getElementById("titleInput").value;
	const url = `/find-image?address=${address}&image_title=${imageTitle}`;

	window.location.href = url;
}

function redirect(url) {
	window.location.href = url;
}
