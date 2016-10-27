var board;
var rotated = false;
var bt_handle;
var log_box;


function Board(canvas_dom_elem, bt_callback) {
	this.canvas = canvas_dom_elem;
	this.bt_callback = bt_callback;
	this.cell_dim_px = 40;

	this.reset_mx();

	this.ctx = this.canvas.getContext("2d");

	var self = this;
	this.canvas.addEventListener('click', function(event){
		self.on_click(event);
	}, false);

	this.redraw();
}

Board.prototype.on_click = function(event) {
	var x;
	var y;
	if (event.x != undefined && event.y != undefined) {
		x = event.x;
		y = event.y;
	}
	else { // Firefox method to get the position
		x = event.clientX
			+ (document.body.scrollLeft || 0)
			+ (document.documentElement.scrollLeft || 0);
		y = event.clientY
			+ (document.body.scrollTop || 0)
			+ (document.documentElement.scrollTop || 0);
	}
	x -= this.canvas.offsetLeft || 0;
	y -= this.canvas.offsetTop || 0;
	var row = Math.floor(y / this.cell_dim_px);
	var col = Math.floor(x / this.cell_dim_px);
	if (rotated) {
		var tmp = col;
		col = row;
		row = 7 - tmp;
	}

	this.matrix[row][col] = !this.matrix[row][col];
	this.redraw();
	this.bt_callback()
}

Board.prototype.reset_mx = function() {
	this.matrix = new Array(8);
	for (var row = 0; row < 8; row++) {
		this.matrix[row] = new Array(16);
		for (var col = 0; col < 16; col++)
			this.matrix[row][col] = false;
	}
}

Board.prototype.redraw = function() {
	this.ctx.shadowColor = '#f00';
	this.ctx.fillStyle = "#000000";
	this.ctx.fillRect(0, 0, this.canvas.width, this.canvas.height);

	var cell_offset = 3;
	var cell_radius = (this.cell_dim_px - cell_offset * 2) / 2;
	for (var row = 0; row < 8; row++) {
		for (var col = 0; col < 16; col++) {
			if (this.matrix[row][col]) {
				this.ctx.fillStyle = "#ff0000";
				this.ctx.shadowBlur = 20;
			}
			else {
				this.ctx.fillStyle = "#ffefd5";
				this.ctx.shadowBlur = 0;
			}

			this.ctx.beginPath();
			this.ctx.arc(
				cell_offset + col * this.cell_dim_px + cell_radius,
				cell_offset + row * this.cell_dim_px + cell_radius,
				cell_radius,
				0, 2 * Math.PI);
			this.ctx.fill();
		}
	}
}

Board.prototype.reset = function() {
	this.reset_mx();
	this.redraw();
	this.bt_callback();

}

Board.prototype.fill_mx = function(bt_arr) {
	this.reset_mx();

	var idx = 0;

	for (var row = 0; row < 8; row++) {
		var val = bt_arr[idx].charCodeAt(0);
		for (var col = 0; col < 8; col++)
			this.matrix[row][col] = ((val & 1 << col) > 0);

		idx++;

		val = bt_arr[idx].charCodeAt(0);
		for (var col = 0; col < 8; col++)
			this.matrix[row][col + 8] = ((val & 1 << col) > 0);

		idx++;
	}

	this.redraw();
}

Board.prototype.flip = function() {
	for (var row = 0; row < 4; row++) {
		var tmp_row = this.matrix[row];
		this.matrix[row] = this.matrix[7 - row];
		this.matrix[7 - row] = tmp_row;
	}

	this.redraw();
	this.bt_callback();
}


window.onload = function() {
	log_box = document.getElementById("log");

	if (!window.navigator.bluetooth)
		log("Warning: The WebBluetooth API is not available in this browser!");

	board = new Board(document.getElementById("draw"), calc_string_and_send);
	calc_string_and_send();
}

function log(string) {
	console.log(string);
	log_box.value += string + "\n";
	log_box.scrollTop = log_box.scrollHeight;
}

// Called from HTML
function reset() {
	board.reset();
}

// Called from HTML
function rotate() {
	var canvas = document.getElementById('draw').classList.toggle('rotated');
	var canvas_container = document.getElementById('draw-container').classList.toggle('rotated');
	rotated = !rotated;
}

// Called from HTML
function flip() {
	board.flip();
}


function calc_string_and_send() {
	var result_array = [];
	for (var row = 0; row < 8; row++) {
		var val = 0;
		for (var col = 0; col < 8; col++)
			val += (board.matrix[row][col] ? 1 : 0) * Math.pow(2, col);

		result_array.push(val);

		val = 0;
		for (var col = 8; col < 16; col++)
			val += (board.matrix[row][col] ? 1 : 0) * Math.pow(2, col - 8);

		result_array.push(val);
	}

	var textfield = document.getElementById("result_field");
	textfield.value = result_array.toString();

	bt_send(result_array);
}


function bt_connect() {
	var serviceUuid = document.getElementById('service').value;
	if (serviceUuid.startsWith('0x'))
		serviceUuid = parseInt(serviceUuid, 16);

	var characteristicUuid = document.getElementById('characteristic').value;
	if (characteristicUuid.startsWith('0x'))
		characteristicUuid = parseInt(characteristicUuid, 16);


	try {
		log_box.value = "";
		if (!window.navigator.bluetooth)
			throw 'The WebBluetooth API is not available in this browser!';

		window.navigator.bluetooth.requestDevice({
			filters: [{
				services: [serviceUuid]
			}]
		})
		.then(device => device.gatt.connect())
		.then(server => server.getPrimaryService(serviceUuid))
		.then(primaryService => primaryService.getCharacteristic(characteristicUuid))
		.then(characteristic => {
			bt_handle = characteristic;
			characteristic.readValue()
			.then(read_arr => board.fill_mx(read_arr));
		});
	}
	catch(err) {
		log("Error: " + err);
	}
}

function bt_send(arr) {
	if (bt_handle)
		bt_handle.writeValue(arr);
}
