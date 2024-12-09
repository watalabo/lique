from dataclasses import dataclass


@dataclass
class DatasetCase:
    rule_id: str | None
    file_name: str
