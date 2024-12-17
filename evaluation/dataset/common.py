from dataclasses import dataclass


@dataclass
class DatasetCase:
    file_name: str
    line_number: int
    rule_id: str


Dataset = dict[str, list[DatasetCase]]


rules_all = ["ql-double-measurement", "ql-operation-after-measurement"]
