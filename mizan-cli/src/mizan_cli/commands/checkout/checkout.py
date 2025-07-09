import shutil
from pathlib import Path
from typing import List
from mizan_cli.commands.checkout.models import MizanDataset
from mizan_cli.utils.logging import get_logger, ProgressManager


logger = get_logger()


class CheckoutManager:
    def __init__(self, dataset_path: Path, output_dir: Path):
        self.dataset_path = dataset_path
        self.output_dir = output_dir
        self.base_dir = dataset_path.parent / "samples"

    def checkout(self, filtered_dataset: MizanDataset) -> None:
        if self.output_dir.exists():
            shutil.rmtree(self.output_dir)
        self.output_dir.mkdir(parents=True)

        samples_dir = self.output_dir / "samples"
        samples_dir.mkdir()
        self._copy_samples(filtered_dataset, samples_dir)

        dependencies = filtered_dataset.get_all_dependencies(
            filtered_dataset.vulnerabilities
        )
        if dependencies:
            self._copy_dependencies(dependencies)

        self._generate_cargo_toml(filtered_dataset)

        # Copy rust-toolchain.toml if it exists
        self._copy_rust_toolchain()

        filtered_dataset.to_file(self.output_dir / "mizan.json")

        logger.success(f"Checkout completed to {self.output_dir}")

    def _copy_samples(self, dataset: MizanDataset, samples_dir: Path) -> None:
        total_samples = sum(len(v.code_samples) for v in dataset.vulnerabilities)

        with ProgressManager() as progress:
            task = progress.add_task("Copying code samples", total=total_samples)

            for vuln in dataset.vulnerabilities:
                for sample in vuln.code_samples:
                    src_path = self.base_dir / sample.path_to_crate
                    dst_path = samples_dir / sample.path_to_crate

                    if src_path.exists():
                        dst_path.parent.mkdir(parents=True, exist_ok=True)
                        shutil.copytree(src_path, dst_path)
                    else:
                        logger.warning(f"Sample not found: {src_path}")

                    progress.update(task, advance=1)

    def _copy_dependencies(self, dependencies: List[str]) -> None:
        deps_dir = self.output_dir / "samples" / "deps"
        deps_dir.mkdir(parents=True, exist_ok=True)

        logger.info(f"Copying {len(dependencies)} dependencies")

        for dep in dependencies:
            src_path = self.base_dir / "deps" / dep
            dst_path = deps_dir / dep

            if src_path.exists():
                shutil.copytree(src_path, dst_path)
            else:
                logger.warning(f"Dependency not found: {src_path}")

    def _generate_cargo_toml(self, dataset: MizanDataset) -> None:
        members = []
        for vuln in dataset.vulnerabilities:
            for sample in vuln.code_samples:
                members.append(f'    "samples/{sample.path_to_crate}",')

        cargo_content = '[workspace]\nresolver = "2"\nmembers = [\n'
        cargo_content += "\n".join(sorted(members))
        cargo_content += "\n]\n"

        (self.output_dir / "Cargo.toml").write_text(cargo_content)

    def _copy_rust_toolchain(self) -> None:
        """Copy rust-toolchain.toml from the project root if it exists."""
        rust_toolchain_path = self.dataset_path.parent / "rust-toolchain.toml"
        if rust_toolchain_path.exists():
            shutil.copy2(rust_toolchain_path, self.output_dir / "rust-toolchain.toml")
            logger.debug("Copied rust-toolchain.toml to output directory")
