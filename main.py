import time

import speedflux
from multiprocessing import Process


def main():
    speedflux.initialize()
    speedflux.LOG.info("Speedtest CLI data logger to InfluxDB started")

    pSpeed = Process(target=speedflux.DATA.speedtest, args=())
    pPing = Process(target=speedflux.DATA.pingtest, args=())

    speedtest_interval = speedflux.CONFIG.SPEEDTEST_INTERVAL
    speedtest_remaining = speedtest_interval

    ping_interval = speedflux.CONFIG.PING_INTERVAL
    ping_remaining = ping_interval

    if speedtest_interval < 0 and ping_interval < 0:
        raise SystemExit("Neither Speedtest or Ping has valid intervals")

    pSpeed.start()
    pPing.start()

    while True:
        to_wait = 60

        if speedtest_interval >= 0:
            if speedtest_remaining <= 0:
                if pSpeed.is_alive():
                    pSpeed.terminate()
                pSpeed = Process(target=speedflux.DATA.speedtest, args=())
                pSpeed.start()
                speedtest_remaining = speedtest_interval
            to_wait = min(to_wait, speedtest_remaining)

        if ping_interval >= 0:
            if ping_remaining <= 0:
                if pPing.is_alive():
                    pPing.terminate()
                pPing = Process(target=speedflux.DATA.pingtest, args=())
                pPing.start()
                ping_remaining = ping_interval
            to_wait = min(to_wait, ping_remaining)

        if to_wait > 0:
            speedflux.LOG.debug(f"Waiting for {to_wait} seconds")

        time.sleep(to_wait)
        speedtest_remaining -= to_wait
        ping_remaining -= to_wait


if __name__ == "__main__":
    main()
