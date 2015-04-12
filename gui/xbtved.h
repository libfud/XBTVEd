#ifndef __XBTVED_H
#define __XBTVED_H

struct XBTVEd;
struct XBTVEd const* create_app();


unsigned int buffers_len(struct XBTVEd const* xbtved);

void undo(struct XBTVEd const* xbtved);
void redo(struct XBTVEd const* xbtved);

void new_buffer(struct XBTVEd const* xbtved);
void prev_buffer(struct XBTVEd const* xbtved);
void next_buffer(struct XBTVEd const* xbtved);

char* get_buffer_name(struct XBTVEd const* xbtved);
void set_buffer_name(struct XBTVEd const* xbtved, char* name);

void add_program(struct XBTVEd const* xbtved, char*, char*);

#endif
