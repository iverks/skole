import preex as preex

def main() -> None: ...

class EventDrivenGas:
    def __init__(self) -> None: ...
    @classmethod
    def new_uniform_v(
        cls, num_particles: int, speed: float, radius: float
    ) -> EventDrivenGas: ...
    def main() -> None: ...
    def step(): ...
    def step_many(num_steps: int): ...
    def get_positions() -> tuple[list[float], list[float]]: ...
    def get_sizes() -> list[float]: ...
    def get_total_energy() -> float: ...
