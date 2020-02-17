// DO NOT EDIT! This test has been generated by /offscreen-canvas/tools/gentest.py.
// OffscreenCanvas test in a worker:2d.path.arcTo.collinear.3
// Description:arcTo() with all points on a line, and P0 between P1/P2, draws a straight line to P1
// Note:

importScripts("/resources/testharness.js");
importScripts("/2dcontext/resources/canvas-tests.js");

var t = async_test("arcTo() with all points on a line, and P0 between P1/P2, draws a straight line to P1");
var t_pass = t.done.bind(t);
var t_fail = t.step_func(function(reason) {
    throw reason;
});
t.step(function() {

var offscreenCanvas = new OffscreenCanvas(100, 50);
var ctx = offscreenCanvas.getContext('2d');

ctx.fillStyle = '#f00';
ctx.fillRect(0, 0, 100, 50);
ctx.lineWidth = 50;
ctx.strokeStyle = '#0f0';
ctx.beginPath();
ctx.moveTo(0, 25);
ctx.arcTo(100, 25, -100, 25, 1);
ctx.stroke();
ctx.strokeStyle = '#f00';
ctx.beginPath();
ctx.moveTo(100, 25);
ctx.arcTo(200, 25, 0, 25, 1);
ctx.stroke();
ctx.beginPath();
ctx.moveTo(-100, 25);
ctx.arcTo(0, 25, -200, 25, 1);
ctx.stroke();
_assertPixel(offscreenCanvas, 50,25, 0,255,0,255, "50,25", "0,255,0,255");
t.done();

});
done();
