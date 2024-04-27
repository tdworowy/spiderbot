import requests

robot_ip = "192.168.149.1"
robot_port = "3000"

if __name__ == "__main__":
    gpio_status = requests.get(f"http://{robot_ip}:{robot_port}/get_gpio_status")
    print(gpio_status.json())

    print("Testing servo mechanism....")
    requests.post(f"http://{robot_ip}:{robot_port}/test_servo", data={
        "period_ms":60,
        "pulse_min_us": 500,
        "pulse_neutral_us": 3500,
        "pulse_max_us": 5500,
    })
