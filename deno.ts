import { getSvgString } from "./script.js";
import { svg_to_png } from "./wasm/pkg/wasm.js";

type Station = {
	number: string,
	ja: string,
	en: string,
	fg: string,
	bg: string,
};

const stations: Station[] = Deno.readTextFileSync("./type.tsv")
	.split('\n')
	.map(v => v.split("\t"))
	.map(v => {return {number: v[0], ja: v[1], en: v[3], bg: v[4] || "#222", fg: v[5] || "#fff"}})

for (const sta of stations) {
	const str = getSvgString(sta.ja, sta.en, sta.bg, sta.fg);
	console.log(str);
	const png = svg_to_png(str);
	Deno.writeFileSync(`output/type-${sta.number}_${sta.ja}.png`, png);
}