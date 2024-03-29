import { Canvas} from './index.js'
 
// console.log('From native', sum(40, 2))

let canvas = new Canvas(2560, 1280);
canvas.scale(1.2, 1.2);
canvas.move_to(36.0, 48.0);
canvas.quad_to(660.0, 880.0, 1200.0, 360.0);
canvas.translate(10.0, 10.0);
canvas.set_line_width(20.0);
canvas.stroke();
canvas.save();
canvas.move_to(30.0, 90.0);
canvas.line_to(110.0, 20.0);
canvas.line_to(240.0, 130.0);
canvas.line_to(60.0, 130.0);
canvas.line_to(190.0, 20.0);
canvas.line_to(270.0, 90.0);
canvas.fill();
canvas.save();
canvas.saveTo("test.png");

// let d = canvas.data();
// let file = Filecreate("test.png").unwrap();
// let bytes = d.as_bytes();
// file.write_all(bytes).unwrap();