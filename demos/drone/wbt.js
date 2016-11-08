var drone;
var gatt;
var characteristic;

function findDrone() {
    try {
        console.log("findDrone");
        if (!drone) {
            var options = {
              filters: [
                  {
                      namePrefix: "Travis",
                  }
              ],
              optionalServices: [
                  "9a66fa00-0800-9191-11e4-012d1540cb8e",
                  "9a66fb00-0800-9191-11e4-012d1540cb8e",
              ]
            };
            window.navigator.bluetooth.requestDevice(options)
            .then(device => {
                drone = device;
                return drone.gatt.connect()
                .then(_ => {
                    console.log("connected");
                    return setup()
                    .then(_ => console.log("setup done"))
                    .catch(err => console.log(err));
                })
                .catch(err => console.log(err));
            })
            .catch(err => console.log(err));
        } else {
            return drone.gatt.connect()
            .catch(err => console.log(err));
        }
    } catch(err) {
        console.log(err);
        alert(err);
    }
}

function setup() {
    drone.gatt.getPrimaryService("9a66fb00-0800-9191-11e4-012d1540cb8e")
    .then(service => {
        service.getCharacteristic("9a66fb0f-0800-9191-11e4-012d1540cb8e")
        .then(c => c.startNotifications())
        .then(c => console.log("fb0f notification enabled"))
        .catch(err => console.log(err));
        service.getCharacteristic("9a66fb0e-0800-9191-11e4-012d1540cb8e")
        .then(c => c.startNotifications())
        .then(c => console.log("fb0e notification enabled"))
        .catch(err => console.log(err));
        service.getCharacteristic("9a66fb1b-0800-9191-11e4-012d1540cb8e")
        .then(c => c.startNotifications())
        .then(c => console.log("fb1b notification enabled"))
        .catch(err => console.log(err));
        service.getCharacteristic("9a66fb1c-0800-9191-11e4-012d1540cb8e")
        .then(c => c.startNotifications())
        .then(c => console.log("fb1c notification enabled"))
        .catch(err => console.log(err));
    })
    .catch(err => console.log(err));
    return drone.gatt.getPrimaryService("9a66fa00-0800-9191-11e4-012d1540cb8e")
    .then(service => {
        console.log('Getting Service...');
        console.log('> UUID:       ' + service.uuid);
        console.log('> Is primary: ' + service.isPrimary);
        console.log('')
        console.log('Getting Characteristic...');
        service.getCharacteristic("9a66fa0b-0800-9191-11e4-012d1540cb8e")
        .then(char => {
            characteristic = char;
            console.log('Characteristic found!');
            console.log('> Characteristic service: ' + characteristic.service.uuid);
            console.log('> Characteristic UUID:    ' + characteristic.uuid);
            console.log('> Broadcast:              ' + characteristic.properties.broadcast);
            console.log('> Read:                   ' + characteristic.properties.read);
            console.log('> Write w/o response:     ' + characteristic.properties.writeWithoutResponse);
            console.log('> Write:                  ' + characteristic.properties.write);
            console.log('> Notify:                 ' + characteristic.properties.notify);
            console.log('> Indicate:               ' + characteristic.properties.indicate);
            console.log('> Signed Write:           ' + characteristic.properties.authenticatedSignedWrites);
            console.log('> Queued Write:           ' + characteristic.properties.reliableWrite);
            console.log('> Writable Auxiliaries:   ' + characteristic.properties.writableAuxiliaries);
        })
        .catch(err => console.log(err));
    })
    .catch(err => console.log(err));
}

function writeCharacteristic(value) {
    console.log("writeCharacteristic: "+value);
    if(!characteristic) {
        console.log('No characteristic!');
        return;
    }
    characteristic.writeValue(value)
    .then(() => console.log("changed value to: "+value))
    .catch(err => console.log(err));
}
