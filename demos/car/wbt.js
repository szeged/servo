var car;
var gatt;
var characteristic;

var car_service = 0x181a;
var car_char = 0x2a23;

function findCar() {
    try {
        console.log("findCar");
        if (!car) {
            var options = {
              filters: [
                  {
                      namePrefix: "SED",
                  }
              ],
              optionalServices: [
                  car_service,
              ]
            };
            window.navigator.bluetooth.requestDevice(options)
            .then(device => {
                car = device;
                return car.gatt.connect()
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
            return car.gatt.connect()
            .catch(err => console.log(err));
        }
    } catch(err) {
        console.log(err);
        alert(err);
    }
}

function setup() {
    return car.gatt.getPrimaryService(car_service)
    .then(service => {
        console.log('Getting Service...');
        console.log('> UUID:       ' + service.uuid);
        console.log('> Is primary: ' + service.isPrimary);
        console.log('')
        console.log('Getting Characteristic...');
        service.getCharacteristic(car_char)
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
