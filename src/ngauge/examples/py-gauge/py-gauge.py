import example

class Example(example.Example):
    def add(x: int, y: int) -> int:
        return x + y

    def hello_world(self) -> str:
        return "Hello from py-gauge!"

    def measure(self, assets: list[str]) -> int:
        print("py-gauge will measure how many strings begin with 'b'")
        for a in assets:
            print(a)
        return len([x for x in assets if x.startswith("b")])