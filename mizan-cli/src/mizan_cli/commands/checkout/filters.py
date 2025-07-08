from typing import List, Optional
from mizan_cli.commands.checkout.models import MizanDataset, Vulnerability
from mizan_cli.utils.logging import get_logger


logger = get_logger()


class DatasetFilter:
    def __init__(self, dataset: MizanDataset):
        self.dataset = dataset

    def apply_filters(
        self,
        level: str = "all",
        vuln_ids: Optional[List[str]] = None,
        year: Optional[int] = None,
        cwe_types: Optional[List[str]] = None,
        only_vulnerable: bool = True,
    ) -> MizanDataset:
        vulnerabilities = self.dataset.filter_vulnerabilities(
            vuln_ids=vuln_ids, year=year, cwe_types=cwe_types
        )

        logger.info(f"Found {len(vulnerabilities)} vulnerabilities after filtering")

        filtered_vulns = []
        total_samples = 0

        for vuln in vulnerabilities:
            samples = vuln.filter_samples(
                level=level if level != "all" else None,
                is_vulnerability=only_vulnerable if only_vulnerable else None,
            )

            if samples:
                filtered_vuln = Vulnerability(
                    **{**vuln.model_dump(), "code_samples": samples}
                )
                filtered_vulns.append(filtered_vuln)
                total_samples += len(samples)

        logger.info(f"Selected {total_samples} code samples")

        return self.dataset.create_filtered_dataset(filtered_vulns)
