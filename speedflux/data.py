import json
import datetime
import subprocess
from pythonping import ping


class Data:
    def __init__(self, config, db, log):
        self.config = config
        self.db = db
        self.log = log

    def speedtest(self):
        if not self.config.SPEEDTEST_SERVER_ID:
            speedtest = subprocess.run(
                [
                    "speedtest",
                    "--accept-license",
                    "--accept-gdpr",
                    "-f",
                    "json"
                ],
                capture_output=True
            )
            self.log.info("Automatic server choice")
        else:
            speedtest = subprocess.run(
                [
                    "speedtest",
                    "--accept-license",
                    "--accept-gdpr",
                    "-f",
                    "json",
                    f"--server-id={self.config.SPEEDTEST_SERVER_ID}"
                ],
                capture_output=True
            )
            self.log.info(
                "Manual server choice: "f"ID = {self.config.SPEEDTEST_SERVER_ID}"
            )

        if speedtest.returncode == 0:  # Speedtest was successful.
            self.log.info("Speedtest Successful")
            data_json = json.loads(speedtest.stdout)
            self.log.info(F"""Speedtest Data:
                time: {data_json["timestamp"]}
                ping: {data_json["ping"]["latency"]}ms
                download: {data_json["download"]["bandwidth"]/125000}Mb/s
                upload: {data_json["upload"]["bandwidth"] / 125000}Mb/s
                isp: {data_json["isp"]}
                ext. IP: {data_json["interface"]["externalIp"]}
                server id: {data_json["server"]["id"]}
                server location: ({data_json["server"]["name"]} @ {data_json["server"]["location"]})
                """)
            self.db.process_data(data_json)
        else:  # Speedtest failed.
            self.log.info("Speedtest Failed")
            self.log.debug(speedtest.stderr)
            self.log.debug(speedtest.stdout)

    def pingtest(self):
        timestamp = datetime.datetime.utcnow()
        data = []
        for target in self.config.PING_TARGETS.split(","):
            target = target.strip()
            self.log.debug("Running ping test")
            pingtest = ping(
                target,
                verbose=False,
                timeout=1,
                count=1,
                size=128
            )
            single = {
                "measurement": "pings",
                "time": timestamp,
                "tags": {
                    "target": target
                },
                "fields": {
                    "success": int(
                        pingtest._responses[0].error_message is None),
                    "rtt": float(
                        0 if pingtest._responses[0].error_message is
                        not None else pingtest.rtt_avg_ms)
                }
            }
            if self.config.NAMESPACE:
                single["tags"]["namespace"] = self.config.NAMESPACE
            data.append(single)
        self.db.write(data, "Ping")
