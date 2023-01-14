from speedflux import config, logs, influx, data

# Speedflux
CONFIG = None
LOG = None
INFLUXDB = None
DATA = None


def initialize():
    global CONFIG
    global LOG
    global INFLUXDB
    global DATA

    try:
        CONFIG = config.Config()
    except Exception as err:
        raise SystemExit("Unable to initialize configuration", err)

    try:
        LOG = logs.Log(CONFIG)
    except Exception as err:
        raise SystemExit("Unable to initialize logging", err)

    try:
        INFLUXDB = influx.Influx(CONFIG, LOG)
    except Exception as err:
        raise SystemExit("Unable to initialize connection to InfluxDB", err)

    try:
        DATA = data.Data(CONFIG, INFLUXDB, LOG)
    except Exception as err:
        raise SystemExit("Unable to initialize data conenction", err)
