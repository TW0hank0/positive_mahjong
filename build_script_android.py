import subprocess


def main():
    command = [
        "cargo",
        "apk",
        "build",
        "--target",
        "aarch64-linux-android",
        "--package",
        "pmj_client",
        "--release",
    ]
    print(f"Run Command: {' '.join(command)}")
    subprocess.run(command)


if __name__ == "__main__":
    main()
