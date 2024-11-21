const esbuild = require("esbuild");

const production = process.argv.includes('--production');

async function main() {
	const ctx = await esbuild.context({
		entryPoints: [
			'src/index.ts'
		],
		bundle: true,
		format: 'cjs',
		minify: production,
		sourcemap: !production,
		sourcesContent: false,
		platform: 'node',
		outfile: '../../dist/client.js',
		external: ['vscode'],
		logLevel: 'silent',
	});
	await ctx.rebuild();
	await ctx.dispose();
}

main().catch(e => {
	console.error(e);
	process.exit(1);
});
