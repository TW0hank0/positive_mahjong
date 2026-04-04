import os
import sys


def main():
    apk_path = os.path.join(__file__, "..", "..", "..", "target", "release", "apk")
    orig_name = os.path.join(apk_path, "pmj_client.apk")
    new_name = os.path.join(apk_path, f"pmj_client_{sys.argv[1]}.apk")
    os.rename(orig_name, new_name)


if __name__ == "__main__":
    main()
