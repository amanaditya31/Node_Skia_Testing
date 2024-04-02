import { SkiaCanvas } from './index.js'
 
// console.log('From native', sum(40, 2))

function draw (){
    let canvas = new SkiaCanvas(2560, 1280);
    canvas.scale(1.2, 1.2);
    canvas.moveTo(36.0, 48.0);
    canvas.quadTo(660.0, 880.0, 1200.0, 360.0);
    canvas.translate(10.0, 10.0);
    canvas.setLineWidth(20.0);
    canvas.stroke();
    canvas.save();
    canvas.moveTo(30.0, 90.0);
    canvas.lineTo(110.0, 20.0);
    canvas.lineTo(240.0, 130.0);
    canvas.lineTo(60.0, 130.0);
    canvas.lineTo(190.0, 20.0);
    canvas.lineTo(270.0, 90.0);
    canvas.fill();
    canvas.save();
    canvas.saveTo("test.png");
}

draw();
