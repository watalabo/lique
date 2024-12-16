from dataclasses import dataclass


@dataclass
class DatasetCase:
    file_name: str
    line_number: int
    rule_id: str
