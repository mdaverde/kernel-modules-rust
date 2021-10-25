set dotenv-load := true


KERNELDIR := env_var("KERNELDIR")
LLVM := env_var("LLVM")
KERNEL_MODULES := "current proc_iter mem_layout lowlevel_mem"

vars:
	echo "KERNELDIR={{KERNELDIR}}"
	echo "LLVM={{LLVM}}"

fmt:
	rustfmt */*.rs

build module:
	#!/usr/bin/env zx
	const kernelModules = "{{module}}" === "all" ? "{{KERNEL_MODULES}}" : "{{module}}";
	const kernelDir = "{{KERNELDIR}}";
	const llvmParam = "{{LLVM}}";
	for (const moduleName of kernelModules.split(" ")) {
		await cd(`./${moduleName}`);
		await $`make KERNELDIR=${kernelDir} LLVM=${llvmParam} modules`;
	}

clean module:
	#!/usr/bin/env zx
	const kernelModules = "{{module}}" === "all" ? "{{KERNEL_MODULES}}" : "{{module}}";
	const kernelDir = "{{KERNELDIR}}";
	const llvmParam = "{{LLVM}}";
	for (const moduleName of kernelModules.split(" ")) {
		await cd(`./${moduleName}`);
		await $`make KERNELDIR=${kernelDir} LLVM=${llvmParam} clean`;
	}

create module:
	cp -r ./mod_template {{module}}
	sed -i 's/mod_template/{{module}}/g' ./{{module}}/Makefile
	sed -i 's/ModTemplate/{{module}}/g' ./{{module}}/main.rs
	# TODO: rust-analyzer generation
	# TODO: add to KERNEL_MODULES variable

rust-analyzer:
	#!/usr/bin/env zx
	const kernelDir = "{{KERNELDIR}}"
	const kernelModules = "{{KERNEL_MODULES}}".split(" ");
	const analyzerPath = "./rust-project.json";

	const analyzerObj = {
		crates: [{
			display_name: "kernel",
			root_module: `${kernelDir}/rust/kernel/lib.rs`,
			edition: "2018",
			deps: []
		}]
	};

	function addKernelModule(kernelModuleName) {
		const kernelModuleCrate = {
			display_name: kernelModuleName,
			root_module: `./${kernelModuleName}/main.rs`,
			edition: "2018",
			deps: [{ crate: 0, name: "kernel" }]
		};
		analyzerObj.crates.push(kernelModuleCrate);
	}

	for (const kernelModule of kernelModules) {
		addKernelModule(kernelModule);
	}

	if (fs.existsSync(analyzerPath)) {
		fs.removeSync(analyzerPath);
	}

	fs.writeFileSync(analyzerPath, JSON.stringify(analyzerObj, null, 2));



