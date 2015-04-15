
#ifndef __XBTVED_H
#define __XBTVED_H
#include <QObject>
#include <QString>

extern "C" {
  struct XBTVEd;
  struct XBTVEd const* create_app();
  void destroy_app(struct XBTVEd const* xbtved);

  void open(struct XBTVEd const* xbtved, const char* path);
  bool save_all(struct XBTVEd const* xbtved);
  bool save(struct XBTVEd const* xbtved);
  bool save_as(struct XBTVEd const* xbtved, const char* path);

  const char* sched_display(struct XBTVEd const* xbtved);

  unsigned int buffers_len(struct XBTVEd const* xbtved);

  bool buffers_modified(struct XBTVEd const* xbtved);
  void undo(struct XBTVEd const* xbtved);
  void redo(struct XBTVEd const* xbtved);

  void new_buffer(struct XBTVEd const* xbtved);
  void prev_buffer(struct XBTVEd const* xbtved);
  void next_buffer(struct XBTVEd const* xbtved);

  char* get_buffer_name(struct XBTVEd const* xbtved);
  void set_buffer_name(struct XBTVEd const* xbtved, char* name);

  void add_program(struct XBTVEd const* xbtved, char* src, char* loc);

}

namespace App{
    class XBTVEditor: public QObject
    {
        Q_OBJECT
    public:
        XBTVEditor();
        ~XBTVEditor();
        bool saveAll();
        bool saveAs(QString& path);
        bool saveFile();
        void loadFile(QString& path);
        void newSchedule();
        std::string getSchedule();
        bool anyBufModified();
        void unDo();
        void reDo();

    signals:
        void bufModified(void);

    private slots:


    private:
        XBTVEd const* xbtved;
        bool anyModified;
    };
}

#endif

