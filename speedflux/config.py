import os
import re

_CONFIG_DEFAULTS = {
    "NAMESPACE": (str, "Database", None),
    "INFLUX_DB_URL": (str, "Database", "http://influxdb:8086"),
    "INFLUX_DB_TOKEN": (str, "Database", None),
    "INFLUX_DB_ORG": (str, "Database", "org"),
    "INFLUX_DB_VERIFY_SSL": (bool, "Database", True),
    "INFLUX_DB_BUCKET": (str, "Database", "speedtests"),
    "INFLUX_DB_TAGS": (str, "Database", None),
    "SPEEDTEST_SERVER_ID": (str, "SpeedTest", None),
    "SPEEDTEST_INTERVAL": (int, "SpeedTest", 300),
    "PING_TARGETS": (str, "PingTest", "1.1.1.1,8.8.8.8"),
    "PING_INTERVAL": (int, "PingTest", 5),
    "LOG_TYPE": (str, "Logs", "info"),
}


class Config:

    def get_setting(self, key):
        """
        Cast any value in the config to the right type or use the default
        """
        key, definition_type, section, default = self._define(key)
        my_val = os.getenv(key, default)
        if my_val is not None:
            my_val = definition_type(my_val)
        return my_val

    def _define(self, name):
        key = name.upper()
        definition = _CONFIG_DEFAULTS[key]
        if len(definition) == 3:
            definition_type, section, default = definition
        return key, definition_type, section, default

    def __getattr__(self, name):
        """
        Retrieves config value for the setting
        """
        if not re.match(r"[A-Z_]+$", name):
            return super(Config, self).__getattr__(name)
        else:
            return self.get_setting(name)
