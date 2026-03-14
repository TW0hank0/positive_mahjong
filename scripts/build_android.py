import subprocess
import sys


def main():
    targets = [
        "aarch64-linux-android",
        # "armv7-linux-androideabi",
    ]
    print("Targets: " + ", ".join(targets))
    for target in targets:
        print("-" * 10)
        command: list[str] = [
            "cargo",
            "apk",
            "build",
            "--target",
            target,
            "--package",
            "pmj_client",
            "--release",
            "--lib",
        ]
        print("Run Command: " + " ".join(command))
        subprocess.run(
            command,
            check=True,
            stderr=sys.stderr,
            stdin=sys.stdin,
            stdout=sys.stdout,
        )


if __name__ == "__main__":
    main()
