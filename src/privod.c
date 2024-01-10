#include <sys/ioctl.h>
#include <fcntl.h>
#include <linux/cdrom.h>
#include <unistd.h>


// Status: 0 - opened, 1 - closed, 2 - failed

int get_status(char *drive_path) {
    int cdrom;
    int status=1;

    if ((cdrom = open(drive_path,O_RDONLY | O_NONBLOCK)) < 0) {
        status=2;
    }

    if (ioctl(cdrom,CDROM_DRIVE_STATUS) == CDS_TRAY_OPEN) {
        status=0;
    }

    close(cdrom);
    return status;
}

void eject(char *drive_path) {
    // Ignore everything
    int cdrom;
    cdrom = open(drive_path, O_RDONLY | O_NONBLOCK);
    ioctl (cdrom, CDROMEJECT);
}
