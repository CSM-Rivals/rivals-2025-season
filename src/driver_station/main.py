import pygame
import msgpack
import socket
import time
import subprocess

subprocess.Popen(['ssh', '-N', '-L', '29230:localhost:29230', 'rivals'])

pygame.init()

screen = pygame.display.set_mode((400, 400))

class Socket:
    def __init__(self, host, addr):
        self._sock = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self._sock.connect((host, addr))

    def write(self, data):
        try:
            return self._sock.send(data)
        except ConnectionAbortedError:
            print('Disconnected!')

    def connect(self):
        pass

    def __getattr__(self, name):
        # Delegate other socket methods to the underlying socket object
        return getattr(self._sock, name)

s = Socket('localhost', 29230)

loop = True
while loop:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            loop = False

    input_x = 0
    input_y = 0
    input_r = 0
    button_x = False
    button_y = False

    keys = pygame.key.get_pressed()
    if keys[pygame.K_w]:
        input_y += 1
    if keys[pygame.K_a]:
        input_x -= 1
    if keys[pygame.K_s]:
        input_y -= 1
    if keys[pygame.K_d]:
        input_x += 1
    if keys[pygame.K_e]:
        input_r += 1
    if keys[pygame.K_q]:
        input_r -= 1
    
    if keys[pygame.K_u]:
        button_x = True
    if keys[pygame.K_i]:
        button_y = True

    msgpack.pack({
        'left_stick_x': input_x,
        'left_stick_y': input_y,
        'right_stick_x': input_r,
        'right_stick_y': 0,

        'a': False,
        'b': False,
        'x': button_x,
        'y': button_y,
    }, s)
 # type: ignore
    time.sleep(0.1)