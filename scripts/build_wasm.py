import subprocess
import sys


def main():
    command = [
        "wasm-pack",
        "build",
        "--release",
        "--target",
        "web",
        "--bin",
        "pmj_client",
    ]
    print("Run Command:" + " ".join(command))
    subprocess.run(
        command,
        stdin=sys.stdin,
        stdout=sys.stdout,
        stderr=sys.stderr,
        check=True,
    )


if __name__ == "__main__":
    main()
