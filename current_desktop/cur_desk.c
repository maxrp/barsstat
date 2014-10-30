#include <X11/Xlib.h>

int cur_desk(char *display) {
    Display *dsp;
    Atom actual_type;
    int fmt;
    long unsigned bytes, nitems;
    unsigned char *data;
    
    if (!(dsp = XOpenDisplay(display))) return 127;

    if (XGetWindowProperty(dsp,
        XDefaultRootWindow(dsp),
        XInternAtom(dsp, "_NET_CURRENT_DESKTOP", False),
        0,
        ~0,
        False,
        AnyPropertyType,
        &actual_type,
        &fmt,
        &nitems,
        &bytes,
        &data) != Success) {
            XSync(dsp, False);
            XCloseDisplay(dsp);
            return 127;
    }

    int current_desktop = *(int *) data;

    XFree(data);
    XSync(dsp, False);
    XCloseDisplay(dsp);
    return current_desktop;
}
