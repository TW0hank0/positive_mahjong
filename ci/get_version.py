import tomllib
import os


def main():
    with open(
        os.path.join(os.path.dirname(__file__), "..", "Cargo.toml"),
        "rb",
    ) as f:
        d = tomllib.load(f)
    v = d["package"]["version"]
    print(v)


main()
