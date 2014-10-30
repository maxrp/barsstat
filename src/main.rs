extern crate libc;
extern crate time;

use std::io::File;
use std::io::timer::sleep;
use std::time::duration::Duration;
use time::{Tm,now,now_utc};

#[link(name = "current_desktop")]
#[link(name = "X11")]
extern {
    fn cur_desk(display: libc::c_int) -> libc::c_int;
}

fn current_desktop(display: i32) -> uint { 
    let desk: i32 = unsafe {
        cur_desk(display)
    };
    return match desk {
        127i32 => panic!("Cannot get current desktop."),
        _ => desk as uint,
    };
}

fn read_bat(path: &str) -> f32 {
    let bat_path = Path::new(path);
    let bat_path_display = bat_path.display();
    let mut file = match File::open(&bat_path) {
        Err(why) => panic!("Could not open {}: {}", bat_path_display, why.desc),
        Ok(file) => file,
    };
    let string = match file.read_to_string() {
        Err(why) => panic!("Could not read {}: {}", bat_path_display, why.desc),
        Ok(string) => string,
    };

    from_str(string.as_slice().trim()).unwrap()
}

fn format_date(time_struct: Tm) -> String {
    let day = match time_struct.tm_wday {
        0 => "Sun",
        1 => "Mon",
        2 => "Tue",
        3 => "Wed",
        4 => "Thu",
        5 => "Fri",
        6 => "Sat",
        _ => panic!("Not a day!")
    };
    let month = match time_struct.tm_mon {
        0 => "Jan",
        1 => "Feb",
        2 => "Mar",
        3 => "Apr",
        5 => "Jun",
        6 => "Jul",
        7 => "Aug",
        8 => "Sep",
        9 => "Oct",
       10 => "Nov",
       11 => "Dec",
        _ => panic!("Not a month!")
    };
    return format!("{:s} {:s} {:02d} {:02d}:{:02d}:{:02d}",
                   day,
                   month,
                   time_struct.tm_mday,
                   time_struct.tm_hour,
                   time_struct.tm_min,
                   time_struct.tm_sec);
}

fn format_desktops(desktop: uint) -> String {
    let desktops = [ " One ", " Two ", " Three ", " Four ", " Five " ];

    let mut desktop_list = String::new();

    for i in range(0, desktops.len()) {
        if i == desktop {
            desktop_list.push_str(colorize(desktops[i], "F#ffb58900").as_slice());
        } else {
            desktop_list.push_str(desktops[i]);
        }
    }

    return desktop_list;
}

fn colorize(content: &str, color: &str) -> String {
    return format!("%{{{}}}{}%{{F-}}", color, content);
}

fn position(content: String, position: &str) -> String {
    return format!("%{{{}}}{}", position, content);
}

fn main(){
    loop {
        let now = format_date(now());
        let now_utc = format_date(now_utc());
        let times = format!("{}  {}", now, now_utc);
        let bat0_now = read_bat("/sys/class/power_supply/BAT0/energy_now");
        let bat0_full = read_bat("/sys/class/power_supply/BAT0/energy_full");
        let bat0_perc = (bat0_now / bat0_full) * 100f32;
        let battery_status = format!("{}%", bat0_perc.trunc());
        
        println!("{} {} {}",
                format_desktops(current_desktop(0)),
                position(times, "c"),
                position(battery_status, "r"));

        sleep(Duration::seconds(1));
    }
}
