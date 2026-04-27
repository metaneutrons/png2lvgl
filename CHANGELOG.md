# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.5](https://github.com/metaneutrons/png2lvgl/compare/v0.3.4...v0.3.5) (2026-04-27)


### Bug Fixes

* **ci:** bump github-actions-deploy-aur to v4.1.3 ([c00d399](https://github.com/metaneutrons/png2lvgl/commit/c00d399e9f71067dd42ec9eb3514243462d0908a))

## [0.3.4](https://github.com/metaneutrons/png2lvgl/compare/v0.3.3...v0.3.4) (2026-04-26)


### Features

* add --stdout option to output C array to stdout ([2807e55](https://github.com/metaneutrons/png2lvgl/commit/2807e55967568194af82db425d49943142fa9fa5))
* add ARM64 Linux builds and deb packages ([c26fadc](https://github.com/metaneutrons/png2lvgl/commit/c26fadc61f8376b370a1dbef78b7f49c8484ed55))
* add AUR package publishing ([d0bed01](https://github.com/metaneutrons/png2lvgl/commit/d0bed011ac89dbd6772e154b05ab46d998df8924))
* add automated crates.io publishing to release workflow ([f720544](https://github.com/metaneutrons/png2lvgl/commit/f7205441631d4ed62d4e8b181decd60081ba766e))
* add Debian package support ([8b181cf](https://github.com/metaneutrons/png2lvgl/commit/8b181cf83b50c8ef592139fff9aedfd83764c794))
* add enterprise-grade CI/CD workflows ([d8e29a8](https://github.com/metaneutrons/png2lvgl/commit/d8e29a8a29603ee5b1624413e2afc57f0326ca8b))
* add enterprise-grade error handling, validation, and logging ([7434ebe](https://github.com/metaneutrons/png2lvgl/commit/7434ebeedd7e10790a4ba99e3d3f18fe29214669))
* add enterprise-grade project infrastructure ([f7ad911](https://github.com/metaneutrons/png2lvgl/commit/f7ad9116f3d753fe9f59edf75a2876a1fde22038))
* add LVGL 8.x and 9.x API compatibility ([139c52d](https://github.com/metaneutrons/png2lvgl/commit/139c52d0760e5205f09b1362be7bdc03cd3b26bc))
* add LVGL target version to generated file header ([8af3d65](https://github.com/metaneutrons/png2lvgl/commit/8af3d65d05da1f7f1ba8e19b4cf2ef596725b27f))
* add LVGL v9 support and improve generated file headers ([a8cf205](https://github.com/metaneutrons/png2lvgl/commit/a8cf20508e2f78b10f2079c0f5647c5836d74c4d))
* add RGB565 endianness comment to generated C files ([e20d589](https://github.com/metaneutrons/png2lvgl/commit/e20d589341d3bdc96f1322af8fada1f1ea26b533))
* add Windows ARM64 build ([c589ce7](https://github.com/metaneutrons/png2lvgl/commit/c589ce7900aa55e3bad4e993a873297a98246135))
* auto-generate enterprise-grade manpage with clap_mangen ([1bed430](https://github.com/metaneutrons/png2lvgl/commit/1bed4300115aba4256633f0979d3d90ce4cf9bbc))
* automate CHANGELOG.md generation with git-cliff ([7ca9559](https://github.com/metaneutrons/png2lvgl/commit/7ca955925aecf5d8e5b2f2851c87c18168423cf1))
* implement all missing image formats and add big-endian support ([e9354ed](https://github.com/metaneutrons/png2lvgl/commit/e9354edb7d6e0659dbe7e724d09126c25add0b8c))
* improve release notes with auto-generation from commits ([5a9dced](https://github.com/metaneutrons/png2lvgl/commit/5a9dcedc416743bcaf29ab7a49d24ae67f2d46f0))
* include man page in release archives ([0d9641a](https://github.com/metaneutrons/png2lvgl/commit/0d9641adabed302c30f51d64476e06e71274fa77))
* initial implementation of png2lvgl converter ([155a8bd](https://github.com/metaneutrons/png2lvgl/commit/155a8bd02eab283b1231911b7b6fde8edf088913))


### Bug Fixes

* add --allow-dirty flag to cargo publish ([89b9bee](https://github.com/metaneutrons/png2lvgl/commit/89b9bee98496cd9e958289db981108a3a12b7652))
* add --allow-dirty flag to cargo publish ([80275e1](https://github.com/metaneutrons/png2lvgl/commit/80275e1bbf18a4d653cd24942584200065d5fed7))
* add clippy allow for too_many_arguments ([0826bb3](https://github.com/metaneutrons/png2lvgl/commit/0826bb3d8d637556e7a88d0c0b81a7822fee3853))
* add linker configuration for ARM64 Linux cross-compilation ([d73f5c2](https://github.com/metaneutrons/png2lvgl/commit/d73f5c2c8d6878271c17791fd8568a3e582321d0))
* build Debian packages before release and include in assets ([5b1cc65](https://github.com/metaneutrons/png2lvgl/commit/5b1cc6579551dbaa6660937df87391ea01dc5858))
* **ci:** bump MSRV to 1.88, fix release-please tag prefix ([222d7b9](https://github.com/metaneutrons/png2lvgl/commit/222d7b904410df7db208737baa743e90693721ee))
* correct publish-crates job dependencies in release workflow ([4f51ce4](https://github.com/metaneutrons/png2lvgl/commit/4f51ce4b1cbd8784c5f0226cd3fc17bdf8482c4e))
* create Formula directory in homebrew tap update ([b30b3c0](https://github.com/metaneutrons/png2lvgl/commit/b30b3c06c2bdbd80745b49d1aaec7dc475980a66))
* implement bit depth validation for alpha formats ([964889f](https://github.com/metaneutrons/png2lvgl/commit/964889f3eb7695d37cd43809b4a5a93a9953722e))
* include .deb files in build artifacts ([68ccbac](https://github.com/metaneutrons/png2lvgl/commit/68ccbac53375df2996094da2a9fe6e0f22623d70))
* include manpage in Homebrew formula ([3e1874d](https://github.com/metaneutrons/png2lvgl/commit/3e1874d02c3660aadfed626d9a46567f7bc1f86b))
* remove duplicate line in upload-artifact path ([5b7d40d](https://github.com/metaneutrons/png2lvgl/commit/5b7d40d926d095a80f9613826cd0c3d365032644))
* remove duplicate target key and fix shellcheck issues ([fb557b0](https://github.com/metaneutrons/png2lvgl/commit/fb557b08d0bd14fcd566a5df0a8b41b054d0dbf8))
* remove unmaintained paste dependency ([003d0d6](https://github.com/metaneutrons/png2lvgl/commit/003d0d62dc4fd08a2bdfe330ac3b3261d68a9051))
* remove unused imports ([8a37b0f](https://github.com/metaneutrons/png2lvgl/commit/8a37b0f787ed4d11727466e61549f78cb2852782))
* skip tag creation if tag already exists in manual trigger ([3c8d9b3](https://github.com/metaneutrons/png2lvgl/commit/3c8d9b3f3d027c0deb637c941a2287fede08a164))
* update Cargo.toml version in release workflow before publishing ([672c2d1](https://github.com/metaneutrons/png2lvgl/commit/672c2d1cb3abdc2587e1bb4fe0fd3362ff81b532))
* update deb package to use generated manpage ([5abf335](https://github.com/metaneutrons/png2lvgl/commit/5abf3355a75ddc9d300ef18a5722bcd23253208f))


### Performance Improvements

* optimize release workflow by building deb in main matrix ([6c6d0af](https://github.com/metaneutrons/png2lvgl/commit/6c6d0afcff0ccbdacb27d18a00f473bde6300df2))
* optimize version update to run once before builds ([8738031](https://github.com/metaneutrons/png2lvgl/commit/873803130405b0c236886c21d16cbe33bbf77f34))

## [0.3.3](https://github.com/metaneutrons/png2lvgl/compare/v0.3.2...v0.3.3) (2026-04-26)


### Features

* add --stdout option to output C array to stdout ([2807e55](https://github.com/metaneutrons/png2lvgl/commit/2807e55967568194af82db425d49943142fa9fa5))
* add ARM64 Linux builds and deb packages ([c26fadc](https://github.com/metaneutrons/png2lvgl/commit/c26fadc61f8376b370a1dbef78b7f49c8484ed55))
* add AUR package publishing ([d0bed01](https://github.com/metaneutrons/png2lvgl/commit/d0bed011ac89dbd6772e154b05ab46d998df8924))
* add automated crates.io publishing to release workflow ([f720544](https://github.com/metaneutrons/png2lvgl/commit/f7205441631d4ed62d4e8b181decd60081ba766e))
* add Debian package support ([8b181cf](https://github.com/metaneutrons/png2lvgl/commit/8b181cf83b50c8ef592139fff9aedfd83764c794))
* add enterprise-grade CI/CD workflows ([d8e29a8](https://github.com/metaneutrons/png2lvgl/commit/d8e29a8a29603ee5b1624413e2afc57f0326ca8b))
* add enterprise-grade error handling, validation, and logging ([7434ebe](https://github.com/metaneutrons/png2lvgl/commit/7434ebeedd7e10790a4ba99e3d3f18fe29214669))
* add enterprise-grade project infrastructure ([f7ad911](https://github.com/metaneutrons/png2lvgl/commit/f7ad9116f3d753fe9f59edf75a2876a1fde22038))
* add LVGL 8.x and 9.x API compatibility ([139c52d](https://github.com/metaneutrons/png2lvgl/commit/139c52d0760e5205f09b1362be7bdc03cd3b26bc))
* add LVGL target version to generated file header ([8af3d65](https://github.com/metaneutrons/png2lvgl/commit/8af3d65d05da1f7f1ba8e19b4cf2ef596725b27f))
* add LVGL v9 support and improve generated file headers ([a8cf205](https://github.com/metaneutrons/png2lvgl/commit/a8cf20508e2f78b10f2079c0f5647c5836d74c4d))
* add RGB565 endianness comment to generated C files ([e20d589](https://github.com/metaneutrons/png2lvgl/commit/e20d589341d3bdc96f1322af8fada1f1ea26b533))
* add Windows ARM64 build ([c589ce7](https://github.com/metaneutrons/png2lvgl/commit/c589ce7900aa55e3bad4e993a873297a98246135))
* auto-generate enterprise-grade manpage with clap_mangen ([1bed430](https://github.com/metaneutrons/png2lvgl/commit/1bed4300115aba4256633f0979d3d90ce4cf9bbc))
* automate CHANGELOG.md generation with git-cliff ([7ca9559](https://github.com/metaneutrons/png2lvgl/commit/7ca955925aecf5d8e5b2f2851c87c18168423cf1))
* implement all missing image formats and add big-endian support ([e9354ed](https://github.com/metaneutrons/png2lvgl/commit/e9354edb7d6e0659dbe7e724d09126c25add0b8c))
* improve release notes with auto-generation from commits ([5a9dced](https://github.com/metaneutrons/png2lvgl/commit/5a9dcedc416743bcaf29ab7a49d24ae67f2d46f0))
* include man page in release archives ([0d9641a](https://github.com/metaneutrons/png2lvgl/commit/0d9641adabed302c30f51d64476e06e71274fa77))
* initial implementation of png2lvgl converter ([155a8bd](https://github.com/metaneutrons/png2lvgl/commit/155a8bd02eab283b1231911b7b6fde8edf088913))


### Bug Fixes

* add --allow-dirty flag to cargo publish ([89b9bee](https://github.com/metaneutrons/png2lvgl/commit/89b9bee98496cd9e958289db981108a3a12b7652))
* add --allow-dirty flag to cargo publish ([80275e1](https://github.com/metaneutrons/png2lvgl/commit/80275e1bbf18a4d653cd24942584200065d5fed7))
* add clippy allow for too_many_arguments ([0826bb3](https://github.com/metaneutrons/png2lvgl/commit/0826bb3d8d637556e7a88d0c0b81a7822fee3853))
* add linker configuration for ARM64 Linux cross-compilation ([d73f5c2](https://github.com/metaneutrons/png2lvgl/commit/d73f5c2c8d6878271c17791fd8568a3e582321d0))
* build Debian packages before release and include in assets ([5b1cc65](https://github.com/metaneutrons/png2lvgl/commit/5b1cc6579551dbaa6660937df87391ea01dc5858))
* **ci:** bump MSRV to 1.88, fix release-please tag prefix ([222d7b9](https://github.com/metaneutrons/png2lvgl/commit/222d7b904410df7db208737baa743e90693721ee))
* correct publish-crates job dependencies in release workflow ([4f51ce4](https://github.com/metaneutrons/png2lvgl/commit/4f51ce4b1cbd8784c5f0226cd3fc17bdf8482c4e))
* create Formula directory in homebrew tap update ([b30b3c0](https://github.com/metaneutrons/png2lvgl/commit/b30b3c06c2bdbd80745b49d1aaec7dc475980a66))
* implement bit depth validation for alpha formats ([964889f](https://github.com/metaneutrons/png2lvgl/commit/964889f3eb7695d37cd43809b4a5a93a9953722e))
* include .deb files in build artifacts ([68ccbac](https://github.com/metaneutrons/png2lvgl/commit/68ccbac53375df2996094da2a9fe6e0f22623d70))
* include manpage in Homebrew formula ([3e1874d](https://github.com/metaneutrons/png2lvgl/commit/3e1874d02c3660aadfed626d9a46567f7bc1f86b))
* remove duplicate line in upload-artifact path ([5b7d40d](https://github.com/metaneutrons/png2lvgl/commit/5b7d40d926d095a80f9613826cd0c3d365032644))
* remove duplicate target key and fix shellcheck issues ([fb557b0](https://github.com/metaneutrons/png2lvgl/commit/fb557b08d0bd14fcd566a5df0a8b41b054d0dbf8))
* remove unmaintained paste dependency ([003d0d6](https://github.com/metaneutrons/png2lvgl/commit/003d0d62dc4fd08a2bdfe330ac3b3261d68a9051))
* remove unused imports ([8a37b0f](https://github.com/metaneutrons/png2lvgl/commit/8a37b0f787ed4d11727466e61549f78cb2852782))
* skip tag creation if tag already exists in manual trigger ([3c8d9b3](https://github.com/metaneutrons/png2lvgl/commit/3c8d9b3f3d027c0deb637c941a2287fede08a164))
* update Cargo.toml version in release workflow before publishing ([672c2d1](https://github.com/metaneutrons/png2lvgl/commit/672c2d1cb3abdc2587e1bb4fe0fd3362ff81b532))
* update deb package to use generated manpage ([5abf335](https://github.com/metaneutrons/png2lvgl/commit/5abf3355a75ddc9d300ef18a5722bcd23253208f))


### Performance Improvements

* optimize release workflow by building deb in main matrix ([6c6d0af](https://github.com/metaneutrons/png2lvgl/commit/6c6d0afcff0ccbdacb27d18a00f473bde6300df2))
* optimize version update to run once before builds ([8738031](https://github.com/metaneutrons/png2lvgl/commit/873803130405b0c236886c21d16cbe33bbf77f34))

## [0.3.2](https://github.com/metaneutrons/png2lvgl/compare/png2lvgl-v0.3.1...png2lvgl-v0.3.2) (2026-04-26)


### Features

* add --stdout option to output C array to stdout ([2807e55](https://github.com/metaneutrons/png2lvgl/commit/2807e55967568194af82db425d49943142fa9fa5))
* add ARM64 Linux builds and deb packages ([c26fadc](https://github.com/metaneutrons/png2lvgl/commit/c26fadc61f8376b370a1dbef78b7f49c8484ed55))
* add AUR package publishing ([d0bed01](https://github.com/metaneutrons/png2lvgl/commit/d0bed011ac89dbd6772e154b05ab46d998df8924))
* add automated crates.io publishing to release workflow ([f720544](https://github.com/metaneutrons/png2lvgl/commit/f7205441631d4ed62d4e8b181decd60081ba766e))
* add Debian package support ([8b181cf](https://github.com/metaneutrons/png2lvgl/commit/8b181cf83b50c8ef592139fff9aedfd83764c794))
* add enterprise-grade CI/CD workflows ([d8e29a8](https://github.com/metaneutrons/png2lvgl/commit/d8e29a8a29603ee5b1624413e2afc57f0326ca8b))
* add enterprise-grade error handling, validation, and logging ([7434ebe](https://github.com/metaneutrons/png2lvgl/commit/7434ebeedd7e10790a4ba99e3d3f18fe29214669))
* add enterprise-grade project infrastructure ([f7ad911](https://github.com/metaneutrons/png2lvgl/commit/f7ad9116f3d753fe9f59edf75a2876a1fde22038))
* add LVGL 8.x and 9.x API compatibility ([139c52d](https://github.com/metaneutrons/png2lvgl/commit/139c52d0760e5205f09b1362be7bdc03cd3b26bc))
* add LVGL target version to generated file header ([8af3d65](https://github.com/metaneutrons/png2lvgl/commit/8af3d65d05da1f7f1ba8e19b4cf2ef596725b27f))
* add LVGL v9 support and improve generated file headers ([a8cf205](https://github.com/metaneutrons/png2lvgl/commit/a8cf20508e2f78b10f2079c0f5647c5836d74c4d))
* add RGB565 endianness comment to generated C files ([e20d589](https://github.com/metaneutrons/png2lvgl/commit/e20d589341d3bdc96f1322af8fada1f1ea26b533))
* add Windows ARM64 build ([c589ce7](https://github.com/metaneutrons/png2lvgl/commit/c589ce7900aa55e3bad4e993a873297a98246135))
* auto-generate enterprise-grade manpage with clap_mangen ([1bed430](https://github.com/metaneutrons/png2lvgl/commit/1bed4300115aba4256633f0979d3d90ce4cf9bbc))
* automate CHANGELOG.md generation with git-cliff ([7ca9559](https://github.com/metaneutrons/png2lvgl/commit/7ca955925aecf5d8e5b2f2851c87c18168423cf1))
* implement all missing image formats and add big-endian support ([e9354ed](https://github.com/metaneutrons/png2lvgl/commit/e9354edb7d6e0659dbe7e724d09126c25add0b8c))
* improve release notes with auto-generation from commits ([5a9dced](https://github.com/metaneutrons/png2lvgl/commit/5a9dcedc416743bcaf29ab7a49d24ae67f2d46f0))
* include man page in release archives ([0d9641a](https://github.com/metaneutrons/png2lvgl/commit/0d9641adabed302c30f51d64476e06e71274fa77))
* initial implementation of png2lvgl converter ([155a8bd](https://github.com/metaneutrons/png2lvgl/commit/155a8bd02eab283b1231911b7b6fde8edf088913))


### Bug Fixes

* add --allow-dirty flag to cargo publish ([89b9bee](https://github.com/metaneutrons/png2lvgl/commit/89b9bee98496cd9e958289db981108a3a12b7652))
* add --allow-dirty flag to cargo publish ([80275e1](https://github.com/metaneutrons/png2lvgl/commit/80275e1bbf18a4d653cd24942584200065d5fed7))
* add clippy allow for too_many_arguments ([0826bb3](https://github.com/metaneutrons/png2lvgl/commit/0826bb3d8d637556e7a88d0c0b81a7822fee3853))
* add linker configuration for ARM64 Linux cross-compilation ([d73f5c2](https://github.com/metaneutrons/png2lvgl/commit/d73f5c2c8d6878271c17791fd8568a3e582321d0))
* build Debian packages before release and include in assets ([5b1cc65](https://github.com/metaneutrons/png2lvgl/commit/5b1cc6579551dbaa6660937df87391ea01dc5858))
* correct publish-crates job dependencies in release workflow ([4f51ce4](https://github.com/metaneutrons/png2lvgl/commit/4f51ce4b1cbd8784c5f0226cd3fc17bdf8482c4e))
* create Formula directory in homebrew tap update ([b30b3c0](https://github.com/metaneutrons/png2lvgl/commit/b30b3c06c2bdbd80745b49d1aaec7dc475980a66))
* implement bit depth validation for alpha formats ([964889f](https://github.com/metaneutrons/png2lvgl/commit/964889f3eb7695d37cd43809b4a5a93a9953722e))
* include .deb files in build artifacts ([68ccbac](https://github.com/metaneutrons/png2lvgl/commit/68ccbac53375df2996094da2a9fe6e0f22623d70))
* include manpage in Homebrew formula ([3e1874d](https://github.com/metaneutrons/png2lvgl/commit/3e1874d02c3660aadfed626d9a46567f7bc1f86b))
* remove duplicate line in upload-artifact path ([5b7d40d](https://github.com/metaneutrons/png2lvgl/commit/5b7d40d926d095a80f9613826cd0c3d365032644))
* remove duplicate target key and fix shellcheck issues ([fb557b0](https://github.com/metaneutrons/png2lvgl/commit/fb557b08d0bd14fcd566a5df0a8b41b054d0dbf8))
* remove unmaintained paste dependency ([003d0d6](https://github.com/metaneutrons/png2lvgl/commit/003d0d62dc4fd08a2bdfe330ac3b3261d68a9051))
* remove unused imports ([8a37b0f](https://github.com/metaneutrons/png2lvgl/commit/8a37b0f787ed4d11727466e61549f78cb2852782))
* skip tag creation if tag already exists in manual trigger ([3c8d9b3](https://github.com/metaneutrons/png2lvgl/commit/3c8d9b3f3d027c0deb637c941a2287fede08a164))
* update Cargo.toml version in release workflow before publishing ([672c2d1](https://github.com/metaneutrons/png2lvgl/commit/672c2d1cb3abdc2587e1bb4fe0fd3362ff81b532))
* update deb package to use generated manpage ([5abf335](https://github.com/metaneutrons/png2lvgl/commit/5abf3355a75ddc9d300ef18a5722bcd23253208f))


### Performance Improvements

* optimize release workflow by building deb in main matrix ([6c6d0af](https://github.com/metaneutrons/png2lvgl/commit/6c6d0afcff0ccbdacb27d18a00f473bde6300df2))
* optimize version update to run once before builds ([8738031](https://github.com/metaneutrons/png2lvgl/commit/873803130405b0c236886c21d16cbe33bbf77f34))

## [0.2.3] - 2025-11-02

### Fixed

- Skip tag creation if tag already exists in manual trigger
