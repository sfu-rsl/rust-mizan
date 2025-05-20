from .check_cve import CheckCVETask
from .identify_crate import IdentifyCrateTask
from .vuln import VulnerabilityTask

__all__ = [
    "TASK_REGISTRY",
    "CheckCVETask",
    "IdentifyCrateTask",
    "VulnerabilityTask",
]

TASK_REGISTRY = {
    CheckCVETask.name: CheckCVETask,  # 1. check_cve
    IdentifyCrateTask.name: IdentifyCrateTask,  # 2. identify_crate
    VulnerabilityTask.name: VulnerabilityTask,  # 3. vuln
}
