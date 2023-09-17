init: install-wasm npm-init
build: cargo-build wasm-build npm-build
ci-build: init build
clean: clean-wasm clean-vue

install-wasm:
		curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

cargo-build:
		cd decryption-mlm-wasm && cargo build

wasm-build:
		cd decryption-mlm-wasm && wasm-pack build --target web --out-dir ../decryption-mlm-vue3/pkg

npm-init:
		cd decryption-mlm-vue3 && npm install

npm-build:
		cd decryption-mlm-vue3 && npm run build

clean-wasm:
		cd decryption-mlm-wasm && rm -rf pkg && rm -rf target

clean-vue:
		cd decryption-mlm-vue3 && rm -rf pkg && rm -rf target