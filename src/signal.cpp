#include <QObject>

class SignalHandler : QObject {
  Q_OBJECT
  
  void (*handler)(void *);
  void *arg;

private slots:
  void handle() {
    handler(arg);
  }

public:
  SignalHandler(QObject *object, const char *signal, void *arg, void (*callback)(void *))
    :handler(callback),
     arg(arg)
  {
    connect(object, signal, SLOT(handle()));
  }

  ~SignalHandler() {}
};

extern "C" {
  SignalHandler *create(void *object, const char *signal, void *arg, const void *callback) {
    return new SignalHandler((QObject*) object, signal, arg, (void (*)(void *)) callback);
  }

  void destroy(void *handler) {
    delete ((SignalHandler*) handler);
  }
}

#include "signal.moc"
