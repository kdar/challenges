/*
Use this function to write data to socket
void write_string_to_socket(int sock_descriptor, char* message, uint32_t length);

Use this function to read data from socket
void read_string_from_socket(int sock_descriptor, char** message, uint32_t *length);
*/

#include <stdio.h>

// All global declarations go here
int config[1000000] = {0};

/*
This function is called only once before any client connection is accepted by the server.
Read any global datasets or configurations here
*/
void init_server()
{
  printf("Reading configuration\n");
  fflush(stdout);

  FILE *read = fopen("training.txt", "r");

  while (!feof(read)) {
    int u, v;
    fscanf(read, "%d,%d", &u, &v);
    config[v] = u;
  }

  fclose(read);
}

/*
Write your code here
This function is called everytime a new connection is accepted by the server
*/
void * process_client_connection(void * ptr)
{
    connection_t * conn;

    if (!ptr) pthread_exit(0);
    conn = (connection_t *)ptr;

    printf("Connection received\n");
     fflush(stdout);

    while (1)
    {
      /* read length of message */
      char *message = NULL;
      uint32_t message_length = 0;

      /* read message */
      read_string_from_socket(conn->sock, &message, &message_length);

      /* End of operation on this clinet */
      if (strcmp(message, "END") == 0) {
        write_string_to_socket(conn->sock, message, message_length);
        break;
      }

      //printf("Received = %s\n", message);

      int a, b, q;
      sscanf(message, "%d,%d,%d", &a, &b, &q);
      int path_a[q], path_b[q];
      for (int i = 1; i < q; i++) { path_a[i] = 0; path_b[i] = 0; }
      path_a[0] = a;
      path_b[0] = b;
      for (int i = 1; i < q; i++) {
        path_a[i] = config[path_a[i-1]];
        if (path_a[i] == 1) {
          break;
        }
      }
      for (int i = 1; i < q; i++) {
        path_b[i] = config[path_b[i-1]];
        if (path_b[i] == 1) {
          break;
        }
      }

      int count = 0;
      for (int i = 0; i < q && path_a[i] != 0 && count == 0; i++) {
        for (int j = 0; j < q && path_b[j] != 0; j++) {
          if (path_b[j] == path_a[i]) {
            count=j+i+1;
            break;
          }
        }
      }

      printf("%d\n", count);

      if (count != 0 && count <= q) {
        write_string_to_socket(conn->sock, "YES", 3);
      } else {
        write_string_to_socket(conn->sock, "NO", 2);
      }

      free(message);
    };

    /* close socket and clean up */
    printf("Closing client on socket %d\n", conn->sock);
     fflush(stdout);
    close(conn->sock);
    free(conn);
    pthread_exit(0);
}
