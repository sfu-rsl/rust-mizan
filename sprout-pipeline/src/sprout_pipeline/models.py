from typing import Dict, List
from pydantic import BaseModel, Field, RootModel


class Finding(BaseModel):
    """Model for a single finding in the codebase."""

    file: str = Field(..., description="Relative path of the file containing the issue")
    line: int = Field(..., description="1-based line number")
    code: str = Field(..., description="Exact problematic snippet")
    reason: str = Field(..., description="Explanation of the issue")


class Findings(RootModel[List[Finding]]):
    """Collection of findings."""

    pass


class VulnerabilityReport(BaseModel):
    """Model for vulnerability detection task output."""

    is_vulnerable: bool = Field(
        ..., description="Whether the code contains vulnerabilities"
    )
    cwe_type: List[str] = Field(..., description="List of CWE types found")
    vulnerable_functions: Dict[str, List[str]] = Field(
        ..., description="Map of file path to vulnerable function names"
    )
    vulnerable_lines: Dict[str, List[int]] = Field(
        ..., description="Map of file path to vulnerable line numbers"
    )


class CVECheckReport(BaseModel):
    """Model for CVE checking task output."""

    has_cve: bool = Field(
        ..., description="Whether any CVEs exist for this crate and year"
    )
    cve_list: List[str] = Field(..., description="List of CVE identifiers")


class CrateIDReport(BaseModel):
    """Model for crate identification task output."""

    crate_name: str | None = Field(None, description="Identified crate name, if known")
    likely_year: int | None = Field(
        None, description="Likely year of the code, if known"
    )
    has_cve: bool = Field(..., description="Whether any CVEs exist for this crate")
    cve_list: List[str] = Field(..., description="List of CVE identifiers")
