import init, { svg_to_png } from "./pkg/wasm.js";

init();

function getSpacing(str) {
	switch (str.length) {
		case 2:
			return 1200;
		case 3:
			return 500;
		case 4:
			return 100;
		default:
			return 0;
	}
}

function getTransformString(str, width) {
	if (str.length < 4) {
		const bairitsu = 1.15;
		return `translate(-${width * (bairitsu-1) * .5} 0) scale(${bairitsu} 1.0)`;
	} else if (str.length == 4) {
		const bairitsu = 1.05;
		return `translate(-${width * (bairitsu-1) * .5} 0) scale(${bairitsu} 1.0)`;
	}

	const size = 1060;
	const scale = (width * 0.9) / (size * str.length);
	
	return `translate(${-width * (scale-1) * .5}) scale(${scale} 1.0)`;
}

export function getSvgString(destJa, destEn, backgroundColor, foregroundColor) {
	const width = 5600;
	const height = 1600;
	const destJaSpacing = getSpacing(destJa);
	const borderString = `stroke-width="30" stroke="${foregroundColor}"`;

	return `<svg xmlns="http://www.w3.org/2000/svg" version="1.0" width="${width}" height="${height}" viewBox="0 0 ${width} ${height}">
<rect 
	x="0" y="0" 
	width="${width}" height="${height}" 
	fill="${backgroundColor}"></rect>
<text 
	font-family="Kosugi Maru" 
	text-anchor="middle" font-size="1060" 
	x="${width/2}" y="1050" 
	fill="${foregroundColor}" 
	letter-spacing="${destJaSpacing}"
	transform="${getTransformString(destJa, width)}"
	${borderString}>${destJa}</text>
<text font-family="Open Sans" text-anchor="middle" x="${width/2}" y="1420" font-size="300" fill="${foregroundColor}">${destEn}</text>
</svg>
	`;
}
function getSvgStringWeb() {
	return getSvgString(
		document.querySelector("#input-dest-ja").value,
		document.querySelector("#input-dest-en").value,
		document.querySelector("#input-bg-color").value,
		document.querySelector("#input-fg-color").value
	)
}

function uint8ArrayToBase64(uint8Array) {
	const decodeBinaryString = uint8Array => uint8Array.reduce(
		(binaryString, uint8) => binaryString + String.fromCharCode(uint8),
		'',
	);
	const binaryStringA = decodeBinaryString(uint8Array);
	const base64 = btoa(binaryStringA);
	
	return base64;
}

function getSvgUrl() {
	const svgString = getSvgStringWeb();
	console.log(svgString);
	const svgPng = svg_to_png(svgString);
	return `data:image/png;base64,${uint8ArrayToBase64(svgPng)}`;
}

document.querySelector("#input-generate").addEventListener('click', v => {
	const svgUrl = getSvgUrl();
	document.querySelector("#export-image").src = svgUrl;
});