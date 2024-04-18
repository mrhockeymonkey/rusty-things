import example

class Example(example.Example):
    def add(x: int, y: int) -> int:
        return x + y

    def hello_world(self) -> str:
        return "Hello from py-gauge!"

    def measure(self, assets: list[str]) -> int:
        for a in assets:
            print(a)
        return len(assets)