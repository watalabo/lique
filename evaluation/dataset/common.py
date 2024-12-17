from dataclasses import dataclass


@dataclass
class DatasetCase:
    file_name: str
    line_number: int
    rule_id: str


rules_all = ["ql-double-measurement", "ql-operation-after-measurement"]
