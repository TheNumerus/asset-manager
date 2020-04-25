#include "mainwindow.h"

#include <QApplication>
#include <QSettings>

auto main(int argc, char *argv[]) -> int {
    QApplication a(argc, argv);
    QCoreApplication::setApplicationName("Copydog");
    QCoreApplication::setOrganizationName("Copydog");

    // load window position
    QSettings qs;
    auto pos = qs.value("pos").toPoint();
    auto size = qs.value("size").toSize();
    MainWindow w;
    w.move(pos);
    w.resize(size);
    w.show();
    return a.exec();
}
