var device;
var gatt;
var service;
var characteristic;

var color_service_uuid        = "f815e810-456c-6761-746f-4d756e696368";
var color_characteristic_uuid = "f815e811-456c-6761-746f-4d756e696368";

function findLamp() {
    try {
        console.log("findLamp");
        var options = {filters: [{namePrefix: "Avea", services: [color_service_uuid]}], optinalServices: []};
        device = window.navigator.bluetooth.requestDevice(options);
        gatt = device.gatt.connect();
        service = gatt.getPrimaryService(color_service_uuid);
        console.log('Getting Service...');
        console.log('> UUID:       ' + service.uuid);
        console.log('> Is primary: ' + service.isPrimary);
        console.log('')
        console.log('Getting Characteristic...');
        characteristic = service.getCharacteristic(color_characteristic_uuid);
        console.log('> Characteristic UUID:    ' + characteristic.uuid);
    } catch(err) {
        console.log(err);
        alert(err);
    }
}

function writeCharacteristic(value) {
    if(!characteristic) {
        console.log('No characteristic!');
        return;
    }
    try {
        characteristic.writeValue(value);
    } catch(err) {
        console.log(err);
        alert(err);
    }
}
