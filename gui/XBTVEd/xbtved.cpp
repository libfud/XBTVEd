/*#include "xbtved.h"

App::XBTVEditor::XBTVEditor(void)
{
    xbtved = create_app();
}

App::XBTVEditor::~XBTVEditor(void)
{
    destroy_app(xbtved);
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

bool App::XBTVEditor::saveAll(void)
{
    return save_all(xbtved);
}

bool App::XBTVEditor::saveFile(void)
{
    return save(xbtved);
}

bool App::XBTVEditor::saveAs(QString &filename)
{
    const char* path = filename.toUtf8().constData();
    return save_as(xbtved, path);
}

void App::XBTVEditor::unDo(void)
{
    undo(xbtved);
}

void App::XBTVEditor::reDo(void)
{
    redo(xbtved);
}*/
