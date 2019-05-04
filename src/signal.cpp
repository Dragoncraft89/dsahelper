#include <QObject>
#include <iostream>

extern "C" void rust_free(void *arg);

class SignalHandler : QObject {
  Q_OBJECT
  
  void (*handler)(void *, void *);
  void *arg;
  void *argument;

private slots:
  void handle() {
    handler(arg, argument);
  }

public:
  SignalHandler(QObject *object, const char *signal, void *arg, void (*callback)(void *, void *), void *argument)
    :handler(callback),
     arg(arg),
     argument(argument)
  {
    connect(object, signal, SLOT(handle()));
  }

  ~SignalHandler() {
    rust_free(argument);
  }
};

extern "C" {
  SignalHandler *create(void *object, const char *signal, void *arg, void *callback, void *argument) {
    return new SignalHandler((QObject*) object, signal, arg, (void (*)(void *, void *)) callback, argument);
  }

  void destroy(void *handler) {
    delete ((SignalHandler*) handler);
  }
}

#include "signal.moc"
