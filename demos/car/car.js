var pingInProgress = false;
var pingVar = 0;
var last = 0;
var speed = {
    forward: 0,
    backward: 0,
    left: 0,
    right: 0,
}

function connect() {
    findCar();
}

function stop() {
    speed.forward = 0;
    speed.backward = 0;
    speed.left = 0;
    speed.right = 0;
}

function setSpeed(prop, value) {
    speed[prop] = value;
}

function setColor(target, value) {
    $(target).css("color", value?"#c33":"#FFD700");
}

function goForward(value) {
    if (!speed['backward']) {
        speed['forward'] = value;
        setColor("#goForward", value);
    }
}

function goBackward(value) {
    if (!speed['forward']) {
        speed['backward'] = value;
        setColor("#goBackward", value);
    }
}

function goToRight(value) {
    if (!speed['left']) {
        speed['right'] = value;
        setColor("#goToRight", value);
    }
}

function goToLeft(value) {
    if (!speed['right']) {
        speed['left'] = value;
        setColor("#goToLeft", value);
    }
}

function toggleOnOff() {
    if(!pingInProgress) {
        pingVar = setInterval(updateSpeed, 250);
        pingInProgress = true;
    } else {
        clearInterval(pingVar);
        pingVar = 0;
        pingInProgress = false;
    }

    if(pingVar)
      document.getElementById("onoff").style.background = "linear-gradient(#44aa44, #db0)";
    else
      document.getElementById("onoff").style.background = null;
}

function updateSpeed() {
    var command = speed.forward | (speed.backward << 1)  | (speed.left << 2) | (speed.right << 3);
    if (last != command) {
        writeCharacteristic([command]);
    }
    last = command;
}

function handleKey(key, value) {
    switch(key)
    {
        case 38: //UPArrow
        case 87: //W
            goForward(value);
            break;
        case 40: //DOWNArrow
        case 83: //S
            goBackward(value);
            break;
        case 39: //RIGHTArrow
        case 68: //D
            goToRight(value);
            break;
        case 37: //LEFTArrow
        case 65: //A
            goToLeft(value);
            break;
        case 32: //SPACE
            stop();
            break;
        case 13: //ENTER
            toggleOnOff();
            break;
        case 17: //CTRL
            break;
        case 16: //SHIFT
            break;
        default:
            console.log(key);
            break;
    }
}

$(document).ready(function() {
    window.onkeyup = function(e) {
        if (e.keyCode != 13)
            handleKey(e.keyCode, 0);
    }
    window.onkeydown = function(e) {
        handleKey(e.keyCode, 1);
    }
    $("#goForward").mousedown(function() {
        goForward(1);
    });
    $("#goForward").mouseup(function() {
        goForward(0);
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
    $("#goBackward").mousedown(function() {
        goBackward(1);
    });
    $("#goBackward").mouseup(function() {
        goBackward(0);
    });
})
