#include "xbtved.h"

App::XBTVEditor::XBTVEditor(void)
{
    xbtved = create_app();
}

App::XBTVEditor::~XBTVEditor(void)
{
    destroy_app(xbtved);
}

bool App::XBTVEditor::saveAll(void)
{
    return save_all(xbtved);
}

void App::XBTVEditor::newSchedule(void)
{
    new_buffer(xbtved);
}

std::string App::XBTVEditor::getSchedule(void)
{
    char* sched = sched_display(xbtved);
    return sched;
}

bool App::XBTVEditor::anyBufModified(void)
{
    bool mod = buffers_modified(xbtved);
    return mod;
}

void App::XBTVEditor::loadFile(QString &fileName)
{
    const char* path = fileName.toUtf8().constData();
    open(xbtved, path);
}
