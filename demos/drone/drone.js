var counter = 0;
var pingInProgress = false;
var pingVar = 0;

var move = 0;
var speed = {
    roll: 0,
    pitch: 0,
    yaw: 0,
    altitude: 0,
}

function connect() {
    findDrone();
}

function takeOff() {
    stopPing();
    writeCharacteristic([0x04, counter++, 0x02, 0x00, 0x01, 0x00]);
    ping();
}

function land() {
    stopPing();
    writeCharacteristic([0x04, counter++, 0x02, 0x00, 0x03, 0x00]);
}

function flip() {
    writeCharacteristic([0x02, counter++, 0x02, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
}

function hover() {
    speed.roll = 0;
    speed.pitch = 0;
    speed.yaw = 0;
    speed.altitude = 0;
}

function setSpeed(prop, value) {
    move = 1;
    speed[prop] = value;
}


function ascend() {
    setSpeed('altitude', 50);
}

function descend() {
    setSpeed('altitude', 200);
}

function turnRight() {
    setSpeed('yaw', 50);
}

function turnLeft() {
    setSpeed('yaw', 200);
}

function goForward() {
    setSpeed('pitch', 50);
}

function goBackward() {
    setSpeed('pitch', 200);
}

function goToRight() {
    setSpeed('roll', 50);
}

function goToLeft() {
    setSpeed('roll', 200);
}

function ping() {
    if(!pingInProgress)
    {
        pingVar = setInterval(updateSpeed, 250);
        pingInProgress = true;
    }
}

function stopPing() {
    clearInterval(pingVar);
    pingInProgress = false;
}

function updateSpeed() {
    console.log("updateSpeed");
    writeCharacteristic([2, counter++, 2, 0, 2, 0, move-- ? 1:0, speed.roll, speed.pitch, speed.yaw, speed.altitude, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
    if (!move) {
        hover();
    }
}

$(document).ready(function(){
    window.onkeyup = function(e)
    {
        switch(e.keyCode)
        {
            case 87: //W
                ascend();
                break;
            case 65: //A
                turnLeft();
                break;
            case 83: //S
                descend();
                break;
            case 68: //D
                turnRight();
                break;
            case 38: //UPArrow
                goForward();
                break;
            case 40: //DOWNArrow
                goBackward();
                break;
            case 39: //RIGHTArrow
                goToRight();
                break;
            case 37: //LEFTArrow
                goToLeft();
                break;
            case 32: //SPACE
                hover();
                break;
            case 13: //ENTER
                land();
                break;
            case 17: //CTRL
                break;
            case 16: //SHIFT
                break;
            default:
                console.log(e.keyCode);
                break;
        }
    }
})
