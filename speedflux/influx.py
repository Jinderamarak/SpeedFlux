import sys
from urllib3.exceptions import NewConnectionError
from influxdb_client import InfluxDBClient, client
from requests.exceptions import ConnectionError


class Influx:
    def __init__(self, config, log):
        self.config = config
        self.log = log
        self._client = None
        self._writing = None

    @property
    def client(self):
        if not self._client:
            self._client = InfluxDBClient(
                url=self.config.INFLUX_DB_URL,
                token=self.config.INFLUX_DB_TOKEN,
                org=self.config.INFLUX_DB_ORG,
                verify_ssl=self.config.INFLUX_DB_VERIFY_SSL,
                debug=False
            )
            self.log.debug("Client established")
        return self._client

    @property
    def writing(self):
        if not self._writing:
            self._writing = self.client.write_api(client.write_api.SYNCHRONOUS)
        return self._writing

    def format_data(self, data):
        influx_data = [
            {
                "measurement": "ping",
                "time": data["timestamp"],
                "fields": {
                    "jitter": data["ping"].get("jitter", 0),
                    "latency": data["ping"].get("latency", 0)
                }
            },
            {
                "measurement": "download",
                "time": data["timestamp"],
                "fields": {
                    # Byte to Megabit
                    "bandwidth": data["download"].get("bandwidth", 0) / 125000,
                    "bytes": data["download"].get("bytes", 0),
                    "elapsed": data["download"]["elapsed"]
                }
            },
            {
                "measurement": "upload",
                "time": data["timestamp"],
                "fields": {
                    # Byte to Megabit
                    "bandwidth": data["upload"].get("bandwidth", 0) / 125000,
                    "bytes": data["upload"]["bytes"],
                    "elapsed": data["upload"]["elapsed"]
                }
            },
            {
                "measurement": "packetLoss",
                "time": data["timestamp"],
                "fields": {
                    "packetLoss": int(data.get("packetLoss", 0))
                }
            },
            {
                "measurement": "speeds",
                "time": data["timestamp"],
                "fields": {
                    "jitter": data["ping"].get("jitter", 0),
                    "latency": data["ping"].get("latency", 0),
                    "packetLoss": int(data.get("packetLoss", 0)),
                    # Byte to Megabit
                    "bandwidth_down": data["download"].get(
                        "bandwidth", 0) / 125000,
                    "bytes_down": data["download"].get(
                        "bytes", 0),
                    "elapsed_down": data["download"].get(
                        "elapsed"),
                    # Byte to Megabit
                    "bandwidth_up": data["upload"].get(
                        "bandwidth", 0) / 125000,
                    "bytes_up": data["upload"].get(
                        "bytes", 0),
                    "elapsed_up": data["upload"].get(
                        "elapsed")
                }
            }
        ]

        tags = self.tag_selection(data)
        if tags is not None:
            for measurement in influx_data:
                measurement["tags"] = tags

        return influx_data

    def write(self, data, data_type, tries=3):
        try:
            self.writing.write(
                self.config.INFLUX_DB_BUCKET,
                self.config.INFLUX_DB_ORG,
                data
            )

            self.log.info(F"{data_type} data written successfully")
            self.log.debug(F"Wrote `{data}` to Influx")
        except (ConnectionError, NewConnectionError, Exception) as \
                bad_connection:
            if tries <= 0:
                self.log.error(
                    "Max retries exceeded for write. Check your database, bucket and token configuration")
                self.log.error("Exiting")
                sys.exit()
                return

            self.log.error("Connection error occurred during write")
            self.log.error(bad_connection)
            self.write(data, data_type, tries - 1)

        except Exception as err:
            self.log.error(F"{err}")

    def tag_selection(self, data):
        tags = self.config.INFLUX_DB_TAGS
        options = {}

        # tag_switch takes in _data and attaches CLIoutput to more readable ids
        tag_switch = {
            "namespace": self.config.NAMESPACE,
            "isp": data["isp"],
            "interface": data["interface"]["name"],
            "internal_ip": data["interface"]["internalIp"],
            "interface_mac": data["interface"]["macAddr"],
            "vpn_enabled": (
                False if data["interface"]["isVpn"] == "false" else True),
            "external_ip": data["interface"]["externalIp"],
            "server_id": data["server"]["id"],
            "server_name": data["server"]["name"],
            "server_location": data["server"]["location"],
            "server_country": data["server"]["country"],
            "server_host": data["server"]["host"],
            "server_port": data["server"]["port"],
            "server_ip": data["server"]["ip"],
            "speedtest_id": data["result"]["id"],
            "speedtest_url": data["result"]["url"]
        }

        if tags is None:
            if self.config.NAMESPACE:
                tags = "namespace"
        elif "*" in tags:
            return tag_switch
        else:
            if self.config.NAMESPACE:
                tags = "namespace," + tags

        tags = tags.split(",")
        for tag in tags:
            # split the tag string, strip and add selected tags to {options}
            # with corresponding tag_switch data
            tag = tag.strip()
            if tag in tag_switch:
                options[tag] = tag_switch[tag]
        return options

    def process_data(self, data):
        data = self.format_data(data)
        self.write(data, "Speedtest")
