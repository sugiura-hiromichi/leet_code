#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_PROGRAM_LEN 100

void create_tiny_program(char content[]) {
  FILE *f;
  if ((f = fopen("tiny.c", "w")) == NULL) {
    printf("ERROR: failed to open file");
    exit(1);
  }

  fprintf(f, "%s", content);
  fclose(f);
}

int main() {
  int dbg = 2;
  if (dbg == 0) {
    printf("reached here -------------");
    char content[] = "#include <stdio.h>\nint main(){";
    char *program = malloc(MAX_PROGRAM_LEN);
    fgets(program, MAX_PROGRAM_LEN, stdin);
    strcat(content, strcat(program, "}"));

    create_tiny_program(content);

    int result = system("cc tiny.c -o TINY ; ./TINY");
    if (result == -1) {
      printf("Error occured when executing cc tiny.c -o TINY");
    }

    // char *args[] = {"./TINY", NULL};
    // execv(args[0], args);
  } else if (dbg == 1) {
    char content[] = "#include <stdio.h>\nint main(){printf(\"Hello World\");}";
    create_tiny_program(content);
    system("cc helowa.c -o HELOWA ; ./HELOWA");
  } else {
    char *program = malloc(MAX_PROGRAM_LEN);
    fgets(program, MAX_PROGRAM_LEN, stdin);
    program = strcat("\n---\n", program);
    printf("%s", program);
  }
}
