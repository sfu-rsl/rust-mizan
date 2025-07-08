from typing import List, Dict, Optional
from pathlib import Path
from pydantic import BaseModel, Field, ConfigDict


class GeneralInformation(BaseModel):
    model_config = ConfigDict(extra="allow")

    benchmark_name: str
    rust_version: str
    dataset_version: str


class CodeSample(BaseModel):
    model_config = ConfigDict(extra="allow")

    path_to_crate: str
    is_vulnerability: bool
    cwe_type: List[str] = Field(default_factory=list)
    vulnerable_functions: Dict[str, List[str]] = Field(default_factory=dict)
    vulnerable_lines: Dict[str, List[int]] = Field(default_factory=dict)
    deps: List[str] = Field(default_factory=list)

    @property
    def level(self) -> str:
        parts = self.path_to_crate.split("-")
        if len(parts) >= 3:
            return parts[-1]
        return "unknown"


class Vulnerability(BaseModel):
    model_config = ConfigDict(extra="allow")

    id: str
    author: str = Field(default="")
    source_link: str = Field(default="")
    crate_name: Optional[str] = Field(default=None)
    year: Optional[int] = Field(default=None)
    code_samples: List[CodeSample]

    def filter_samples(
        self, level: Optional[str] = None, is_vulnerability: Optional[bool] = None
    ) -> List[CodeSample]:
        """Filter code samples by criteria."""
        samples = self.code_samples

        if level and level != "all":
            samples = [s for s in samples if s.level == level]

        if is_vulnerability is not None:
            samples = [s for s in samples if s.is_vulnerability == is_vulnerability]

        return samples


class MizanDataset(BaseModel):
    model_config = ConfigDict(extra="allow")

    general_information: GeneralInformation
    vulnerabilities: List[Vulnerability]

    @classmethod
    def from_file(cls, path: Path) -> "MizanDataset":
        import json

        with open(path, "r") as f:
            data = json.load(f)

        return cls(**data)

    def to_file(self, path: Path, indent: int = 2):
        import json

        path.parent.mkdir(parents=True, exist_ok=True)

        with open(path, "w") as f:
            json.dump(self.model_dump(), f, indent=indent)

    def filter_vulnerabilities(
        self,
        vuln_ids: Optional[List[str]] = None,
        year: Optional[int] = None,
        cwe_types: Optional[List[str]] = None,
    ) -> List[Vulnerability]:
        vulns = self.vulnerabilities

        if vuln_ids:
            vulns = [v for v in vulns if v.id in vuln_ids]

        if year is not None:
            vulns = [v for v in vulns if v.year == year]

        if cwe_types:
            vulns = [
                v
                for v in vulns
                if any(
                    any(cwe in sample.cwe_type for cwe in cwe_types)
                    for sample in v.code_samples
                )
            ]

        return vulns

    def get_all_dependencies(self, vulnerabilities: List[Vulnerability]) -> List[str]:
        deps = set()

        for vuln in vulnerabilities:
            for sample in vuln.code_samples:
                deps.update(sample.deps)

        return sorted(list(deps))

    def create_filtered_dataset(
        self, vulnerabilities: List[Vulnerability]
    ) -> "MizanDataset":
        return MizanDataset(
            general_information=self.general_information,
            vulnerabilities=vulnerabilities,
        )
