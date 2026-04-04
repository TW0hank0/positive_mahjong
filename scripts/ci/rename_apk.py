import os
import sys


def main():
    apk_path = os.path.abspath(
        os.path.join(
            os.path.dirname(os.path.dirname(os.path.dirname(__file__))),
            "target",
            "release",
            "apk",
        )
    )
    orig_name = os.path.join(apk_path, "pmj_client.apk")
    new_name = os.path.join(apk_path, f"pmj_client_{sys.argv[1]}.apk")
    print(f"Rename `{orig_name}` to `{new_name}`")
    os.rename(orig_name, new_name)


if __name__ == "__main__":
    main()
