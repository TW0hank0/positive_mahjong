import datetime

from zoneinfo import ZoneInfo

tz = ZoneInfo("Asia/Taipei")
print(datetime.datetime.now(tz).strftime("%Y-%m-%d_%H-%M-%S"))
