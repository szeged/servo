// byte values  |some magic numbers |white 8 | red 3 |green 2| blue 1|
// hex values   | 35  f4  01  0a 00 | ww  8w | rr 3r | gg 2g | bb 1b |

var preset_off     = [53,244,1,10,0,  0,128,  0,48,  0,32,  0,16];
var preset_white   = [53,244,1,10,0,255,143,  0,48,  0,32,  0,16];
var preset_white2  = [53,244,1,10,0,255,143,255,63,255,47,255,31];
var preset_red     = [53,244,1,10,0,  0,128,255,63,  0,32,  0,16];
var preset_green   = [53,244,1,10,0,  0,128,  0,48,255,47,  0,16];
var preset_blue    = [53,244,1,10,0,  0,128,  0,48,  0,32,255,31];
var preset_yellow  = [53,244,1,10,0,  0,128,255,63,255,47,  0,16];
var preset_cyan    = [53,244,1,10,0,  0,128,  0,48,255,47,255,31];
var preset_magenta = [53,244,1,10,0,  0,128,255,63,  0,32,255,31];

function setValue(value) {
    writeCharacteristic(value);
}

function setRed() {
    setValue(preset_red);
}

function setGreen() {
    setValue(preset_green);
}

function setBlue() {
    setValue(preset_blue);
}

function setYellow() {
    setValue(preset_yellow);
}

function setCyan() {
    setValue(preset_cyan);
}

function setMagenta() {
    setValue(preset_magenta);
}

function setWhite() {
    setValue(preset_white);
}

function setWhite2() {
    setValue(preset_white2);
}

function setOff() {
    setValue(preset_off);
}
