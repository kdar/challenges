# NOT WORKING. kept getting runtime errors from hackerrank

#!python3.5


## Use this function to write data to socket
## write_string_to_socket(connection, message) where connection is the socket object and message is string

## Use this function to read data from socket
## def read_string_from_socket(connection) where connection is the socket object

## All global declarations go here
config = {}

## This function is called only once before any client connection is accepted by the server.
## Read any global datasets or configurations here
def init_server():
  global config

  print("Reading training set")
  sys.stdout.flush()

  with open("training.txt", "r") as f:
    f.readline()
    for line in f.readlines():
      u,v = [int(x) for x in line.split(',')]
      config[v] = u


## This function is called everytime a new connection is accepted by the server.
## Service the client here
def process_client_connection(connection):
  global config

  print("reading connection")

  while True:
    # read message
    message = read_string_from_socket(connection)

    if message == "END":
      write_string_to_socket(connection, message)
      connection.close()
      break

    print ("Message received = ", message)

    a,b,q = [int(x) for x in message.split(',')]
    path_a = [a]
    path_b = [b]
    while path_a[-1] != 1:
      path_a.append(config[path_a[-1]])
    while path_b[-1] != 1:
      path_b.append(config[path_b[-1]])

    count=0
    for (i, node) in enumerate(path_a):
      try:
        index = path_b.index(node)
        count=index+1+i
        break
      except ValueError:
        continue

    if count <= q:
      write_string_to_socket(connection, "YES")
    else:
      write_string_to_socket(connection, "NO")

    # write message




# import sys
#
# config = {}
#
# def init_server():
#   global config
#
#   print("Reading training set")
#   sys.stdout.flush()
#
#   with open("training.txt", "r") as f:
#     f.readline()
#     for line in f.readlines():
#       u,v = [int(x) for x in line.split(',')]
#       config[v] = u
#
# init_server()
#
# def process_client_connection(data):
#   global config
#
#   a,b,q = [int(x) for x in data.split(',')]
#   path_a = [a]
#   path_b = [b]
#   while path_a[-1] != 1:
#     path_a.append(config[path_a[-1]])
#   while path_b[-1] != 1:
#     path_b.append(config[path_b[-1]])
#
#   count=0
#   for (i, node) in enumerate(path_a):
#     try:
#       index = path_b.index(node)
#       count=index+1+i
#       break
#     except ValueError:
#       continue
#
#   if count <= q:
#     print("YES")
#   else:
#     print("NO")
#
# process_client_connection("4,2,2")
# process_client_connection("5,1,2")
# process_client_connection("2,4,1")
# process_client_connection("9,10,5")
# process_client_connection("7,10,100")
# process_client_connection("1,5,8")
# process_client_connection("9,10,2")
# process_client_connection("7,10,3")
