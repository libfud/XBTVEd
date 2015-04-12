#include <stdio.h>
#include "xbtved.h"

int main(void)
{
  //  *XBTVEd xbtved;
  struct XBTVEd const* xbtved;
  xbtved = create_app();
  unsigned int len = buffers_len(xbtved);
  printf("%u\n", len);

  printf("%s\n", get_buffer_name(xbtved));

  new_buffer(xbtved);
  new_buffer(xbtved);

  printf("%u\n", buffers_len(xbtved));

  prev_buffer(xbtved);
  set_buffer_name(xbtved, "test");
  
  printf("%s\n", get_buffer_name(xbtved));

  prev_buffer(xbtved);
  set_buffer_name(xbtved, "wetfish");
  printf("%s\n", get_buffer_name(xbtved));
  undo(xbtved);
  printf("%s\n", get_buffer_name(xbtved));
  redo(xbtved);
  printf("%s\n", get_buffer_name(xbtved));

  add_program(xbtved, "local", "/foo/bar/baz");

  return 0;
}
