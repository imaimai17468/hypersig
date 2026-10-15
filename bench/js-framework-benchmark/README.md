# hypersig in js-framework-benchmark

`hypersig/` is an entry for [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark), the keyed table benchmark that compares browser rendering across frameworks. Like the benchmark's Leptos entry, it commits its built output: `index.html` and `bundled-dist/hypersig.js` come from `cargo run --release` in `hypersig/`, and the entry's `build-prod` script does nothing.

## Run it next to other frameworks

```sh
git clone --depth 1 https://github.com/krausest/js-framework-benchmark.git
cd js-framework-benchmark
cp -R ../hypersig/bench/js-framework-benchmark/hypersig frameworks/keyed/hypersig
(cd server && npm ci && npm start &)
(cd webdriver-ts && npm ci && npm run compile)
for f in react-hooks vue solid; do (cd frameworks/keyed/$f && npm ci && npm run build-prod); done
cd webdriver-ts
node dist/isKeyed.js --headless --framework keyed/hypersig
node dist/benchmarkRunner.js --headless --count 10 --framework keyed/hypersig keyed/vanillajs keyed/react-hooks keyed/vue keyed/solid keyed/leptos
```

Results land in `webdriver-ts/results/`. `results.md` here records the last run.
