import requests

robot_ip = "192.168.149.1"
robot_port = "3000"

if __name__ == "__main__":
    gpio_status = requests.get(f"http://{robot_ip}:{robot_port}/get_gpio_status")
    print(gpio_status.json())