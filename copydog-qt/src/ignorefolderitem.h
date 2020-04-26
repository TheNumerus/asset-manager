#ifndef IGNOREFOLDERITEM_H
#define IGNOREFOLDERITEM_H

#include <QWidget>

namespace Ui {
class IgnoreFolderItem;
}

class IgnoreFolderItem : public QWidget
{
    Q_OBJECT

public:
    explicit IgnoreFolderItem(QWidget *parent = nullptr);
    IgnoreFolderItem(QWidget *parent, const QString& path);
    ~IgnoreFolderItem();

private:
    Ui::IgnoreFolderItem *ui;
};

#endif // IGNOREFOLDERITEM_H
