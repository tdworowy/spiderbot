import json

import requests

robot_ip = "192.168.149.1"
robot_port = "3000"


# data= {
#         "period_ms":100,
#         "pulse_min_us": 500,
#         "pulse_neutral_us": 1500,
#         "pulse_max_us": 3500,
#         "step_by":100
#     }
#
# data= {
#         "period_ms":300,
#         "pulse_min_us": 500,
#         "pulse_neutral_us": 2500,
#         "pulse_max_us": 3500,
#         "step_by":100
#     }

# data= {
#         "period_ms":10,
#         "pulse_min_us": 500,
#         "pulse_neutral_us": 2500,
#         "pulse_max_us": 3500,
#         "step_by":100
#     }


data= {
        "period_ms":40,
        "pulse_min_us": 3000,
        "pulse_neutral_us": 4000,
        "pulse_max_us": 7000,
        "step_by":150
    }
#none of thoes move the robot

if __name__ == "__main__":
    gpio_status = requests.get(f"http://{robot_ip}:{robot_port}/get_gpio_status")
    print(gpio_status.json())

    print("Testing servo mechanism....")
    response = requests.post(f"http://{robot_ip}:{robot_port}/test_servo", data=json.dumps(data),
    headers={'Content-type': 'application/json', 'Accept': 'text/plain'})

    print(response.status_code)
    print(response.text)