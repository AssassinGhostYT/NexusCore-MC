# EventPriority.py — Event priority constants
# Lower number = runs first (LOWEST=5 runs first, MONITOR=0 runs last)


class EventPriority:
    MONITOR = 0
    HIGHEST = 1
    HIGH = 2
    NORMAL = 3
    LOW = 4
    LOWEST = 5

    ALL = [LOWEST, LOW, NORMAL, HIGH, HIGHEST, MONITOR]

    @staticmethod
    def from_string(name):
        priorities = {
            "MONITOR": EventPriority.MONITOR,
            "HIGHEST": EventPriority.HIGHEST,
            "HIGH": EventPriority.HIGH,
            "NORMAL": EventPriority.NORMAL,
            "LOW": EventPriority.LOW,
            "LOWEST": EventPriority.LOWEST,
        }
        return priorities.get(name.upper(), EventPriority.NORMAL)
