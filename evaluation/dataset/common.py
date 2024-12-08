from dataclasses import dataclass


@dataclass
class DatasetCase:
    rule_id: str
    label: bool
    file_name: str
