// Thanks, https://stackoverflow.com/a/73871944
function openTarget() {
	const hash = location.hash.substring(1);
	if (hash) {
		console.log(hash);
		const target = document.getElementById(hash);
		if (target) {
			const details = target.closest('details');
			if (details) {
				details.open = true;
				details.scrollIntoView();
			}
		}
	}
}
window.addEventListener('hashchange', openTarget);
document.addEventListener('DOMContentLoaded', openTarget);

function closeAllDetails() {
	Array.from(document.getElementsByTagName("details")).forEach((ele) => ele.open = false);
}
