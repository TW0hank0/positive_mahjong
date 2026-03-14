import subprocess
import sys


def main():
    targets = [
        "armv7-linux-androideabi",
        "aarch64-linux-android",
    ]
    print("Targets: ", ", ".join(targets))
    for target in targets:
        print("-" * 10)
        command = [
            "cargo",
            "apk",
            "build",
            "--target",
            target,
            "--package",
            "pmj_client",
            "--release",
        ]
        print(f"Run Command: {' '.join(command)}")
        subprocess.run(
            command,
            check=True,
            stderr=sys.stderr,
            stdin=sys.stdin,
            stdout=sys.stdout,
        )


if __name__ == "__main__":
    main()
