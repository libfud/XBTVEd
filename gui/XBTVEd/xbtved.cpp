#include "xbtved.h"

App::XBTVEditor::XBTVEditor(void)
{
    xbtved = create_app();
}

App::XBTVEditor::~XBTVEditor(void)
{
    destroy_app(xbtved);
}
