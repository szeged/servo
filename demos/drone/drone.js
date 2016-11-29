var counter = 0;
var pingInProgress = false;
var pingVar = 0;

var ctrl = {
    forward: 0,
    backward: 0,
    left: 0,
    right: 0,
    ascend: 0,
    descend: 0,
    turnLeft: 0,
    turnRight: 0,
}

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
    ctrl.forward = 0;
    ctrl.backward = 0;
    ctrl.left = 0;
    ctrl.right = 0;
    ctrl.ascend = 0;
    ctrl.descend = 0;
    ctrl.turnLeft = 0;
    ctrl.turnRight = 0;
}

function setColor(target, value) {
    $(target).css("color", value?"#c33":"#FFD700");
}

function ascend(value) {
    if (!ctrl['descend']) {
        ctrl['ascend'] = value;
        setColor("#ascend", value);
    }
}

function descend(value) {
    if (!ctrl['ascend']) {
        ctrl['descend'] = value;
        setColor("#descend", value);
    }
}

function turnRight(value) {
    if (!ctrl['turnLeft']) {
        ctrl['turnRight'] = value;
        setColor("#turnRight", value);
    }
}

function turnLeft(value) {
    if (!ctrl['turnRight']) {
        ctrl['turnLeft'] = value;
        setColor("#turnLeft", value);
    }
}

function goForward(value) {
    if (!ctrl['backward']) {
        ctrl['forward'] = value;
        setColor("#goForward", value);
    }
}

function goBackward(value) {
    if (!ctrl['forward']) {
        ctrl['backward'] = value;
        setColor("#goBackward", value);
    }
}

function goToRight(value) {
    if (!ctrl['left']) {
        ctrl['right'] = value;
        setColor("#goToRight", value);
    }
}

function goToLeft(value) {
    if (!ctrl['right']) {
        ctrl['left'] = value;
        setColor("#goToLeft", value);
    }
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

function setSpeed(prop, v1, v2) {
    speed[prop] = ctrl[v1] ? 50 : (ctrl[v2] ? 200 : 0);
}

function updateSpeed() {
    setSpeed('roll', 'right', 'left');
    setSpeed('pitch', 'forward', 'backward');
    setSpeed('yaw', 'turnRight', 'turnLeft');
    setSpeed('altitude', 'ascend', 'descend');
    writeCharacteristic([2, counter++, 2, 0, 2, 0, (speed.pitch || speed.roll) ? 1 : 0, speed.roll, speed.pitch, speed.yaw, speed.altitude, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}

function handleKey(key, value) {
    console.log(key, value);
    switch(key)
    {
        case 87: //W
            ascend(value);
            break;
        case 65: //A
            turnLeft(value);
            break;
        case 83: //S
            descend(value);
            break;
        case 68: //D
            turnRight(value);
            break;
        case 38: //UPArrow
            goForward(value);
            break;
        case 40: //DOWNArrow
            goBackward(value);
            break;
        case 39: //RIGHTArrow
            goToRight(value);
            break;
        case 37: //LEFTArrow
            goToLeft(value);
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

$(document).ready(function() {
    window.onkeyup = function(e) {
        handleKey(e.keyCode, 0);
    }
    window.onkeydown = function(e) {
        handleKey(e.keyCode, 1);
    }
    $("#ascend").mousedown(function() {
        ascend(1);
    });
    $("#ascend").mouseup(function() {
        ascend(0);
    });
    $("#descend").mousedown(function() {
        descend(1);
    });
    $("#descend").mouseup(function() {
        descend(0);
    });
    $("#turnRight").mousedown(function() {
        turnRight(1);
    });
    $("#turnRight").mouseup(function() {
        turnRight(0);
    });
    $("#turnLeft").mousedown(function() {
        turnLeft(1);
    });
    $("#turnLeft").mouseup(function() {
        turnLeft(0);
    });
    $("#goForward").mousedown(function() {
        goForward(1);
    });
    $("#goForward").mouseup(function() {
        goForward(0);
    });
    $("#goBackward").mousedown(function() {
        goBackward(1);
    });
    $("#goBackward").mouseup(function() {
        goBackward(0);
    });
    $("#goToLeft").mousedown(function() {
        goToLeft(1);
    });
    $("#goToLeft").mouseup(function() {
        goToLeft(0);
    });
    $("#goToRight").mousedown(function() {
        goToRight(1);
    });
    $("#goToRight").mouseup(function() {
        goToRight(0);
    });
})
